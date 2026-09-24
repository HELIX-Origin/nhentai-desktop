use std::path::{Path, PathBuf};

pub const PRODUCT_NAME: &str = "nhentai";
const EXE_NAME: &str = "nhentai";

pub fn executable_name() -> String {
    format!("{EXE_NAME}.exe")
}

pub fn default_install_dir() -> String {
    if let Some(local) = dirs::data_local_dir() {
        return local
            .join("Programs")
            .join(PRODUCT_NAME)
            .to_string_lossy()
            .into_owned();
    }
    format!(r"C:\Program Files\{PRODUCT_NAME}")
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
    Ok(dest)
}

fn run_ps(script: &str) -> Result<(), String> {
    let dir = std::env::temp_dir();
    let path = dir.join(format!("nhentai_setup_{}.ps1", std::process::id()));
    std::fs::write(&path, script).map_err(|e| format!("Failed to write script: {e}"))?;
    let out = std::process::Command::new("powershell.exe")
        .args([
            "-NoProfile",
            "-NonInteractive",
            "-ExecutionPolicy",
            "Bypass",
            "-File",
        ])
        .arg(&path)
        .output();
    let _ = std::fs::remove_file(&path);
    match out {
        Ok(o) if o.status.success() => Ok(()),
        Ok(o) => {
            let stderr = String::from_utf8_lossy(&o.stderr).trim().to_string();
            let stdout = String::from_utf8_lossy(&o.stdout).trim().to_string();
            let msg = if !stderr.is_empty() { stderr } else { stdout };
            Err(if msg.is_empty() {
                "PowerShell command failed".to_string()
            } else {
                msg
            })
        }
        Err(e) => Err(format!("Failed to run PowerShell: {e}")),
    }
}

fn ps_q(s: &str) -> String {
    s.replace('\'', "''")
}

fn windows_shortcut(exe: &Path, lnk: &Path, description: &str) -> Result<(), String> {
    let script = format!(
        r#"$ws = New-Object -ComObject WScript.Shell
$sc = $ws.CreateShortcut('{lnk}')
$sc.TargetPath = '{exe}'
$sc.WorkingDirectory = '{dir}'
$sc.Description = '{desc}'
$sc.Save()"#,
        lnk = ps_q(&lnk.to_string_lossy()),
        exe = ps_q(&exe.to_string_lossy()),
        dir = ps_q(&exe.parent().map(|p| p.to_string_lossy().into_owned()).unwrap_or_default()),
        desc = ps_q(description),
    );
    run_ps(&script)
}

fn desktop_lnk() -> Option<PathBuf> {
    dirs::desktop_dir().map(|d| d.join(format!("{PRODUCT_NAME}.lnk")))
}

fn start_menu_folder() -> Option<PathBuf> {
    std::env::var_os("APPDATA").map(|appdata| {
        Path::new(&appdata)
            .join("Microsoft")
            .join("Windows")
            .join("Start Menu")
            .join("Programs")
            .join(PRODUCT_NAME)
    })
}

pub fn create_desktop_shortcut(exe: &Path) -> Result<(), String> {
    match desktop_lnk() {
        Some(lnk) => windows_shortcut(exe, &lnk, "nhentai desktop client"),
        None => Err("Could not resolve desktop directory".to_string()),
    }
}

pub fn create_start_menu_shortcut(exe: &Path) -> Result<(), String> {
    match start_menu_folder() {
        Some(folder) => {
            std::fs::create_dir_all(&folder).map_err(|e| e.to_string())?;
            let lnk = folder.join(format!("{PRODUCT_NAME}.lnk"));
            windows_shortcut(exe, &lnk, "nhentai desktop client")
        }
        None => Err("Could not resolve Start Menu directory".to_string()),
    }
}

pub fn remove_desktop_shortcut() -> Result<(), String> {
    match desktop_lnk() {
        Some(lnk) if lnk.exists() => std::fs::remove_file(&lnk).map_err(|e| e.to_string()),
        _ => Ok(()),
    }
}

pub fn remove_start_menu_shortcut() -> Result<(), String> {
    match start_menu_folder() {
        Some(folder) if folder.exists() => {
            let _ = std::fs::remove_file(folder.join(format!("{PRODUCT_NAME}.lnk")));
            std::fs::remove_dir_all(&folder).map_err(|e| e.to_string())
        }
        _ => Ok(()),
    }
}

pub fn register_uninstall(exe: &Path, install_dir: &Path) -> Result<(), String> {
    let key = r"HKCU:\Software\Microsoft\Windows\CurrentVersion\Uninstall\nhentai";
    let uninstall_string = format!("\"{}\" --installer --maintenance", exe.to_string_lossy());
    let script = format!(
        r#"New-Item -Path '{key}' -Force | Out-Null
New-ItemProperty -Path '{key}' -Name 'DisplayName' -Value '{name}' -PropertyType String -Force | Out-Null
New-ItemProperty -Path '{key}' -Name 'DisplayVersion' -Value '{ver}' -PropertyType String -Force | Out-Null
New-ItemProperty -Path '{key}' -Name 'Publisher' -Value 'nhentai' -PropertyType String -Force | Out-Null
New-ItemProperty -Path '{key}' -Name 'DisplayIcon' -Value '{icon}' -PropertyType String -Force | Out-Null
New-ItemProperty -Path '{key}' -Name 'InstallLocation' -Value '{dir}' -PropertyType String -Force | Out-Null
New-ItemProperty -Path '{key}' -Name 'UninstallString' -Value '{us}' -PropertyType String -Force | Out-Null
New-ItemProperty -Path '{key}' -Name 'NoModify' -Value 1 -PropertyType DWord -Force | Out-Null
New-ItemProperty -Path '{key}' -Name 'NoRepair' -Value 0 -PropertyType DWord -Force | Out-Null"#,
        key = key,
        name = PRODUCT_NAME,
        ver = crate::installer::current_version(),
        icon = ps_q(&exe.to_string_lossy()),
        dir = ps_q(&install_dir.to_string_lossy()),
        us = ps_q(&uninstall_string),
    );
    run_ps(&script)
}

pub fn unregister_uninstall() -> Result<(), String> {
    let key = r"HKCU:\Software\Microsoft\Windows\CurrentVersion\Uninstall\nhentai";
    run_ps(&format!(
        r#"if (Test-Path '{key}') {{ Remove-Item -Path '{key}' -Recurse -Force }}"#
    ))
}

pub fn add_to_path(dir: &Path) -> Result<(), String> {
    let script = format!(
        r#"$p = [Environment]::GetEnvironmentVariable('Path', 'User')
if ($p -notlike '*{dir}*') {{
  $p = $p.TrimEnd(';') + ';{dir}'
  [Environment]::SetEnvironmentVariable('Path', $p, 'User')
}}"#,
        dir = ps_q(&dir.to_string_lossy())
    );
    run_ps(&script)
}

pub fn remove_from_path(dir: &Path) -> Result<(), String> {
    let d = dir.to_string_lossy().to_string();
    let script = format!(
        r#"$p = [Environment]::GetEnvironmentVariable('Path', 'User')
if ($p) {{
  $parts = $p.Split(';') | Where-Object {{ $_ -and ($_ -ne '{d}') }}
  $np = $parts -join ';'
  [Environment]::SetEnvironmentVariable('Path', $np, 'User')
}}"#,
        d = ps_q(&d)
    );
    run_ps(&script)
}

pub fn launch(exe: &Path) -> Result<(), String> {
    std::process::Command::new(exe)
        .spawn()
        .map(|_| ())
        .map_err(|e| format!("Launch failed: {e}"))
}