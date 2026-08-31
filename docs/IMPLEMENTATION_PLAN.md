The final core document should be **`IMPLEMENTATION_PLAN.md`**.

This turns all the previous documents into a concrete build sequence. We should build in phases so we don't jump around or mix unfinished systems.

````md
# IMPLEMENTATION_PLAN.md

# Universal Project Launcher — Implementation Plan

## 1. Purpose

This document defines the implementation sequence for the Universal Project Launcher.

The purpose is to ensure that development follows a clear order.

We will not build everything at once.

The application will be built in layers:

```text
Foundation
    ↓
Data
    ↓
Command Parsing
    ↓
Execution Engine
    ↓
Application Launching
    ↓
Launcher UI
    ↓
Configuration UI
    ↓
Global Shortcut
    ↓
Testing and Packaging
````

Each layer should be working before moving to the next.

---

# 2. Development Principle

The core product flow is:

```text
Alt + Space
    ↓
Launcher Opens
    ↓
User Types Command
    ↓
Command Parsed
    ↓
Data Resolved
    ↓
Execution Plan Created
    ↓
Applications Launched
    ↓
Launcher Closes
```

Everything implemented should support this flow.

We should avoid building features that do not directly support the Version 1 product.

---

# 3. Implementation Phases

The project will be built in the following phases:

```text
Phase 1
Project Foundation

Phase 2
Data Storage

Phase 3
Command Engine

Phase 4
Execution Engine

Phase 5
Application Launching

Phase 6
Launcher UI

Phase 7
Project Configuration

Phase 8
Application Configuration

Phase 9
Application Groups

Phase 10
Global Shortcut

Phase 11
Error Handling and Testing

Phase 12
Packaging and Distribution
```

The phases should generally be completed in order.

---

# 4. Phase 1 — Project Foundation

## Goal

Create the basic Tauri + React + TypeScript project structure.

At the end of this phase:

```text
Tauri Application Starts
```

and:

```text
React UI Renders
```

No launcher functionality is required yet.

---

## Tasks

### 1. Create Tauri Project

The project should contain:

```text
Frontend:
React + TypeScript

Backend:
Rust

Desktop:
Tauri
```

---

### 2. Verify Development Environment

Confirm:

```text
Node.js
npm
Rust
Cargo
Tauri CLI
```

are functioning.

---

### 3. Verify Development Application

Run the application.

Expected result:

```text
Tauri Window Opens
```

The frontend should render successfully.

---

## Phase 1 Completion

The phase is complete when:

```text
npm run tauri dev
```

starts the desktop application successfully.

---

# 5. Phase 2 — Data Storage

## Goal

Create the system responsible for storing:

```text
Projects
Applications
Application Groups
Settings
```

The data structure must follow:

```text
DATA_MODEL.md
```

---

## Tasks

### 1. Define Rust Data Structures

Create Rust structures representing:

```text
Project
Application
ApplicationGroup
Settings
```

Example concept:

```text
Project
```

contains:

```text
id
name
command
path
url
runCommands
workingDirectory
```

Application contains:

```text
id
name
command
executablePath
normalLaunch
projectLaunch
workingDirectory
```

---

### 2. Create Storage Layer

The storage layer should provide:

```text
Create
Read
Update
Delete
```

operations.

Conceptually:

```text
StorageManager
│
├── getProjects()
├── getProject()
├── saveProject()
├── updateProject()
├── deleteProject()
│
├── getApplications()
├── getApplication()
├── saveApplication()
├── updateApplication()
├── deleteApplication()
│
├── getGroups()
├── saveGroup()
│
└── getSettings()
```

---

### 3. Create Initial Storage

For Version 1, storage should remain simple.

Recommended structure:

```text
App Data Directory
│
├── projects.json
├── applications.json
├── groups.json
└── settings.json
```

The storage format should be private to the application.

The frontend should never directly read or modify these files.

---

### 4. Create Tauri Commands

Expose backend operations to React.

Example:

```text
get_projects
```

```text
create_project
```

```text
get_applications
```

```text
create_application
```

---

## Phase 2 Completion

The phase is complete when:

```text
Project can be created
Project can be read

Application can be created
Application can be read

Data remains after application restart
```

---

# 6. Phase 3 — Command Engine

## Goal

Build the command parsing system.

The parser must follow:

```text
COMMANDS.md
```

The parser does not launch anything.

Its only responsibility is:

```text
Raw Input
    ↓
