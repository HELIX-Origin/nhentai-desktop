use std::fs;
use std::hash::Hasher;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

pub struct ImageCache {
    dir: PathBuf,
}

impl ImageCache {
    pub fn new(dir: PathBuf) -> Self {
        let _ = fs::create_dir_all(&dir);
        Self { dir }
    }

    fn cache_name(&self, url: &str) -> String {
        let parsed = url::Url::parse(url).ok();
        if let Some(p) = parsed {
            let host = p.host_str().unwrap_or("img").replace('.', "_");
            let path = p.path().trim_start_matches('/').replace('/', "_");
            let safe: String = format!("{host}_{path}")
                .chars()
                .filter(|c| c.is_ascii_alphanumeric() || *c == '.' || *c == '_')
                .collect();
            if !safe.is_empty() {
                return safe;
            }
        }
        let mut h = std::collections::hash_map::DefaultHasher::new();
        std::hash::Hash::hash(&url, &mut h);
        format!("{:016x}.img", h.finish())
    }

    pub fn path_for(&self, url: &str) -> PathBuf {
        self.dir.join(self.cache_name(url))
    }

    pub fn has(&self, url: &str) -> bool {
        self.path_for(url).is_file()
    }

    pub fn get(&self, url: &str) -> Option<Vec<u8>> {
        fs::read(self.path_for(url)).ok()
    }

    pub fn put(&self, url: &str, bytes: &[u8]) -> std::io::Result<()> {
        let dest = self.path_for(url);
        let tmp = self.dir.join(format!("{}.tmp", self.cache_name(url)));
        fs::write(&tmp, bytes)?;
        if dest.exists() {
            let _ = fs::remove_file(&dest);
        }
        fs::rename(&tmp, dest)?;
        Ok(())
    }

    pub fn prune(&self, max_age: Duration) -> usize {
        let cutoff = SystemTime::now()
            .checked_sub(max_age)
            .unwrap_or(SystemTime::UNIX_EPOCH);
        let Ok(entries) = fs::read_dir(&self.dir) else {
            return 0;
        };
        let mut removed = 0;
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().is_some_and(|e| e == "tmp") {
                continue;
            }
            let stale = fs::metadata(&path)
                .and_then(|m| m.modified())
                .map(|t| t < cutoff)
                .unwrap_or(false);
            if stale {
                let _ = fs::remove_file(&path);
                removed += 1;
            }
        }
        removed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip_and_distinct_names() {
        let dir = std::env::temp_dir().join(format!("nh-ic-test-{}", std::process::id()));
        let cache = ImageCache::new(dir.clone());
        let a = "https://t.nhentai.net/galleries/123/thumb.jpg";
        let b = "https://i.nhentai.net/galleries/123/1.jpg";
        assert_ne!(cache.cache_name(a), cache.cache_name(b));
        cache.put(a, &[1, 2, 3]).unwrap();
        assert!(cache.has(a));
        assert_eq!(cache.get(a), Some(vec![1, 2, 3]));
        assert!(!cache.has(b));
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn prune_removes_old_only() {
        use std::fs::File;
        use std::fs::FileTimes;
        use std::time::UNIX_EPOCH;

        let dir = std::env::temp_dir().join(format!("nh-ic-prune-{}", std::process::id()));
        let cache = ImageCache::new(dir.clone());
        let url = "https://t.nhentai.net/galleries/9/thumb.jpg";
        cache.put(url, &[1]).unwrap();

        let path = cache.path_for(url);
        let old = UNIX_EPOCH;
        File::options()
            .write(true)
            .open(&path)
            .unwrap()
            .set_times(FileTimes::new().set_modified(old))
            .unwrap();

        assert_eq!(cache.prune(Duration::from_secs(1)), 1);

        cache.put(url, &[1]).unwrap();
        assert_eq!(cache.prune(Duration::from_secs(3600)), 0);

        let _ = fs::remove_dir_all(&dir);
    }
}