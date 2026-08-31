````md
# COMMANDS.md

# Universal Project Launcher — Command System

## 1. Purpose

This document defines the command language used by the Universal Project Launcher.

The command system is the primary way users interact with the application.

The user opens the launcher and enters a command such as:

```text
deepfake /v /b
````

The launcher interprets the command, determines what the user wants to open, executes the required actions, and closes.

The command system must remain:

* Simple
* Predictable
* Easy to type
* Easy to extend
* Consistent

Commands should have one clear meaning.

---

# 2. Core Command Concepts

The command system contains four primary concepts:

```text
1. Project
2. Application Command
3. Application Group
4. System Command
```

Conceptually:

```text
PROJECT
    +
APPLICATION COMMANDS
```

Example:

```text
deepfake /v /b
```

Or:

```text
APPLICATION COMMANDS ONLY
```

Example:

```text
/a /v /b
```

Or:

```text
PROJECT
    +
APPLICATION GROUP
```

Example:

```text
deepfake //
```

---

# 3. Project Command

A project is referenced using its registered command name.

Example registered project:

```text
Project Name:
Deepfake

Command Name:
deepfake

Project Path:
D:\Projects\Deepfake
```

The project can be referenced by typing:

```text
deepfake
```

The project name acts as context for the rest of the command.

Example:

```text
deepfake /v
```

The launcher understands:

```text
Project:
deepfake

Application:
/v
```

The project command itself does not begin with `/`.

---

# 4. Application Command

An application command always starts with a single slash.

Format:

```text
/COMMAND
```

Examples:

```text
/v
/a
/b
/p
/t
```

Example application registry:

```text
/v → VS Code
/a → Android Studio
/b → Browser
/p → Postman
/t → Terminal
```

The actual command names are configurable by the user.

The application command does not need to be limited to one letter.

Examples of valid command styles:

```text
/code
/browser
/terminal
/android
/postman
```

However, short commands are recommended for fast usage.

---

# 5. Launching One Application

Example:

```text
/v
```

Meaning:

```text
Open VS Code normally.
```

Example:

```text
/b
```

Meaning:

```text
Open the configured browser normally.
```

No project context is used.

---

# 6. Launching Multiple Applications

Multiple application commands can be entered in one command.

Example:

```text
/a /v /b
```

Meaning:

```text
Open Android Studio
Open VS Code
Open Browser
```

The command is interpreted as multiple independent application launch requests.

Conceptually:

```text
/a
+
/v
+
/b
```

Each application launches normally because no project was specified.

---

# 7. Project + Application Command

A project can be combined with one or more application commands.

Example:

```text
deepfake /v
```

Meaning:

```text
Project:
deepfake

Application:
/v
```

The launcher should:

```text
1. Find the Deepfake project
2. Find the application assigned to /v
3. Determine how /v uses project context
4. Launch the application
```

Example:

```text
deepfake /v
```

May result in:

```text
VS Code
→ Opens:
D:\Projects\Deepfake
```

---

# 8. Project + Multiple Applications

A project can be combined with multiple applications.

Example:

```text
deepfake /a /v /b
```

Meaning:

```text
Project:
deepfake

Applications:
/a
/v
/b
```

The launcher should resolve the project once and provide project context to every application where applicable.

Conceptually:

```text
Deepfake Project
        │
        ├── /a
        │
        ├── /v
        │
        └── /b
```

Each application may use the project differently.

Example:

```text
/a
→ Project Path

/v
→ Project Path

/b
→ Project URL
```

The application's project behavior is configurable.

---

# 9. Application Group Command

The double slash command:

```text
//
```

represents an application group.

It does not represent a single application.

Example:

```text
//
```

The launcher should:

```text
1. Find the configured default application group
2. Resolve the applications in the group
3. Launch all applications
```

Example group:

```text
Development Group

/v → VS Code
/b → Browser
/p → Postman
/t → Terminal
```

Running:

```text
//
```

is conceptually equivalent to:

```text
/v /b /p /t
```

The group exists to provide a fast shortcut for launching a complete workspace.

---

# 10. Project + Application Group

A project can be combined with the application group command.

Example:

```text
deepfake //
```

Meaning:

```text
Project:
deepfake

Application Group:
Default Group
```

The launcher should:

```text
1. Find the Deepfake project
2. Find the default application group
3. Resolve all applications in the group
4. Apply project context to compatible applications
5. Launch applications
```

Example:

```text
deepfake //
```

May produce:

```text
VS Code
→ Opens Deepfake project

Browser
→ Opens Deepfake URL

Terminal
→ Opens in Deepfake directory

Postman
→ Opens normally
```

The exact behavior depends on each application's configuration.

---

# 11. Difference Between `/` and `//`

This distinction is fundamental.

