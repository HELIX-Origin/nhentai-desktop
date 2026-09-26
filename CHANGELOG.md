# 📜 NH Desktop Changelog

Historical record of every change to the NH Desktop client. Newer releases are added at the
top; the current development state lives under `Unreleased`.

## Unreleased

(No unreleased changes.)

## [v0.3.0](https://github.com/HELIX-Origin/nhentai-desktop/releases/tag/v0.3.0)

**Release date:** 2026-09-25

### ✨ Added

* **Localization & language packs (M9)**: The app interface can now be displayed in a language of
  your choice, defaulting to the operating system locale.
    * **Language packs**: Complete packs for **English** (source), **Japanese (`ja`)**, **Chinese
      Simplified (`zh-Hans`)**, and **Chinese Traditional (`zh-Hant`)** — 78 keys each. The other
      nhentai content languages are registered and selectable, falling back to English per-key;
      their packs are deferred to future work and community contributions.
    * **Installer language picker**: The setup wizard's *Options* step now includes a **Language**
      dropdown; the installer itself renders in the system language so the options are readable
      before you choose.
    * **Settings language picker**: `Settings → Appearance → Language` changes the interface
      immediately, with no restart.
    * **System locale detection**: New `get_system_locale` Tauri command (`sys-locale`) resolves
      the OS locale; regional variants map sensibly (`zh-*` → Simplified by default) and unknown
      locales fall back to English.
    * **Graceful fallback**: `t('key')` resolves against the active dictionary and falls back to
      English for any missing key, so a partial pack degrades gracefully instead of rendering raw
      keys.
    * **Validator**: `npm run i18n:check` reports missing keys, untranslated leftovers, and
      unknown keys for every pack. Fully offline and free.
    * **Contributor documentation**: New `wiki/Localization.md` covering both improving existing
      packs and adding new languages, including free cross-reference resources for verifying
      terminology. Linked from `Home`, `Settings & API Key`, `Installation & Maintenance`, the
      sidebar, and the wiki index.
* **Context-aware titlebar search**: The titlebar quick-search now acts on the page you are
  actually on instead of always jumping to a global search.
    * **Search page**: submitting updates the existing search query in place.
    * **Favorites / History**: submitting filters the local list by title; clearing the box
      restores it.
    * **Every other page**: falls back to a global search.
* **Titlebar layout**: The quick-search box is now centred, and the account chip sits adjacent to
  the window controls (left of them on Windows/Linux, right of them on macOS) instead of at the
  far edge.

### 🔧 Changed

* **Milestone tracking**: Added **M9 — Localization & language packs** to `ROADMAP.md` (with a
  three-phase plan) and split `TODO.md` into **M9.1** (shipped core) and **M9.2** (the 14
  remaining language packs, itemised but deliberately not started).
* **Version sync**: `package.json`, `src-tauri/Cargo.toml`, `src-tauri/tauri.conf.json`, and
  `src-tauri/Cargo.lock` bumped to `0.3.0`; the sidebar version readout was corrected from the
  stale `v0.2.0` to `v0.3.0`.

### 🐛 Fixed

* **Installer launched the app before you finished**: The setup wizard used to start NH Desktop
  while `perform_install` was still running, so the app appeared to jump the queue on top of the
  installer.
    * **Deferred launch**: Removed `launch_after` from `InstallOptions` and the launch side effect
      from `perform_install`.
    * **Explicit launch step**: New `installer_launch_app` command, invoked only when the user
      clicks **Finish** with "Launch after finish" checked. A launch failure is non-fatal — the
      wizard still closes cleanly.

## [v0.2.1](https://github.com/HELIX-Origin/nhentai-desktop/releases/tag/v0.2.1)

### ✨ Added

* **(none)**: No new user-facing features in this patch release.

### ✅ Changed

