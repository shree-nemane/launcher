use crate::storage::StorageManager;
use tauri::menu::{MenuBuilder, MenuItemBuilder};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};

#[cfg(target_os = "windows")]
pub fn configure_webview2_environment() {
    let current_args = std::env::var("WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS").unwrap_or_default();
    let memory_flags = [
        "--in-process-gpu",
        "--enable-features=NetworkServiceInProcess",
        "--disable-features=CalculateNativeWinOcclusion,SpareRendererForSitePerProcess,AudioServiceOutOfProcess",
        "--disable-gpu-shader-disk-cache",
        "--disable-background-networking",
        "--disable-component-update",
        "--disable-domain-reliability",
        "--disable-sync",
        "--disable-speech-api",
        "--renderer-process-limit=1",
        "--enable-low-end-device-mode",
        "--process-per-site",
        "--js-flags=--max-old-space-size=64",
    ]
    .join(" ");

    let combined_args = if current_args.is_empty() {
        memory_flags
    } else {
        format!("{} {}", current_args, memory_flags)
    };

    std::env::set_var("WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS", combined_args);
}

#[cfg(not(target_os = "windows"))]
pub fn configure_webview2_environment() {}

#[cfg(target_os = "windows")]
pub fn trim_process_memory() {
    #[allow(non_snake_case)]
    #[repr(C)]
    struct PROCESSENTRY32W {
        dwSize: u32,
        cntUsage: u32,
        th32ProcessID: u32,
        th32DefaultHeapID: usize,
        th32ModuleID: u32,
        cntThreads: u32,
        th32ParentProcessID: u32,
        pcPriClassBase: i32,
        dwFlags: u32,
        szExeFile: [u16; 260],
    }

    extern "system" {
        fn GetCurrentProcess() -> *mut std::ffi::c_void;
        fn GetCurrentProcessId() -> u32;
        fn OpenProcess(dwDesiredAccess: u32, bInheritHandle: i32, dwProcessId: u32) -> *mut std::ffi::c_void;
        fn SetProcessWorkingSetSize(
            hProcess: *mut std::ffi::c_void,
            dwMinimumWorkingSetSize: usize,
            dwMaximumWorkingSetSize: usize,
        ) -> i32;
        fn CreateToolhelp32Snapshot(dwFlags: u32, th32ProcessID: u32) -> *mut std::ffi::c_void;
        fn Process32FirstW(hSnapshot: *mut std::ffi::c_void, lppe: *mut PROCESSENTRY32W) -> i32;
        fn Process32NextW(hSnapshot: *mut std::ffi::c_void, lppe: *mut PROCESSENTRY32W) -> i32;
        fn CloseHandle(hObject: *mut std::ffi::c_void) -> i32;
    }

    const TH32CS_SNAPPROCESS: u32 = 0x00000002;
    const PROCESS_SET_QUOTA: u32 = 0x0100;
    const PROCESS_QUERY_INFORMATION: u32 = 0x0400;

    unsafe {
        // 1. Trim the parent process
        let parent = GetCurrentProcess();
        SetProcessWorkingSetSize(parent, usize::MAX, usize::MAX);

        // 2. Enumerate and trim all child WebView2 processes (Manager, Renderer, etc.)
        let current_pid = GetCurrentProcessId();
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
        let invalid_handle = (-1isize) as *mut std::ffi::c_void;

        if !snapshot.is_null() && snapshot != invalid_handle {
            let mut entry: PROCESSENTRY32W = std::mem::zeroed();
            entry.dwSize = std::mem::size_of::<PROCESSENTRY32W>() as u32;

            if Process32FirstW(snapshot, &mut entry) != 0 {
                loop {
                    if entry.th32ParentProcessID == current_pid {
                        let child = OpenProcess(
                            PROCESS_SET_QUOTA | PROCESS_QUERY_INFORMATION,
                            0,
                            entry.th32ProcessID,
                        );
                        if !child.is_null() {
                            SetProcessWorkingSetSize(child, usize::MAX, usize::MAX);
                            CloseHandle(child);
                        }
                    }
                    if Process32NextW(snapshot, &mut entry) == 0 {
                        break;
                    }
                }
            }
            CloseHandle(snapshot);
        }
    }
}

