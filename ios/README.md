iLonhro iOS Client
==================

This directory contains a minimal SwiftUI client for the iLonhro
WebSocket server.

Setup (Xcode)
-------------
1. Create a new iOS App project in Xcode named "iLonhroTerminal".
2. Replace the generated files with:
   - iLonhroTerminalApp.swift
   - ContentView.swift
   - TerminalViewModel.swift
3. Build and run on a device or simulator.

Server Settings
---------------
Set the server URL and token in the app UI:
- URL example: ws://YOUR_HOST:7070/ws
- Token: the value of ILONHRO_TOKEN

Notes
-----
This client is App Store safe because it connects to a remote shell
instead of running local processes. It is a starting point and should
be expanded with better input handling, resizing, and keybindings.
