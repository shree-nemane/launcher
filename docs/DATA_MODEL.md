````md
# DATA_MODEL.md

# Universal Project Launcher — Data Model

## 1. Purpose

This document defines the data structure of the Universal Project Launcher.

It answers:

- What data does the application store?
- What is required to add a project?
- What is required to add an application?
- How are application commands assigned?
- How do applications receive project context?
- How are application groups structured?
- What global settings exist?

This document defines **what data exists**, not how the UI displays it or how the data is physically stored internally.

---

# 2. Core Data Entities

Version 1 contains four primary data entities:

```text
Project
Application
Application Group
Settings
````

Relationship:

```text
                    SETTINGS
                       │
                       │
                       ▼
              DEFAULT APP GROUP
                       │
                       ▼
                APPLICATION GROUP
                       │
             ┌─────────┼─────────┐
             │         │         │
             ▼         ▼         ▼
          APP /v    APP /b    APP /t


PROJECT
   │
   │ Project Context
   │
   ├───────────────► Application
   │
   ├───────────────► Application
   │
   └───────────────► Application
```

Projects and applications are stored independently.

This is important because:

```text
One application
can be used with
many projects.
```

For example:

```text
VS Code
   │
   ├── Deepfake
   ├── Kumbh
   ├── Project Launcher
   └── Other Projects
```

The application configuration should not need to be recreated for every project.

---

# 3. Project

A Project represents a development project, workspace, or folder that the user wants to launch.

Example:

```text
Name:
Deepfake Forensic AI

Command:
deepfake

Path:
D:\Projects\Deepfake
```

The project provides context when combined with application commands.

Example:

```text
deepfake /v
```

The launcher finds the project and provides its information to VS Code.

---

# 4. Project Data Structure

Conceptually:

```text
Project
│
├── id
├── name
├── command
├── path
├── url
├── runCommands
├── workingDirectory
├── createdAt
└── updatedAt
```

Example:

```json
{
  "id": "project_deepfake",
  "name": "Deepfake Forensic AI",
  "command": "deepfake",
  "path": "D:\\Projects\\Deepfake",
  "url": "http://localhost:5173",
  "runCommands": [],
  "workingDirectory": "D:\\Projects\\Deepfake",
  "createdAt": "2026-08-30T00:00:00Z",
  "updatedAt": "2026-08-30T00:00:00Z"
}
```

The exact storage implementation may differ, but these fields define the logical model.

---

# 5. Project Fields

## 5.1 id

```text
Required
```

A unique internal identifier.

Example:

```text
project_deepfake
```

The user does not need to manually enter this.

It should be automatically generated when the project is created.

Purpose:

```text
Internal identification
Data updates
Data deletion
Relationships
```

---

## 5.2 name

```text
Required
```

The human-readable project name.

Example:

```text
Deepfake Forensic AI
```

This name is used in:

* UI
* Suggestions
* Project management
* Display messages

The name does not need to be the same as the command.

Example:

```text
Name:
Deepfake Forensic AI

Command:
deepfake
```

---

## 5.3 command

```text
Required
```

The command used to reference the project from the launcher.

Example:

```text
deepfake
```

Usage:

```text
deepfake /v
```

Rules:

* Must be unique
* Cannot contain `/`
* Cannot conflict with reserved system commands
* Case-insensitive matching
* Should be easy to type

Invalid examples:

```text
/deepfake
add-project
add-app
```

Valid examples:

```text
deepfake
kumbh
launcher
portfolio
```

---

## 5.4 path

```text
Required
```

The absolute path to the project directory.

Example:

```text
D:\Projects\Deepfake
```

The user should preferably select this using a folder picker rather than manually typing the path.

The launcher uses this path for:

```text
Open Project Folder
Open in VS Code
Open in Android Studio
Open Terminal
Working Directory
```

The path must exist when the project is created.

---

## 5.5 url

```text
Optional
```

A project-related URL.

Examples:

```text
http://localhost:5173
```

```text
http://localhost:3000
```

```text
https://example.com
```

The URL may be used by applications such as:

```text
Browser
```

Example:

```text
deepfake /b
```

If the Browser application is configured to use:

```text
PROJECT_URL
```

Then the project URL is opened.

A project does not require a URL.

---

## 5.6 runCommands

```text
Optional
```

A list of commands associated with starting the project.

Example:

```text
Frontend:
npm run dev
```

```text
Backend:
python main.py
```

Conceptually:

```json
[
  {
    "name": "Frontend",
    "command": "npm run dev"
  },
  {
    "name": "Backend",
    "command": "python main.py"
  }
]
```

For Version 1:

> Run commands are stored as project data but are not automatically executed by the basic project command system unless explicitly configured later.

This allows the data model to support future functionality without making project startup automation part of Version 1.

---

## 5.7 workingDirectory

```text
Optional
Default: Project Path
```

The directory from which project-related commands should execute.

Example:

```text
D:\Projects\Deepfake
```

If not provided:

```text
workingDirectory = path
```

This avoids requiring the user to enter the same path twice.

---

## 5.8 createdAt

```text
Automatically generated
```

Records when the project was created.

---

## 5.9 updatedAt

```text
Automatically updated
```

Records when the project was last modified.

---

# 6. Adding a Project

When the user enters:

```text
add-project
```

The application should collect the following information.

## Required Information

```text
1. Project Name
2. Project Command
3. Project Path
```

Example:

```text
Project Name:
Deepfake Forensic AI

