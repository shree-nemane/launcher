#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::path::PathBuf;
use std::sync::atomic::{AtomicPtr, AtomicU32, Ordering};

type HWND = *mut std::ffi::c_void;
type HINSTANCE = *mut std::ffi::c_void;
type HICON = *mut std::ffi::c_void;
type HMENU = *mut std::ffi::c_void;
type HMODULE = *mut std::ffi::c_void;
type HANDLE = *mut std::ffi::c_void;
type LRESULT = isize;
type WPARAM = usize;
type LPARAM = isize;
type UINT = u32;
type BOOL = i32;
type DWORD = u32;
type LPCWSTR = *const u16;
type LPWSTR = *mut u16;

const WM_DESTROY: UINT = 0x0002;
const WM_HOTKEY: UINT = 0x0312;
const WM_USER: UINT = 0x0400;
const WM_TRAYICON: UINT = WM_USER + 1;
const WM_COMMAND: UINT = 0x0111;
const WM_RBUTTONUP: UINT = 0x0205;
const WM_LBUTTONUP: UINT = 0x0202;
const WM_LBUTTONDBLCLK: UINT = 0x0203;
const WM_QUERYENDSESSION: UINT = 0x0011;
const WM_ENDSESSION: UINT = 0x0016;

const MOD_ALT: UINT = 0x0001;
const MOD_NOREPEAT: UINT = 0x4000;
const VK_SPACE: UINT = 0x20;
const HOTKEY_ID: i32 = 1001;

const NIM_ADD: DWORD = 0x00000000;
const NIM_MODIFY: DWORD = 0x00000001;
const NIM_DELETE: DWORD = 0x00000002;
const NIF_MESSAGE: UINT = 0x00000001;
const NIF_ICON: UINT = 0x00000002;
const NIF_TIP: UINT = 0x00000004;

const EMBEDDED_ICON_ID: usize = 1;
const IDI_APPLICATION: usize = 32512;
const IMAGE_ICON: UINT = 1;
const LR_LOADFROMFILE: UINT = 0x0010;
const LR_SHARED: UINT = 0x8000;
const SM_CXICON: i32 = 11;
const SM_CYICON: i32 = 12;
const SM_CXSMICON: i32 = 49;
const SM_CYSMICON: i32 = 50;

const MF_STRING: UINT = 0x00000000;
const TPM_RIGHTBUTTON: UINT = 0x0002;

const IDM_TOGGLE: usize = 2000;
const IDM_OPEN: usize = 2001;
const IDM_QUIT: usize = 2002;

const ERROR_ALREADY_EXISTS: DWORD = 183;
const STILL_ACTIVE: DWORD = 259;

const JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE: DWORD = 0x00002000;
const JOB_OBJECT_EXTENDED_LIMIT_INFORMATION: DWORD = 9;

const MUTEX_NAME: &str = "Local\\UniversalLauncherHostMutex";
const CLASS_NAME: &str = "UniversalLauncherHostClass";
const WINDOW_TITLE: &str = "UniversalLauncherHost";

#[repr(C)]
struct WNDCLASSEXW {
    cb_size: UINT,
    style: UINT,
    lpfn_wnd_proc: Option<unsafe extern "system" fn(HWND, UINT, WPARAM, LPARAM) -> LRESULT>,
    cb_cls_extra: i32,
    cb_wnd_extra: i32,
    h_instance: HINSTANCE,
    h_icon: HICON,
    h_cursor: *mut std::ffi::c_void,
    hbr_background: *mut std::ffi::c_void,
    lpsz_menu_name: LPCWSTR,
    lpsz_class_name: LPCWSTR,
    h_icon_sm: HICON,
}

#[repr(C)]
struct POINT {
    x: i32,
    y: i32,
}

#[repr(C)]
struct MSG {
    hwnd: HWND,
    message: UINT,
    w_param: WPARAM,
    l_param: LPARAM,
    time: DWORD,
    pt: POINT,
}

#[repr(C)]
struct NOTIFYICONDATAW {
    cb_size: DWORD,
    h_wnd: HWND,
    u_id: UINT,
    u_flags: UINT,
    u_callback_message: UINT,
    h_icon: HICON,
    sz_tip: [u16; 128],
    dw_state: DWORD,
    dw_state_mask: DWORD,
    sz_info: [u16; 256],
    u_timeout_or_version: UINT,
    sz_info_title: [u16; 64],
    dw_info_flags: DWORD,
    guid_item: [u8; 16],
    h_balloon_icon: HICON,
}

