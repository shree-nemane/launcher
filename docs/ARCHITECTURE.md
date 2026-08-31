````md
# ARCHITECTURE.md

# Universal Project Launcher — Architecture

## 1. Purpose

This document defines how the Universal Project Launcher is structured internally.

The application has one primary responsibility:

> Receive a user command, understand what it means, launch the required projects or applications, and close itself.

The architecture should remain:

- Simple
- Modular
- Lightweight
- Easy to extend
- Easy to debug

The application should not contain unnecessary layers or abstractions.

---

# 2. Technology Stack

The application will use:

```text
Frontend:
React

Language:
TypeScript

Desktop Framework:
Tauri

Native Backend:
Rust

Target Platform:
Windows
````

High-level structure:

```text
┌───────────────────────────────────────────────┐
│                   USER                        │
└───────────────────────┬───────────────────────┘
                        │
                        │ Shortcut / Interaction
                        ▼
┌───────────────────────────────────────────────┐
│                 TAURI WINDOW                  │
│                                               │
│              React Frontend                   │
│                                               │
│  Search Input                                │
│  Suggestions                                 │
│  Command Validation                          │
│  Add/Edit Interfaces                         │
└───────────────────────┬───────────────────────┘
                        │
                        │ Tauri IPC
                        ▼
┌───────────────────────────────────────────────┐
│                 RUST BACKEND                  │
│                                               │
│  Command Execution                           │
│  Project Resolution                          │
│  Application Resolution                      │
│  Launch Planning                             │
│  Process Launching                           │
│  Window Lifecycle                            │
│  Data Access                                 │
└───────────────────────┬───────────────────────┘
                        │
                        ▼
┌───────────────────────────────────────────────┐
│                  WINDOWS OS                   │
│                                               │
│  Applications                                │
│  Executables                                 │
│  Browser                                     │
│  File System                                 │
│  Project Directories                         │
└───────────────────────────────────────────────┘
```

---

# 3. High-Level Architecture

The application is divided into two main layers.

```text
React Layer
    ↓
Tauri Bridge
    ↓
Rust Layer
```

## React Layer

Responsible for:

```text
User Interface
Input Handling
Suggestions
Visual Validation
Keyboard Navigation
Configuration Screens
```

React should not directly launch executables or access Windows processes.

---

## Rust Layer

Responsible for:

```text
Loading Stored Data
Command Resolution
Execution Planning
Launching Applications
Passing Project Context
Managing Windows
Native OS Operations
```

Rust is responsible for operations that interact with the operating system.

---

# 4. Core Architecture Flow

The normal execution flow should be:

```text
User Input
    ↓
React receives input
    ↓
Command is submitted
    ↓
Rust receives command
    ↓
Command is parsed
    ↓
Project is resolved
    ↓
Applications are resolved
    ↓
Execution plan is created
    ↓
Plan is validated
    ↓
Applications are launched
    ↓
Launcher closes
```

Example:

```text
deepfake /v /b
```

Flow:

```text
Input
deepfake /v /b
        ↓
Command Parser
        ↓
Project:
deepfake
        ↓
Applications:
/v
/b
        ↓
Resolve Project
        ↓
Resolve Applications
        ↓
Create Execution Plan
        ↓
VS Code → Deepfake Path
Browser → Deepfake URL
        ↓
Execute
        ↓
Close Launcher
```

---

# 5. Frontend Architecture

The frontend should remain focused on user interaction.

Recommended conceptual structure:

```text
src/
│
├── components/
│   ├── Launcher/
│   ├── Suggestions/
│   └── common/
│
├── pages/
│   ├── LauncherPage/
│   ├── AddProject/
│   └── AddApplication/
│
├── hooks/
│
├── services/
│
├── types/
│
├── utils/
│
├── App.tsx
└── main.tsx
```

The exact folder structure may evolve during implementation, but the responsibilities should remain stable.

---

# 6. Frontend Responsibilities

The frontend should handle:

## 6.1 User Input

Example:

```text
deepfake /v /b
```

The frontend:

* Captures input
* Displays the text
* Handles keyboard events
* Shows suggestions
* Sends the final command for execution

---

## 6.2 Suggestions

The frontend may provide suggestions while the user types.

Example:

```text
dee
```

Suggestions:

```text
Deepfake
Deep Learning Project
```

Example:

```text
deepfake /
```

Suggestions:

```text
/v → VS Code
/b → Browser
/a → Android Studio
```

Suggestions should improve usability but should not change the underlying command meaning.

---

## 6.3 Keyboard Interaction

The frontend manages:

```text
Enter
Escape
Arrow Up
Arrow Down
Tab
```

Expected behavior:

```text
Enter
→ Execute selected command