#[cfg(not(target_os = "windows"))]
pub fn trim_process_memory() {}

#[cfg(target_os = "windows")]
pub fn suspend_webview(window: &tauri::WebviewWindow) {
    let _ = window.with_webview(|webview| {
        use webview2_com::Microsoft::Web::WebView2::Win32::ICoreWebView2_3;
        use windows_core::Interface;

        unsafe {
            let controller = webview.controller();
            if let Ok(core) = controller.CoreWebView2() {
                if let Ok(core3) = core.cast::<ICoreWebView2_3>() {
                    let _ = core3.TrySuspend(None);
                }
            }
        }
    });
}

#[cfg(not(target_os = "windows"))]
pub fn suspend_webview(_window: &tauri::WebviewWindow) {}

#[cfg(target_os = "windows")]
pub fn resume_webview(window: &tauri::WebviewWindow) {
    let _ = window.with_webview(|webview| {
        use webview2_com::Microsoft::Web::WebView2::Win32::ICoreWebView2_3;
        use windows_core::Interface;

        unsafe {
            let controller = webview.controller();
            if let Ok(core) = controller.CoreWebView2() {
                if let Ok(core3) = core.cast::<ICoreWebView2_3>() {
                    let _ = core3.Resume();
                }
            }
        }
    });
}

#[cfg(not(target_os = "windows"))]
pub fn resume_webview(_window: &tauri::WebviewWindow) {}

pub fn show_and_reset_launcher(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        resume_webview(&window);
        let _ = window.unminimize();
        let _ = window.show();
        let _ = window.center();
        let _ = window.set_focus();
        let _ = window.emit("launcher://show", ());
    }
}

pub fn show_and_navigate(app: &AppHandle, view: &str) {
    if let Some(window) = app.get_webview_window("main") {
        resume_webview(&window);
        let _ = window.unminimize();
        let _ = window.show();
        let _ = window.center();
        let _ = window.set_focus();
        let _ = window.emit("launcher://navigate", view);
    }
}

use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

static LAST_TOGGLE_MS: AtomicU64 = AtomicU64::new(0);
const DEBOUNCE_MS: u64 = 150;

fn current_time_millis() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

pub fn toggle_launcher(app: &AppHandle) {
    let now = current_time_millis();
    let last = LAST_TOGGLE_MS.load(Ordering::Relaxed);
    if now.saturating_sub(last) < DEBOUNCE_MS {
        return;
    }
    LAST_TOGGLE_MS.store(now, Ordering::Relaxed);

    if let Some(window) = app.get_webview_window("main") {
        let is_visible = window.is_visible().unwrap_or(false);
        let is_focused = window.is_focused().unwrap_or(false);

        if is_visible && is_focused {
            let _ = window.emit("launcher://hide", ());
            let _ = window.hide();
            suspend_webview(&window);
            trim_process_memory();
        } else {
            show_and_reset_launcher(app);
        }
    }
}

pub fn setup_system_tray(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let open_item = MenuItemBuilder::with_id("open_launcher", "Open Launcher (Alt+Space)")
        .build(app)?;
    let projects_item = MenuItemBuilder::with_id("manage_projects", "Manage Projects")
        .build(app)?;
    let apps_item = MenuItemBuilder::with_id("manage_apps", "Manage Applications")
        .build(app)?;
    let groups_item = MenuItemBuilder::with_id("manage_groups", "Manage Application Groups")
        .build(app)?;
    let help_item = MenuItemBuilder::with_id("help", "Help & Quickstart Guide")
        .build(app)?;
    let settings_item = MenuItemBuilder::with_id("settings", "Settings")
        .build(app)?;
    let quit_item = MenuItemBuilder::with_id("quit", "Quit Universal Launcher")
        .build(app)?;

    let menu = MenuBuilder::new(app)
        .item(&open_item)
        .separator()
        .item(&projects_item)
        .item(&apps_item)
        .item(&groups_item)
        .separator()
        .item(&settings_item)
        .item(&help_item)
        .separator()
        .item(&quit_item)
        .build()?;

    let builder = TrayIconBuilder::with_id("main_tray")
        .menu(&menu)
        .show_menu_on_left_click(false);

    let builder = if let Some(icon) = app.default_window_icon() {
        builder.icon(icon.clone())
    } else {
        builder
    };

    let _tray = builder
        .on_menu_event(|app, event| match event.id.as_ref() {
            "open_launcher" => {
                show_and_reset_launcher(app);
            }
            "manage_projects" => {
                show_and_navigate(app, "manageProjects");
            }
            "manage_apps" => {
                show_and_navigate(app, "manageApps");
            }
            "manage_groups" => {
                show_and_navigate(app, "manageGroups");
            }
            "settings" => {
                show_and_navigate(app, "settings");
            }
            "help" => {
                show_and_navigate(app, "help");
            }
            "quit" => {
                app.exit(0);
            }
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let app = tray.app_handle();
                toggle_launcher(app);
            }
        })
        .build(app)?;

    Ok(())
}

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ShortcutRegistrationStatus {
    pub requested: String,
    pub active: Option<String>,
    pub status: String,
    pub message: String,
}

