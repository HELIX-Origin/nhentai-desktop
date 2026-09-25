use crate::db::Db;
use crate::image_cache::ImageCache;
use crate::nh_desktop::NhDesktopClient;
use futures_util::StreamExt;
use serde::Serialize;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use tauri::{AppHandle, Emitter};
use tokio::io::AsyncWriteExt;
use tokio::sync::mpsc;

const EVENT_JOB: &str = "service://job";
const EVENT_REFRESH: &str = "service://refresh";
const REFRESH_SETTINGS_KEY: &str = "settings:auto-refresh";
pub const DOWNLOADS_DIR_KEY: &str = "settings:downloads-dir";
const MIN_REFRESH_INTERVAL: u32 = 15;
const MAINTENANCE_IMAGE_AGE_DAYS: u64 = 30;
const MAINTENANCE_CACHE_AGE_DAYS: u64 = 7;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum JobKind {
    Download,
    Prefetch,
    Maintenance,
    Refresh,
    Sync,
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "state", rename_all = "snake_case")]
pub enum ServiceEvent {
    Queued {
        job_id: u64,
        kind: JobKind,
    },
    Started {
        job_id: u64,
        kind: JobKind,
    },
    Progress {
        job_id: u64,
        kind: JobKind,
        done: u64,
        total: Option<u64>,
        label: String,
    },
    Finished {
        job_id: u64,
        kind: JobKind,
        message: String,
    },
    Failed {
        job_id: u64,
        kind: JobKind,
        error: String,
    },
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct ServiceStatus {
    pub pending: u64,
}

enum Job {
    DownloadGallery { id: u64, format: String },
    PrefetchImages { urls: Vec<String> },
    Maintenance,
    RefreshPopular,
    SyncAccount,
}

impl Job {
    fn kind(&self) -> JobKind {
        match self {
            Job::DownloadGallery { .. } => JobKind::Download,
            Job::PrefetchImages { .. } => JobKind::Prefetch,
            Job::Maintenance => JobKind::Maintenance,
            Job::RefreshPopular => JobKind::Refresh,
            Job::SyncAccount => JobKind::Sync,
        }
    }
}

pub struct BackgroundService {
    tx: mpsc::Sender<(u64, Job)>,
    next_id: AtomicU64,
}

struct Worker {
    app: AppHandle,
    client: NhDesktopClient,
    cache: Arc<ImageCache>,
    db: Db,
    default_downloads_dir: PathBuf,
}

impl BackgroundService {
    pub fn spawn(
        app: AppHandle,
        client: NhDesktopClient,
        cache: Arc<ImageCache>,
        db_path: &std::path::Path,
        downloads_dir: PathBuf,
    ) -> Arc<Self> {
        let (tx, rx) = mpsc::channel(64);
        let svc = Arc::new(Self {
            tx,
            next_id: AtomicU64::new(0),
        });

        let worker = Worker {
            app: app.clone(),
            client,
            cache,
            db: Db::new(db_path).expect("failed to open background db"),
            default_downloads_dir: downloads_dir,
        };
        tauri::async_runtime::spawn(worker.run(rx));

        let scheduler = svc.clone();
        let app2 = app.clone();
        let db_path = db_path.to_path_buf();
        tauri::async_runtime::spawn(async move {
            scheduler.refresh_loop(app2, db_path).await;
        });

        svc
    }

    async fn enqueue(&self, job: Job) -> Result<u64, String> {
        let job_id = self.next_id.fetch_add(1, Ordering::Relaxed);
        self.tx
            .send((job_id, job))
            .await
            .map_err(|e| format!("background service unavailable: {e}"))?;
        Ok(job_id)
    }

    pub async fn enqueue_download(&self, id: u64, format: String) -> Result<u64, String> {
        self.enqueue(Job::DownloadGallery { id, format }).await
    }

    pub async fn enqueue_prefetch(&self, urls: Vec<String>) -> Result<u64, String> {
        self.enqueue(Job::PrefetchImages { urls }).await
    }

    pub async fn enqueue_maintenance(&self) -> Result<u64, String> {
        self.enqueue(Job::Maintenance).await
    }

    pub async fn enqueue_sync(&self) -> Result<u64, String> {
        self.enqueue(Job::SyncAccount).await
    }

    pub fn status(&self) -> ServiceStatus {
        ServiceStatus {
            pending: self.next_id.load(Ordering::Relaxed),
        }
    }