Escape
→ Close launcher

Arrow Keys
→ Navigate suggestions

Tab
→ Accept suggestion where applicable
```

---

## 6.4 Configuration Interfaces

The frontend displays forms for:

```text
add-project
add-app
```

These forms collect data from the user and send validated information to the backend.

The frontend should not decide how executables are launched.

---

# 7. Backend Architecture

The Rust backend handles all native operations.

Conceptually:

```text
Rust Backend
│
├── Command Handler
│
├── Command Parser
│
├── Project Resolver
│
├── Application Resolver
│
├── Group Resolver
│
├── Execution Planner
│
├── Application Launcher
│
├── Storage Manager
│
└── Window Manager
```

Each module should have a focused responsibility.

---

# 8. Command Handling

When the frontend submits:

```text
deepfake /v /b
```

The backend receives the raw command.

Conceptually:

```text
execute_command(
    "deepfake /v /b"
)
```

The backend should not immediately launch applications.

It should first:

```text
Parse
    ↓
Resolve
    ↓
Validate
    ↓
Plan
    ↓
Execute
```

This is important because invalid commands should be detected before partially launching applications.

---

# 9. Command Parsing

The command parser converts user input into structured information.

Example:

```text
deepfake /v /b
```

Conceptually becomes:

```text
Project:
deepfake

Application Commands:
/v
/b

Group Command:
None
```

Example:

```text
/a /v /b
```

Becomes:

```text
Project:
None

Application Commands:
/a
/v
/b

Group Command:
None
```

Example:

```text
deepfake //
```

Becomes:

```text
Project:
deepfake

Application Commands:
None

Group:
Default Group
```

The exact syntax rules are defined in:

```text
COMMANDS.md
```

---

# 10. Project Resolution

If a project exists in the command:

```text
deepfake /v
```

The backend searches the stored project registry.

Conceptually:

```text
Input:
deepfake

        ↓

Project Registry

        ↓

Found:
Deepfake Project
Path:
D:\Projects\Deepfake
```

If the project does not exist:

```text
unknown-project /v
```

The backend should return a clear error.

Applications should not launch unless the command is valid.

---

# 11. Application Resolution

For:

```text
/v /b
```

The backend resolves:

```text
/v
↓
VS Code

/b
↓
Browser
```

The application registry determines:

```text
Application Name
Executable
Arguments
Project Context Behavior
```

Applications should be configurable and not permanently hardcoded.

---

# 12. Application Group Resolution

The command:

```text
//
```

represents a configured application group.

Conceptually:

```text
//
    ↓
Default Group
    ↓
/v
/b
/p
/t
```

The backend resolves the group into individual applications.

Example:

```text
deepfake //
```

Becomes conceptually:

```text
Project:
Deepfake

Applications:
VS Code
Browser
Postman
Terminal
```

Each application is then prepared for execution.

---

# 13. Execution Planner

The execution planner is one of the most important parts of the backend.

The planner converts a validated command into a list of actions.

Example input:

```text
deepfake /v /b
```

Execution plan:

```text
Action 1
Application:
VS Code

Executable:
Code.exe

Arguments:
D:\Projects\Deepfake
```

```text
Action 2
Application:
Browser

Executable:
Configured Browser

Arguments:
http://localhost:5173
```

The execution planner should:

```text
Resolve everything first
        ↓
Validate everything
        ↓
Create complete plan
        ↓
Only then execute
```

This prevents situations such as:

```text
VS Code launches successfully

BUT

Browser command fails
```

because the application configuration was invalid.

---

# 14. Application Launching

The application launcher is responsible for starting processes.

Conceptually:

```text
Execution Action
        ↓
Executable Path
        ↓
Arguments
        ↓
Working Directory
        ↓
Windows Process
```

Example:

```text
Code.exe
```

with:

```text
D:\Projects\Deepfake
```

as an argument.

The launcher should support:

```text
Executable Applications
Browser URLs
Terminal Commands
Project Paths
Custom Arguments
```

The detailed data required for each application will be defined in:

```text
DATA_MODEL.md
```

---

# 15. Project Context

Project context is what allows:

```text
deepfake /v
```

to behave differently from:

```text
/v
```

Without project context:

```text
/v
```

Result:

```text
Open VS Code normally
```

With project context:

```text
deepfake /v
```

Result:

```text
Open VS Code
with:
D:\Projects\Deepfake
```

Each application may define how it consumes project context.

Possible project context values include:

```text
PROJECT_PATH
PROJECT_URL
PROJECT_NAME
PROJECT_COMMAND
PROJECT_WORKING_DIRECTORY
```

An application may use one, multiple, or none of these values.

---

# 16. Storage Architecture

The application requires persistent local storage for:

```text
Projects
Applications
Application Groups
Settings
```

The first version should prioritize simplicity.

The exact data structures are defined in:

```text
DATA_MODEL.md
```

The storage mechanism should allow:

```text
Create
Read
Update
Delete
```

for projects and applications.

The frontend should not directly manipulate storage files.

Instead:

```text
React
    ↓
