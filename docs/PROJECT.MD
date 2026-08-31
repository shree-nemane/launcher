````md
# PROJECT.md

# Universal Project Launcher

## 1. Project Overview

Universal Project Launcher is a lightweight, keyboard-driven Windows desktop application built to quickly launch development projects and desktop applications using a simple command-based interface.

Instead of opening multiple applications manually, navigating through folders, and starting a development environment step by step, the user can press a global shortcut, enter a command, and launch everything needed.

Example:

```text
deepfake /v /b
````

The launcher interprets:

* `deepfake` → the selected project
* `/v` → open the project in VS Code
* `/b` → open the project's configured browser URL

Another example:

```text
/a /v /b
```

This opens the configured applications normally without a project context.

The application is designed to behave like a lightweight command palette rather than a traditional desktop application.

---

# 2. Core Vision

The goal is to create a personal universal launcher that allows the user to quickly:

* Open projects
* Open desktop applications
* Open multiple applications together
* Open projects inside specific applications
* Launch predefined groups of applications
* Manage projects and applications through the launcher itself

The primary interaction should be:

```text
Press Shortcut
      ↓
Launcher Appears
      ↓
Enter Command
      ↓
Press Enter
      ↓
Actions Execute
      ↓
Launcher Closes
```

The launcher should feel fast, minimal, and invisible when not actively being used.

---

# 3. Target Platform

The first version of the application will target:

```text
Windows 11
```

The application will be built using:

```text
Tauri
React
TypeScript
Rust
```

The architecture should remain flexible enough to support additional platforms in the future, but cross-platform support is not a requirement for the first version.

---

# 4. Core User Experience

The application should not behave like a traditional desktop application with a permanent main window.

The expected experience is:

```text
User is working on Windows
        ↓
Press Alt + Space
        ↓
Launcher appears
        ↓
Search bar receives focus automatically
        ↓
User enters command
        ↓
Command is validated
        ↓
User presses Enter
        ↓
Requested applications/projects launch
        ↓
Launcher closes
```

The launcher should also close when:

```text
Escape is pressed
```

or:

```text
The manual close button is pressed
```

The primary purpose of the application is to perform a task quickly and then disappear.

---

# 5. Main Features

The first version of the application will contain the following core capabilities.

## 5.1 Project Launching

The user can register projects inside the launcher.

Example registered project:

```text
Name: Deepfake
Command Name: deepfake
Path: D:\Projects\Deepfake
```

The project can then be referenced directly:

```text
deepfake
```

Projects can also be combined with application commands:

```text
deepfake /v
```

```text
deepfake /v /b
```

The selected project provides context to the applications being launched.

---

## 5.2 Individual Application Launching

Users can register applications and assign each application a command.

Example:

```text
/v → VS Code
/b → Browser
/a → Android Studio
/p → Postman
/t → Terminal
```

The user can launch applications directly:

```text
/v
```

or multiple applications:

```text
/v /b /p
```

When no project is specified, the applications should launch normally without project-specific context.

Example:

```text
/a /v /b
```

Result:

```text
Open Android Studio
Open VS Code
Open Browser
```

---

## 5.3 Project Context Launching

When a project name is combined with application commands, the project information should be provided to the applications.

Example:

```text
deepfake /v /b
```

Possible execution:

```text
VS Code
→ Opens D:\Projects\Deepfake

Browser
→ Opens the configured Deepfake project URL
```

Another example:

```text
deepfake /a
```

Could result in:

```text
Android Studio
→ Opens the Deepfake project folder
```

Each application may define how it handles project context.

The launcher should not assume that every application uses projects in the same way.

---

## 5.4 Application Group Launching

The command:

```text
//
```

represents a predefined group of applications.

Example group:

```text
Development Workspace

/v → VS Code
/b → Browser
/p → Postman
/t → Terminal
```

Running:

```text
//
```

opens all applications in the configured group normally.

Running:

```text
deepfake //
```

opens the entire configured application group using the Deepfake project as context where applicable.

Example:

```text
deepfake //
```

May result in:

```text
VS Code
→ Opens Deepfake project

Browser
→ Opens Deepfake project URL