    async fn refresh_loop(&self, app: AppHandle, db_path: PathBuf) {
        let mut ticker = tokio::time::interval(Duration::from_secs(60));
        let mut last_refresh: Option<Instant> = None;
        loop {
            ticker.tick().await;
            let db = Db::new(&db_path).ok();
            let Some(db) = db else { continue };
            let cfg = db
                .get(REFRESH_SETTINGS_KEY)
                .ok()
                .flatten()
                .and_then(|s| serde_json::from_str::<AutoRefreshConfig>(&s).ok());
            let Some(cfg) = cfg else { continue };
            if !cfg.enabled {
                last_refresh = None;
                continue;
            }
            let due = last_refresh
                .map(|t| t.elapsed() >= Duration::from_secs(u64::from(cfg.interval_minutes) * 60))
                .unwrap_or(true);
            if due {
                let _ = self.enqueue(Job::RefreshPopular).await;
                last_refresh = Some(Instant::now());
                let _ = app.emit(EVENT_REFRESH, "popular");
            }
        }
    }
}

impl Worker {
    async fn run(self, mut rx: mpsc::Receiver<(u64, Job)>) {
        while let Some((job_id, job)) = rx.recv().await {
            let kind = job.kind();
            let _ = self
                .app
                .emit(EVENT_JOB, ServiceEvent::Queued { job_id, kind });
            let _ = self
                .app
                .emit(EVENT_JOB, ServiceEvent::Started { job_id, kind });
            match self.run_job(job, job_id).await {
                Ok(message) => {
                    let _ = self
                        .app
                        .emit(EVENT_JOB, ServiceEvent::Finished { job_id, kind, message });
                }
                Err(error) => {
                    let _ = self
                        .app
                        .emit(EVENT_JOB, ServiceEvent::Failed { job_id, kind, error });
                }
            }
        }
    }

    async fn run_job(&self, job: Job, job_id: u64) -> Result<String, String> {
        match job {
            Job::DownloadGallery { id, format } => self.download_gallery(job_id, id, &format).await,
            Job::PrefetchImages { urls } => self.prefetch_images(job_id, urls).await,
            Job::Maintenance => self.maintenance().await,
            Job::RefreshPopular => self.refresh_popular().await,
            Job::SyncAccount => self.sync_account().await,
        }
    }

    fn resolve_downloads_dir(&self) -> PathBuf {
        get_downloads_dir(&self.db).unwrap_or_else(|| self.default_downloads_dir.clone())
    }

    async fn download_gallery(&self, job_id: u64, id: u64, format: &str) -> Result<String, String> {
        let key = self
            .db
            .api_key()
            .map_err(|e| format!("failed to read api key: {e}"))?
            .ok_or_else(|| "No nhentai API key configured. Add one in Settings.".to_string())?;

        let dl = self
            .client
            .download(&key, id, format)
            .await
            .map_err(|e| e.to_string())?;

        let downloads_dir = self.resolve_downloads_dir();
        let ext = match format {
            "cbz" => "cbz",
            "torrent" => "torrent",
            _ => "zip",
        };
        let tmp = downloads_dir.join(format!("gallery-{id}.{ext}.part"));
        let dest = downloads_dir.join(format!("gallery-{id}.{ext}"));
        std::fs::create_dir_all(&downloads_dir).map_err(|e| e.to_string())?;

        let resp = self
            .client
            .open_bytes(&dl.url)
            .await
            .map_err(|e| e.to_string())?;
        let total = resp.content_length();
        let mut file = tokio::fs::File::create(&tmp).await.map_err(|e| e.to_string())?;

        let mut stream = resp.bytes_stream();
        let mut done: u64 = 0;
        let mut last_emit = Instant::now();
        while let Some(chunk) = stream.next().await {
            let chunk = chunk.map_err(|e| format!("download failed mid-stream: {e}"))?;
            file.write_all(&chunk).await.map_err(|e| e.to_string())?;
            done += chunk.len() as u64;
            if last_emit.elapsed() >= Duration::from_millis(200) {
                self.emit_progress(job_id, done, total, format!("Downloading gallery {id}")).await;
                last_emit = Instant::now();
            }
        }
        file.flush().await.map_err(|e| e.to_string())?;
        file.shutdown().await.map_err(|e| e.to_string())?;
        std::fs::rename(&tmp, &dest).map_err(|e| e.to_string())?;

        Ok(format!(
            "Downloaded gallery {id} to {} ({:.1} MB)",
            dest.display(),
            done as f64 / 1_048_576.0
        ))
    }