Project Command:
deepfake

Project Path:
D:\Projects\Deepfake
```

## Optional Information

```text
Project URL
Run Commands
Custom Working Directory
```

The minimum valid project is:

```text
Name
+
Command
+
Existing Project Path
```

---

# 7. Project Validation

Before saving a project:

```text
Project Name
→ Cannot be empty

Project Command
→ Cannot be empty
→ Must be unique
→ Cannot start with /
→ Cannot use reserved commands

Project Path
→ Must exist
→ Must be a directory
```

Example invalid project:

```text
Name:
Deepfake

Command:
/deepfake
```

Reason:

```text
Project commands cannot begin with /
```

Example invalid project:

```text
Name:
Deepfake

Command:
add-project
```

Reason:

```text
Reserved system command
```

---

# 8. Application

An Application represents a program or executable that the launcher can start.

Examples:

```text
VS Code
Android Studio
Google Chrome
Postman
Windows Terminal
```

Each application has a user-defined command.

Example:

```text
VS Code
→ /v
```

The application configuration defines:

```text
How the application launches normally
```

and:

```text
How the application launches with project context
```

---

# 9. Application Data Structure

Conceptually:

```text
Application
│
├── id
├── name
├── command
├── executablePath
├── normalLaunch
├── projectLaunch
├── workingDirectory
├── icon
├── createdAt
└── updatedAt
```

Example:

```json
{
  "id": "app_vscode",
  "name": "VS Code",
  "command": "/v",
  "executablePath": "C:\\Users\\User\\AppData\\Local\\Programs\\Microsoft VS Code\\Code.exe",
  "normalLaunch": {
    "arguments": []
  },
  "projectLaunch": {
    "enabled": true,
    "arguments": ["{PROJECT_PATH}"]
  },
  "workingDirectory": null,
  "icon": null,
  "createdAt": "2026-08-30T00:00:00Z",
  "updatedAt": "2026-08-30T00:00:00Z"
}
```

---

# 10. Application Fields

## 10.1 id

```text
Required
Automatically generated
```

Unique internal identifier.

Example:

```text
app_vscode
```

---

## 10.2 name

```text
Required
```

Human-readable application name.

Example:

```text
VS Code
```

Used for:

* UI
* Suggestions
* Errors
* Application management

---

## 10.3 command

```text
Required
```

The launcher command assigned to the application.

Example:

```text
/v
```

Rules:

* Must begin with `/`
* Cannot be exactly `//`
* Must be unique
* Cannot contain spaces
* Case-insensitive matching

Valid:

```text
/v
/code
/browser
/android
```

Invalid:

```text
v
//
/vs code
```

---

## 10.4 executablePath

```text
Required
```

The path to the application executable.

Example:

```text
C:\Program Files\Microsoft VS Code\Code.exe
```

The user should preferably select the executable using a file picker.

The application should validate that the file exists before saving.

---

## 10.5 normalLaunch

```text
Required
```

Defines how the application launches when no project is provided.

Example command:

```text
/v
```

Example configuration:

```json
{
  "arguments": []
}
```

This means:

```text
Launch VS Code normally
```

Another example:

```json
{
  "arguments": ["--new-window"]
}
```

---

## 10.6 projectLaunch

```text
Optional
```

Defines how the application behaves when a project is provided.

Example:

```text
deepfake /v
```

VS Code configuration:

```json
{
  "enabled": true,
  "arguments": ["{PROJECT_PATH}"]
}
```

Result:

```text
Code.exe D:\Projects\Deepfake
```

Another example for Browser:

```json
{
  "enabled": true,
  "arguments": ["{PROJECT_URL}"]
}
```

