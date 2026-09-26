mod commands;
mod db;
mod error;
mod image_cache;
mod installer;
mod nh_desktop;
mod platform;
mod service;

use db::Db;
use image_cache::ImageCache;
use nh_desktop::NhDesktopClient;
use service::BackgroundService;
use std::sync::Arc;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.unminimize();
                let _ = window.set_focus();
            }
        }))
        .setup(|app| {
            let data_dir = app.path().app_data_dir()?;
            let default_downloads_dir = app
                .path()
                .document_dir()
                .unwrap_or_else(|_| data_dir.clone())
                .join("NH Desktop")
                .join("downloads");

            let client = NhDesktopClient::new()?;
            app.manage(client.clone());
            app.manage(Db::new(&data_dir.join("nh-desktop.db"))?);

            let cache = Arc::new(ImageCache::new(data_dir.join("cache/images")));
            app.manage(cache.clone());

            let service = BackgroundService::spawn(
                app.handle().clone(),
                client,
                cache,
                &data_dir.join("nh-desktop.db"),
                default_downloads_dir,
            );
            app.manage(service);

            use tauri::menu::{Menu, MenuItem, PredefinedMenuItem};
            use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};

            let show_i = MenuItem::with_id(app, "show", "Show NH Desktop", true, None::<&str>)?;
            let min_i = MenuItem::with_id(app, "minimize", "Minimize to Tray", true, None::<&str>)?;
            let sep = PredefinedMenuItem::separator(app)?;
            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;

            let menu = Menu::with_items(app, &[&show_i, &min_i, &sep, &quit_i])?;

            let mut tray_builder = TrayIconBuilder::new()
                .menu(&menu)
                .show_menu_on_left_click(false)
                .tooltip("NH Desktop")
                .on_menu_event(|app, event| match event.id().as_ref() {
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.unminimize();
                            let _ = window.set_focus();
                        }
                    }
                    "minimize" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.hide();
                        }
                    }
                    "quit" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.destroy();
                        }
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
                        if let Some(window) = app.get_webview_window("main") {
                            if window.is_visible().unwrap_or(false) {
                                let _ = window.hide();
                            } else {
                                let _ = window.show();
                                let _ = window.unminimize();
                                let _ = window.set_focus();
                            }
                        }
                    }
                });

            let tray_icon =
                tauri::image::Image::from_bytes(include_bytes!("../icons/tray-icon.png")).ok();
            if let Some(icon) = tray_icon.or_else(|| app.default_window_icon().cloned()) {
                tray_builder = tray_builder.icon(icon);
            }

            tray_builder.build(app)?;

            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                if window.label() == "main" {
                    api.prevent_close();
                    let _ = window.hide();
                }
            }
        })
        .invoke_handler(tauri::generate_handler![
            commands::fetch_new,
            commands::fetch_popular,
            commands::fetch_tagged,
            commands::search_galleries,
            commands::fetch_gallery,
            commands::related_galleries,
            commands::fetch_tag_info,
            commands::fetch_tags_by_type,
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
            commands::service_enqueue_download,
            commands::service_enqueue_prefetch,
            commands::service_enqueue_maintenance,
            commands::service_enqueue_sync,
            commands::service_status,
            commands::service_set_auto_refresh,
            commands::service_get_auto_refresh,
            commands::service_get_downloads_dir,
            commands::service_set_downloads_dir,
            commands::service_reset_downloads_dir,
            commands::open_downloads_folder,
            commands::app_quit,
            commands::get_system_locale,
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
	let _ = crate::platform::quit_running_app();
	let is_uninstall = std::env::args().any(|a| a == "--uninstall" || a == "--maintenance");
	let mode = if is_uninstall { "uninstall" } else { "install" };
	tauri::Builder::default()
		.plugin(tauri_plugin_opener::init())
		.setup(move |app| {
			let data_dir = app.path().app_data_dir()?;
			app.manage(NhDesktopClient::new()?);
			app.manage(Db::new(&data_dir.join("nh-desktop.db"))?);
			if let Some(main) = app.get_webview_window("main") {
				let _ = main.destroy();
			}
			open_installer_window(app, mode)?;
			Ok(())
		})
        .invoke_handler(tauri::generate_handler![
            commands::installer_status,
            commands::installer_disk_space,
            commands::installer_install,
            commands::installer_uninstall,
            commands::installer_launch_app,
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
    let mut builder = WebviewWindowBuilder::new(app, "installer", url)
        .title("NH Desktop Setup")
        .inner_size(820.0, 620.0)
        .min_inner_size(720.0, 560.0)
        .resizable(true)
        .decorations(false)
        .center();
    if let Some(icon) = app.default_window_icon() {
        builder = builder.icon(icon.clone())?;
    }
    builder.build()?;
    Ok(())
}