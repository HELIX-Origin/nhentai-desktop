use crate::platform;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

pub const PRODUCT_NAME: &str = "NH Desktop";
const REQUIRED_BYTES: u64 = 128 * 1024 * 1024;

#[derive(Clone, Serialize, Deserialize)]
pub struct InstallerStatus {
    pub is_installed: bool,
    pub installed_version: Option<String>,
    pub current_version: String,
    pub default_install_dir: String,
    pub current_exe_path: Option<String>,
    pub os: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DiskSpaceInfo {
    pub available_bytes: u64,
    pub required_bytes: u64,
    pub has_sufficient_space: bool,
}

#[derive(Deserialize)]
pub struct InstallOptions {
    pub target_dir: String,
    pub create_desktop_shortcut: bool,
    pub create_start_menu_shortcut: bool,
    pub add_to_path: bool,
    pub launch_after: bool,
}

#[derive(Deserialize)]
pub struct UninstallOptions {
    pub remove_user_data: bool,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct OperationResult {
    pub success: bool,
    pub message: String,
    pub details: Vec<String>,
}

pub fn current_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

pub fn detect_status() -> InstallerStatus {
    let exe = platform::installed_exe_path();
    InstallerStatus {
        is_installed: exe.is_some(),
        installed_version: Some(current_version()),
        current_version: current_version(),
        default_install_dir: platform::default_install_dir(),
        current_exe_path: exe.map(|p| p.to_string_lossy().into_owned()),
        os: std::env::consts::OS.to_string(),
    }
}

pub fn check_disk_space(target_dir: &str) -> DiskSpaceInfo {
    let required = REQUIRED_BYTES;
    let available = fs2::available_space(&platform::nearest_existing_dir(Path::new(target_dir)))
        .unwrap_or(required);
    DiskSpaceInfo {
        available_bytes: available,
        required_bytes: required,
        has_sufficient_space: available >= required,
    }
}

pub fn perform_install(options: InstallOptions) -> OperationResult {
    let mut details = Vec::new();
    let mut warnings = Vec::new();

    let target = PathBuf::from(&options.target_dir);
    if target.as_os_str().is_empty() {
        return OperationResult {
            success: false,
            message: "Target directory is empty.".to_string(),
            details: vec!["No installation location was provided.".to_string()],
        };
    }

    let exe_src = match std::env::current_exe() {
        Ok(p) => p,
        Err(e) => {
            return OperationResult {
                success: false,
                message: "Could not resolve the running executable.".to_string(),
                details: vec![format!("current_exe failed: {e}")],
            }
        }
    };

    if let Err(e) = std::fs::create_dir_all(&target) {
        return OperationResult {
            success: false,
            message: "Could not create the target directory.".to_string(),
            details: vec![e.to_string()],
        };
    }
    details.push(format!("Created directory: {}", target.display()));

    let dest = match platform::place_executable(&exe_src, &target) {
        Ok(d) => d,
        Err(e) => {
            return OperationResult {
                success: false,
                message: "Failed to copy the program to the target directory.".to_string(),
                details: vec![e],
            }
        }
    };
    details.push(format!("Installed executable: {}", dest.display()));

    if options.create_desktop_shortcut {
        match platform::create_desktop_shortcut(&dest) {
            Ok(()) => details.push("Desktop shortcut created.".to_string()),
            Err(e) => warnings.push(format!("Desktop shortcut failed: {e}")),
        }
    }

    if options.create_start_menu_shortcut {
        match platform::create_start_menu_shortcut(&dest) {
            Ok(()) => details.push("Start Menu shortcut created.".to_string()),
            Err(e) => warnings.push(format!("Start Menu shortcut failed: {e}")),
        }
    }

    match platform::register_uninstall(&dest, &target) {
        Ok(()) => details.push("Registered uninstall entry.".to_string()),
        Err(e) => warnings.push(format!("Uninstall registration failed: {e}")),
    }

    if options.add_to_path {
        match platform::add_to_path(&target) {
            Ok(()) => details.push("Added install directory to PATH.".to_string()),
            Err(e) => warnings.push(format!("PATH update failed: {e}")),
        }
    }

    if options.launch_after {
        match platform::launch(&dest) {
            Ok(()) => details.push("Launching application.".to_string()),
            Err(e) => warnings.push(format!("Launch after install failed: {e}")),
        }
    }

    details.extend(warnings.iter().cloned());
    OperationResult {
        success: warnings.is_empty(),
        message: if warnings.is_empty() {
            format!("{PRODUCT_NAME} has been installed successfully.")
        } else {
            "Installation completed with warnings.".to_string()
        },
        details,
    }
}

pub fn perform_uninstall(options: UninstallOptions) -> OperationResult {
    let mut details = Vec::new();
    let mut warnings = Vec::new();

    let install_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|d| d.to_path_buf()))
        .filter(|d| d.join(platform::executable_name()).exists())
        .unwrap_or_else(|| PathBuf::from(platform::default_install_dir()));

