use crate::error::AppError;
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};
use tokio::sync::Mutex;

const BASE_API: &str = "https://nhentai.net/api/v2";
const THROTTLE: Duration = Duration::from_millis(500);
const ALLOWED_IMAGE_HOSTS: [&str; 2] = ["t.nhentai.net", "i.nhentai.net"];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagResponse {
    pub id: u64,
    #[serde(rename = "type")]
    pub kind: String,
    pub name: String,
    pub slug: String,
    pub url: String,
    pub count: u64,
    pub description: Option<String>,
    pub is_community: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GalleryListItem {
    pub id: u64,
    pub media_id: String,
    pub english_title: String,
    pub japanese_title: Option<String>,
    pub thumbnail: String,
    pub thumbnail_width: Option<u64>,
    pub thumbnail_height: Option<u64>,
    pub num_pages: Option<u64>,
    pub num_favorites: Option<u64>,
    pub tag_ids: Vec<u64>,
    pub blacklisted: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GalleryTitle {
    pub english: String,
    pub japanese: Option<String>,
    pub pretty: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoverInfo {
    pub path: String,
    pub width: u64,
    pub height: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageInfo {
    pub number: u64,
    pub path: String,
    pub width: u64,
    pub height: u64,
    pub thumbnail: String,
    pub thumbnail_width: u64,
    pub thumbnail_height: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GalleryDetail {
    pub id: u64,
    pub media_id: String,
    pub title: GalleryTitle,
    pub cover: CoverInfo,
    pub thumbnail: CoverInfo,
    pub upload_date: Option<u64>,
    pub scanlator: Option<String>,
    pub num_pages: Option<u64>,
    pub num_favorites: Option<u64>,
    pub tags: Vec<TagResponse>,
    pub pages: Option<Vec<PageInfo>>,
    pub is_favorited: Option<bool>,
    pub related: Option<Vec<GalleryListItem>>,
    pub comment_count: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Paginated<T> {
    pub result: Vec<T>,
    pub num_pages: u64,
    pub per_page: Option<u64>,
    pub total: Option<u64>,
}

pub type GalleryList = Paginated<GalleryListItem>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelatedGalleries {
    pub result: Vec<GalleryListItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FavoriteResponse {
    pub favorited: bool,
    pub num_favorites: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserMeResponse {
    pub id: u64,
    pub username: String,
    pub slug: String,
    pub avatar_url: String,
    pub theme: Option<String>,
    pub is_staff: Option<bool>,
    pub is_superuser: Option<bool>,
    pub about: Option<String>,
    pub favorite_tags: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlacklistListResponse {
    pub tags: Vec<BlacklistedTagResponse>,
    pub count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlacklistedTagResponse {
    pub id: u64,
    #[serde(rename = "type")]
    pub kind: String,
    pub name: String,
    pub slug: String,
    pub count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadResponse {
    pub url: String,
    pub expires_at: u64,
}

pub struct NhentaiClient {
    http: reqwest::Client,
    last_request: Mutex<Instant>,
}

impl NhentaiClient {
    pub fn new() -> Result<Self, reqwest::Error> {
        let http = reqwest::Client::builder()
            .user_agent(format!("nhentai/{}", env!("CARGO_PKG_VERSION")))
            .timeout(Duration::from_secs(30))
            .build()?;
        Ok(Self {
            http,
            last_request: Mutex::new(Instant::now()),
        })
    }

    async fn pace(&self) {
        let mut last = self.last_request.lock().await;
        let elapsed = last.elapsed();
        if elapsed < THROTTLE {
            tokio::time::sleep(THROTTLE - elapsed).await;
        }
        *last = Instant::now();
    }

    pub(crate) async fn request<T: for<'de> Deserialize<'de>>(
        &self,
        key: Option<&str>,
        method: reqwest::Method,
        path: &str,
        params: &[(&str, String)],
        body: Option<serde_json::Value>,
    ) -> Result<T, AppError> {
        let url = url::Url::parse_with_params(&format!("{BASE_API}{path}"), params)
            .map_err(|e| AppError::InvalidInput(e.to_string()))?;
        self.pace().await;
        let mut req = self.http.request(method, url);
        if let Some(key) = key {
            req = req.header("Authorization", format!("Key {key}"));
        }
        if let Some(body) = body {
            req = req.json(&body);
        }
        let resp = req.send().await.map_err(AppError::Http)?;
        let status = resp.status();
        if !status.is_success() {
            return Err(AppError::Status(status));
        }
        resp.json().await.map_err(AppError::Http)
    }

    pub async fn list_galleries(&self, key: Option<&str>, page: u32, per_page: u32) -> Result<GalleryList, AppError> {
        self.request(key, reqwest::Method::GET, "/galleries", &[("page", page.to_string()), ("per_page", per_page.to_string())], None).await
    }

    pub async fn tagged(&self, key: Option<&str>, tag_id: u64, sort: &str, page: u32, per_page: u32) -> Result<GalleryList, AppError> {
        self.request(
            key,
            reqwest::Method::GET,
            "/galleries/tagged",
            &[
                ("tag_id", tag_id.to_string()),
                ("sort", sort.to_string()),
                ("page", page.to_string()),
                ("per_page", per_page.to_string()),
            ],
            None,
        )
        .await
    }

    pub async fn popular(&self, key: Option<&str>) -> Result<Vec<GalleryListItem>, AppError> {
        self.request(key, reqwest::Method::GET, "/galleries/popular", &[], None).await
    }

    pub async fn gallery(&self, key: Option<&str>, id: u64, include: &str) -> Result<GalleryDetail, AppError> {
        let path = format!("/galleries/{id}");
        let params: &[(&str, String)] = if include.is_empty() {
            &[]
        } else {
            &[("include", include.to_string())]
        };
        self.request(key, reqwest::Method::GET, &path, params, None).await
    }

    pub async fn related(&self, key: Option<&str>, id: u64) -> Result<RelatedGalleries, AppError> {
        self.request(key, reqwest::Method::GET, &format!("/galleries/{id}/related"), &[], None).await
    }

    pub async fn search(&self, key: Option<&str>, query: &str, sort: &str, page: u32) -> Result<GalleryList, AppError> {
        self.request(
            key,
            reqwest::Method::GET,
            "/search",
            &[("query", query.to_string()), ("sort", sort.to_string()), ("page", page.to_string())],
            None,
        )
        .await
    }

    pub async fn check_favorite(&self, key: &str, id: u64) -> Result<FavoriteResponse, AppError> {
        self.request(Some(key), reqwest::Method::GET, &format!("/galleries/{id}/favorite"), &[], None).await
    }

    pub async fn add_favorite(&self, key: &str, id: u64) -> Result<FavoriteResponse, AppError> {
        self.request(Some(key), reqwest::Method::POST, &format!("/galleries/{id}/favorite"), &[], None).await
    }

    pub async fn remove_favorite(&self, key: &str, id: u64) -> Result<FavoriteResponse, AppError> {
        self.request(Some(key), reqwest::Method::DELETE, &format!("/galleries/{id}/favorite"), &[], None).await
    }

    pub async fn my_favorites(&self, key: &str, query: &str, page: u32) -> Result<GalleryList, AppError> {
        let params: &[(&str, String)] = if query.is_empty() {
            &[("page", page.to_string())]
        } else {
            &[("q", query.to_string()), ("page", page.to_string())]
        };
        self.request(Some(key), reqwest::Method::GET, "/favorites", params, None).await
    }

    pub async fn current_user(&self, key: &str) -> Result<UserMeResponse, AppError> {
        self.request(Some(key), reqwest::Method::GET, "/user", &[], None).await
    }

    pub async fn blacklist(&self, key: &str) -> Result<BlacklistListResponse, AppError> {
        self.request(Some(key), reqwest::Method::GET, "/blacklist", &[], None).await
    }

    pub async fn update_blacklist(&self, key: &str, added: &[u64], removed: &[u64]) -> Result<serde_json::Value, AppError> {
        let body = serde_json::json!({ "added": added, "removed": removed });
        self.request(Some(key), reqwest::Method::POST, "/blacklist", &[], Some(body)).await
    }

    pub async fn download(&self, key: &str, id: u64, format: &str) -> Result<DownloadResponse, AppError> {
        self.request(
            Some(key),
            reqwest::Method::GET,
            &format!("/galleries/{id}/download"),
            &[("format", format.to_string())],
            None,
        )
        .await
    }

    pub async fn image_bytes(&self, url: &str) -> Result<Vec<u8>, AppError> {
        let parsed = url::Url::parse(url).map_err(|e| AppError::InvalidInput(format!("bad url: {e}")))?;
        if parsed.scheme() != "https" {
            return Err(AppError::InvalidInput("only https is allowed".into()));
        }
        let host = parsed.host_str().unwrap_or_default();
        if !ALLOWED_IMAGE_HOSTS.contains(&host) {
            return Err(AppError::InvalidInput(format!("host {host} is not allowed")));
        }
        self.pace().await;
        let resp = self.http.get(url).send().await.map_err(AppError::Http)?;
        let status = resp.status();
        if !status.is_success() {
            return Err(AppError::Status(status));
        }
        resp.bytes().await.map(|b| b.to_vec()).map_err(AppError::Http)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserializes_gallery_list_from_v2_fixture() {
        let json = r#"{
            "result": [
                {
                    "id": 577010,
                    "media_id": "2188313",
                    "english_title": "A Sample Doujin",
                    "japanese_title": null,
                    "thumbnail": "https://t.nhentai.net/galleries/2188313/thumb.jpg",
                    "thumbnail_width": 250,
                    "thumbnail_height": 350,
                    "num_pages": 24,
                    "num_favorites": 123,
                    "tag_ids": [12227, 33172],
                    "blacklisted": false
                }
            ],
            "num_pages": 42,
            "per_page": 25,
            "total": 1050
        }"#;
        let list: GalleryList = serde_json::from_str(json).expect("fixture must parse");
        assert_eq!(list.result.len(), 1);
        assert_eq!(list.num_pages, 42);
        assert_eq!(list.total, Some(1050));
        let item = &list.result[0];
        assert_eq!(item.id, 577010);
        assert_eq!(item.media_id, "2188313");
        assert_eq!(item.english_title, "A Sample Doujin");
        assert_eq!(item.thumbnail, "https://t.nhentai.net/galleries/2188313/thumb.jpg");
        assert_eq!(item.tag_ids, vec![12227, 33172]);
        assert_eq!(item.blacklisted, Some(false));
    }

    #[test]
    fn deserializes_gallery_detail_from_v2_fixture() {
        let json = r#"{
            "id": 577010,
            "media_id": "2188313",
            "title": {
                "english": "A Sample Doujin",
                "japanese": null,
                "pretty": "[Artist] A Sample Doujin"
            },
            "cover": { "path": "https://t.nhentai.net/galleries/2188313/cover.jpg", "width": 350, "height": 500 },
            "thumbnail": { "path": "https://t.nhentai.net/galleries/2188313/thumb.jpg", "width": 250, "height": 350 },
            "upload_date": 1737072000,
            "scanlator": "",
            "num_pages": 24,
            "num_favorites": 123,
            "tags": [
                { "id": 12227, "type": "language", "name": "english", "slug": "english", "url": "/language/english/", "count": 123456 }
            ],
            "pages": [
                { "number": 1, "path": "https://i.nhentai.net/galleries/2188313/1.jpg", "width": 900, "height": 1400,
                  "thumbnail": "https://t.nhentai.net/galleries/2188313/1t.jpg", "thumbnail_width": 250, "thumbnail_height": 350 }
            ],
            "is_favorited": false,
            "related": null,
            "comment_count": 5
        }"#;
        let detail: GalleryDetail = serde_json::from_str(json).expect("fixture must parse");
        assert_eq!(detail.id, 577010);
        assert_eq!(detail.media_id, "2188313");
        assert_eq!(detail.title.english, "A Sample Doujin");
        assert_eq!(detail.num_pages, Some(24));
        assert_eq!(detail.is_favorited, Some(false));
        assert_eq!(detail.pages.as_ref().unwrap().len(), 1);
        assert_eq!(detail.pages.as_ref().unwrap()[0].path, "https://i.nhentai.net/galleries/2188313/1.jpg");
        assert_eq!(detail.tags[0].kind, "language");
        assert_eq!(detail.tags[0].count, 123456);
        assert_eq!(detail.comment_count, Some(5));
    }

    #[test]
    fn deserializes_favorite_and_user_fixtures() {
        let fav: FavoriteResponse = serde_json::from_str(r#"{ "favorited": true, "num_favorites": 124 }"#).expect("fav");
        assert!(fav.favorited);
        assert_eq!(fav.num_favorites, Some(124));

        let user: UserMeResponse = serde_json::from_str(
            r#"{ "id": 123, "username": "dev", "slug": "dev", "avatar_url": "https://static.nhentai.net/avatars/123.png" }"#,
        )
        .expect("user");
        assert_eq!(user.username, "dev");
        assert!(user.email.is_none());
    }

    #[test]
    fn parses_only_https_allowlisted_image_hosts() {
        let cases: [(&str, bool); 3] = [
            ("https://i.nhentai.net/galleries/123/1.jpg", true),
            ("https://t.nhentai.net/galleries/123/thumb.jpg", true),
            ("http://evil.example/x.jpg", false),
        ];
        for (url, should_pass) in cases {
            let parsed = url::Url::parse(url).unwrap();
            let host = parsed.host_str().unwrap_or_default();
            let pass = parsed.scheme() == "https" && ALLOWED_IMAGE_HOSTS.contains(&host);
            assert_eq!(pass, should_pass, "case {url}");
        }
    }
}