Structured Command
```

---

## Tasks

### 1. Create Command Types

The parser should identify:

```text
System Command
Project Command
Application Commands
Application Group
```

Example:

```text
deepfake /v /b
```

becomes conceptually:

```text
{
  project: "deepfake",
  applications: [
    "/v",
    "/b"
  ]
}
```

---

### 2. Normalize Input

The parser should:

```text
Trim whitespace
Normalize spaces
Handle case-insensitive matching
```

Example:

```text
   DEEPFAKE     /V
```

becomes:

```text
deepfake /v
```

---

### 3. Validate Syntax

Detect:

```text
Unknown slash formats
Invalid group combinations
Multiple projects
Invalid command order
```

Example:

```text
// /v
```

should fail.

---

### 4. Remove Duplicate Applications

Example:

```text
/v /v /v
```

becomes:

```text
/v
```

---

## Phase 3 Completion

The phase is complete when the parser can correctly process:

```text
deepfake

/v

/a /v /b

deepfake /v

deepfake /a /v /b

//

deepfake //

add-project

add-app
```

No application launching is required yet.

---

# 7. Phase 4 — Execution Engine

## Goal

Convert a valid parsed command into an execution plan.

The execution engine should not directly depend on the UI.

Flow:

```text
Structured Command
    ↓
Resolver
    ↓
Validation
    ↓
Execution Plan
```

---

## Tasks

### 1. Project Resolver

Find projects using:

```text
Project Command
```

Example:

```text
deepfake
```

returns:

```text
Deepfake Project Object
```

---

### 2. Application Resolver

Resolve:

```text
/v
```

into:

```text
VS Code Application Object
```

---

### 3. Group Resolver

Resolve:

```text
//
```

into:

```text
Default Application Group
```

Then resolve all application IDs.

---

### 4. Project Context Resolver

Replace variables:

```text
{PROJECT_PATH}

{PROJECT_URL}

{PROJECT_NAME}

{PROJECT_COMMAND}

{PROJECT_WORKING_DIRECTORY}
```

with actual project values.

---

### 5. Create Execution Plan

Example:

```text
deepfake /v /b
```

produces:

```text
Execution Plan

1.
Application:
VS Code

Executable:
Code.exe

Arguments:
D:\Projects\Deepfake


2.
Application:
Browser

Executable:
browser.exe

Arguments:
http://localhost:5173
```

---

## Phase 4 Completion

The execution engine should generate a complete plan without launching applications.

---

# 8. Phase 5 — Application Launching

## Goal

Implement real Windows process launching.

The launcher receives:

```text
Execution Plan
```

and starts the required applications.

---

## Tasks

### 1. Process Launcher

Create a dedicated module responsible for:

```text
Executable Path
Arguments
Working Directory
```

Conceptually:

```text
ApplicationLauncher
```

receives:

```text
LaunchAction
```

and starts the process.

---

### 2. Launch Normal Applications

Example:

```text
/v
```

should launch:

```text
VS Code
```

normally.

---

### 3. Launch Project Applications

Example:

```text
deepfake /v
```

should launch:

```text
VS Code
```

with:

```text
D:\Projects\Deepfake
```

---

### 4. Launch Project Folder

Example:

```text
deepfake
```

should open:

```text
D:\Projects\Deepfake
```

using Windows Explorer.

---

### 5. Launch Multiple Applications

Example:

```text
/a /v /b
```

should initiate all applications.

The launcher should not wait for one application to fully finish before starting the next.

---

## Phase 5 Completion

The complete command pipeline should work from backend logic:

```text
Command
    ↓
Parser
    ↓
Resolver
    ↓
Execution Plan
    ↓
Windows Applications Launch
```

At this point, the system can be tested without the final UI.

---

# 9. Phase 6 — Launcher UI

## Goal

Build the primary command launcher interface.

The UI must follow:

```text
UI_UX.md
```

---

## Tasks

### 1. Create Minimal Launcher Window

The launcher should contain:

```text
Search Input
Close Button
```

Concept:

```text
┌───────────────────────────────────────┐
│ 🔍 Type a command...              ✕  │
└───────────────────────────────────────┘
```

---

### 2. Window Configuration

Configure:

```text
No traditional window frame
Small window
Centered
Always on top when active
Input automatically focused
```

The background outside the launcher should remain untouched.

---

### 3. Input Handling

Support:

```text
Typing
Enter
Escape
Arrow Keys
Tab
```

---

### 4. Connect UI to Backend

The UI should:

```text
Input
    ↓
Invoke Rust Command
    ↓
Receive Result
```

For valid execution:

```text
Launch
    ↓
