use anyhow::{Context, Result};
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use portable_pty::{native_pty_system, CommandBuilder, MasterPty, PtySize};
use portable_pty::Child as _;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Terminal,
};
use std::{
    io::{self, Read, Write},
    path::Path,
    sync::mpsc,
    thread,
    time::Duration,
};

const DEFAULT_MAX_LINES: usize = 2000;
const TICK_RATE: Duration = Duration::from_millis(16);

#[derive(Clone, Copy)]
struct Theme {
    background: Color,
    foreground: Color,
    accent: Color,
    accent_alt: Color,
}

impl Theme {
    fn pink_n_black() -> Self {
        Self {
            background: Color::Rgb(8, 8, 12),
            foreground: Color::Rgb(230, 230, 240),
            accent: Color::Rgb(255, 0, 128),
            accent_alt: Color::Rgb(0, 255, 204),
        }
    }
}

struct OutputBuffer {
    lines: Vec<String>,
    carry: String,
    max_lines: usize,
}

impl OutputBuffer {
    fn new(max_lines: usize) -> Self {
        Self {
            lines: Vec::with_capacity(max_lines),
            carry: String::new(),
            max_lines,
        }
    }

    fn push_line(&mut self, line: impl Into<String>) {
        self.lines.push(line.into());
        self.trim();
    }

    fn push_bytes(&mut self, bytes: &[u8]) {
        let input = String::from_utf8_lossy(bytes);
        let cleaned = strip_ansi(&input);
        for ch in cleaned.chars() {
            match ch {
                '\n' | '\r' => {
                    let line = std::mem::take(&mut self.carry);
                    self.lines.push(line);
                    self.trim();
                }
                '\u{08}' => {
                    self.carry.pop();
                }
                _ if ch.is_control() => {}
                _ => self.carry.push(ch),
            }
        }
    }

    fn render_text(&self, height: usize) -> String {
        let mut lines = self.lines.clone();
        if !self.carry.is_empty() {
            lines.push(self.carry.clone());
        }
        let start = lines.len().saturating_sub(height);
        lines[start..].join("\n")
    }

    fn trim(&mut self) {
        if self.lines.len() > self.max_lines {
            let overflow = self.lines.len() - self.max_lines;
            self.lines.drain(0..overflow);
        }
    }
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

struct AppState {
    output: OutputBuffer,
    input: String,
    shell_name: String,
    shell_exited: bool,
}

impl AppState {
    fn new(shell_name: String) -> Self {
        let mut output = OutputBuffer::new(DEFAULT_MAX_LINES);
        output.push_line("iLonhro Terminal by Lonhro");
        output.push_line("Theme: Pink_n_Black (cyberpunk)");
        output.push_line("Ctrl+Q to quit. Ctrl+C sends SIGINT.");
        Self {
            output,
            input: String::new(),
            shell_name,
            shell_exited: false,
        }
    }
}

fn main() -> Result<()> {
    let mut terminal = setup_terminal()?;
    let result = run_app(&mut terminal);
    restore_terminal(&mut terminal)?;
    result
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
    let theme = Theme::pink_n_black();
    let (mut pty, rx, shell_name) = spawn_shell()?;
    let mut app = AppState::new(shell_name);
    let mut should_quit = false;
    let mut last_size = (0_u16, 0_u16);

    loop {
        loop {
            match rx.try_recv() {
                Ok(bytes) => app.output.push_bytes(&bytes),
                Err(mpsc::TryRecvError::Empty) => break,
                Err(mpsc::TryRecvError::Disconnected) => {
                    app.shell_exited = true;
                    break;
                }
            }
        }

        terminal.draw(|frame| {
            let size = frame.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(1), Constraint::Length(3)])
                .split(size);
            let pty_cols = chunks[0].width.saturating_sub(2);
            let pty_rows = chunks[0].height.saturating_sub(2);
            if pty_cols > 0 && pty_rows > 0 && (pty_cols, pty_rows) != last_size {
                last_size = (pty_cols, pty_rows);
                pty.resize(pty_cols, pty_rows);
            }

            let background = Block::default().style(Style::default().bg(theme.background));
            frame.render_widget(background, size);

            let output_height = chunks[0].height.saturating_sub(2) as usize;
            let output_text = app.output.render_text(output_height.max(1));
            let output_block = Block::default()
                .title(Line::from(vec![
                    Span::styled("iLonhro Session", Style::default().fg(theme.accent)),
                ]))
                .borders(Borders::ALL)
                .style(Style::default().fg(theme.accent).bg(theme.background));
            let output_widget = Paragraph::new(output_text)
                .block(output_block)
                .style(Style::default().fg(theme.foreground).bg(theme.background))
                .wrap(Wrap { trim: false });
            frame.render_widget(output_widget, chunks[0]);

            let max_input_width = chunks[1].width.saturating_sub(4) as usize;
            let display_input = tail_chars(&app.input, max_input_width);
            let input_line = format!("> {}", display_input);
            let mut title = format!(
                "iLonhro Terminal by Lonhro | Pink_n_Black | {} | Ctrl+Q Quit",
                app.shell_name
            );
            if app.shell_exited {
                title.push_str(" | Shell exited");
            }
            let input_block = Block::default()
                .title(Line::from(Span::styled(
                    title,
                    Style::default().fg(theme.accent_alt).add_modifier(Modifier::BOLD),
                )))
                .borders(Borders::ALL)
                .style(Style::default().fg(theme.accent_alt).bg(theme.background));
            let input_widget = Paragraph::new(input_line)
                .block(input_block)
                .style(Style::default().fg(theme.accent).bg(theme.background));
            frame.render_widget(input_widget, chunks[1]);

            let cursor_x = chunks[1].x + 3 + display_input.chars().count() as u16;
            let cursor_y = chunks[1].y + 1;
            frame.set_cursor(cursor_x, cursor_y);
        })?;

