````md
# UI_UX.md

# Universal Project Launcher — UI & UX Specification

## 1. Purpose

This document defines the user interface and user experience of the Universal Project Launcher.

The application is designed around one primary interaction:

```text
Press Shortcut
        ↓
Launcher Appears
        ↓
Type Command
        ↓
Execute
        ↓
Launcher Disappears
````

The launcher should feel:

* Fast
* Minimal
* Native
* Focused
* Distraction-free

The application is not intended to behave like a traditional desktop application with multiple screens always visible.

Its primary interface is a command launcher.

---

# 2. Core UI Philosophy

The main launcher should contain:

```text
One Search Bar
```

The user should not see:

* Sidebars
* Navigation menus
* Large dashboards
* Persistent windows
* Complex controls

The primary experience should feel similar to:

```text
macOS Spotlight
```

or:

```text
Windows PowerToys Run
```

but designed specifically for project and application launching.

The launcher should appear temporarily and disappear after completing its task.

---

# 3. Main Launcher Window

The default launcher window contains:

```text
┌──────────────────────────────────────────────┐
│                                              │
│   🔍  Type a command...                  ✕   │
│                                              │
└──────────────────────────────────────────────┘
```

The window should contain:

```text
Search Input
Close Button
```

The search input is the primary focus.

Immediately after opening the launcher:

```text
Cursor Focus
        ↓
Search Input
```

The user should be able to type immediately without clicking anything.

---

# 4. Window Appearance

The launcher window should:

```text
Appear Centered
```

The window should:

* Have no unnecessary title bar
* Have no visible application frame
* Not behave like a traditional desktop window
* Appear above other applications
* Focus automatically
* Be small and compact

Conceptually:

```text
                    SCREEN

        ┌───────────────────────────┐
        │                           │
        │   🔍  deepfake /v /b  ✕  │
        │                           │
        └───────────────────────────┘
```

The rest of the desktop remains visible.

The launcher should not create a large background panel.

---

# 5. Transparency

The launcher should visually float above the desktop.

Conceptually:

```text
Desktop
────────────────────────────

        Launcher

     ┌───────────────┐
     │ 🔍 command    │
     └───────────────┘