Terminal
→ Opens at Deepfake project directory

Postman
→ Opens normally or using project-specific configuration
```

The exact behavior of application groups and project context will be defined in the command and data model documentation.

---

## 5.5 Add Project

Projects should be added from inside the application.

The user enters:

```text
add-project
```

The launcher recognizes this as a system command.

A project configuration interface opens and collects the necessary information.

Potential information includes:

```text
Project Name
Command Name
Project Path
Project URL
Project Run Commands
Project Metadata
```

The exact required and optional fields will be defined in:

```text
DATA_MODEL.md
```

---

## 5.6 Add Application

Applications should also be added from inside the launcher.

The user enters:

```text
add-app
```

A configuration interface opens.

The user can provide information such as:

```text
Application Name
Application Command
Executable Path
Launch Arguments
Project Context Arguments
Working Directory
```

Example:

```text
Application Name:
VS Code

Command:
/v

Executable:
Code.exe
```

After saving:

```text
/v
```

becomes a registered application command.

---

# 6. Command-Based Philosophy

The application should remain command-driven.

The main launcher interface should not require users to navigate through multiple menus to perform common actions.

The preferred interaction model is:

```text
Type what you want
        ↓
Launcher understands intent
        ↓
Execute
```

Examples:

```text
deepfake /v
```

```text
deepfake /v /b
```

```text
/a /v /b
```

```text
//
```

```text
deepfake //
```

```text
add-project
```

```text
add-app
```

The command system will be formally defined in:

```text
COMMANDS.md
```

---

# 7. User Interface Philosophy

The primary interface should be minimal.

The launcher should resemble a command palette or system launcher rather than a traditional desktop application.

Conceptually:

```text
                Desktop


       ┌──────────────────────────────┐
       │ 🔍 deepfake /v /b        ✕  │
       └──────────────────────────────┘
```

The launcher should prioritize:

* Speed
* Keyboard interaction
* Minimal visual distraction
* Fast opening
* Fast closing
* Clear command feedback

The main interface should not contain unnecessary permanent navigation.

Additional configuration interfaces may appear when needed for actions such as:

```text
add-project
add-app
settings
```

The detailed UI behavior will be defined in:

```text
UI_AND_FLOW.md
```

---

# 8. Application Lifecycle

The desired lifecycle is:

```text
Launcher Inactive
        ↓
User triggers shortcut
        ↓
Launcher Opens
        ↓
Input receives focus
        ↓
User enters command
        ↓
Command executes
        ↓
Launcher closes
```

The launcher should also close when:

```text
ESC
```

is pressed.

The manual close button should also close the launcher interface.

The final technical implementation of the global shortcut and application lifecycle will be defined in:

```text
ARCHITECTURE.md
```

---

# 9. Primary Use Cases

## Use Case 1: Open a Project in VS Code

Input:

```text
deepfake /v
```

Expected result:

```text
Find project: deepfake
        ↓
Find application: /v
        ↓
Launch VS Code
        ↓
Pass project path
        ↓
VS Code opens Deepfake project
```

---

## Use Case 2: Open Multiple Applications Normally

Input:

```text
/a /v /b
```

Expected result:

```text
Open Android Studio
Open VS Code
Open Browser
```

No project context is used.

---

## Use Case 3: Open Project Across Multiple Applications

Input:

```text
deepfake /a /v /b
```

Expected result:

```text
Find Deepfake project
        ↓
Launch Android Studio with project context
Launch VS Code with project context
Launch Browser with project context
        ↓
Close launcher
```

---

## Use Case 4: Launch Application Group

Input:

```text
//
```

Expected result:

```text
Load default application group
        ↓
Launch every application in the group
        ↓
Close launcher
```

---

## Use Case 5: Launch Project Workspace

Input:

```text
deepfake //
```

Expected result:

```text
Find Deepfake project
        ↓
Load default application group
        ↓
Launch compatible applications
using Deepfake project context
        ↓
Close launcher
```

---

## Use Case 6: Add a Project

Input:

```text
add-project
```

Expected result:

```text
Recognize system command
        ↓
Open Add Project interface
        ↓
User enters project information
        ↓