#[repr(C)]
struct STARTUPINFOW {
    cb: DWORD,
    lp_reserved: LPWSTR,
    lp_desktop: LPWSTR,
    lp_title: LPWSTR,
    dw_x: DWORD,
    dw_y: DWORD,
    dw_x_size: DWORD,
    dw_y_size: DWORD,
    dw_x_count_chars: DWORD,
    dw_y_count_chars: DWORD,
    dw_fill_attribute: DWORD,
    dw_flags: DWORD,
    w_show_window: u16,
    cb_reserved2: u16,
    lp_reserved2: *mut u8,
    h_std_input: HANDLE,
    h_std_output: HANDLE,
    h_std_error: HANDLE,
}

#[repr(C)]
struct PROCESS_INFORMATION {
    h_process: HANDLE,
    h_thread: HANDLE,
    dw_process_id: DWORD,
    dw_thread_id: DWORD,
}

#[repr(C)]
struct IO_COUNTERS {
    read_operation_count: u64,
    write_operation_count: u64,
    other_operation_count: u64,
    read_transfer_count: u64,
    write_transfer_count: u64,
    other_transfer_count: u64,
}

#[repr(C)]
struct JOBOBJECT_BASIC_LIMIT_INFORMATION {
    per_process_user_time_limit: i64,
    per_job_user_time_limit: i64,
    limit_flags: DWORD,
    minimum_working_set_size: usize,
    maximum_working_set_size: usize,
    active_process_limit: DWORD,
    affinity: usize,
    priority_class: DWORD,
    scheduling_class: DWORD,
}

#[repr(C)]
struct JOBOBJECT_EXTENDED_LIMIT_INFORMATION {
    basic_limit_information: JOBOBJECT_BASIC_LIMIT_INFORMATION,
    io_info: IO_COUNTERS,
    process_memory_limit: usize,
    job_memory_limit: usize,
    peak_process_memory_limit: usize,
    peak_job_memory_limit: usize,
}

#[link(name = "user32")]
extern "system" {
    fn RegisterClassExW(lpwcx: *const WNDCLASSEXW) -> u16;
    fn CreateWindowExW(
        dw_ex_style: DWORD,
        lp_class_name: LPCWSTR,
        lp_window_name: LPCWSTR,
        dw_style: DWORD,
        x: i32,
        y: i32,
        n_width: i32,
        n_height: i32,
        h_wnd_parent: HWND,
        h_menu: HMENU,
        h_instance: HINSTANCE,
        lp_param: *mut std::ffi::c_void,
    ) -> HWND;
    fn DefWindowProcW(h_wnd: HWND, msg: UINT, w_param: WPARAM, l_param: LPARAM) -> LRESULT;
    fn RegisterHotKey(h_wnd: HWND, id: i32, fs_modifiers: UINT, vk: UINT) -> BOOL;
    fn UnregisterHotKey(h_wnd: HWND, id: i32) -> BOOL;
    fn GetMessageW(lp_msg: *mut MSG, h_wnd: HWND, w_msg_filter_min: UINT, w_msg_filter_max: UINT) -> BOOL;
    fn TranslateMessage(lp_msg: *const MSG) -> BOOL;
    fn DispatchMessageW(lp_msg: *const MSG) -> LRESULT;
    fn PostQuitMessage(n_exit_code: i32);
    fn GetCursorPos(lp_point: *mut POINT) -> BOOL;
    fn SetForegroundWindow(h_wnd: HWND) -> BOOL;
    fn CreatePopupMenu() -> HMENU;
    fn AppendMenuW(h_menu: HMENU, u_flags: UINT, u_id_new_item: usize, lp_new_item: LPCWSTR) -> BOOL;
    fn TrackPopupMenu(
        h_menu: HMENU,
        u_flags: UINT,
        x: i32,
        y: i32,
        n_reserved: i32,
        h_wnd: HWND,
        prc_rect: *const std::ffi::c_void,
    ) -> BOOL;
    fn DestroyMenu(h_menu: HMENU) -> BOOL;
    fn LoadIconW(h_instance: HINSTANCE, lp_icon_name: LPCWSTR) -> HICON;
    fn LoadImageW(
        h_inst: HINSTANCE,
        name: LPCWSTR,
        r#type: UINT,
        cx: i32,
        cy: i32,
        fu_load: UINT,
    ) -> HANDLE;
    fn GetSystemMetrics(n_index: i32) -> i32;
    fn FindWindowW(lp_class_name: LPCWSTR, lp_window_name: LPCWSTR) -> HWND;
    fn IsWindowVisible(h_wnd: HWND) -> BOOL;
    fn PostMessageW(h_wnd: HWND, msg: UINT, w_param: WPARAM, l_param: LPARAM) -> BOOL;
    fn RegisterWindowMessageW(lp_string: LPCWSTR) -> UINT;
}