Result:

```text
Browser.exe http://localhost:5173
```

---

# 11. Project Context Variables

Applications can use predefined project variables.

Version 1 supports:

```text
{PROJECT_PATH}
```

The project's main directory.

Example:

```text
D:\Projects\Deepfake
```

---

```text
{PROJECT_URL}
```

The project's configured URL.

Example:

```text
http://localhost:5173
```

---

```text
{PROJECT_NAME}
```

The human-readable project name.

Example:

```text
Deepfake Forensic AI
```

---

```text
{PROJECT_COMMAND}
```

The project's launcher command.

Example:

```text
deepfake
```

---

```text
{PROJECT_WORKING_DIRECTORY}
```

The configured project working directory.

Example:

```text
D:\Projects\Deepfake
```

Applications may use these variables in their launch arguments.

---

# 12. Application Project Support

Not every application needs project context.

Example:

```text
Postman
```

Configuration:

```json
{
  "enabled": false
}
```

Command:

```text
deepfake /p
```

Behavior:

```text
Open Postman normally
```

The project context does not cause an error.

The launcher simply opens the application using its normal launch configuration.

This allows application groups to contain applications that do not directly support projects.

---

# 13. Working Directory

Applications may optionally define a working directory.

Example:

```text
C:\SomeDirectory
```

For applications using project context, the application may also use:

```text
{PROJECT_PATH}
```

Example:

```json
{
  "workingDirectory": "{PROJECT_PATH}"
}
```

This is useful for:

```text
Terminal
Command Prompt
PowerShell
Project Scripts
```

---

# 14. Application Icon

```text
Optional
```

An application may have a custom icon.

The icon may be:

```text
Automatically detected
```

or:

```text
User-provided
```

For Version 1, automatic executable icon extraction is optional.

The application should still function without custom icon support.

---

# 15. Adding an Application

When the user enters:

```text
add-app
```

The application should collect:

## Required

```text
1. Application Name
2. Application Command
3. Executable Path
```

Example:

```text
Application Name:
VS Code

Command:
/v

Executable:
C:\Program Files\Microsoft VS Code\Code.exe
```

## Optional

```text
Normal Launch Arguments
Project Launch Configuration
Project Launch Arguments
Working Directory
Icon
```

---

# 16. Application Validation

Before saving:

## Application Name

```text
Cannot be empty
```

---

## Application Command

```text
Cannot be empty
Must start with /
Cannot equal //
Must be unique
Cannot contain spaces
```

---

## Executable Path

```text
Must exist
Must point to a valid file
```

---

# 17. Application Group

An Application Group represents a predefined collection of applications.

The command:

```text
//
```

launches the configured default application group.

Example:

```text
Development Workspace
```

Contains:

```text
/v → VS Code
/b → Browser
/p → Postman
/t → Terminal
```

---

# 18. Application Group Data Structure

Conceptually:

```text
ApplicationGroup
│
├── id
├── name
├── applications
├── executionOrder
├── createdAt
└── updatedAt
```

Example:

```json
{
  "id": "group_default",
  "name": "Default Workspace",
  "applications": [
    "app_vscode",
    "app_browser",
    "app_postman",
    "app_terminal"
  ],
  "executionOrder": [
    "app_vscode",
    "app_browser",
    "app_postman",
    "app_terminal"
  ],
  "createdAt": "2026-08-30T00:00:00Z",
  "updatedAt": "2026-08-30T00:00:00Z"
}
```

Applications are referenced using their internal IDs.

This prevents problems if:

```text
Application Name Changes
```

or:

```text
Application Command Changes
```

---

# 19. Group Execution Order

The group stores an execution order.

Example:

```text
1. VS Code
2. Terminal
3. Browser
4. Postman
```

The launcher should attempt to launch applications in this order.

However:

> Launching an application does not require waiting for the previous application to fully start.

The launcher initiates each process according to the execution plan.

The execution order provides predictable behavior without unnecessarily slowing down the launcher.

---

# 20. Default Application Group

Version 1 uses one default application group.

The command:

```text
//
```

always references:

```text
Default Application Group
```

The active default group is stored in Settings.

Example:

```text
//
        ↓
Default Group
        ↓
Development Workspace
```

Future versions may support named groups, but named group commands are outside Version 1.

---

# 21. Settings

Settings contain application-wide configuration.

Conceptually:

```text
Settings
│
├── globalShortcut
├── defaultApplicationGroupId
├── launcherPreferences
└── createdAt
```

Example:

```json
{
  "globalShortcut": "Alt+Space",
  "defaultApplicationGroupId": "group_default"
}
```