        if event::poll(TICK_RATE)? {
            match event::read()? {
                Event::Key(key) => handle_key(key, &mut app, &mut pty, &mut should_quit)?,
                Event::Paste(paste) => {
                    app.input.push_str(&paste);
                }
                _ => {}
            }
        }

        if should_quit {
            let _ = pty.send(b"exit\n");
            break;
        }
    }

    let _ = pty.child.kill();
    let _ = pty.child.wait();
    Ok(())
}

fn handle_key(
    key: KeyEvent,
    app: &mut AppState,
    pty: &mut ShellPty,
    should_quit: &mut bool,
) -> Result<()> {
    match (key.code, key.modifiers) {
        (KeyCode::Char('q'), KeyModifiers::CONTROL) => {
            *should_quit = true;
        }
        (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
            pty.send(&[0x03])?;
        }
        (KeyCode::Char('d'), KeyModifiers::CONTROL) => {
            pty.send(&[0x04])?;
        }
        (KeyCode::Enter, _) => {
            let mut line = app.input.clone();
            line.push('\n');
            pty.send(line.as_bytes())?;
            app.input.clear();
        }
        (KeyCode::Backspace, _) => {
            app.input.pop();
        }
        (KeyCode::Char(ch), KeyModifiers::NONE | KeyModifiers::SHIFT) => {
            app.input.push(ch);
        }
        _ => {}
    }
    Ok(())
}

fn spawn_shell() -> Result<(ShellPty, mpsc::Receiver<Vec<u8>>, String)> {
    let shell = resolve_shell();
    let shell_name = shell_name_from_path(&shell);
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
    let (tx, rx) = mpsc::channel::<Vec<u8>>();

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
        shell_name,
    ))
}

fn resolve_shell() -> String {
    if let Ok(shell) = std::env::var("ILONHRO_SHELL") {
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

fn shell_name_from_path(shell: &str) -> String {
    Path::new(shell)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("shell")
        .to_string()
}

fn strip_ansi(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut in_escape = false;
    for ch in input.chars() {
        if in_escape {
            if ('@'..='~').contains(&ch) {
                in_escape = false;
            }
            continue;
        }
        if ch == '\u{1b}' {
            in_escape = true;
            continue;
        }
        out.push(ch);
    }
    out
}

fn tail_chars(input: &str, max_chars: usize) -> String {
    if max_chars == 0 {
        return String::new();
    }
    let count = input.chars().count();
    if count <= max_chars {
        return input.to_string();
    }
    input
        .chars()
        .rev()
        .take(max_chars)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect()
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>> {
    enable_raw_mode().context("enable raw mode")?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen).context("enter alternate screen")?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).context("create terminal")?;
    terminal.clear().context("clear terminal")?;
    Ok(terminal)
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
    disable_raw_mode().context("disable raw mode")?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        crossterm::cursor::Show
    )
    .context("leave alternate screen")?;
    terminal.show_cursor().context("show cursor")?;
    Ok(())
}