    async fn emit_progress(&self, job_id: u64, done: u64, total: Option<u64>, label: String) {
        let _ = self.app.emit(
            EVENT_JOB,
            ServiceEvent::Progress {
                job_id,
                kind: JobKind::Download,
                done,
                total,
                label,
            },
        );
    }

    async fn prefetch_images(&self, job_id: u64, urls: Vec<String>) -> Result<String, String> {
        let mut cached = 0usize;
        let total = urls.len();
        for url in &urls {
            if self.cache.has(url) {
                cached += 1;
                continue;
            }
            let bytes = match self.client.image_bytes(url).await {
                Ok(b) => b,
                Err(_) => continue,
            };
            let _ = self.cache.put(url, &bytes);
            cached += 1;
            if job_id % 5 == 0 {
                self.emit_progress(job_id, cached as u64, Some(total as u64), "Prefetching images".into()).await;
            }
        }
        Ok(format!("Prefetched {cached}/{total} images"))
    }

    async fn maintenance(&self) -> Result<String, String> {
        let removed_images = self
            .cache
            .prune(Duration::from_secs(MAINTENANCE_IMAGE_AGE_DAYS * 24 * 3600));
        let before = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0)
            - (MAINTENANCE_CACHE_AGE_DAYS * 24 * 3600) as i64;
        let removed_entries = self
            .db
            .prune_cache(before)
            .map_err(|e| format!("failed to prune cache: {e}"))?;
        Ok(format!(
            "Maintenance: pruned {removed_entries} cache entries, {removed_images} image files"
        ))
    }

    async fn refresh_popular(&self) -> Result<String, String> {
        let list = self
            .client
            .popular(None)
            .await
            .map_err(|e| e.to_string())?;
        let json = serde_json::to_string(&list).map_err(|e| e.to_string())?;
        self.db
            .set("nh-desktop:cache:popular", &json)
            .map_err(|e| e.to_string())?;
        Ok(format!("Refreshed popular ({})", list.len()))
    }

    async fn sync_account(&self) -> Result<String, String> {
        let key = self
            .db
            .api_key()
            .map_err(|e| format!("failed to read api key: {e}"))?
            .ok_or_else(|| "No nhentai API key configured. Add one in Settings.".to_string())?;
        let favs = self
            .client
            .my_favorites(&key, "", 1)
            .await
            .map_err(|e| e.to_string())?;
        let blacklist = self
            .client
            .blacklist(&key)
            .await
            .map_err(|e| e.to_string())?;
        let fav_json = serde_json::to_string(&favs).map_err(|e| e.to_string())?;
        let bl_json = serde_json::to_string(&blacklist).map_err(|e| e.to_string())?;
        self.db
            .set("nh-desktop:cache:account:favorites:v1", &fav_json)
            .map_err(|e| e.to_string())?;
        self.db
            .set("nh-desktop:cache:account:blacklist:v1", &bl_json)
            .map_err(|e| e.to_string())?;
        Ok(format!(
            "Synced account: {} favorites, {} blacklist tags",
            favs.result.len(),
            blacklist.tags.len()
        ))
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AutoRefreshConfig {
    pub enabled: bool,
    pub interval_minutes: u32,
}

impl Default for AutoRefreshConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            interval_minutes: MIN_REFRESH_INTERVAL,
        }
    }
}

pub fn set_auto_refresh(db: &Db, cfg: &AutoRefreshConfig) -> Result<(), String> {
    let json =
        serde_json::to_string(cfg).map_err(|e| format!("failed to encode settings: {e}"))?;
    db.set(REFRESH_SETTINGS_KEY, &json)
        .map_err(|e| e.to_string())
}

pub fn get_auto_refresh(db: &Db) -> AutoRefreshConfig {
    db.get(REFRESH_SETTINGS_KEY)
        .ok()
        .flatten()
        .and_then(|s| serde_json::from_str::<AutoRefreshConfig>(&s).ok())
        .unwrap_or_default()
}

pub fn set_downloads_dir(db: &Db, dir: &str) -> Result<(), String> {
    db.set(DOWNLOADS_DIR_KEY, dir).map_err(|e| e.to_string())
}

pub fn get_downloads_dir(db: &Db) -> Option<PathBuf> {
    db.get(DOWNLOADS_DIR_KEY)
        .ok()
        .flatten()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .map(PathBuf::from)
}