Tauri Command
    ↓
Rust Storage Layer
    ↓
Persistent Storage
```

---

# 17. Window Architecture

The launcher has two primary UI states.

## Launcher State

```text
┌───────────────────────────────┐
│ 🔍 Command Input          ✕   │
└───────────────────────────────┘
```

Purpose:

```text
Receive commands
Show suggestions
Execute commands
```

---

## Configuration State

Used for:

```text
add-project
add-app
```

Purpose:

```text
Collect configuration
Validate data
Save data
```

The configuration interface may be implemented as:

```text
A secondary window
```

or:

```text
A temporary state in the main window
```

The final implementation should prioritize simplicity and a smooth user experience.

---

# 18. Application Lifecycle

The desired lifecycle is:

```text
Inactive
    ↓
Shortcut Trigger
    ↓
Launcher Opens
    ↓
Input Focused
    ↓
User Interaction
    ↓
Execute Command
    ↓
Close Launcher
```

The launcher should also close when:

```text
Escape
```

is pressed.

The launcher should close after successful execution.

For invalid commands:

```text
Invalid Command
    ↓
Show Error
    ↓
Keep Launcher Open
    ↓
Allow User to Correct Input
```

---

# 19. Global Shortcut Constraint

The primary shortcut is:

```text
Alt + Space
```

The desired behavior is:

```text
Alt + Space
    ↓
Launcher Opens
```

However, there is an important operating system constraint:

> A completely terminated process cannot listen for a keyboard shortcut.

Therefore, the final implementation must explicitly define how the launcher is activated when it is not visible.

Possible approaches include:

```text
Background Resident Process
```

or:

```text
Minimal Native Helper
```

or another Windows-supported shortcut mechanism.

The chosen solution must satisfy the product principle:

> The launcher should not behave like a large continuously running application.

This decision should be finalized before implementing the global shortcut system.

---

# 20. Error Handling

Errors should be returned before execution whenever possible.

Examples:

```text
Project does not exist
```

```text
Application command does not exist
```

```text
Duplicate application command
```

```text
Executable path does not exist
```

```text
Project path does not exist
```

```text
Application group is empty
```

The execution flow should be:

```text
Parse
    ↓
Validate
    ↓
Resolve
    ↓
Create Plan
    ↓
Execute
```

Applications should not be partially launched when the entire command can be validated beforehand.

---

# 21. Architecture Principles

The architecture should follow these rules.

## Separation of Responsibilities

```text
React
→ User interaction

Rust
→ Native operations

Storage
→ Persistent data

Execution Engine
→ Launch planning and execution
```

---

## No Hardcoded Applications

The system should not contain logic such as:

```text
if command == "/v"
    open VS Code
```

Instead:

```text
/v
    ↓
Application Registry
    ↓
Configured Application
    ↓
Launch
```

This allows users to customize commands.

---

## Validate Before Execution

The application should resolve all required resources before launching processes.

---

## Keep the Core Small

The first version should focus on:

```text
Command
→ Parse
→ Resolve
→ Execute
→ Close
```

Advanced automation should not complicate the core architecture.

---

## Native Operations Stay in Rust

Launching applications, accessing the operating system, and managing native processes should remain in the Rust/Tauri layer.

---

# 22. Architecture Summary

The complete system can be summarized as:

```text
                    USER
                     │
                     │ Alt + Space
                     ▼
             ┌───────────────┐
             │   LAUNCHER    │
             │   React UI    │
             └───────┬───────┘
                     │
                     │ Command
                     ▼
             ┌───────────────┐
             │    PARSER     │
             └───────┬───────┘
                     │
                     ▼
             ┌───────────────┐
             │   RESOLVER    │
             │               │
             │ Project       │
             │ Applications  │
             │ Groups        │
             └───────┬───────┘
                     │
                     ▼
             ┌───────────────┐
             │ EXECUTION PLAN│
             └───────┬───────┘
                     │
                     ▼
             ┌───────────────┐
             │ RUST LAUNCHER │
             └───────┬───────┘
                     │
                     ▼
             ┌───────────────┐
             │  WINDOWS OS   │
             └───────────────┘
                     │
                     ▼
             Applications Launch
                     │
                     ▼
             Launcher Closes
```

The core architectural principle is:

> Parse first, validate everything, create an execution plan, launch the requested applications, and disappear.

```
```
