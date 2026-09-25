# Architecture

High-level picture of how the app is built and why.

Here's the shape of the whole system — SvelteKit on top, Rust under it, nhentai.net below:

```mermaid
flowchart TD
    A[SvelteKit SPA] -->|"invoke()"| B[Rust backend]
    A -->|"direct img load"| E[nhentai.net CDN]
    B --> C[NhDesktopClient]
    C -->|"HTTPS throttled"| D[nhentai.net API]
    C -->|"proxy_image fallback"| K[ImageCache cache/images]
    B --> F[SQLite nh-desktop.db]
    S[BackgroundService service.rs] -->|"download/prefetch/maintenance/sync/refresh"| C
    S --> D
    S --> K
```

Rust owns all networking and local data; the SPA only renders. Images load straight from the
CDNs, with Rust as the proxy fallback — served from the on-disk image cache
(`cache/images`) when present. A background service (`service.rs`) runs a throttled worker
queue for gallery downloads, image prefetch, cache/image maintenance, account sync, and
periodic Popular refreshes.

## 🏗️ Design decisions

1. ✅ **Desktop shell = Tauri 2.** Rust backend owns all networking and persistence; the SPA is
   just presentation. Everything the site's API exposes is imported and re-sliced locally.
2. **One binary = app + installer.** The same executable is the app, the installer, and the
   uninstaller (arg-route based, `main.rs`). No NSIS/WiX/MSI packaging.
3. **Local-first.** No server, no accounts required. Favorites/history/blacklist/settings and
   the cache live on-device by default; an optional nhentai.net API key adds account sync.
4. **Images = direct + proxy fallback, URLs derived from the API.** URLs are built from the
   API's relative path fragments (`image.ts` `pagePath` / `thumbPath` / `avatarUrl`); `<img>`
   loads from the `*.nhentai.net` CDNs first, and `proxy_image` (Rust, served from the disk
   image cache `cache/images`) returns bytes → `blob:` URL if the CDN 404s (legacy galleries
   are known to).
5. **Blacklist is two layers.** Server-side `-tag:` excludes in every query, plus client-side
   hide/blur in grids, plus a master toggle. See [Blacklist](Blacklist.md).
6. **Plain CSS tokens.** Custom design system in `src/lib/design/tokens.css` + `base.css`;
   no CSS framework, no Tailwind — deliberate (see README).
7. **Background services.** `service.rs` runs a throttled one-at-a-time worker queue
   (downloads, prefetch, maintenance, account sync, periodic Popular refresh), surfaced as
   `service_*` commands with `service://job` / `service://refresh` events; Settings shows a
   live recent-jobs list. A single-instance guard (`tauri-plugin-single-instance`) focuses the
   running window instead of launching a duplicate (installer/maintenance mode is outside it).
8. **Disk image cache.** `image_cache.rs` stores fetched images under `cache/images`
   (atomic tmp+rename writes) and backs `proxy_image` cache-first; maintenance prunes images
   after 30 days and cached lists after 7.

## ⚡ Frontend → Backend contract

- ⚡ Frontend calls typed commands through `src/lib/client.ts` (a thin `invoke()` wrapper) and
  `src/lib/api.ts` (the higher-level typed API surface).
- Command names are camelCase (`fetch_new`, `search_galleries`, `proxy_image`, …); the full
  list lives in [Backend (Rust)](Backend-Rust.md).
- Background jobs use the `service_*` commands (`service_enqueue_download`, …) and stream
  progress to the SPA as `service://job` / `service://refresh` Tauri events.
- Errors cross the bridge as `Result<_, String>` (never Rust panics) and render as friendly
  notice components.

## 🦀 Backend → nhentai.net

- 🦀 All requests go through `NhDesktopClient` (reqwest, `rustls`, gzip). A `THROTTLE` delay
  between requests respects the site's public API (see `nh_desktop.rs`). No hammering. Ever.
- Type-accurate serde models mirror the API JSON.

## 💾 Persistence model

| Store | Location | Purpose |
| --- | --- | --- |
| Favorites / history / blacklist / settings | `localStorage` | user state, runes-backed |
| Cache mirror + API key | `nh-desktop.db` (SQLite) | fast startup, offline-ish lists, key at-rest |
| Image cache | `cache/images` (data dir) | disk cache backing `proxy_image`, pruned by maintenance |
| Downloads | `downloads/` (data dir) | zip archives from background-service downloads |

## 🤝 Related pages

- [Backend (Rust)](Backend-Rust.md) · [Frontend (SvelteKit)](Frontend-SvelteKit.md) ·
  [Installer Engine](Installer-Engine.md) · [Security](Security.md) · [Privacy](Privacy.md)