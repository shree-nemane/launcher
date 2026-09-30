fn main() {
    #[cfg(target_os = "windows")]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("../icons/icon.ico");
        res.set("FileDescription", "Universal Launcher Host");
        res.set("ProductName", "Universal Project Launcher");
        res.set("CompanyName", "Atom");
        res.set("FileVersion", env!("CARGO_PKG_VERSION"));
        res.set("ProductVersion", env!("CARGO_PKG_VERSION"));
        res.set("OriginalFilename", "launcher-host.exe");
        res.set("InternalName", "launcher-host");
        res.set("LegalCopyright", "Copyright © 2026 Atom");
        if let Err(e) = res.compile() {
            eprintln!("Warning: failed to compile Windows resources for launcher-host: {}", e);
        }
    }
}
