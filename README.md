# Universal Project Launcher

Universal Project Launcher is a keyboard-driven Windows desktop application for quickly opening development projects, launching desktop tools, and executing development scripts via a command palette interface.

Built with Tauri v2, React, and Rust.

---

## Overview

Instead of manually navigating directories, opening individual editors, and starting local servers, the application allows launching full development environments through composable commands triggered from a global shortcut.

```text
Alt + Space  ->  myproject /v /run /b  ->  Enter
```

Upon execution, the launcher resolves all commands, spawns the required processes, and automatically hides the window.

---

## Core Features

- **Global Hotkey & Fast Invocation**: Summon from anywhere on Windows via `Alt + Space` with automatic input focus and instant dismissal on execution or `Escape`.
- **Project-Aware Execution**: Register projects with custom directory paths, URLs, and startup commands.
- **Composable Application Flags**: Launch applications standalone (`/v`, `/b`) or pass project context dynamically (`project /v`).
- **Development Script Execution**: Execute configured project startup commands in PowerShell 7 (`pwsh`) using the `/run` flag.
- **Application Groups**: Launch predefined stacks of applications using the `//` shorthand.
- **Dynamic Argument Interpolation**: Substitute project paths, URLs, and metadata into application launch arguments using template variables.
- **Integrated Management**: Configure projects, applications, and groups directly from the search bar via management commands.

---

## Tech Stack

- **Desktop Framework**: Tauri v2
- **Backend**: Rust
- **Frontend**: React 19, Vite
- **Styling**: Tailwind CSS v4
- **Plugins**: `tauri-plugin-global-shortcut`, `tauri-plugin-opener`
- **Data Persistence**: Local JSON storage in `%APPDATA%`

---

## Command Syntax

Commands follow the structure:
```text
[project_name] [/<app_flag>...] [//] [/run]
```

### Examples

| Command | Description |
|---|---|
| `deepfake` | Opens the project folder in Windows Explorer. |
| `deepfake /v` | Opens the project in VS Code (or configured editor). |
| `deepfake /run` | Runs configured project startup commands in PowerShell 7. |
| `deepfake /v /run /b` | Opens the project in VS Code, starts the dev server, and opens the configured URL in the browser. |
| `deepfake //` | Opens the project across all applications in the default group. |
| `/v` | Launches VS Code standalone without project context. |
| `//` | Launches all applications in the default group standalone. |

---

## System Commands

Type these commands directly into the search bar to access management interfaces:

| Command | Description |
|---|---|
| `add-project` | Open the project creation form. |
| `manage-projects` | View, edit, or delete existing projects. |
| `add-app` | Register an application executable and launch arguments. |
| `manage-apps` | View and edit registered applications. |
| `add-group` | Create an application launch group. |
| `manage-groups` | Manage application groups and set the default group. |
| `help` | Open the in-app command reference. |

---

## Argument Variables

When configuring application launch arguments, the following placeholders can be used:

| Variable | Replacement Value | Example |
|---|---|---|
| `{PROJECT_PATH}` | Absolute path to the project directory | `Code.exe "{PROJECT_PATH}"` |
| `{PROJECT_URL}` | Project web URL | `chrome.exe "{PROJECT_URL}"` |
| `{PROJECT_NAME}` | Project display name | `wt.exe --title "{PROJECT_NAME}"` |
| `{PROJECT_COMMAND}` | Project command alias | `{PROJECT_COMMAND}` |

---

## Keyboard Shortcuts

| Shortcut | Context | Action |
|---|---|---|
| `Alt + Space` | Global | Toggle launcher window visibility. |
| `Up` / `Down` | Launcher | Navigate autocomplete suggestions. |
| `Tab` | Launcher | Complete selected suggestion into the search bar. |
| `Enter` | Launcher | Execute current command or selected item. |
| `Ctrl + Enter` | Forms | Save configuration form. |
| `Escape` | Global / App | Dismiss launcher or return to search view. |

---

## Getting Started

### Prerequisites

- Node.js (v18+)
- Rust toolchain ([rustup.rs](https://rustup.rs/))
- Visual Studio C++ Build Tools
- PowerShell 7 (`pwsh`) for running project dev scripts

### Development Setup

1. Install dependencies:
   ```bash
   npm install
   ```

2. Start development mode:
   ```bash
   npm run tauri dev
   ```

3. Build for production:
   ```bash
   npm run tauri build
   ```

The compiled binaries will be output to `src-tauri/target/release/bundle/`.

---

## Project Structure

```text
launcher/
├── docs/               # Specifications and architecture documentation
├── src/                # React frontend application
│   ├── components/     # UI views (Launcher palette, Config forms)
│   ├── hooks/          # React hooks (useLauncher, useSuggestions)
│   ├── services/       # Tauri IPC service client
│   └── App.jsx         # Root component
└── src-tauri/          # Rust backend
    ├── src/
    │   ├── commands/   # Tauri command handlers
    │   ├── execution/  # Process execution and command planning
    │   ├── models/     # Data structures
    │   ├── parser/     # Command parser and tokenizer
    │   ├── resolver/   # Target resolution logic
    │   ├── storage/    # Local storage manager
    │   └── lib.rs      # Tauri application initialization
    ├── Cargo.toml      # Rust dependencies
    └── tauri.conf.json # Tauri configuration
```

---

## Storage

Configuration files are stored locally in the application data directory:
- Windows: `%APPDATA%\com.username.launcher\`
  - `projects.json`
  - `apps.json`
  - `groups.json`
  - `settings.json`

---