* **Agent ecosystem & project docs audit**: Performed a full correctness pass over `.agents/`, root docs, and the wiki so AI agents have a single, non-contradictory source of truth.
    * **Persistence model**: Corrected all references from browser `localStorage` to the SQLite-backed KV cache (`src/lib/cache.ts` → Tauri `db_*` commands → `nh-desktop.db`).
    * **Frontend structure**: Documented the flat `src/lib/` module layout (`api.ts`, `client.ts`, `types.ts`, `query.ts`, `image.ts`, `format.ts`, `cache.ts`) and `src/lib/design/` tokens/base styles.
    * **Component naming**: Codified PascalCase Svelte component files (`GalleryCard.svelte`) and component names.
    * **Backend module layout**: Added `db.rs`, `image_cache.rs`, `service.rs`, `installer.rs`, and `platform/` to the documented backend structure.
    * **CSP & image hosts**: Aligned the security rule with `tauri.conf.json` and the `AGENTS.md` decision log: `img-src 'self' data: blob: https://nhentai.net https://*.nhentai.net`.
    * **Command count**: Updated all command-count references to 43 Tauri commands.
    * **Installer artifacts**: Corrected Windows platform tag from `windows-x64` to `win-x64` and binary name from `nhentai` to `nh-desktop`.
    * **App data paths**: Updated privacy/troubleshooting paths to the Tauri bundle identifier `net.nh-desktop.client` (`%APPDATA%\net.nh-desktop.client`, `~/Library/Application Support/net.nh-desktop.client`, `~/.local/share/net.nh-desktop.client`).
    * **Requirements tracking**: Created `.agents/tracking/requirements/.gitkeep` so the requirements sub-agent's documented path resolves.
* **Milestone status sync**: Updated `TODO.md`, `ROADMAP.md`, and `wiki/Roadmap.md` to mark shipped M7/M8 work (downloads UI, light theme, configurable downloads folder) and set current focus to M1 Foundation closeout plus remaining M8 polish.
* **Version bump**: Synchronized `package.json`, `src-tauri/Cargo.toml`, and `src-tauri/tauri.conf.json` to `0.2.1`.

### 🐛 Fixed

* **Installer titlebar & window controls**: Simplified the installer window chrome so controls respond reliably and the bar remains draggable.
    * **Direct window APIs**: Finish/Close/Min/Max now call `getCurrentWindow().close()`, `.minimize()`, and `.toggleMaximize()` directly instead of nested try/catch fallback chains.
    * **Draggable header**: Kept drag-to-move via `startDragging()` on the custom header; removed stale `data-tauri-drag-region` and the installer-specific `app_quit` fallback.
    * **No duplicated chrome**: Confirmed the installer window uses `.decorations(false)` so only the custom HTML titlebar renders.

## [v0.2.0](https://github.com/HELIX-Origin/nhentai-desktop/releases/tag/v0.2.0)

### ✨ Added

* **Custom UI shell**: Built a native-feeling desktop shell inside the Tauri WebView.
    * **Sidebar navigation**: Quick access to New, Popular, Favorites, History, Blacklist, and Settings.
    * **Frameless titlebar**: macOS-style traffic lights with per-OS ordering, drag-to-move, and double-click maximize.
    * **System tray**: Show/minimize/quit menu, tray icon, and hide-to-tray on main-window close.
    * **Single-instance support**: `tauri-plugin-single-instance` focuses the running window instead of spawning duplicates.
* **Search & filters**: A filter drawer covering text query, language, category, per-type tag include/exclude (artist, character, parody, group, language, tag), page-count ranges, and six sort modes, all compiled into native nhentai query syntax.
* **Global blacklist**: Two-layer blacklist that never silently breaks the grid.
    * **Server-side excludes**: Blacklisted tags are appended as `-tag:` / `-type:` tokens to every search query.
    * **Client-side hide/blur**: Grid cards matching blacklist entries are hidden or blurred according to user preference.
    * **Master toggle**: Instantly enable or disable the entire blacklist.
* **Library views**: Favorites and History pages with runes-backed state and SQLite persistence through `src/lib/cache.ts`.
* **Reader**: Full gallery consumption experience.
    * **Detail page**: Cover, title, artists, parody, language, category, page count, score, related galleries, and quick-add to blacklist.
    * **Paged & strip modes**: Thumbnail strip, arrow-key navigation, click zones, fit modes (width/height/contain), preload, and fullscreen.
    * **Progress tracking**: Reading position is recorded in history.
