# TODO.md

> Actionable task ledger. Statuses: ⬜ backlog · 🚧 in progress · ✅ done.
> High-level direction lives in `ROADMAP.md`; detailed per-item notes (optional) live in
> `.agents/tracking/todos/TODO-###.md`.
>
> When a task changes behavior or scope, update this file **in the same change**.
> When a task is done, move it to the bottom under **Done** and link the PR/commit if any.

## 🚧 Current focus: M1 closeout + M7/M8 polish

M1 Foundation is shipped (core). M9.1 localization core is shipped (English / Japanese / Chinese).
Active work is finishing the clean-clone verification gate and polishing the
downloads/background-service (M7) and release-hardening (M8) milestones. **M9.2 — the remaining
14 language packs — is planned in this file but deliberately not started.**

- ✅ **v0.3.0 released 2026-09-25** (M9.1 localization + installer launch fix + titlebar search) — gates
  green: `cargo test` 9/9, `npm run check` 0/0, `npm run i18n:check` 3/3, `npm run build`,
  `npm run build:installer` produced `NH Desktop-Setup-0.3.0-win-x64.exe`

## ✅ M9.1 — Localization core (shipped)

- ✅ i18n infrastructure: locale store, `t()` helper with English fallback, system-locale detection
  (new `get_system_locale` command via `sys-locale`)
- ✅ Language selector in the installer's *Options* step
- ✅ Language selector in `Settings → Appearance → Language`
- ✅ `t()` wired through the app shell (titlebar, sidebar, account chip) and the full installer
- ✅ Validator `npm run i18n:check` (missing / untranslated / unknown keys), fully offline
- ✅ Contributor documentation: `wiki/Localization.md` (improve existing pack + add new language,
  with free terminology cross-reference resources), linked from Home / Settings / Installation
- ✅ Core packs: `en` (source), `ja`, `zh-Hans`, `zh-Hant` — 78/78 keys each

## ⬜ M9.2 — Remaining language packs (planned, not started)

Deferred by decision — nothing here is scheduled or in progress. All 14 locales are already
registered in `src/lib/i18n/locales.ts` and selectable in the app; each falls back to English
per-key until its pack lands. Per the plan in `wiki/Localization.md`, each item is the same
four steps: copy `en.json` → translate values → register the import in `src/lib/i18n/index.ts` →
`npm run i18n:check` clean.

| # | Pack | Code | Notes |
| --- | --- | --- | --- |
| 1 | Korean | `ko` | High-content language on nhentai; first candidate |
| 2 | Spanish | `es` | Large reader base; `es-419` can be a follow-up split |
| 3 | French | `fr` | Mind non-breaking space before `:` |
| 4 | German | `de` | Straightforward |
| 5 | Russian | `ru` | Straightforward |
| 6 | Portuguese | `pt` | Decide `pt` vs `pt-BR` at registration |
| 7 | Italian | `it` | Straightforward |
| 8 | Thai | `th` | Straightforward |
| 9 | Vietnamese | `vi` | Straightforward |
| 10 | Indonesian | `id` | Straightforward |
| 11 | Polish | `pl` | Straightforward |
| 12 | Dutch | `nl` | Straightforward |
| 13 | Turkish | `tr` | Straightforward |
| 14 | Arabic | `ar` | **Requires an RTL pass first** (see below) |

Dependency: Arabic (`ar`) needs an RTL layout pass — `dir="rtl"` on the app root, mirrored
titlebar/window controls/sidebar, and flipped directional icons — before the pack is meaningful.
Hebrew (`he`) is not currently registered; add it alongside Arabic if an RTL pack is taken.

Also deferred, independent of the packs above:

- ⬜ Extend `en.json` and wrap the remaining untranslated strings — `SettingsView.svelte`
  (account panel, reader, downloads, background services, data), plus the Search, Blacklist,
  Downloads, Reader, and Gallery-detail views. Done opportunistically as each view is touched.
- ⬜ Per-locale date and number formatting (`relativeDate` / `formatCount` in `src/lib/format.ts`
  currently assume English conventions).

## ✅ M1 — Done

