use anyhow::{Context, Result};
use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    http::{header::AUTHORIZATION, HeaderMap, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};
use futures_util::{SinkExt, StreamExt};
use portable_pty::{native_pty_system, CommandBuilder, MasterPty, PtySize};
use portable_pty::Child as _;
use std::{
    env,
    io::{Read, Write},
    net::SocketAddr,
    path::Path,
    sync::Arc,
    thread,
};
use tokio::sync::mpsc;

#[derive(Clone)]
struct ServerState {
    token: Option<String>,
    shell: String,
}

struct ShellPty {
    master: Box<dyn MasterPty>,
    writer: Box<dyn Write + Send>,
    child: Box<dyn portable_pty::Child + Send>,
    _reader_thread: thread::JoinHandle<()>,
}

impl ShellPty {
    fn send(&mut self, data: &[u8]) -> Result<()> {
        self.writer.write_all(data).context("write to pty")?;
        self.writer.flush().context("flush pty writer")?;
        Ok(())
    }

    fn resize(&mut self, cols: u16, rows: u16) {
        let _ = self.master.resize(PtySize {
            rows,
            cols,
            pixel_width: 0,
            pixel_height: 0,
        });
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let bind = env::var("ILONHRO_BIND").unwrap_or_else(|_| "0.0.0.0:7070".to_string());
    let token = env::var("ILONHRO_TOKEN").ok();
    let shell = resolve_shell();

    if token.is_none() {
        eprintln!("Warning: ILONHRO_TOKEN is not set. Server allows all connections.");
    }

    let state = Arc::new(ServerState { token, shell });
    let app = Router::new()
        .route("/ws", get(ws_handler))
        .with_state(state);

    let addr: SocketAddr = bind.parse().context("parse ILONHRO_BIND")?;
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .context("bind server")?;

    println!("iLonhro server listening on ws://{addr}/ws");
    axum::serve(listener, app).await.context("serve")?;
    Ok(())
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    headers: HeaderMap,
    State(state): State<Arc<ServerState>>,
) -> impl IntoResponse {
    if !authorized(&headers, state.token.as_deref()) {
        return StatusCode::UNAUTHORIZED;
    }

    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

fn authorized(headers: &HeaderMap, token: Option<&str>) -> bool {
    let Some(expected) = token else {
        return true;
    };
    let Some(value) = headers.get(AUTHORIZATION).and_then(|h| h.to_str().ok()) else {
        return false;
    };
    if let Some(bearer) = value.strip_prefix("Bearer ") {
        bearer == expected
    } else {
        value == expected
    }
}

async fn handle_socket(socket: WebSocket, state: Arc<ServerState>) {
    let (mut pty, mut output_rx) = match spawn_shell(&state.shell) {
        Ok(value) => value,
        Err(err) => {
            let _ = send_error(socket, format!("Failed to start shell: {err}")).await;
            return;
        }
    };

    let (mut ws_tx, mut ws_rx) = socket.split();
    let (outgoing_tx, mut outgoing_rx) = mpsc::unbounded_channel::<Message>();
    let output_tx = outgoing_tx.clone();

    let writer_task = tokio::spawn(async move {
        while let Some(message) = outgoing_rx.recv().await {
            if ws_tx.send(message).await.is_err() {
                break;
            }
        }
    });

    let output_task = tokio::spawn(async move {
        while let Some(bytes) = output_rx.recv().await {
            if output_tx.send(Message::Binary(bytes)).is_err() {
                break;
            }
        }
    });

    while let Some(result) = ws_rx.next().await {
        let message = match result {
            Ok(message) => message,
            Err(_) => break,
        };

        match message {
            Message::Text(text) => {
                if handle_control_message(&text, &mut pty) {
                    continue;
                }
                let _ = pty.send(text.as_bytes());
            }
            Message::Binary(data) => {
                let _ = pty.send(&data);
            }
            Message::Close(_) => break,
            Message::Ping(payload) => {
                let _ = outgoing_tx.send(Message::Pong(payload));
            }
            Message::Pong(_) => {}
        }
    }

    let _ = pty.child.kill();
    let _ = pty.child.wait();
    output_task.abort();
    writer_task.abort();
}

async fn send_error(mut socket: WebSocket, message: String) -> Result<()> {
    socket
        .send(Message::Text(message))
        .await
        .context("send websocket error")
}

fn handle_control_message(text: &str, pty: &mut ShellPty) -> bool {
    let trimmed = text.trim();
    if let Some(rest) = trimmed.strip_prefix("__RESIZE__") {
        let mut parts = rest.trim().split_whitespace();
        let cols = parts.next().and_then(|value| value.parse::<u16>().ok());
        let rows = parts.next().and_then(|value| value.parse::<u16>().ok());
        if let (Some(cols), Some(rows)) = (cols, rows) {
            pty.resize(cols, rows);
        }
        return true;
    }
    false
}

fn spawn_shell(shell: &str) -> Result<(ShellPty, mpsc::UnboundedReceiver<Vec<u8>>)> {
    let pty_system = native_pty_system();
    let pair = pty_system
        .openpty(PtySize {
            rows: 24,
            cols: 80,
            pixel_width: 0,
            pixel_height: 0,
        })
        .context("open pty")?;

    let mut cmd = CommandBuilder::new(shell);
    cmd.args(&["-i"]);
    let child = pair
        .slave
        .spawn_command(cmd)
        .context("spawn shell")?;

    let mut reader = pair.master.try_clone_reader().context("pty reader")?;
    let writer = pair.master.take_writer().context("pty writer")?;
    let (tx, rx) = mpsc::unbounded_channel::<Vec<u8>>();

    let reader_thread = thread::spawn(move || {
        let mut buf = [0_u8; 4096];
        loop {
            match reader.read(&mut buf) {
                Ok(0) => break,
                Ok(n) => {
                    if tx.send(buf[..n].to_vec()).is_err() {
                        break;
                    }
                }
                Err(_) => break,
            }
        }
    });

    Ok((
        ShellPty {
            master: pair.master,
            writer,
            child,
            _reader_thread: reader_thread,
        },
        rx,
    ))
}

fn resolve_shell() -> String {
    if let Ok(shell) = env::var("ILONHRO_SHELL") {
        return shell;
    }
    let candidates = [
        "/bin/bash",
        "/usr/bin/bash",
        "/bin/sh",
        "/usr/bin/sh",
    ];
    for path in candidates {
        if Path::new(path).exists() {
            return path.to_string();
        }
    }
    "bash".to_string()
}