#[link(name = "kernel32")]
extern "system" {
    fn CreateMutexW(lp_mutex_attributes: *mut std::ffi::c_void, b_initial_owner: BOOL, lp_name: LPCWSTR) -> HANDLE;
    fn GetLastError() -> DWORD;
    fn CloseHandle(h_object: HANDLE) -> BOOL;
    fn GetModuleHandleW(lp_module_name: LPCWSTR) -> HMODULE;
    fn CreateProcessW(
        lp_application_name: LPCWSTR,
        lp_command_line: LPWSTR,
        lp_process_attributes: *mut std::ffi::c_void,
        lp_thread_attributes: *mut std::ffi::c_void,
        b_inherit_handles: BOOL,
        dw_creation_flags: DWORD,
        lp_environment: *mut std::ffi::c_void,
        lp_current_directory: LPCWSTR,
        lp_startup_info: *mut STARTUPINFOW,
        lp_process_information: *mut PROCESS_INFORMATION,
    ) -> BOOL;
    fn GetExitCodeProcess(h_process: HANDLE, lp_exit_code: *mut DWORD) -> BOOL;
    fn TerminateProcess(h_process: HANDLE, u_exit_code: UINT) -> BOOL;
    fn CreateJobObjectW(lp_job_attributes: *mut std::ffi::c_void, lp_name: LPCWSTR) -> HANDLE;
    fn SetInformationJobObject(
        h_job: HANDLE,
        job_object_information_class: DWORD,
        lp_job_object_information: *const std::ffi::c_void,
        cb_job_object_information_length: DWORD,
    ) -> BOOL;
    fn AssignProcessToJobObject(h_job: HANDLE, h_process: HANDLE) -> BOOL;
    fn TerminateJobObject(h_job: HANDLE, u_exit_code: UINT) -> BOOL;
}

#[link(name = "shell32")]
extern "system" {
    fn Shell_NotifyIconW(dw_message: DWORD, lp_data: *mut NOTIFYICONDATAW) -> BOOL;
}

static UI_PROCESS_HANDLE: AtomicPtr<std::ffi::c_void> = AtomicPtr::new(std::ptr::null_mut());
static UI_JOB_HANDLE: AtomicPtr<std::ffi::c_void> = AtomicPtr::new(std::ptr::null_mut());
static TRAY_ICON_HANDLE: AtomicPtr<std::ffi::c_void> = AtomicPtr::new(std::ptr::null_mut());
static WM_TASKBAR_CREATED: AtomicU32 = AtomicU32::new(0);