```

The application should not create:

```text
Fullscreen Overlay
```

The background should remain visible.

Only the launcher component itself should be visible.

---

# 6. Initial State

When the user presses:

```text
Alt + Space
```

The launcher should:

```text
1. Appear
2. Move to foreground
3. Focus the input
4. Clear previous command
5. Display placeholder text
```

Example:

```text
┌──────────────────────────────────────────────┐
│ 🔍  Type a command...                    ✕   │
└──────────────────────────────────────────────┘
```

Placeholder text:

```text
Type a command...
```

Alternative examples may be:

```text
Open project or app...
```

```text
Type a project or command...
```

The final wording should remain short.

---

# 7. Typing a Command

Example:

```text
deepfake /v /b
```

The UI becomes:

```text
┌──────────────────────────────────────────────┐
│ 🔍  deepfake /v /b                       ✕   │
└──────────────────────────────────────────────┘
```

The input should behave like a normal text input.

The user should be able to:

* Type
* Delete
* Paste
* Select text
* Use arrow keys

---

# 8. Suggestions

Suggestions should appear below the search bar when useful.

Example input:

```text
deep
```

UI:

```text
┌──────────────────────────────────────────────┐
│ 🔍  deep                                      │
├──────────────────────────────────────────────┤
│     Deepfake Forensic AI                      │
│     Deep Learning Project                     │
└──────────────────────────────────────────────┘
```

Suggestions should be subtle.

The launcher should not feel like a large dropdown application.

---

# 9. Project Suggestions

When the user types:

```text
deep
```

The launcher may display matching projects.

Example:

```text
┌──────────────────────────────────────────────┐
│ 🔍  deep                                      │
├──────────────────────────────────────────────┤
│     Deepfake Forensic AI                      │
│     command: deepfake                         │
└──────────────────────────────────────────────┘
```

Selecting the suggestion inserts:

```text
deepfake
```

into the search input.

---

# 10. Application Suggestions

When the user types:

```text
/
```

The launcher should show available application commands.

Example:

```text
┌──────────────────────────────────────────────┐
│ 🔍  /                                         │
├──────────────────────────────────────────────┤
│     /v      VS Code                           │
│     /b      Browser                           │
│     /a      Android Studio                    │
│     /t      Terminal                          │
│     //      Default Workspace                 │
└──────────────────────────────────────────────┘
```

This helps users remember commands.

---

# 11. Context-Aware Suggestions

Suggestions should understand the current command state.

Example:

```text
deepfake /
```

Suggestions:

```text
/v    VS Code
/b    Browser
/a    Android Studio
/t    Terminal
//    Default Workspace
```

The project context should remain in the input.

Selecting:

```text
/v
```

produces:

```text
deepfake /v
```

---

# 12. Keyboard Navigation

The launcher should be fully usable without a mouse.

Supported keys:

```text
Arrow Up
Arrow Down
Enter
Escape
Tab
```

---

## Arrow Up

Moves the suggestion selection upward.

---

## Arrow Down

Moves the suggestion selection downward.

---

## Enter

Behavior depends on state.

If a suggestion is selected:

```text
Accept Suggestion
```

Otherwise:

```text
Execute Command
```

---

## Tab

If a suggestion is available:

```text
Accept Suggestion
```

The input remains focused.

---

## Escape

Immediately closes the launcher.

No command is executed.

---

# 13. Close Button

The launcher contains a manual close button.

Example:

```text
┌──────────────────────────────────────────────┐
│ 🔍  deepfake /v /b                       ✕   │
└──────────────────────────────────────────────┘
```

Clicking:

```text
✕
```

closes the launcher.

It performs the same action as:

```text
Escape
```

---

# 14. Command Execution State

When the user presses:

```text
Enter
```

the launcher validates the command.

Example:

```text
deepfake /v /b
```

The flow:

```text
Enter
    ↓
Validation
    ↓
Command Resolution
    ↓
Execution Plan
    ↓
Launch Applications
    ↓
Close Launcher
```

The launcher should not display a long loading screen.

For fast operations, the user may only see:

```text
Launching...
```

briefly.

Example:

```text
┌──────────────────────────────────────────────┐
│                                              │
│        Launching workspace...                │
│                                              │
└──────────────────────────────────────────────┘
```

Then:

```text
Launcher Closes
```

---

# 15. Successful Execution

After a command is successfully sent for execution:

```text
deepfake /v /b
```

The launcher should close automatically.

The lifecycle:

```text
Alt + Space
    ↓
Launcher Appears
    ↓
User Types Command
    ↓
Enter
    ↓
Applications Start
    ↓
Launcher Closes
```

The launcher should not remain visible after successful execution.

---

# 16. Invalid Command State

Example:

```text
deepfake /x
```

If `/x` does not exist:

```text
┌──────────────────────────────────────────────┐
│ 🔍  deepfake /x                               │
├──────────────────────────────────────────────┤
│ ⚠ Application command "/x" was not found     │
└──────────────────────────────────────────────┘
```

The launcher remains open.

The user can immediately correct the command.

The error should:

* Be clear
* Be short
* Explain the problem
* Not use technical stack traces

---

# 17. Project Not Found

Example:

```text
unknownproject /v
```

Display:

```text
┌──────────────────────────────────────────────┐
│ 🔍  unknownproject /v                         │
├──────────────────────────────────────────────┤
│ ⚠ Project "unknownproject" was not found     │
└──────────────────────────────────────────────┘
```

The launcher remains open.

---

# 18. Add Project Flow

The user enters:

```text
add-project
```

and presses:

```text
Enter
```

The launcher transitions into project creation.

The recommended approach is:

```text
Separate Configuration Window
```

Flow:

```text
Main Launcher
      ↓
