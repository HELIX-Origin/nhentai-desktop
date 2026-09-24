use std::path::{Path, PathBuf};

pub const PRODUCT_NAME: &str = "nhentai";

pub fn executable_name() -> String {
    PRODUCT_NAME.to_string()
}

pub fn default_install_dir() -> String {
    dirs::home_dir()
        .map(|h| {
            h.join(".local")
                .join("share")
                .join(PRODUCT_NAME)
                .to_string_lossy()
                .into_owned()
        })
        .unwrap_or_else(|| format!(".local/share/{PRODUCT_NAME}"))
}

pub fn installed_exe_path() -> Option<PathBuf> {
    let p = Path::new(&default_install_dir()).join(executable_name());
    if p.exists() {
        Some(p)
    } else {
        None
    }
}

pub fn place_executable(src: &Path, target_dir: &Path) -> Result<PathBuf, String> {
    let dest = target_dir.join(executable_name());
    std::fs::copy(src, &dest).map_err(|e| format!("copy {} -> {}: {e}", src.display(), dest.display()))?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = std::fs::set_permissions(&dest, std::fs::Permissions::from_mode(0o755));
    }
    Ok(dest)
}

fn applications_dir() -> PathBuf {
    dirs::home_dir()
        .map(|h| h.join(".local").join("share").join("applications"))
        .unwrap_or_else(|| PathBuf::from("/usr/share/applications"))
}

fn desktop_entry_path() -> PathBuf {
    applications_dir().join(format!("{PRODUCT_NAME}.desktop"))
}

pub fn create_desktop_shortcut(exe: &Path) -> Result<(), String> {
    let dir = applications_dir();
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let entry = format!(
        "[Desktop Entry]\nName={name}\nExec={exe}\nTerminal=false\nType=Application\nCategories=Network;\n",
        name = PRODUCT_NAME,
        exe = exe.to_string_lossy()
    );
    std::fs::write(desktop_entry_path(), entry)
        .map_err(|e| format!("Failed to write desktop entry: {e}"))
}

pub fn create_start_menu_shortcut(exe: &Path) -> Result<(), String> {
    create_desktop_shortcut(exe)
}

pub fn remove_desktop_shortcut() -> Result<(), String> {
    let entry = desktop_entry_path();
    if entry.exists() {
        std::fs::remove_file(&entry).map_err(|e| e.to_string())
    } else {
        Ok(())
    }
}

pub fn remove_start_menu_shortcut() -> Result<(), String> {
    remove_desktop_shortcut()
}

pub fn register_uninstall(exe: &Path, _install_dir: &Path) -> Result<(), String> {
    let dir = applications_dir();
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let entry = format!(
        "[Desktop Entry]\nName=Uninstall {name}\nExec={exe} --installer --maintenance\nTerminal=false\nType=Application\nCategories=Settings;\n",
        name = PRODUCT_NAME,
        exe = exe.to_string_lossy()
    );
    std::fs::write(dir.join(format!("{PRODUCT_NAME}-uninstall.desktop")), entry)
        .map_err(|e| format!("Failed to write uninstall desktop entry: {e}"))
}

pub fn unregister_uninstall() -> Result<(), String> {
    let entry = applications_dir().join(format!("{PRODUCT_NAME}-uninstall.desktop"));
    if entry.exists() {
        std::fs::remove_file(&entry).map_err(|e| e.to_string())
    } else {
        Ok(())
    }
}

fn bin_symlink() -> PathBuf {
    dirs::home_dir()
        .map(|h| h.join(".local").join("bin").join(PRODUCT_NAME))
        .unwrap_or_else(|| PathBuf::from(format!("/usr/local/bin/{PRODUCT_NAME}")))
}

pub fn add_to_path(install_dir: &Path) -> Result<(), String> {
    let link = bin_symlink();
    let target = install_dir.join(executable_name());
    if let Some(parent) = link.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    if link.exists() {
        let _ = std::fs::remove_file(&link);
    }
    std::os::unix::fs::symlink(&target, &link).map_err(|e| format!("Failed to create symlink: {e}"))
}

pub fn remove_from_path(_install_dir: &Path) -> Result<(), String> {
    let link = bin_symlink();
    if link.exists() || std::fs::symlink_metadata(&link).is_ok() {
        std::fs::remove_file(&link).map_err(|e| e.to_string())
    } else {
        Ok(())
    }
}

pub fn launch(exe: &Path) -> Result<(), String> {
    std::process::Command::new(exe)
        .spawn()
        .map(|_| ())
        .map_err(|e| format!("Launch failed: {e}"))
}