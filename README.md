iLonhro Terminal
================

iLonhro Terminal is a Linux TUI terminal emulator inspired by the visual
language of iPhone and iCloud. Branding: **iLonhro by Lonhro**.

Theme
-----
Default theme: **Pink_n_Black** (cyberpunk).

Status
------
- Local TUI bash terminal.
- Remote WebSocket server for iOS clients.

Getting Started (TUI)
---------------------
1. Install Rust (https://rustup.rs).
2. Build and run:
   cargo run --bin ilonhro_terminal

Controls
--------
- Ctrl+Q: Quit iLonhro Terminal
- Ctrl+C: Send SIGINT to the shell

Server Mode (iOS Backend)
-------------------------
Run the WebSocket PTY server:
```
export ILONHRO_BIND=0.0.0.0:7070
export ILONHRO_TOKEN=change_me
export ILONHRO_SHELL=/bin/bash
cargo run --bin ilonhro_server
```

Clients connect to:
```
ws://HOST:7070/ws
```
Use `Authorization: Bearer <token>` if `ILONHRO_TOKEN` is set.

iOS Client
----------
See `ios/README.md` for the SwiftUI client scaffold and setup steps.

Notes
-----
If you want specific UI layout, keybindings, or distro targeting, share
the details so the roadmap can be refined.