---

# 22. Settings Fields

## globalShortcut

The keyboard shortcut used to activate the launcher.

Default:

```text
Alt + Space
```

This setting exists in the data model so it can be configurable later.

---

## defaultApplicationGroupId

References the application group used by:

```text
//
```

Example:

```text
group_default
```

Only one group is active as the default at a time.

---

# 23. Complete Data Relationship

The complete model can be represented as:

```text
PROJECTS
│
├── Project A
│   ├── Path
│   ├── URL
│   └── Run Commands
│
├── Project B
│
└── Project C


APPLICATIONS
│
├── VS Code
│   ├── /v
│   ├── Normal Launch
│   └── Project Launch
│
├── Browser
│   ├── /b
│   ├── Normal Launch
│   └── Project Launch
│
└── Terminal
    ├── /t
    ├── Normal Launch
    └── Project Launch


APPLICATION GROUPS
│
└── Default Group
    │
    ├── VS Code
    ├── Browser
    └── Terminal


SETTINGS
│
├── Global Shortcut
└── Default Group
```

---

# 24. Complete Example

## Project

```json
{
  "id": "project_deepfake",
  "name": "Deepfake Forensic AI",
  "command": "deepfake",
  "path": "D:\\Projects\\Deepfake",
  "url": "http://localhost:5173",
  "runCommands": [],
  "workingDirectory": "D:\\Projects\\Deepfake"
}
```

---

## VS Code

```json
{
  "id": "app_vscode",
  "name": "VS Code",
  "command": "/v",
  "executablePath": "C:\\Program Files\\Microsoft VS Code\\Code.exe",

  "normalLaunch": {
    "arguments": []
  },

  "projectLaunch": {
    "enabled": true,
    "arguments": [
      "{PROJECT_PATH}"
    ]
  }
}
```

---

## Browser

```json
{
  "id": "app_browser",
  "name": "Browser",
  "command": "/b",
  "executablePath": "C:\\Program Files\\Browser\\browser.exe",

  "normalLaunch": {
    "arguments": []
  },

  "projectLaunch": {
    "enabled": true,
    "arguments": [
      "{PROJECT_URL}"
    ]
  }
}
```

---

## Default Group

```json
{
  "id": "group_default",
  "name": "Development Workspace",

  "applications": [
    "app_vscode",
    "app_browser"
  ],

  "executionOrder": [
    "app_vscode",
    "app_browser"
  ]
}
```

---

## Settings

```json
{
  "globalShortcut": "Alt+Space",
  "defaultApplicationGroupId": "group_default"
}
```

---

# 25. Example Command Resolution Using Data

Input:

```text
deepfake /v /b
```

The launcher resolves:

```text
Project:
deepfake
```

Find:

```text
project_deepfake
```

Then:

```text
/v
```

Find:

```text
app_vscode
```

Then replace:

```text
{PROJECT_PATH}
```

with:

```text
D:\Projects\Deepfake
```

Result:

```text
VS Code
→ D:\Projects\Deepfake
```

Then:

```text
/b
```

Find:

```text
app_browser
```

Replace:

```text
{PROJECT_URL}
```

with:

```text
http://localhost:5173
```

Result:

```text
Browser
→ http://localhost:5173
```

---

# 26. Data Model Principles

The data model follows these principles.

## Applications Are Reusable

One application can work with many projects.

---

## Projects Do Not Store Application Definitions

A project should not duplicate:

```text
VS Code executable path
Browser executable path
Terminal executable path
```

Applications are stored globally.

---

## Commands Are Configurable

The system should not permanently define:

```text
/v = VS Code
```

The user defines the mapping.

---

## Project Context Is Dynamic

Applications receive project information only when a project is part of the command.

---

## Minimum Required Data

A project requires:

```text
Name
Command
Path
```

An application requires:

```text
Name
Command
Executable Path
```

Everything else is optional.

---

# 27. Version 1 Data Model Summary

The final Version 1 entities are:

```text
Project
Application
Application Group
Settings
```

A Project contains:

```text
id
name
command
path
url
runCommands
workingDirectory
timestamps
```

An Application contains:

```text
id
name
command
executablePath
normalLaunch
projectLaunch
workingDirectory
icon
timestamps
```

An Application Group contains:

```text
id
name
applications
executionOrder
timestamps
```

Settings contain:

```text
globalShortcut
defaultApplicationGroupId
```

The central data principle is:

> Projects define what you are working on. Applications define what can be opened. Application groups define what opens together. Settings define global behavior.

```
```