add-project
      ↓
Enter
      ↓
Main Launcher Closes
      ↓
Add Project Window Opens
```

This keeps the main launcher minimal.

---

# 19. Add Project Window

The Add Project window should contain only the required information.

Conceptually:

```text
┌──────────────────────────────────────────────┐
│ Add Project                                  │
│                                              │
│ Project Name                                 │
│ [____________________________]               │
│                                              │
│ Command                                      │
│ [____________________________]               │
│                                              │
│ Project Folder                               │
│ [________________________] [Browse]          │
│                                              │
│ Project URL (Optional)                       │
│ [____________________________]               │
│                                              │
│                 [ Cancel ] [ Save ]          │
└──────────────────────────────────────────────┘
```

Required fields:

```text
Project Name
Project Command
Project Path
```

Optional fields:

```text
Project URL
Run Commands
Working Directory
```

The interface should prioritize the required fields.

---

# 20. Add Project Validation

Validation should happen before saving.

Example:

```text
Project Command:
deepfake
```

If already used:

```text
⚠ Command "deepfake" is already assigned.
```

Example:

```text
Project Path:
D:\InvalidFolder
```

If the folder does not exist:

```text
⚠ The selected project folder does not exist.
```

Errors should appear near the relevant field.

---

# 21. Add Application Flow

The user enters:

```text
add-app
```

and presses:

```text
Enter
```

Flow:

```text
Main Launcher
      ↓
add-app
      ↓
Enter
      ↓
Main Launcher Closes
      ↓
Add Application Window Opens
```

---

# 22. Add Application Window

Conceptually:

```text
┌──────────────────────────────────────────────┐
│ Add Application                              │
│                                              │
│ Application Name                             │
│ [____________________________]               │
│                                              │
│ Command                                      │
│ [____________________________]               │
│                                              │
│ Application Executable                       │
│ [________________________] [Browse]          │
│                                              │
│ Normal Arguments                             │
│ [____________________________]               │
│                                              │
│ Project Launch                               │
│ [ Enable Project Context ]                   │
│                                              │
│                 [ Cancel ] [ Save ]          │
└──────────────────────────────────────────────┘
```

Required:

```text
Application Name
Application Command
Executable Path
```

Optional:

```text
Normal Arguments
Project Arguments
Working Directory
```

---

# 23. Folder and File Selection

The application should use native operating system dialogs.

For projects:

```text
Browse
    ↓
Windows Folder Picker
```

For applications:

```text
Browse
    ↓
Windows File Picker
    ↓
Select .exe
```

The user should not need to manually copy paths unless they want to.

---

# 24. Configuration Window Behavior

Configuration windows behave differently from the launcher.

They should:

```text
Remain Open
```

until:

```text
Save
```

or:

```text
Cancel
```

After successful save:

```text
Save
    ↓
Validate
    ↓
Store Data
    ↓
Close Window
```

The user can then reopen the launcher using:

```text
Alt + Space
```

---

# 25. Empty State

When no projects or applications exist, the launcher should still work.

Typing:

```text
/
```

with no applications configured:

```text
┌──────────────────────────────────────────────┐
│ 🔍  /                                         │
├──────────────────────────────────────────────┤
│ No applications configured                   │
│                                              │
│ Type "add-app" to add one                    │
└──────────────────────────────────────────────┘
```

Typing a project that does not exist:

```text
deepfake
```

should provide a clear error rather than silently doing nothing.

---

# 26. Search Input States

The search bar has several states.

## Idle

```text
🔍 Type a command...
```

---

## Typing

```text
🔍 deepfake /v
```

---

## Suggestion Active

```text
🔍 deep
────────────────────
> Deepfake Forensic AI
```

---

## Error

```text
🔍 deepfake /x
────────────────────
⚠ Application "/x" not found
```

---

## Executing

```text
Launching...
```

---

# 27. Window Size Philosophy

The launcher should remain compact.

Suggested behavior:

```text
No Suggestions
→ Smaller Height
```

```text
Suggestions Visible
→ Height Expands
```

Example:

```text
Normal:

