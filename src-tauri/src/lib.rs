mod commands;
mod db;
mod error;
mod installer;
mod nh_desktop;
mod platform;

use db::Db;
use nh_desktop::NhDesktopClient;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let data_dir = app.path().app_data_dir()?;
            app.manage(NhDesktopClient::new()?);
            app.manage(Db::new(&data_dir.join("nh-desktop.db"))?);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::fetch_new,
            commands::fetch_popular,
            commands::fetch_tagged,
            commands::search_galleries,
            commands::fetch_gallery,
            commands::related_galleries,
            commands::fetch_tag_info,
            commands::proxy_image,
            commands::db_get,
            commands::db_set,
            commands::db_del,
            commands::db_dump,
            commands::db_clear,
            commands::set_api_key,
            commands::get_api_key_status,
            commands::clear_api_key,
            commands::verify_api_key,
            commands::get_current_user,
            commands::check_favorite,
            commands::add_favorite,
            commands::remove_favorite,
            commands::fetch_favorites,
            commands::fetch_account_blacklist,
            commands::update_account_blacklist,
            commands::download_gallery,
            commands::installer_status,
            commands::installer_disk_space,
            commands::installer_install,
            commands::installer_uninstall,
            commands::open_maintenance_window,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

pub fn run_installer() {
    let is_uninstall = std::env::args().any(|a| a == "--uninstall" || a == "--maintenance");
    let mode = if is_uninstall { "uninstall" } else { "install" };
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(move |app| {
            let data_dir = app.path().app_data_dir()?;
            app.manage(NhDesktopClient::new()?);
            app.manage(Db::new(&data_dir.join("nh-desktop.db"))?);
            open_installer_window(app, mode)?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::installer_status,
            commands::installer_disk_space,
            commands::installer_install,
            commands::installer_uninstall,
        ])
        .run(tauri::generate_context!())
        .expect("error while running installer");
}

fn open_installer_window(app: &tauri::App, mode: &str) -> tauri::Result<()> {
    use tauri::WebviewUrl;
    use tauri::WebviewWindowBuilder;

    if let Some(window) = app.get_webview_window("installer") {
        let _ = window.set_focus();
        return Ok(());
    }

    let url = WebviewUrl::App(format!("installer?mode={mode}").into());
    WebviewWindowBuilder::new(app, "installer", url)
        .title("NH Desktop Setup")
        .inner_size(820.0, 620.0)
        .min_inner_size(720.0, 560.0)
        .resizable(true)
        .center()
        .build()?;
    Ok(())
}