* **Background service**: `src-tauri/src/service.rs` runs a throttled one-at-a-time worker queue surfaced through `service_*` commands and `service://job` / `service://refresh` events.
    * **Gallery downloads**: ZIP/CBZ/torrent downloads to disk with live progress in Settings.
    * **Image prefetch**: Preload image URLs into the disk cache.
    * **Cache maintenance**: Prune stale image cache entries and cached lists.
    * **Account sync**: Pull account favorites and blacklist from nhentai.net when an API key is configured.
    * **Popular auto-refresh**: Optional periodic refresh of the Popular list.
* **Unified installer/uninstaller**: Tauri-native setup wizard built into the same binary as the app.
    * **Install flow**: Welcome → Location → Options → Install → Finish with Desktop/Start Menu shortcuts and PATH registration.
    * **Maintenance mode**: Reinstall, repair shortcuts, and uninstall with optional user-data removal.
    * **Per-OS implementations**: `src-tauri/src/platform/{windows,macos,linux}.rs`.
* **Disk image cache**: `src-tauri/src/image_cache.rs` stores fetched images under `cache/images` with atomic tmp+rename writes; backs `proxy_image` cache-first and is pruned by maintenance.
* **Agent ecosystem**: Introduced `.agents/` with rules, primary/sub agent roles, skills, templates, and tracking directories to guide AI-assisted development.

### ✅ Changed

* **Rebrand to NH Desktop**: Product identity, executable name (`NH Desktop.exe`), bundle identifier (`net.nh-desktop.client`), crate/package names (`nh-desktop` / `nh_desktop_lib`), and cache prefix (`nh-desktop:`) unified.
* **CI packaging**: Added GitHub Actions workflow and `scripts/build-installer.mjs` to build and publish the unified installer binary.
* **Fixed dev port**: Vite dev server locked to `14440` (HMR `14441`); removed auto-incrementing `scripts/dev.mjs`.
* **CSP hardening**: `img-src` allow-listed for nhentai hosts plus `data:`/`blob:`, IPC-only `connect-src`, and a separate `devCsp` for Vite HMR.
* **App icon wired**: Favicon, sidebar brand, installer brand marks, tray icon, and installer/maintenance window icons.
* **Version metadata**: Synchronized `package.json`, `Cargo.toml`, and `tauri.conf.json` to `0.2.0`.

### 🐛 Fixed

* **(none)**: No explicit fixes in this milestone release.

## [v0.1.1](https://github.com/HELIX-Origin/nhentai-desktop/releases/tag/v0.1.1)

### ✨ Added

* **(none)**: No new features in this patch release.

### ✅ Changed

* **(none)**: No changes in this patch release.

### 🐛 Fixed

* **Installer download names**: Added platform and architecture tags to installer filenames (e.g. `NH Desktop-Setup-v0.1.1-win-x64.exe`, `-macos-arm64`, `-linux-x64`) so users can identify the correct download and CI assets do not collide across platforms.

## [v0.1.0](https://github.com/HELIX-Origin/nhentai-desktop/releases/tag/v0.1.0)

### ✨ Added

* **Initial project scaffold**: NH Desktop client foundation for nhentai.net.
    * **Tech stack**: Tauri 2 Rust backend + SvelteKit static SPA.
    * **API client**: Throttled reqwest client with typed serde models for nhentai.net API v2.

### ✅ Changed

* **(none)**: No changes in initial release.

### 🐛 Fixed

* **(none)**: No fixes in initial release.

---

| Version | Title | Description |
| :---: | :---: | :---: |
| 0.2.1 | v0.2.1 — installer titlebar polish + agent/docs audit | Patch release fixing installer titlebar/window controls and auditing agent ecosystem + project docs |
| 0.2.0 | v0.2.0 — custom UI shell + background service | Milestone release with full SPA shell, search/filter, blacklist, reader, background service, unified installer, and agent ecosystem |
| 0.1.1 | v0.1.1 — platform-tag installer names | Patch release adding platform/arch tags to installer download filenames |
| 0.1.0 | v0.1.0 — initial release | Initial NH Desktop client scaffold for nhentai.net |

---

**Last updated:** 2026-09-25