Validate information
        ↓
Save project
        ↓
Project becomes available as a command
```

---

## Use Case 7: Add an Application

Input:

```text
add-app
```

Expected result:

```text
Recognize system command
        ↓
Open Add Application interface
        ↓
User selects executable
        ↓
User assigns command
        ↓
Validate command uniqueness
        ↓
Save application
```

The application can then be launched using its assigned command.

---

# 10. Core Requirements

The first version must support:

```text
✓ Command-based project launching
✓ Command-based application launching
✓ Multiple application commands
✓ Project context
✓ Application groups
✓ // group command
✓ Add Project
✓ Add Application
✓ Minimal launcher UI
✓ Keyboard-first interaction
✓ Manual close button
✓ Escape to close
✓ Automatic closing after task execution
✓ Local data persistence
```

---

# 11. Important Constraints

The application should remain:

```text
Lightweight
Simple
Fast
Local-first
Keyboard-driven
Minimal
```

The application should avoid unnecessary complexity.

The first version should not attempt to become:

```text
A full IDE
A task manager
A process monitor
A complete automation platform
A cloud service
A project management application
```

The application's primary responsibility is:

> Quickly interpret a command and launch the requested projects and applications.

---

# 12. Non-Goals for Version 1

The following features are intentionally outside the first version unless explicitly added later.

```text
Cloud synchronization
User accounts
Multi-user support
Plugin marketplace
AI command interpretation
Complex workflow automation
Cross-device synchronization
Detailed process monitoring
Application analytics
Remote execution
Team collaboration
```

The focus should remain on building a reliable and fast local launcher first.

---

# 13. Product Principles

The project should follow these principles.

## Keyboard First

The user should be able to perform most common actions without touching the mouse.

---

## Fast by Default

Opening the launcher, parsing commands, and launching applications should feel immediate.

---

## Minimal Interface

The launcher should only show what is necessary for the current interaction.

---

## Explicit Commands

Commands should have predictable meanings.

Example:

```text
/v
```

should always refer to the registered application assigned to `/v`.

---

## Configurable

Applications and commands should not be permanently hardcoded.

The user should be able to register:

```text
Application
Command
Executable
Arguments
Project behavior
```

---

## Local First

Project and application configuration should initially remain local to the user's machine.

---

## Extensible Architecture

The architecture should allow future additions without requiring a redesign of the core command system.

---

# 14. Documentation Structure

The project documentation structure is intentionally limited to six files.

```text
docs/
│
├── PROJECT.md
├── ARCHITECTURE.md
├── COMMANDS.md
├── DATA_MODEL.md
├── UI_AND_FLOW.md
└── DEVELOPMENT.md
```

Each document has a specific responsibility.

| Document          | Purpose                                                |
| ----------------- | ------------------------------------------------------ |
| `PROJECT.md`      | Defines what the product is and what it should do      |
| `ARCHITECTURE.md` | Defines the technical architecture                     |
| `COMMANDS.md`     | Defines command syntax and interpretation              |
| `DATA_MODEL.md`   | Defines projects, applications, groups, and settings   |
| `UI_AND_FLOW.md`  | Defines the interface and user interactions            |
| `DEVELOPMENT.md`  | Defines implementation phases, progress, and decisions |

No additional documentation files should be created unless the documentation structure is deliberately redesigned.

---

# 15. Current Project Definition

Universal Project Launcher is a lightweight Windows desktop application that provides a keyboard-driven command interface for launching projects and applications.

The core interaction is:

```text
Alt + Space
        ↓
Command Launcher Opens
        ↓
User enters command
        ↓
Command is interpreted
        ↓
Requested actions execute
        ↓
Launcher closes
```

The application supports:

```text
PROJECT + APP COMMANDS

Example:
deepfake /v /b
```

```text
APP COMMANDS ONLY

Example:
/a /v /b
```

```text
APPLICATION GROUP

Example:
//
```

```text
PROJECT + APPLICATION GROUP

Example:
deepfake //
```

The system is designed to replace repetitive manual steps involved in starting development environments and frequently used desktop applications.

The guiding principle of the project is:

> Open what you need, execute the task, and disappear.