pub fn register_global_shortcut_with_fallback(
    app: &AppHandle,
    requested_str: &str,
) -> ShortcutRegistrationStatus {
    let _ = app.global_shortcut().unregister_all();

    let clean_requested = if requested_str.trim().is_empty() {
        "Alt+Space"
    } else {
        requested_str.trim()
    };

    // 1. Attempt preferred shortcut
    if let Ok(shortcut) = clean_requested.parse::<Shortcut>() {
        let app_handle = app.clone();
        let result = app.global_shortcut().on_shortcut(shortcut, move |_app, _shortcut, event| {
            if event.state == ShortcutState::Pressed {
                toggle_launcher(&app_handle);
            }
        });

        if result.is_ok() {
            return ShortcutRegistrationStatus {
                requested: clean_requested.to_string(),
                active: Some(clean_requested.to_string()),
                status: "success".to_string(),
                message: format!("Registered global shortcut '{}'", clean_requested),
            };
        }
    }

    // 2. Conflict detected - attempt fallbacks
    let fallbacks = if clean_requested.eq_ignore_ascii_case("Alt+Space") {
        vec!["Alt+Shift+Space", "Ctrl+Alt+Space"]
    } else {
        vec!["Alt+Space", "Alt+Shift+Space", "Ctrl+Alt+Space"]
    };

    for fallback_str in fallbacks {
        if let Ok(shortcut) = fallback_str.parse::<Shortcut>() {
            let app_handle = app.clone();
            let result = app.global_shortcut().on_shortcut(shortcut, move |_app, _shortcut, event| {
                if event.state == ShortcutState::Pressed {
                    toggle_launcher(&app_handle);
                }
            });

            if result.is_ok() {
                let msg = format!(
                    "Shortcut conflict: '{}' is in use by another application. Registered fallback '{}'.",
                    clean_requested, fallback_str
                );
                eprintln!("{}", msg);
                let _ = app.emit("launcher://shortcut-status", ShortcutRegistrationStatus {
                    requested: clean_requested.to_string(),
                    active: Some(fallback_str.to_string()),
                    status: "fallback".to_string(),
                    message: msg.clone(),
                });

                return ShortcutRegistrationStatus {
                    requested: clean_requested.to_string(),
                    active: Some(fallback_str.to_string()),
                    status: "fallback".to_string(),
                    message: msg,
                };
            }
        }
    }

    // 3. Fallback exhausted
    let err_msg = format!(
        "Shortcut conflict: Could not register '{}' or fallback shortcuts. You can still use the system tray icon.",
        clean_requested
    );
    eprintln!("{}", err_msg);
    let _ = app.emit("launcher://shortcut-status", ShortcutRegistrationStatus {
        requested: clean_requested.to_string(),
        active: None,
        status: "failed".to_string(),
        message: err_msg.clone(),
    });

    ShortcutRegistrationStatus {
        requested: clean_requested.to_string(),
        active: None,
        status: "failed".to_string(),
        message: err_msg,
    }
}

pub fn setup_global_shortcut(app: &AppHandle, storage: &StorageManager) {
    let shortcut_str = storage
        .get_settings()
        .map(|s| s.global_shortcut)
        .unwrap_or_else(|_| "Alt+Space".to_string());

    register_global_shortcut_with_fallback(app, &shortcut_str);
}
