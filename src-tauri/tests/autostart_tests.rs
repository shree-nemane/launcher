#[cfg(target_os = "windows")]
use std::sync::Mutex;

#[cfg(target_os = "windows")]
static REGISTRY_LOCK: Mutex<()> = Mutex::new(());

#[cfg(target_os = "windows")]
#[test]
fn test_host_autostart_lifecycle() {
    let _lock = REGISTRY_LOCK.lock().unwrap();
    use launcher_lib::autostart_manager;

    // 1. Initially clean up any pre-existing entry
    let _ = autostart_manager::disable_host_autostart();
    assert!(!autostart_manager::is_host_autostart_enabled().unwrap());

    // 2. Enable host autostart
    let enable_res = autostart_manager::enable_host_autostart();
    assert!(enable_res.is_ok(), "Failed to enable autostart: {:?}", enable_res);
    assert!(autostart_manager::is_host_autostart_enabled().unwrap(), "Autostart should be reported as enabled");

    // 3. Verify registry content directly
    let host_path = autostart_manager::get_host_executable_path().unwrap();
    assert!(host_path.to_string_lossy().ends_with("launcher-host.exe"));

    // 4. Disable host autostart
    let disable_res = autostart_manager::disable_host_autostart();
    assert!(disable_res.is_ok(), "Failed to disable autostart: {:?}", disable_res);
    assert!(!autostart_manager::is_host_autostart_enabled().unwrap(), "Autostart should be reported as disabled");

    // 5. Test idempotence: disabling again should succeed without error
    let disable_again = autostart_manager::disable_host_autostart();
    assert!(disable_again.is_ok(), "Idempotent disable should succeed");
}

#[cfg(target_os = "windows")]
#[test]
fn test_enable_autostart_command() {
    let _lock = REGISTRY_LOCK.lock().unwrap();
    use launcher_lib::autostart_manager;
    let res = autostart_manager::enable_host_autostart();
    assert!(res.is_ok());
    assert!(autostart_manager::is_host_autostart_enabled().unwrap());
    let _ = autostart_manager::disable_host_autostart();
}

#[cfg(target_os = "windows")]
#[test]
fn test_disable_autostart_command() {
    let _lock = REGISTRY_LOCK.lock().unwrap();
    use launcher_lib::autostart_manager;
    let res = autostart_manager::disable_host_autostart();
    assert!(res.is_ok());
    assert!(!autostart_manager::is_host_autostart_enabled().unwrap());
}
