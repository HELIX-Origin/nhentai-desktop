// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    let exe_name = std::env::current_exe()
        .ok()
        .and_then(|p| p.file_name().map(|n| n.to_string_lossy().to_lowercase()))
        .unwrap_or_default();

    // Auto-launch unified installer wizard if named *installer* or *setup*, or passed flags
    if exe_name.contains("installer")
        || exe_name.contains("setup")
        || args.iter().any(|a| a == "--installer" || a == "--setup" || a == "--uninstall" || a == "--maintenance")
    {
        nh_desktop_lib::run_installer();
        return ExitCode::SUCCESS;
    }
    nh_desktop_lib::run();
    ExitCode::SUCCESS
}