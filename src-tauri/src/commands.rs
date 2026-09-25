use crate::db::Db;
use crate::image_cache::ImageCache;
use crate::nh_desktop::{
    DownloadResponse, FavoriteResponse, GalleryDetail, GalleryList, NhDesktopClient, Paginated, RelatedGalleries,
    TagResponse, UserMeResponse, GalleryListItem,
};
use crate::service::{get_auto_refresh, set_auto_refresh, AutoRefreshConfig, BackgroundService, ServiceStatus};
use serde::Serialize;
use std::sync::Arc;
use tauri::{Manager, State};

#[derive(Serialize)]
pub struct ApiKeyStatus {
    pub configured: bool,
    pub prefix: Option<String>,
}

fn optional_key(db: &Db) -> Option<String> {
    db.api_key().ok().flatten()
}

fn require_key(db: &Db) -> Result<String, String> {
    db.api_key()
        .map_err(|e| format!("Failed to read stored api key: {e}"))?
        .ok_or_else(|| "No nhentai API key configured. Add one in Settings.".to_string())
}

#[tauri::command]
pub fn app_quit(app: tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.destroy();
    }
    if let Some(window) = app.get_webview_window("installer") {
        let _ = window.destroy();
    }
    app.exit(0);
}

#[tauri::command]
pub async fn fetch_new(client: State<'_, NhDesktopClient>, db: State<'_, Db>, page: Option<u32>, per_page: Option<u32>) -> Result<GalleryList, String> {
    client
        .list_galleries(optional_key(&db).as_deref(), page.unwrap_or(1), per_page.unwrap_or(28))
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn fetch_popular(client: State<'_, NhDesktopClient>, db: State<'_, Db>) -> Result<Vec<GalleryListItem>, String> {
    client.popular(optional_key(&db).as_deref()).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn fetch_tagged(client: State<'_, NhDesktopClient>, db: State<'_, Db>, tag_id: u64, sort: Option<String>, page: Option<u32>, per_page: Option<u32>) -> Result<GalleryList, String> {
    client
        .tagged(optional_key(&db).as_deref(), tag_id, sort.as_deref().unwrap_or("date"), page.unwrap_or(1), per_page.unwrap_or(28))
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn search_galleries(client: State<'_, NhDesktopClient>, db: State<'_, Db>, query: String, sort: Option<String>, page: Option<u32>) -> Result<GalleryList, String> {
    client
        .search(optional_key(&db).as_deref(), &query, sort.as_deref().unwrap_or("date"), page.unwrap_or(1))
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn fetch_gallery(client: State<'_, NhDesktopClient>, db: State<'_, Db>, id: u64, include: Option<String>) -> Result<GalleryDetail, String> {
    client
        .gallery(optional_key(&db).as_deref(), id, include.as_deref().unwrap_or("favorite"))
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn related_galleries(client: State<'_, NhDesktopClient>, db: State<'_, Db>, id: u64) -> Result<RelatedGalleries, String> {
    client.related(optional_key(&db).as_deref(), id).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn fetch_tag_info(client: State<'_, NhDesktopClient>, tag_type: String, slug: String) -> Result<TagResponse, String> {
    client
        .request::<TagResponse>(None, reqwest::Method::GET, &format!("/tags/{tag_type}/{slug}"), &[], None)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn fetch_tags_by_type(
    client: State<'_, NhDesktopClient>,
    db: State<'_, Db>,
    tag_type: String,
    sort: Option<String>,
    page: Option<u32>,
    per_page: Option<u32>,
) -> Result<Paginated<TagResponse>, String> {
    client
        .tags_by_type(
            optional_key(&db).as_deref(),
            &tag_type,
            sort.as_deref().unwrap_or("popular"),
            page.unwrap_or(1),
            per_page.unwrap_or(24),
        )
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn proxy_image(client: State<'_, NhDesktopClient>, cache: State<'_, Arc<ImageCache>>, url: String) -> Result<Vec<u8>, String> {
    if let Some(bytes) = cache.get(&url) {
        return Ok(bytes);
    }
    let bytes = client.image_bytes(&url).await.map_err(|e| e.to_string())?;
    let _ = cache.put(&url, &bytes);
    Ok(bytes)
}

#[tauri::command]
pub fn db_get(db: State<'_, Db>, key: String) -> Result<Option<String>, String> {
    db.get(&key).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn db_set(db: State<'_, Db>, key: String, value: String) -> Result<(), String> {
    db.set(&key, &value).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn db_del(db: State<'_, Db>, key: String) -> Result<(), String> {
    db.del(&key).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn db_dump(db: State<'_, Db>) -> Result<Vec<(String, String)>, String> {
    db.dump().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn db_clear(db: State<'_, Db>) -> Result<(), String> {
    db.clear().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn set_api_key(db: State<'_, Db>, key: String) -> Result<(), String> {
    let trimmed = key.trim();
    if trimmed.is_empty() {
        return Err("API key must not be empty.".to_string());
    }
    db.set_api_key(trimmed).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_api_key_status(db: State<'_, Db>) -> Result<ApiKeyStatus, String> {
    Ok(match db.api_key().map_err(|e| e.to_string())? {
        Some(key) => ApiKeyStatus {
            configured: true,
            prefix: Some(key.chars().take(4).collect()),
        },
        None => ApiKeyStatus { configured: false, prefix: None },
    })
}

#[tauri::command]
pub fn clear_api_key(db: State<'_, Db>) -> Result<(), String> {
    db.clear_api_key().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn verify_api_key(client: State<'_, NhDesktopClient>, db: State<'_, Db>) -> Result<UserMeResponse, String> {
    let key = require_key(&db)?;
    client.current_user(&key).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_current_user(client: State<'_, NhDesktopClient>, db: State<'_, Db>) -> Result<UserMeResponse, String> {
    let key = require_key(&db)?;
    client.current_user(&key).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn check_favorite(client: State<'_, NhDesktopClient>, db: State<'_, Db>, id: u64) -> Result<FavoriteResponse, String> {
    let key = require_key(&db)?;
    client.check_favorite(&key, id).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn add_favorite(client: State<'_, NhDesktopClient>, db: State<'_, Db>, id: u64) -> Result<FavoriteResponse, String> {
    let key = require_key(&db)?;
    client.add_favorite(&key, id).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn remove_favorite(client: State<'_, NhDesktopClient>, db: State<'_, Db>, id: u64) -> Result<FavoriteResponse, String> {
    let key = require_key(&db)?;
    client.remove_favorite(&key, id).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn fetch_favorites(client: State<'_, NhDesktopClient>, db: State<'_, Db>, query: Option<String>, page: Option<u32>) -> Result<GalleryList, String> {
    let key = require_key(&db)?;
    client.my_favorites(&key, query.as_deref().unwrap_or(""), page.unwrap_or(1)).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn fetch_account_blacklist(client: State<'_, NhDesktopClient>, db: State<'_, Db>) -> Result<crate::nh_desktop::BlacklistListResponse, String> {
    let key = require_key(&db)?;
    client.blacklist(&key).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_account_blacklist(client: State<'_, NhDesktopClient>, db: State<'_, Db>, added: Vec<u64>, removed: Vec<u64>) -> Result<serde_json::Value, String> {
    let key = require_key(&db)?;
    client.update_blacklist(&key, &added, &removed).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn download_gallery(client: State<'_, NhDesktopClient>, db: State<'_, Db>, id: u64, format: Option<String>) -> Result<DownloadResponse, String> {
    let key = require_key(&db)?;
    client.download(&key, id, format.as_deref().unwrap_or("zip")).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn service_enqueue_download(service: State<'_, Arc<BackgroundService>>, id: u64, format: Option<String>) -> Result<u64, String> {
    service.enqueue_download(id, format.unwrap_or_else(|| "zip".to_string())).await
}

#[tauri::command]
pub async fn service_enqueue_prefetch(service: State<'_, Arc<BackgroundService>>, urls: Vec<String>) -> Result<u64, String> {
    service.enqueue_prefetch(urls).await
}

#[tauri::command]
pub async fn service_enqueue_maintenance(service: State<'_, Arc<BackgroundService>>) -> Result<u64, String> {
    service.enqueue_maintenance().await
}

#[tauri::command]
pub async fn service_enqueue_sync(service: State<'_, Arc<BackgroundService>>) -> Result<u64, String> {
    service.enqueue_sync().await
}

#[tauri::command]
pub fn service_status(service: State<'_, Arc<BackgroundService>>) -> ServiceStatus {
    service.status()
}

#[tauri::command]
pub fn service_set_auto_refresh(db: State<'_, Db>, enabled: bool, interval_minutes: u32) -> Result<(), String> {
    let cfg = AutoRefreshConfig {
        enabled,
        interval_minutes: interval_minutes.max(15),
    };
    set_auto_refresh(&db, &cfg)
}

#[tauri::command]
pub fn service_get_auto_refresh(db: State<'_, Db>) -> AutoRefreshConfig {
    get_auto_refresh(&db)
}

#[tauri::command]
pub fn service_get_downloads_dir(app: tauri::AppHandle, db: State<'_, Db>) -> String {
    let data_dir = app
        .path()
        .app_data_dir()
        .unwrap_or_else(|_| std::env::temp_dir());
    let default = app
        .path()
        .document_dir()
        .unwrap_or(data_dir)
        .join("NH Desktop")
        .join("downloads");
    crate::service::get_downloads_dir(&db)
        .unwrap_or(default)
        .to_string_lossy()
        .into_owned()
}

#[tauri::command]
pub fn service_set_downloads_dir(db: State<'_, Db>, dir: String) -> Result<(), String> {
    crate::service::set_downloads_dir(&db, &dir)
}

#[tauri::command]
pub fn service_reset_downloads_dir(db: State<'_, Db>) -> Result<(), String> {
    db.del(crate::service::DOWNLOADS_DIR_KEY)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn open_downloads_folder(app: tauri::AppHandle, db: State<'_, Db>) -> Result<(), String> {
    let dir = crate::service::get_downloads_dir(&db).unwrap_or_else(|| {
        let data_dir = app
            .path()
            .app_data_dir()
            .unwrap_or_else(|_| std::env::temp_dir());
        app.path()
            .document_dir()
            .unwrap_or(data_dir)
            .join("NH Desktop")
            .join("downloads")
    });
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let dir_str = dir.to_string_lossy();
    tauri_plugin_opener::OpenerExt::opener(&app)
        .open_path(dir_str.to_string(), None::<&str>)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn installer_status() -> Result<crate::installer::InstallerStatus, String> {
    Ok(crate::installer::detect_status())
}

#[tauri::command]
pub fn installer_disk_space(target_dir: String) -> Result<crate::installer::DiskSpaceInfo, String> {
    Ok(crate::installer::check_disk_space(&target_dir))
}

#[tauri::command]
pub fn installer_install(options: crate::installer::InstallOptions) -> Result<crate::installer::OperationResult, String> {
    Ok(crate::installer::perform_install(options))
}

#[tauri::command]
pub fn installer_uninstall(options: crate::installer::UninstallOptions) -> Result<crate::installer::OperationResult, String> {
    Ok(crate::installer::perform_uninstall(options))
}

#[tauri::command]
pub fn open_maintenance_window(app: tauri::AppHandle) -> Result<(), String> {
	let is_uninstall = std::env::args().any(|a| a == "--uninstall");
	let flag = if is_uninstall { "--uninstall" } else { "--maintenance" };
	let exe = std::env::current_exe().map_err(|e| e.to_string())?;
	std::process::Command::new(exe)
		.arg(flag)
		.spawn()
		.map_err(|e| e.to_string())?;
	app.exit(0);
	Ok(())
}