fn load_application_icons(h_instance: HINSTANCE) -> (HICON, HICON) {
    let sm_cx = unsafe { GetSystemMetrics(SM_CXSMICON) };
    let sm_cy = unsafe { GetSystemMetrics(SM_CYSMICON) };
    let lg_cx = unsafe { GetSystemMetrics(SM_CXICON) };
    let lg_cy = unsafe { GetSystemMetrics(SM_CYICON) };

    unsafe {
        // 1. Primary: load embedded resource ID 1 (stamped into PE by winres)
        let mut icon_sm = LoadImageW(
            h_instance,
            EMBEDDED_ICON_ID as LPCWSTR,
            IMAGE_ICON,
            sm_cx,
            sm_cy,
            LR_SHARED,
        ) as HICON;

        let mut icon_main = LoadImageW(
            h_instance,
            EMBEDDED_ICON_ID as LPCWSTR,
            IMAGE_ICON,
            lg_cx,
            lg_cy,
            LR_SHARED,
        ) as HICON;

        // 2. Fallback: LoadIconW on embedded resource ID 1
        if icon_sm.is_null() {
            icon_sm = LoadIconW(h_instance, EMBEDDED_ICON_ID as LPCWSTR);
        }
        if icon_main.is_null() {
            icon_main = LoadIconW(h_instance, EMBEDDED_ICON_ID as LPCWSTR);
        }

        // 3. Fallback: load directly from icons/icon.ico next to executable
        if icon_sm.is_null() || icon_main.is_null() {
            let host_dir = get_host_dir();
            let icon_file = host_dir.join("icons").join("icon.ico");
            if icon_file.exists() {
                let icon_path_wide = to_wide_chars(&icon_file.to_string_lossy());
                if icon_sm.is_null() {
                    icon_sm = LoadImageW(
                        std::ptr::null_mut(),
                        icon_path_wide.as_ptr(),
                        IMAGE_ICON,
                        sm_cx,
                        sm_cy,
                        LR_LOADFROMFILE,
                    ) as HICON;
                }
                if icon_main.is_null() {
                    icon_main = LoadImageW(
                        std::ptr::null_mut(),
                        icon_path_wide.as_ptr(),
                        IMAGE_ICON,
                        lg_cx,
                        lg_cy,
                        LR_LOADFROMFILE,
                    ) as HICON;
                }
            }
        }

        // 4. Fallback: standard system application icon (must use NULL h_instance for system icons)
        if icon_sm.is_null() {
            icon_sm = LoadIconW(std::ptr::null_mut(), IDI_APPLICATION as LPCWSTR);
        }
        if icon_main.is_null() {
            icon_main = LoadIconW(std::ptr::null_mut(), IDI_APPLICATION as LPCWSTR);
        }

        (icon_main, icon_sm)
    }
}

fn add_or_update_tray_icon(hwnd: HWND) {
    let h_icon_tray = TRAY_ICON_HANDLE.load(Ordering::SeqCst);
    let mut nid: NOTIFYICONDATAW = unsafe { std::mem::zeroed() };
    nid.cb_size = std::mem::size_of::<NOTIFYICONDATAW>() as DWORD;
    nid.h_wnd = hwnd;
    nid.u_id = 1;
    nid.u_flags = NIF_MESSAGE | NIF_ICON | NIF_TIP;
    nid.u_callback_message = WM_TRAYICON;
    nid.h_icon = h_icon_tray;

    let tip = to_wide_chars("Universal Launcher Host");
    for (i, &ch) in tip.iter().take(127).enumerate() {
        nid.sz_tip[i] = ch;
    }

    unsafe {
        if Shell_NotifyIconW(NIM_ADD, &mut nid) == 0 {
            Shell_NotifyIconW(NIM_MODIFY, &mut nid);
        }
    }
}

fn remove_tray_icon(hwnd: HWND) {
    let mut nid: NOTIFYICONDATAW = unsafe { std::mem::zeroed() };
    nid.cb_size = std::mem::size_of::<NOTIFYICONDATAW>() as DWORD;
    nid.h_wnd = hwnd;
    nid.u_id = 1;
    unsafe {
        Shell_NotifyIconW(NIM_DELETE, &mut nid);
    }
}

fn to_wide_chars(s: &str) -> Vec<u16> {
    OsStr::new(s).encode_wide().chain(std::iter::once(0)).collect()
}

fn is_ui_running() -> bool {
    let handle = UI_PROCESS_HANDLE.load(Ordering::SeqCst);
    if handle.is_null() {
        return false;
    }
    unsafe {
        let mut exit_code: DWORD = 0;
        if GetExitCodeProcess(handle, &mut exit_code) != 0 {
            if exit_code == STILL_ACTIVE {
                return true;
            }
        }
        cleanup_handles();
        false
    }
}

fn cleanup_handles() {
    let proc_handle = UI_PROCESS_HANDLE.swap(std::ptr::null_mut(), Ordering::SeqCst);
    if !proc_handle.is_null() {
        unsafe { CloseHandle(proc_handle); }
    }
    let job_handle = UI_JOB_HANDLE.swap(std::ptr::null_mut(), Ordering::SeqCst);
    if !job_handle.is_null() {
        unsafe { CloseHandle(job_handle); }
    }
}