- ✅ CSP tightened (`img-src` for nhentai `t.`/`i.` hosts + data:/blob:, IPC `connect-src`, `devCsp` for Vite HMR) — verified with `npm run check` 0/0
- ✅ Scaffold Tauri 2 + SvelteKit (static SPA) project — template generated
- ✅ Rust: `nh_desktop.rs` API client (`reqwest`): gallery, search, new/popular/tagged lists, related, tag info, image proxy
- ✅ Rust: types (`Gallery`, `Tag`, `SearchResponse`, …) with serde + frontend-consistent shape (`src/lib/types.ts`)
- ✅ Rust: `error.rs` — friendly error type, no panics across command boundary
- ✅ Rust: `commands.rs` — 43 commands incl. `fetch_gallery`, `search_galleries`, `fetch_new`, `fetch_popular`, `fetch_tagged`, `fetch_tag_info`, `proxy_image`, `download_gallery`, favorites/blacklist/api-key/db, and the `service_*` background-service commands
- ✅ Rust: throttle for API calls (`THROTTLE` + `tokio::time::sleep` in `nh_desktop.rs`) with ioredis-mock-style cache (frontend `cache.ts`)
- ✅ Frontend: `src/lib/api.ts` + `client.ts` (typed invoke wrapper) + `query.ts` query builder
- ✅ Frontend: stores — settings, library (favorites/history), blacklist, account (runes +
  SQLite-backed cache via `src/lib/cache.ts`)
- ✅ Frontend: design tokens (CSS variables; dark + light themes live, system-aware via
  `settings.svelte.ts`)
- ✅ Frontend: app shell — sidebar nav (Latest, Popular, Favorites, History, Blacklist, Settings) + frameless child-window handling
- ✅ Frontend: `GalleryCard` + `GalleryGrid` with lazy images + image-proxy fallback
- ✅ Frontend: Latest (Home) + Popular views with pagination
- ✅ Frontend: Search view — filter drawer (query, language, category, tags include/exclude, page ranges, sort)
- ✅ Frontend: query builder → nhentai search syntax, `-tag:`/exclude wiring (`buildQuery` + `src/lib/query.ts`)
- ✅ Frontend: Blacklist view — manage tags/text, hide/blur mode, master toggle, server-side excludes
- ✅ Frontend: Favorites + History views
- ✅ Frontend: Gallery detail + reader (paged thumbnails, strip mode, preload, fullscreen)
- ✅ Background service (`src-tauri/src/service.rs`): throttled worker queue — gallery downloads
  (zip → disk, with progress), image prefetch, cache/image maintenance, account sync, Popular
  auto-refresh (off / 15 min / 1 h / daily); live `service://job` + `service://refresh` events
- ✅ Frontend service store (`src/lib/stores/service.svelte.ts`) + Settings "Background services"
  panel: auto-refresh toggle + interval presets, Sync account, Run maintenance, Recent jobs list
- ✅ Rust disk image cache (`src-tauri/src/image_cache.rs`, `cache/images`) backing `proxy_image`
  (cache-first; pruned by service maintenance)
- ✅ Single-instance support (`tauri-plugin-single-instance`): a second launch focuses the
  running window instead of spawning a duplicate
- ✅ Fixed image loading — absolute URLs now derived correctly from API v2 relative paths
  (`image.ts` `pagePath`/`thumbPath`/`avatarUrl`); image hosts accepted as any `*.nhentai.net`
  (incl. `static.nhentai.net` avatars) + matching CSP `img-src`
- ✅ Fixed sign-in status — topbar chip reflects `keyStatus.configured` (username / "Connected" /
  "Sign in") instead of a possibly-failed user fetch
- ✅ App icon wired everywhere: `static/favicon.png`, sidebar + installer brand marks, and the
  installer/maintenance window icons via `default_window_icon()`
- ✅ Static dev port 14440 (Vite + `tauri.conf.json`); removed `scripts/dev.mjs` auto-incrementing
- ✅ Docs: this pass — TODO/ROADMAP/BUGS/CHANGELOG/README/AGENTS + wiki updated to match

## ⬜ Backlog

- ⬜ Downloads UI (M7): queue resume/persist, cache-consumer wiring, storage-management polish
  (per-gallery Download button, ZIP/CBZ/torrent formats, configurable folder, and open-folder
  are already shipped)
- ⬜ Sync auto-refresh cache consumers: use the maintained `nh-desktop:cache:popular` /
  `nh-desktop:cache:account:*` mirrors in the Popular / account views
- ⬜ Import/export favorites + blacklist as JSON (M5)
- ⬜ Settings screen: theme accent, image quality, reader preload distance (M8)
- ⬜ End-to-end installer verification: run `npm run build:installer` and smoke-test generated
  `dist/installer/NH Desktop-Setup-{version}-win-x64.exe` (M8)

## 🔁 Recurring

- ⬜ Run `npm run check` + `cargo check`/`test` before any task is marked done.