## Single Slash

```text
/v
```

Means:

```text
Launch one registered application.
```

Example:

```text
/a /v /b
```

Means:

```text
Launch these specific applications.
```

---

## Double Slash

```text
//
```

Means:

```text
Launch the configured application group.
```

Example:

```text
deepfake //
```

Means:

```text
Launch the configured application group
using the Deepfake project as context.
```

The two syntaxes must never have overlapping meanings.

---

# 12. Project-Only Command

A user may enter only a project name.

Example:

```text
deepfake
```

The final behavior of a project-only command must be explicitly defined.

For Version 1, the default behavior is:

```text
deepfake
```

Result:

```text
Open the project's default location.
```

The project's default location is:

```text
Project Path
```

On Windows, this means opening the project folder in the system file explorer.

Example:

```text
deepfake
```

Result:

```text
Open:

D:\Projects\Deepfake
```

This provides useful behavior without requiring an application command.

Project-specific default actions may be considered in future versions but are not part of the Version 1 command system.

---

# 13. System Commands

System commands are special commands used to manage the launcher itself.

Examples:

```text
add-project
add-app
```

System commands do not begin with `/`.

Example:

```text
add-project
```

The launcher should:

```text
Recognize command
        ↓
Stop normal command execution
        ↓
Open Add Project interface
```

Example:

```text
add-app
```

The launcher should:

```text
Recognize command
        ↓
Open Add Application interface
```

System commands are reserved keywords.

A project should not be allowed to use the same command name as a reserved system command.

---

# 14. Reserved Commands

The following command names are reserved by the system:

```text
add-project
add-app
```

These commands cannot be used as:

```text
Project Commands
```

Additional reserved commands may be added only when a new system-level feature is intentionally introduced.

Reserved commands should remain limited.

---

# 15. Command Grammar

The Version 1 command structure can be represented as:

```text
PROJECT?
APPLICATION_COMMAND*
```

or:

```text
PROJECT?
GROUP_COMMAND
```

or:

```text
SYSTEM_COMMAND
```

Conceptually:

```text
COMMAND
│
├── System Command
│
├── Project
│     │
│     ├── Nothing
│     │
│     ├── Application Commands
│     │
│     └── Group Command
│
└── Application Commands
      │
      └── No Project Context
```

---

# 16. Valid Command Examples

## Open Project Folder

```text
deepfake
```

---

## Open One Application

```text
/v
```

---

## Open Multiple Applications

```text
/v /b
```

```text
/a /v /b
```

---

## Open Project in One Application

```text
deepfake /v
```

---

## Open Project in Multiple Applications

```text
deepfake /a /v /b
```

---

## Open Default Application Group

```text
//
```

---

## Open Project Workspace

```text
deepfake //
```

---

## Add a Project

```text
add-project
```

---

## Add an Application

```text
add-app
```

---

# 17. Invalid Commands

The system must detect invalid commands before execution.

## Unknown Project

Example:

```text
unknownproject /v
```

Result:

```text
Error:
Project "unknownproject" was not found.
```

No application should launch.

---

## Unknown Application

Example:

```text
deepfake /x
```

If `/x` does not exist:

```text
Error:
Application command "/x" was not found.
```

No application should launch.

---

## Duplicate Application Commands

Example:

```text
deepfake /v /v
```

For Version 1, duplicate commands should be ignored after the first occurrence.

Conceptually:

```text
/v /v /v
```

Becomes:

```text
/v
```

This prevents the same application from being launched multiple times accidentally.

---

## Invalid Slash Syntax

Examples:

```text
///
////
```

These are invalid in Version 1.

Only:

```text
/command
```

and:

```text
//
```

are valid slash formats.

---

## Group Mixed With Individual Apps

For Version 1:

```text
deepfake // /v
```

is invalid.

Similarly:

```text
// /v
```

is invalid.

Reason:

```text
//
```

already represents a complete application group.

Mixing a group with individual applications introduces unnecessary ambiguity.

The user should either use:

```text
//
```

or:

```text
/v /b /p
```

but not both in the same command.

---

## Multiple Projects

Example:

```text
deepfake launcher /v
```

is invalid if both:

```text
deepfake
```

and:

```text
launcher
```

are recognized project names.

Version 1 supports only one project per command.

---

# 18. Command Order

The recommended command order is:

```text
PROJECT FIRST
```

followed by:

```text
APPLICATION COMMANDS
```

Example:

```text
deepfake /v /b
```

This is valid.

The following format is not supported:

```text
/v deepfake
```

The project must appear before application commands.

This keeps parsing simple and predictable.

---

# 19. Whitespace Rules

Commands should ignore unnecessary extra spaces.

Example:

```text
deepfake     /v     /b
```