┌───────────────────────────────┐
│ 🔍 Type command...        ✕  │
└───────────────────────────────┘
```

With suggestions:

```text
┌───────────────────────────────┐
│ 🔍 deep                    ✕  │
├───────────────────────────────┤
│ Deepfake Forensic AI          │
│ Deep Learning Project         │
└───────────────────────────────┘
```

The window should not be unnecessarily large.

---

# 28. Mouse Support

Although keyboard usage is the primary interaction, the UI should support the mouse.

The user should be able to:

* Click the input
* Click suggestions
* Click the close button
* Click Browse buttons
* Click Save
* Click Cancel

Mouse support should complement keyboard interaction.

---

# 29. Accessibility

The application should support:

* Keyboard navigation
* Visible focus states
* Clear error messages
* Readable text
* Sufficient contrast

The core launcher should not depend on icons alone.

For example:

```text
/v    VS Code
```

is better than only displaying a VS Code icon.

---

# 30. UI Animation

Animations should be minimal.

Recommended:

```text
Launcher Appears
→ Quick Fade / Scale
```

```text
Suggestions
→ Smooth Expansion
```

```text
Launcher Closes
→ Quick Fade
```

Animations should not slow down interaction.

The application should feel responsive.

---

# 31. Main User Journey

The primary user journey:

```text
USER WORKING ON DESKTOP
        ↓
Press Alt + Space
        ↓
Launcher Appears
        ↓
Input Automatically Focused
        ↓
Type:

deepfake //

        ↓
Press Enter
        ↓
VS Code Opens Project
Browser Opens Project URL
Terminal Opens Project Directory
Other Configured Apps Launch
        ↓
Launcher Closes
        ↓
USER CONTINUES WORKING
```

---

# 32. Add Project User Journey

```text
Alt + Space
        ↓
Launcher Appears
        ↓
Type:

add-project

        ↓
Enter
        ↓
Add Project Window Opens
        ↓
Enter:

Name
Command
Project Path

        ↓
Save
        ↓
Project Stored
        ↓
Window Closes
```

The project can now be used:

```text
Alt + Space
        ↓
deepfake /v
```

---

# 33. Add Application User Journey

```text
Alt + Space
        ↓
Type:

add-app

        ↓
Enter
        ↓
Add Application Window Opens
        ↓
Enter:

Application Name
Command
Executable Path

        ↓
Save
        ↓
Application Stored
```

The application can now be launched:

```text
Alt + Space
        ↓
/v
```

---

# 34. UI Principles

The UI must follow these principles.

## Minimal by Default

The launcher should only show what is necessary.

---

## Keyboard First

The entire primary workflow should be usable without touching the mouse.

---

## Instant Focus

The user should never need to click the search bar after opening the launcher.

---

## Temporary Presence

The launcher appears only when needed and disappears after completing its task.

---

## No Dashboard

Version 1 should not introduce a permanent dashboard or application home screen.

Configuration is accessed through commands.

---

## Errors Should Be Actionable

Instead of:

```text
Error 404
```

show:

```text
Application command "/x" was not found.
```

---

# 35. Final UI Summary

The main experience is:

```text
┌──────────────────────────────────────────────┐
│ 🔍  deepfake /v /b                       ✕   │
└──────────────────────────────────────────────┘
```

Optional suggestions:

```text
┌──────────────────────────────────────────────┐
│ 🔍  deepfake /                               │
├──────────────────────────────────────────────┤
│ /v       VS Code                             │
│ /b       Browser                             │
│ /t       Terminal                            │
│ //       Default Workspace                   │
└──────────────────────────────────────────────┘
```

The complete product experience is built around:

> Open instantly. Type once. Launch everything. Disappear.

The launcher should feel like a small utility rather than a traditional application.