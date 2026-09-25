use std::path::{Path, PathBuf};

pub const PRODUCT_NAME: &str = "NH Desktop";

pub fn executable_name() -> String {
    format!("{PRODUCT_NAME}.app")
}

pub fn default_install_dir() -> String {
    "/Applications".to_string()
}

pub fn installed_exe_path() -> Option<PathBuf> {
    let p = Path::new(&default_install_dir()).join(executable_name());
    if p.exists() {
        Some(p)
    } else {
        None
    }
}

fn app_bundle(target_dir: &Path) -> PathBuf {
    target_dir.join(executable_name())
}

pub fn place_executable(src: &Path, target_dir: &Path) -> Result<PathBuf, String> {
    let bundle = app_bundle(target_dir);
    let contents = bundle.join("Contents");
    let macos = contents.join("MacOS");
    std::fs::create_dir_all(&macos).map_err(|e| format!("create bundle dir: {e}"))?;

    let dest = macos.join(PRODUCT_NAME);
    std::fs::copy(src, &dest).map_err(|e| format!("copy {} -> {}: {e}", src.display(), dest.display()))?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = std::fs::set_permissions(&dest, std::fs::Permissions::from_mode(0o755));
    }

    let plist = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
	<key>CFBundleDisplayName</key>
	<string>{name}</string>
	<key>CFBundleExecutable</key>
	<string>{name}</string>
	<key>CFBundleIdentifier</key>
	<string>net.nh-desktop.client</string>
	<key>CFBundleName</key>
	<string>{name}</string>
	<key>CFBundlePackageType</key>
	<string>APPL</string>
	<key>CFBundleShortVersionString</key>
	<string>{ver}</string>
	<key>CFBundleVersion</key>
	<string>{ver}</string>
	<key>LSMinimumSystemVersion</key>
	<string>10.13</string>
</dict>
</plist>"#,
        name = PRODUCT_NAME,
        ver = crate::installer::current_version(),
    );
    std::fs::write(contents.join("Info.plist"), plist).map_err(|e| format!("write Info.plist: {e}"))?;
    Ok(bundle)
}

fn alias_path() -> Option<PathBuf> {
    dirs::desktop_dir().map(|d| d.join(format!("{PRODUCT_NAME}.app")))
}

fn run_osascript(script: &str) -> Result<(), String> {
    let out = std::process::Command::new("osascript")
        .arg("-e")
        .arg(script)
        .output();
    match out {
        Ok(o) if o.status.success() => Ok(()),
        Ok(o) => {
            let stderr = String::from_utf8_lossy(&o.stderr).trim().to_string();
            Err(if stderr.is_empty() { "osascript failed".to_string() } else { stderr })
        }
        Err(e) => Err(format!("Failed to run osascript: {e}")),
    }
}

fn quote(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

pub fn create_desktop_shortcut(exe: &Path) -> Result<(), String> {
    let desktop = dirs::desktop_dir().ok_or_else(|| "Could not resolve Desktop directory".to_string())?;
    let script = format!(
        "tell application \"Finder\"\nmake alias file to POSIX file \"{exe}\" at POSIX file \"{desktop}\" with properties {{name:\"{name}.app\"}}\nend tell",
        exe = quote(&exe.to_string_lossy()),
        desktop = quote(&desktop.to_string_lossy()),
        name = PRODUCT_NAME,
    );
    run_osascript(&script)
}

pub fn create_start_menu_shortcut(_exe: &Path) -> Result<(), String> {
    Ok(())
}

pub fn remove_desktop_shortcut() -> Result<(), String> {
    match alias_path() {
        Some(alias) if alias.exists() => {
            let script = format!(
                "tell application \"Finder\"\ndelete POSIX file \"{alias}\"\nend tell",
                alias = quote(&alias.to_string_lossy())
            );
            run_osascript(&script)
        }
        _ => Ok(()),
    }
}

pub fn remove_start_menu_shortcut() -> Result<(), String> {
    Ok(())
}

pub fn register_uninstall(_exe: &Path, _install_dir: &Path) -> Result<(), String> {
    Ok(())
}

pub fn unregister_uninstall() -> Result<(), String> {
    Ok(())
}

pub fn add_to_path(_install_dir: &Path) -> Result<(), String> {
    Ok(())
}

pub fn remove_from_path(_install_dir: &Path) -> Result<(), String> {
    Ok(())
}

pub fn launch(exe: &Path) -> Result<(), String> {
    std::process::Command::new("open")
        .arg(exe)
        .spawn()
        .map(|_| ())
        .map_err(|e| format!("Launch failed: {e}"))
}

pub fn quit_running_app() -> Result<(), String> {
    let Some(bundle) = installed_exe_path() else {
        return Ok(());
    };
    let bin = bundle.join("Contents").join("MacOS").join(PRODUCT_NAME);
    if std::env::current_exe().ok().as_deref() == Some(bin.as_path()) {
        return Ok(());
    }
    let out = std::process::Command::new("pkill")
        .args(["-f", &bin.to_string_lossy()])
        .output();
    match out {
        Ok(o) if o.status.success() => Ok(()),
        Ok(_) => Ok(()),
        Ok(o) => Err(String::from_utf8_lossy(&o.stderr).trim().to_string()),
        Err(e) => Err(format!("Failed to run pkill: {e}")),
    }
}