fn get_host_dir() -> PathBuf {
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            return dir.to_path_buf();
        }
    }
    PathBuf::from(".")
}

fn get_ui_executable_path() -> PathBuf {
    let host_dir = get_host_dir();
    let primary = host_dir.join("universal-launcher.exe");
    if primary.exists() {
        return primary;
    }

    // Failsafe fallback: check %LOCALAPPDATA%\Programs\UniversalLauncher\universal-launcher.exe
    if let Ok(local_appdata) = std::env::var("LOCALAPPDATA") {
        let fallback = PathBuf::from(local_appdata)
            .join("Programs")
            .join("UniversalLauncher")
            .join("universal-launcher.exe");
        if fallback.exists() {
            return fallback;
        }
    }

    PathBuf::from("universal-launcher.exe")
}

fn spawn_ui() {
    if is_ui_running() {
        return;
    }

    let host_dir = get_host_dir();
    let host_dir_wide = to_wide_chars(&host_dir.to_string_lossy());

    let ui_exe = get_ui_executable_path();
    let cmd_string = format!("\"{}\" --two-process", ui_exe.to_string_lossy());
    let mut cmd_wide = to_wide_chars(&cmd_string);

    unsafe {
        // Create Job Object with KILL_ON_JOB_CLOSE so all child processes terminate atomically
        let h_job = CreateJobObjectW(std::ptr::null_mut(), std::ptr::null());
        if !h_job.is_null() {
            let mut info: JOBOBJECT_EXTENDED_LIMIT_INFORMATION = std::mem::zeroed();
            info.basic_limit_information.limit_flags = JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE;
            SetInformationJobObject(
                h_job,
                JOB_OBJECT_EXTENDED_LIMIT_INFORMATION,
                &info as *const _ as _,
                std::mem::size_of::<JOBOBJECT_EXTENDED_LIMIT_INFORMATION>() as DWORD,
            );
        }

        let mut si: STARTUPINFOW = std::mem::zeroed();
        si.cb = std::mem::size_of::<STARTUPINFOW>() as DWORD;
        let mut pi: PROCESS_INFORMATION = std::mem::zeroed();

        let success = CreateProcessW(
            std::ptr::null(),
            cmd_wide.as_mut_ptr(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            0,
            0,
            std::ptr::null_mut(),
            host_dir_wide.as_ptr(),
            &mut si,
            &mut pi,
        );

        if success != 0 {
            CloseHandle(pi.h_thread);
            if !h_job.is_null() {
                AssignProcessToJobObject(h_job, pi.h_process);
                UI_JOB_HANDLE.store(h_job, Ordering::SeqCst);
            }
            UI_PROCESS_HANDLE.store(pi.h_process, Ordering::SeqCst);
        } else if !h_job.is_null() {
            CloseHandle(h_job);
        }
    }
}

fn is_ui_visible() -> bool {
    unsafe {
        let class_wide = to_wide_chars("Tauri Window");
        let title_wide = to_wide_chars("Universal launcher");
        let mut ui_hwnd = FindWindowW(class_wide.as_ptr(), title_wide.as_ptr());
        if ui_hwnd.is_null() {
            ui_hwnd = FindWindowW(std::ptr::null(), title_wide.as_ptr());
        }
        !ui_hwnd.is_null() && IsWindowVisible(ui_hwnd) != 0
    }
}

fn toggle_or_spawn_ui() {
    if is_ui_running() {
        if is_ui_visible() {
            terminate_ui_if_running();
        } else {
            ensure_ui_open_and_focused();
        }
    } else {
        spawn_ui();
    }
}

fn ensure_ui_open_and_focused() {
    if is_ui_running() {
        unsafe {
            let class_wide = to_wide_chars("Tauri Window");
            let title_wide = to_wide_chars("Universal launcher");
            let mut ui_hwnd = FindWindowW(class_wide.as_ptr(), title_wide.as_ptr());
            if ui_hwnd.is_null() {
                ui_hwnd = FindWindowW(std::ptr::null(), title_wide.as_ptr());
            }
            if !ui_hwnd.is_null() {
                SetForegroundWindow(ui_hwnd);
            }
        }
        return;
    }
    spawn_ui();
}

fn terminate_ui_if_running() {
    let job_handle = UI_JOB_HANDLE.swap(std::ptr::null_mut(), Ordering::SeqCst);
    if !job_handle.is_null() {
        unsafe {
            TerminateJobObject(job_handle, 0);
            CloseHandle(job_handle);
        }
    }

    let proc_handle = UI_PROCESS_HANDLE.swap(std::ptr::null_mut(), Ordering::SeqCst);
    if !proc_handle.is_null() {
        unsafe {
            let mut exit_code: DWORD = 0;
            if GetExitCodeProcess(proc_handle, &mut exit_code) != 0 && exit_code == STILL_ACTIVE {
                TerminateProcess(proc_handle, 0);
            }
            CloseHandle(proc_handle);
        }
    }
}

unsafe extern "system" fn window_proc(
    hwnd: HWND,
    msg: UINT,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_HOTKEY => {
            if wparam as i32 == HOTKEY_ID {
                toggle_or_spawn_ui();
            }
            0
        }
        WM_TRAYICON => {
            let event = lparam as UINT;
            if event == WM_RBUTTONUP {
                let mut pt: POINT = std::mem::zeroed();
                GetCursorPos(&mut pt);
                SetForegroundWindow(hwnd);

                let hmenu = CreatePopupMenu();
                let open_text = to_wide_chars("Open Launcher (Alt+Space)");
                let quit_text = to_wide_chars("Quit");

                AppendMenuW(hmenu, MF_STRING, IDM_OPEN, open_text.as_ptr());
                AppendMenuW(hmenu, MF_STRING, IDM_QUIT, quit_text.as_ptr());

                TrackPopupMenu(hmenu, TPM_RIGHTBUTTON, pt.x, pt.y, 0, hwnd, std::ptr::null());
                DestroyMenu(hmenu);
            } else if event == WM_LBUTTONUP || event == WM_LBUTTONDBLCLK {
                toggle_or_spawn_ui();
            }
            0
        }
        WM_COMMAND => {
            match wparam {
                IDM_TOGGLE => {
                    toggle_or_spawn_ui();
                }
                IDM_OPEN => {
                    ensure_ui_open_and_focused();
                }
                IDM_QUIT => {
                    terminate_ui_if_running();
                    PostQuitMessage(0);
                }
                _ => {}
            }
            0
        }
        WM_QUERYENDSESSION => {
            // Windows is querying if the application is ready to end
            1
        }
        WM_ENDSESSION => {
            // If wparam is non-zero, Windows is shutting down or logging off
            if wparam != 0 {
                terminate_ui_if_running();
                UnregisterHotKey(hwnd, HOTKEY_ID);
                PostQuitMessage(0);
            }
            0
        }
        WM_DESTROY => {
            terminate_ui_if_running();
            PostQuitMessage(0);
            0
        }
        _ => {
            let taskbar_msg = WM_TASKBAR_CREATED.load(Ordering::Relaxed);
            if taskbar_msg != 0 && msg == taskbar_msg {
                add_or_update_tray_icon(hwnd);
                return 0;
            }
            DefWindowProcW(hwnd, msg, wparam, lparam)
        }
    }
}

