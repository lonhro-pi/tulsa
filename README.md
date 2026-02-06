iLonhro Terminal
================

iLonhro Terminal is a Linux TUI terminal emulator inspired by the visual
language of iPhone and iCloud. Branding: **iLonhro by Lonhro**.

Theme
-----
Default theme: **Pink_n_Black** (cyberpunk).

Status
------
TUI prototype with a PTY-backed bash session and a themed interface.

Getting Started
---------------
1. Install Rust (https://rustup.rs).
2. Build and run:
   cargo run

Controls
--------
- Ctrl+Q: Quit iLonhro Terminal
- Ctrl+C: Send SIGINT to the shell

Shell
-----
By default the app launches **bash**. Override via:
```
export ILONHRO_SHELL=/bin/bash
```

Roadmap (Draft)
---------------
- Improve interactive editing (cursor movement, history).
- Add tabs and split panes.
- Expand theming and settings.

Notes
-----
If you want specific UI layout, keybindings, or distro targeting, share
the details so the roadmap can be refined.