Should be interpreted as:

```text
deepfake /v /b
```

Leading and trailing spaces should also be ignored.

Example:

```text
   deepfake /v /b
```

Should behave normally.

---

# 20. Case Handling

For Version 1, command matching should be case-insensitive.

Example:

```text
DEEPFAKE /V
```

Should behave the same as:

```text
deepfake /v
```

Stored project names and application display names may preserve their original capitalization.

Example:

```text
Command:
deepfake

Display Name:
Deepfake
```

---

# 21. Command Resolution Priority

The launcher should interpret commands using the following priority.

```text
1. System Command
2. Project Command
3. Application Command
4. Group Command
```

Example:

```text
add-project
```

The launcher first checks whether the entire input matches a system command.

Example:

```text
deepfake /v
```

The launcher:

```text
1. Checks system commands
2. Identifies deepfake as a project
3. Identifies /v as an application command
```

---

# 22. Parsing Flow

Example input:

```text
deepfake /v /b
```

Parsing process:

```text
Raw Input
deepfake /v /b
        ↓
Normalize Whitespace
        ↓
Tokenize

deepfake
/v
/b
        ↓
Identify Command Types

deepfake → Project
/v       → Application
/b       → Application
        ↓
Validate
        ↓
Structured Command
```

Conceptually:

```text
{
  type: "project-applications",

  project: "deepfake",

  applications: [
    "/v",
    "/b"
  ]
}
```

---

# 23. Group Parsing Flow

Example:

```text
deepfake //
```

Parsing:

```text
Raw Input
deepfake //
        ↓
Tokenize

deepfake
//
        ↓
Identify

deepfake
→ Project

//
→ Application Group
        ↓
Structured Command
```

Conceptually:

```text
{
  type: "project-group",

  project: "deepfake",

  group: "default"
}
```

---

# 24. System Command Parsing

Example:

```text
add-project
```

Parsing:

```text
Raw Input
add-project
        ↓
Check Reserved Commands
        ↓
Match Found
        ↓
System Action
```

Conceptually:

```text
{
  type: "system",

  command: "add-project"
}
```

---

# 25. Command Execution Rule

The command parser only determines:

```text
What the user requested.
```

The command parser does not:

```text
Launch applications
Open files
Access executables
Manage processes
```

The flow is:

```text
User Command
      ↓
Command Parser
      ↓
Structured Command
      ↓
Resolver
      ↓
Execution Planner
      ↓
Application Launcher
```

This separation must remain consistent.

---

# 26. Version 1 Command Reference

| Input               | Meaning                                      |
| ------------------- | -------------------------------------------- |
| `deepfake`          | Open project folder                          |
| `/v`                | Open configured application                  |
| `/a /v /b`          | Open multiple configured applications        |
| `deepfake /v`       | Open project in application                  |
| `deepfake /a /v /b` | Open project across multiple applications    |
| `//`                | Open default application group               |
| `deepfake //`       | Open project using default application group |
| `add-project`       | Open Add Project interface                   |
| `add-app`           | Open Add Application interface               |

---

# 27. Command System Principles

The command system must follow these principles.

## One Command, One Meaning

Every command must have predictable behavior.

---

## Project First

When a project is present, it must appear before application commands.

---

## Single Slash = Application

```text
/v
```

Always represents one registered application.

---

## Double Slash = Group

```text
//
```

Always represents the configured application group.

---

## No Hidden Interpretation

The system should not guess what the user means.

If a command is invalid:

```text
Show an error.
```

Do not silently execute something different.

---

## Configurable Commands

Application commands should be user-defined.

Example:

```text
/v
```

is not permanently tied to VS Code.

The user may configure:

```text
/v → VS Code
```

or another command:

```text
/code → VS Code
```

---

# 28. Future Command Extensions

The Version 1 command system is intentionally limited.

Future versions may introduce commands such as:

```text
edit-project
edit-app
delete-project
delete-app
settings
```

Or project actions such as:

```text
deepfake run
deepfake terminal
```

These are not part of Version 1 unless explicitly added later.

The current command system should remain stable:

```text
PROJECT

/APP

//

SYSTEM COMMAND
```

---

# 29. Final Command Model

The complete Version 1 command model is:

```text
SYSTEM COMMAND

add-project
add-app
```

```text
PROJECT

deepfake
```

```text
APPLICATION

/v
/a
/b
```

```text
MULTIPLE APPLICATIONS

/a /v /b
```

```text
PROJECT + APPLICATIONS

deepfake /a /v /b
```

```text
APPLICATION GROUP

//
```

```text
PROJECT + APPLICATION GROUP

deepfake //
```

The command system is built around one central idea:

> The user describes what they want to open in one line, and the launcher converts that request into a predictable set of actions.

```
```