fn main() {
    let should_open = std::env::args().any(|arg| arg == "--open");

    // 1. Single-instance enforcement with production mutex
    let mutex_name = to_wide_chars(MUTEX_NAME);
    let mutex = unsafe { CreateMutexW(std::ptr::null_mut(), 1, mutex_name.as_ptr()) };
    let already_exists = unsafe { GetLastError() == ERROR_ALREADY_EXISTS };

    if mutex.is_null() || already_exists {
        // If an existing host is already running, notify it to open/focus the UI
        let class_wide = to_wide_chars(CLASS_NAME);
        let title_wide = to_wide_chars(WINDOW_TITLE);
        unsafe {
            let existing_hwnd = FindWindowW(class_wide.as_ptr(), title_wide.as_ptr());
            if !existing_hwnd.is_null() {
                PostMessageW(existing_hwnd, WM_COMMAND, IDM_OPEN, 0);
            }
        }
        if !mutex.is_null() {
            unsafe { CloseHandle(mutex); }
        }
        return;
    }

    // Register TaskbarCreated message to restore icon if Explorer restarts
    let taskbar_str = to_wide_chars("TaskbarCreated");
    let msg_taskbar = unsafe { RegisterWindowMessageW(taskbar_str.as_ptr()) };
    WM_TASKBAR_CREATED.store(msg_taskbar, Ordering::SeqCst);

    // 2. Load embedded branded application icon from resources
    let h_instance = unsafe { GetModuleHandleW(std::ptr::null()) };
    let (h_icon_main, h_icon_tray) = load_application_icons(h_instance);
    TRAY_ICON_HANDLE.store(h_icon_tray, Ordering::SeqCst);

    // 3. Register hidden window class
    let class_name = to_wide_chars(CLASS_NAME);
    let wnd_class = WNDCLASSEXW {
        cb_size: std::mem::size_of::<WNDCLASSEXW>() as UINT,
        style: 0,
        lpfn_wnd_proc: Some(window_proc),
        cb_cls_extra: 0,
        cb_wnd_extra: 0,
        h_instance,
        h_icon: h_icon_main,
        h_cursor: std::ptr::null_mut(),
        hbr_background: std::ptr::null_mut(),
        lpsz_menu_name: std::ptr::null(),
        lpsz_class_name: class_name.as_ptr(),
        h_icon_sm: h_icon_tray,
    };

    unsafe {
        RegisterClassExW(&wnd_class);
    }

    // 4. Create hidden message window
    let window_title = to_wide_chars(WINDOW_TITLE);
    let hwnd = unsafe {
        CreateWindowExW(
            0,
            class_name.as_ptr(),
            window_title.as_ptr(),
            0,
            0,
            0,
            0,
            0,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            h_instance,
            std::ptr::null_mut(),
        )
    };

    if hwnd.is_null() {
        if !mutex.is_null() {
            unsafe { CloseHandle(mutex); }
        }
        return;
    }

    // 5. Register global Alt+Space hotkey
    unsafe {
        let _ = RegisterHotKey(hwnd, HOTKEY_ID, MOD_ALT | MOD_NOREPEAT, VK_SPACE);
    }

    // 6. Create System Tray icon with embedded branded icon
    add_or_update_tray_icon(hwnd);

    // 7. If --open was passed on startup, open UI immediately
    if should_open {
        ensure_ui_open_and_focused();
    }

    // 8. Message Loop
    let mut msg: MSG = unsafe { std::mem::zeroed() };
    unsafe {
        while GetMessageW(&mut msg, std::ptr::null_mut(), 0, 0) > 0 {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }

        // Cleanup on exit
        UnregisterHotKey(hwnd, HOTKEY_ID);
        remove_tray_icon(hwnd);
        terminate_ui_if_running();
        if !mutex.is_null() {
            CloseHandle(mutex);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tray_icon_registration() {
        let h_instance = unsafe { GetModuleHandleW(std::ptr::null()) };
        let (icon_main, icon_tray) = load_application_icons(h_instance);
        assert!(!icon_main.is_null(), "Main icon should not be null");
        assert!(!icon_tray.is_null(), "Tray icon should not be null");

        let class_name = to_wide_chars("TestTrayClass");
        let wnd_class = WNDCLASSEXW {
            cb_size: std::mem::size_of::<WNDCLASSEXW>() as UINT,
            style: 0,
            lpfn_wnd_proc: Some(window_proc),
            cb_cls_extra: 0,
            cb_wnd_extra: 0,
            h_instance,
            h_icon: icon_main,
            h_cursor: std::ptr::null_mut(),
            hbr_background: std::ptr::null_mut(),
            lpsz_menu_name: std::ptr::null(),
            lpsz_class_name: class_name.as_ptr(),
            h_icon_sm: icon_tray,
        };
        unsafe {
            RegisterClassExW(&wnd_class);
            let hwnd = CreateWindowExW(
                0,
                class_name.as_ptr(),
                class_name.as_ptr(),
                0,
                0,
                0,
                0,
                0,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                h_instance,
                std::ptr::null_mut(),
            );
            assert!(!hwnd.is_null(), "Test window should be created");

            TRAY_ICON_HANDLE.store(icon_tray, Ordering::SeqCst);
            add_or_update_tray_icon(hwnd);
            remove_tray_icon(hwnd);
        }
    }
}
