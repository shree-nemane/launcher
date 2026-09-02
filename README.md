# Universal Project Launcher

Universal Project Launcher is a high-performance, keyboard-driven Windows desktop application for rapid navigation, tool invocation, and automated environment orchestration via a composable command palette.

Built on **Tauri v2**, **Rust**, and **React 19**.

---

## Architecture & Technical Overview

The application functions as a background-resident desktop launcher. It registers a global system shortcut (`Alt + Space`), manages system tray integration, and parses composable execution pipelines that resolve projects, application shortcuts, development groups, and system commands into concurrent detached processes.

```text
Global Shortcut (Alt + Space)
            │
            ▼
┌───────────────────────┐
│ Command Palette (UI)  │  <─── Autocomplete / Suggestions
└───────────┬───────────┘
            │  IPC (Tauri Invoke)
            ▼
┌───────────────────────┐
│ Command Parser (Rust) │  <─── Tokenization & Grammar Validation
└───────────┬───────────┘
            ▼
┌───────────────────────┐
│ Resolver & Storage    │  <─── Atomic JSON Persistence (%APPDATA%)
└───────────┬───────────┘
            ▼
┌───────────────────────┐
│ Process Executor      │  <─── Win32 Process Detachment & Shell Normalization
└───────────────────────┘
```

---

## Core Capabilities

- **Global Hotkey & Fast Invocation**: Summoned globally via `Alt + Space` with sub-millisecond focus handling and automatic window dismissal upon execution or `Escape`.
- **Project-Aware Execution**: Register software repositories with paths, URLs, working directories, and startup scripts.
- **Composable Application Flags**: Launch applications standalone (`/v`, `/wt`) or pass project context dynamically (`project /v`).
- **Development Script Execution**: Execute configured project startup commands directly in PowerShell 7 (`pwsh`) via the `/run` flag without intermediate shell windows.
- **Application Groups**: Launch complete multi-tool environments simultaneously using the `//` workspace convention.
- **Dynamic Argument Interpolation**: Substitute project paths, URLs, and metadata into process argument lists using template variables.
- **In-Palette Management**: Manage projects, applications, and groups directly from the search bar via reserved system commands.

---

## Technology Stack

- **Desktop Framework**: Tauri v2.0
- **Core Engine**: Rust (2021 Edition)
- **UI Framework**: React 19, Vite
- **Styling Layer**: Tailwind CSS v4
- **Plugins**: `tauri-plugin-global-shortcut`, `tauri-plugin-opener`, `tauri-plugin-autostart`, `tauri-plugin-dialog`
- **Persistence**: ACID-compliant atomic local JSON storage in `%APPDATA%`

---

## Command Grammar & Syntax Specification

Commands evaluated by the parser adhere to the following grammar:

```text
COMMAND := SYSTEM_COMMAND | LAUNCH_COMMAND
LAUNCH_COMMAND := [PROJECT_IDENTIFIER]? ([APPLICATION_FLAG]+ | GROUP_FLAG | RUN_FLAG)*
```

### Execution Matrix

| Syntax Pattern | Concrete Example | Execution Semantics |
| :--- | :--- | :--- |
| `<project>` | `deepfake` | Resolves the project directory and opens it in Windows File Explorer. |
| `<project> /<app>` | `deepfake /v` | Resolves the project and launches the target application with interpolated project parameters. |
| `<project> /run` | `deepfake /run` | Spawns configured project startup commands sequentially/concurrently in PowerShell 7 (`pwsh`). |
| `<project> /<app> /run` | `deepfake /v /run` | Concurrently opens the editor with project context and executes the dev script runner. |
| `<project> //` | `deepfake //` | Executes all applications assigned to the Default Application Group in configured sequence. |
| `//` | `//` | Executes the Default Application Group standalone without project context. |
| `/<app>` | `/v` | Launches the specified application standalone without project context. |

---

## System Management Commands

Typing system commands into the palette shifts the interface into administrative view states:

| System Command | Target Interface | Functional Scope |
| :--- | :--- | :--- |
| `add-project` | Project Creation View | Register repository paths, preview URLs, working directories, and startup run commands. |
| `manage-projects` | Project Management View | Search, inspect, edit, or delete existing project registrations. |
| `add-app` | Application Registration View | Register executable binaries, launch parameters, working directories, and project-aware flags. |
| `manage-apps` | Application Management View | Modify executable paths, configure argument lists, and adjust launch parameters. |
| `add-group` | Group Creation View | Create multi-application launch bundles and define execution order. |
| `manage-groups` | Group Management View | Manage application groups and assign the default workspace (`//`) target. |
| `help` | Interactive Reference View | Access in-palette command cheat sheets, argument syntax, and keyboard bindings. |
| `settings` | Settings View | Configure autostart behavior, global summon shortcuts, and theme properties. |

---

## Argument Interpolation Variables

When configuring application command parameters, template placeholders are dynamically replaced at runtime:

| Variable Placeholder | Interpolated Output | Typical Configuration |
| :--- | :--- | :--- |
| `{PROJECT_PATH}` | Absolute filesystem path to project root | `Code.exe "{PROJECT_PATH}"` |
| `{PROJECT_URL}` | Project web URL endpoint | `chrome.exe "{PROJECT_URL}"` |
| `{PROJECT_NAME}` | Human-readable project display name | `wt.exe --title "{PROJECT_NAME}"` |
| `{PROJECT_COMMAND}` | Project shortcut command alias | `{PROJECT_COMMAND}` |

> **Process Working Directory Rule:** The launcher automatically assigns the process working directory (`current_dir`) to the project directory upon launch. Consequently, console shells (PowerShell, CMD) do not require `{PROJECT_PATH}` in their argument lists.

---

## Keyboard Shortcuts & Input Matrix

| Key Combination | Scope | Functional Behavior |
| :--- | :--- | :--- |
| `Alt + Space` | Global System | Toggle palette window visibility and focus input. |
| `Up` / `Down` | Palette Input | Navigate active autocomplete and history suggestions. |
| `Tab` | Palette Input | Complete the highlighted suggestion into the active input. |
| `Enter` | Palette Input | Execute the current command string or selected item. |
| `Ctrl + Enter` | Form Views | Commit and persist form changes. |
| `Escape` | Global / Application | Dismiss the palette or exit sub-views back to the main search bar. |

---

## Getting Started

### Prerequisites

- [Node.js](https://nodejs.org/) (v18.0 or higher)
- [Rust Toolchain](https://rustup.rs/) (Cargo, rustc 1.75+)
- Visual Studio C++ Build Tools (Windows SDK)
- PowerShell 7 (`pwsh.exe`) for modern terminal execution

### Development

```bash
# Install frontend dependencies
npm install

# Run application in development mode with hot-reloading
npm run tauri dev
```

### Production Build & Installer Generation

```bash
# Compile optimized binaries and generate Windows installers (.exe / .msi)
npm run tauri build
```

The compiled release packages will be located in:
- NSIS Installer: `src-tauri/target/release/bundle/nsis/`
- WiX MSI Package: `src-tauri/target/release/bundle/msi/`

---

## Data Storage & Persistence Model

Configuration state is stored in JSON format within the user application data directory:
`%APPDATA%\com.username.launcher\` (or `%LOCALAPPDATA%\universal-launcher\`)

- `projects.json`: Registered project definitions, paths, URLs, and run command arrays.
- `applications.json`: Application registry, executable paths, normal and project launch configurations.
- `groups.json`: Application group definitions and ordered execution sequences.
- `settings.json`: Global runtime settings, startup preferences, and default group identifiers.