Close Launcher
```

For errors:

```text
Display Error
    ↓
Keep Launcher Open
```

---

## Phase 6 Completion

The user should be able to manually open the application and run commands.

Example:

```text
deepfake /v
```

---

# 10. Phase 7 — Project Configuration

## Goal

Implement:

```text
add-project
```

---

## Tasks

### 1. Detect System Command

Input:

```text
add-project
```

should not go to normal command execution.

Instead:

```text
Open Project Configuration
```

---

### 2. Create Project Form

Required:

```text
Project Name
Project Command
Project Path
```

Optional:

```text
Project URL
Run Commands
Working Directory
```

---

### 3. Native Folder Picker

The user should select:

```text
Project Directory
```

using the Windows folder picker.

---

### 4. Validation

Validate:

```text
Command uniqueness
Reserved commands
Project path exists
Project path is a directory
```

---

### 5. Save Project

The project should persist in storage.

---

## Phase 7 Completion

The user can:

```text
Alt + Space
    ↓
add-project
    ↓
Create Project
    ↓
Save
```

Then:

```text
Alt + Space
    ↓
deepfake
```

should resolve the created project.

---

# 11. Phase 8 — Application Configuration

## Goal

Implement:

```text
add-app
```

---

## Tasks

### 1. Detect System Command

Input:

```text
add-app
```

opens the application configuration window.

---

### 2. Create Application Form

Required:

```text
Application Name
Application Command
Executable Path
```

Optional:

```text
Normal Launch Arguments
Project Launch Arguments
Working Directory
```

---

### 3. Native File Picker

Allow the user to select:

```text
.exe
```

files.

---

### 4. Configure Project Context

Allow arguments such as:

```text
{PROJECT_PATH}
```

Example:

```text
VS Code
```

Project arguments:

```text
{PROJECT_PATH}
```

---

### 5. Save Application

Validate:

```text
Command uniqueness
Executable exists
Valid slash syntax
```

Save to persistent storage.

---

## Phase 8 Completion

The user can create:

```text
VS Code
→ /v
```

and immediately use:

```text
/v
```

or:

```text
deepfake /v
```

---

# 12. Phase 9 — Application Groups

## Goal

Implement:

```text
//
```

---

## Tasks

### 1. Group Storage

Create and store:

```text
Default Application Group
```

---

### 2. Add Applications to Group

The group stores:

```text
Application IDs
Execution Order
```

---

### 3. Resolve Group

Input:

```text
//
```

resolves:

```text
Default Group
```

---

### 4. Project Group Execution

Input:

```text
deepfake //
```

should:

```text
Resolve Project
    ↓
Resolve Default Group
    ↓
Apply Project Context
    ↓
Launch Applications
```

---

## Phase 9 Completion

The user can launch an entire configured workspace with:

```text
//
```

or:

```text
deepfake //
```

---

# 13. Phase 10 — Global Shortcut

## Goal

Implement:

```text
Alt + Space
```

to activate the launcher.

---

## Important Constraint

A completely terminated application cannot detect:

```text
Alt + Space
```

Therefore, the application requires a mechanism that remains available to detect the shortcut.

The implementation should choose the lightest acceptable approach.

Possible architecture:

```text
Small Background Launcher Process
    ↓
Wait for Alt + Space
    ↓
Show Launcher Window
```

The launcher window itself should not remain visible.

The user experience remains:

```text
Application appears only when needed
```

---

## Tasks

### 1. Register Global Shortcut

Register:

```text
Alt + Space
```

---

### 2. Shortcut Trigger

When pressed:

```text
Show Launcher
Focus Window
Clear Previous Input
Focus Search Input
```

---

### 3. Launcher Close Behavior

After:

```text
Successful Execution
```

the launcher UI closes or hides.

The shortcut listener remains available.

---

## Phase 10 Completion

The complete user experience works:

```text
Working on Desktop
    ↓
Alt + Space
    ↓
Launcher Appears
    ↓
Command
    ↓
Enter
    ↓
Applications Launch
    ↓
Launcher Disappears
```

---

# 14. Phase 11 — Error Handling and Testing

## Goal

Make the application reliable.

---

## Command Tests

Test:

```text
deepfake

/v

/v /b

deepfake /v

deepfake /v /b

//

deepfake //

add-project

add-app
```

---

## Invalid Command Tests

Test:

```text
unknownproject

deepfake /x

///

/v deepfake

// /v

deepfake // /v
```

---

## Data Validation Tests

Test:

```text
Duplicate Project Commands