    match platform::remove_desktop_shortcut() {
        Ok(()) => details.push("Desktop shortcut removed.".to_string()),
        Err(e) => warnings.push(format!("Removing desktop shortcut failed: {e}")),
    }
    match platform::remove_start_menu_shortcut() {
        Ok(()) => details.push("Start Menu shortcut removed.".to_string()),
        Err(e) => warnings.push(format!("Removing Start Menu shortcut failed: {e}")),
    }
    match platform::unregister_uninstall() {
        Ok(()) => details.push("Uninstall entry removed.".to_string()),
        Err(e) => warnings.push(format!("Uninstall deregistration failed: {e}")),
    }
    match platform::remove_from_path(&install_dir) {
        Ok(()) => details.push("Removed install directory from PATH.".to_string()),
        Err(e) => warnings.push(format!("PATH cleanup failed: {e}")),
    }

    if install_dir.exists() {
        match std::fs::remove_dir_all(&install_dir) {
            Ok(()) => details.push(format!("Removed program files: {}", install_dir.display())),
            Err(e) => warnings.push(format!("Removing program files failed: {e}")),
        }
    }

    if options.remove_user_data {
        let mut removed = Vec::new();
        if let Some(data) = dirs::data_dir() {
            let candidate = data.join(PRODUCT_NAME);
            if candidate.exists() {
                match std::fs::remove_dir_all(&candidate) {
                    Ok(()) => removed.push(candidate.to_string_lossy().into_owned()),
                    Err(e) => warnings.push(format!("Removing user data failed: {e}")),
                }
            }
        }
        if let Some(local) = dirs::data_local_dir() {
            let candidate = local.join(PRODUCT_NAME);
            if candidate.exists() {
                match std::fs::remove_dir_all(&candidate) {
                    Ok(()) => removed.push(candidate.to_string_lossy().into_owned()),
                    Err(e) => warnings.push(format!("Removing user data failed: {e}")),
                }
            }
        }
        if removed.is_empty() {
            details.push("No user data found to remove.".to_string());
        } else {
            for r in removed {
                details.push(format!("Removed user data: {r}"));
            }
        }
    }

    details.extend(warnings.iter().cloned());
    OperationResult {
        success: warnings.is_empty(),
        message: if warnings.is_empty() {
            format!("{PRODUCT_NAME} has been uninstalled.")
        } else {
            "Uninstallation completed with warnings.".to_string()
        },
        details,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::platform;

    #[test]
    fn detect_status_returns_valid_info() {
        let s = detect_status();
        assert!(!s.current_version.is_empty());
        assert!(!s.default_install_dir.is_empty());
        assert!(!s.os.is_empty());
        assert_eq!(s.current_exe_path.is_some(), s.is_installed);
    }

    #[test]
    fn check_disk_space_reports_space() {
        let info = check_disk_space(&platform::default_install_dir());
        assert!(info.required_bytes > 0);
        assert_eq!(
            info.has_sufficient_space,
            info.available_bytes >= info.required_bytes
        );
    }

    #[test]
    fn executable_name_matches_platform() {
        let name = platform::executable_name();
        if cfg!(target_os = "windows") {
            assert!(name.ends_with(".exe"));
        } else if cfg!(target_os = "macos") {
            assert!(name.ends_with(".app"));
        } else {
            assert_eq!(name, PRODUCT_NAME);
        }
    }
}