Duplicate Application Commands

Invalid Executable Paths

Invalid Project Paths

Reserved Commands
```

---

## Execution Tests

Test:

```text
Single Application

Multiple Applications

Project Application

Project Group

Application Without Project Support
```

---

# 15. Phase 12 — Packaging

## Goal

Prepare the application for normal Windows usage.

---

## Tasks

### 1. Production Build

Create a production build.

---

### 2. Windows Installer

Create an installable Windows package.

---

### 3. Startup Behavior

Decide whether the lightweight shortcut listener should start automatically when Windows starts.

This should be optional.

Possible setting:

```text
Start Project Launcher with Windows
```

---

### 4. Test Installed Application

Test:

```text
Install
Restart Windows
Trigger Shortcut
Launch Projects
Launch Applications
Save Data
Restart Application
Verify Data
```

---

# 16. Recommended Development Order

The exact development sequence should be:

```text
STEP 1
Tauri + React Foundation

STEP 2
Rust Data Models

STEP 3
Persistent Storage

STEP 4
Project CRUD

STEP 5
Application CRUD

STEP 6
Command Parser

STEP 7
Command Resolver

STEP 8
Execution Planner

STEP 9
Windows Process Launcher

STEP 10
Minimal Launcher UI

STEP 11
Connect UI + Backend

STEP 12
Project Configuration UI

STEP 13
Application Configuration UI

STEP 14
Application Groups

STEP 15
Global Shortcut

STEP 16
Testing

STEP 17
Production Packaging
```

---

# 17. Development Milestones

## Milestone 1 — Backend Core

Working:

```text
Projects
Applications
Storage
Command Parsing
```

No final UI required.

---

## Milestone 2 — Launching Engine

Working:

```text
Command
    ↓
Execution Plan
    ↓
Windows Applications Launch
```

---

## Milestone 3 — Minimal Product

Working:

```text
Open App
    ↓
Type Command
    ↓
Launch Application
    ↓
Close
```

---

## Milestone 4 — Configuration

Working:

```text
add-project
add-app
```

---

## Milestone 5 — Workspace Launching

Working:

```text
//
```

and:

```text
project //
```

---

## Milestone 6 — Final Experience

Working:

```text
Alt + Space
    ↓
Launcher
    ↓
Command
    ↓
Execution
    ↓
Disappear
```

---

# 18. Definition of Version 1 Complete

Version 1 is complete when the user can:

```text
1. Press Alt + Space

2. Type a project name

3. Open the project folder

4. Open a configured application

5. Open multiple applications

6. Open a project in configured applications

7. Use // to launch the default application group

8. Use project // to launch a project workspace

9. Add projects

10. Add applications

11. Restart the application without losing data

12. Receive clear errors for invalid commands

13. Automatically close the launcher after successful execution
```

---

# 19. Things Explicitly Not Included in Version 1

To prevent scope expansion, Version 1 does not include:

```text
Cloud Sync

User Accounts

Online Database

Project Collaboration

Plugin System

AI Command Interpretation

Natural Language Commands

Complex Automation

Project Templates

Task Management

Analytics

Multiple User Profiles
```

These can be considered later.

---

# 20. Final Development Principle

The project should always be developed according to:

```text
Simple
    ↓
Working
    ↓
Reliable
    ↓
Polished
```

We should not build a visually complex launcher before the command and execution engine works.

The most important path is:

```text
Alt + Space
        ↓
Type Command
        ↓
Parse
        ↓
Resolve
        ↓
Launch
        ↓
Close
```

Everything else supports this experience.

---

# Final Architecture

The complete Version 1 system is:

```text
                    USER
                      │
                      │ Alt + Space
                      ▼
              ┌──────────────┐
              │   LAUNCHER   │
              │   React UI   │
              └──────┬───────┘
                     │
                     ▼
              COMMAND PARSER
                     │
                     ▼
               COMMAND TYPE
                     │
          ┌──────────┼──────────┐
          │          │          │
          ▼          ▼          ▼
       Project     Apps       Group
          │          │          │
          └──────────┼──────────┘
                     ▼
                  RESOLVER
                     │
                     ▼
              EXECUTION PLAN
                     │
                     ▼
              RUST LAUNCHER
                     │
                     ▼
               WINDOWS OS
                     │
                     ▼
             Apps / Projects Open
                     │
                     ▼
              LAUNCHER CLOSES
```

The complete product is built around one promise:

> Press one shortcut, type one command, open everything you need, and immediately return to your work.
