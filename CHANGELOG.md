# Changelog

All notable changes to the **NH Desktop** client are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this
project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0] - 2026-09-24

### ✨ Added

- ✨ **Light theme**: dimmed, easy-on-the-eyes light palette applied via a three-way
  appearance toggle (Light / Dark / System) in Settings; System mode follows the OS
  `prefers-color-scheme` live.
- ✨ **Gallery downloads**: per-gallery Download button (ZIP / CBZ / torrent) feeding the
  background service queue, plus a new **Downloads** page listing every download job with
  live progress, retry, and clear controls.
- ✨ **Configurable downloads folder**: defaults to
  `Documents/NH Desktop/downloads`, overridable in Settings (Save / Use default / Open
  folder).
- ✨ **Download folder opener**: `open_downloads_folder` command reveals the folder in the
  system file manager.

### ✅ Changed

- ✅ Reader page backgrounds now use theme tokens (`--reader-bg`) instead of hard-coded
  `#060609`; edge-nav buttons adapt to theme text color.
- ✅ Scrollbars use tokenized thumb colors for correct rendering in both themes.
- ✅ Version metadata bumped to `0.2.0`.

### 🐛 Fixed

- 🐛 Install/uninstall could fail when the app was running — installs and uninstalls now
  automatically terminate a running instance of **NH Desktop** first (OS-specific:
  `Stop-Process` on Windows, `pkill` on macOS/Linux, with the installer process itself
  excluded).

## [0.1.1] - 2026-09-24

### ✅ Changed

- ✅ Installer binaries are now named with their target platform and architecture
  (`NH Desktop-Setup-<version>-win-x64.exe`, `-macos-arm64`, `-linux-x64`, …) so users can
  tell which platform a download is for.

## [0.1.0] - 2026-09-24

Initial release.

### ✨ Added

- ✨ **Tauri 2 + SvelteKit static SPA shell**; Svelte 5 runes, strict TypeScript, plain CSS
  design tokens (dark theme, violet `#7c5cff` accent).
- ✨ **Rust backend**: `nh_desktop.rs` API client (reqwest, throttled), `commands.rs` (39
  Tauri commands), `db.rs` (local SQLite: key/value cache + API key), `error.rs`.
- ✨ **Custom title bar**: macOS-style traffic lights (position/order per OS), window
  dragging/double-click maximize, quick search + account chip, minimize/maximize/close.
- ✨ **System tray**: show/minimize/quit menu, tray icon, hide-to-tray on window close.
- ✨ **Single-instance support** (`tauri-plugin-single-instance`): a second launch focuses
  the running window.
- ✨ **Core browse**: New (home), Popular, tag pages; uniform gallery grid/cards with fixed
  2:3 covers, lazy images + image-proxy fallback; Popular paginated client-side (28/page).
- ✨ **Search & filters**: query builder (text, language, category, per-type tag
  include/exclude, page ranges, 6 sort modes) compiled to native nhentai syntax.
- ✨ **Global blacklist**: server-side `-tag:` excludes + client-side hide/blur, master
  toggle, quick-add from any gallery, tag-picker management panel with type tabs.
- ✨ **Library**: favorites + history (localStorage); account favorites/blacklist sync via
  optional nhentai API key.
- ✨ **Reader**: gallery detail, paged thumbnails, strip/paged modes, preload, fullscreen,
  fit-mode fixes.
- ✨ **Background service** (`src-tauri/src/service.rs`): throttled worker queue for gallery
  downloads (zip → disk, with progress), image prefetch, cache/image maintenance, account
  sync, and Popular auto-refresh. New `service_*` commands + live `service://job` /
  `service://refresh` events; Settings gets a "Background services" panel with a recent-jobs
  list.
- ✨ **Disk image cache** (`src-tauri/src/image_cache.rs`, `cache/images`) backing
  `proxy_image` (cache-first, atomic tmp+rename writes, pruned by service maintenance).
- ✨ **Native unified installer/uninstaller** (Tauri-native wizard — no NSIS/WiX): install,
  reinstall/update, repair shortcuts, and uninstall flows in one binary; platform split
  behind `src-tauri/src/platform/`; `scripts/build-installer.mjs` assembles
  `dist/installer/NH Desktop-Setup-<version>.<platform>-<arch>(.exe)`.
- ✨ **App icon wired everywhere**: `static/favicon.png`, sidebar + installer brand marks,
  tray icon, and installer/maintenance window icons.
- ✨ **Fixed dev port 14440** (Vite + `tauri.conf.json`, HMR 14441); removed
  `scripts/dev.mjs` auto-incrementing.
- ✨ **Content Security Policy** (`tauri.conf.json`): image/`connect-src` allow-list for
  nhentai hosts + IPC, separate `devCsp` for Vite HMR.
- ✨ **Project documentation**: `README.md`, `PRIVACY.md`, `TOS.md`, `SECURITY.md`,
  `CHANGELOG.md`, and a full `wiki/` (GitHub-wiki-ready).
- ✨ **`.agents/` ecosystem** (agent roles, rules, skills, templates) incl. issue protocol,
  release standards, commit messages, YAML GitHub issue templates, and DCP context
  management.

### ✅ Changed

- ✅ Image URLs derived correctly from the API's relative path fragments (`image.ts`
  `pagePath`/`thumbPath`/`avatarUrl`, `joinUrl`); image hosts accepted as any
  `*.nhentai.net`; CSP `img-src` broadened to `https://nhentai.net https://*.nhentai.net`.
- ✅ Topbar sign-in chip reflects true login status (`keyStatus.configured`).
- ✅ Rebranded the user-facing product to **NH Desktop** (`NH Desktop.exe`,
  `Programs\NH Desktop`); backend identifiers kept (`nh_desktop_lib`,
  `net.nh-desktop.client`), SQLite db `nh-desktop.db`, cache prefix `nh-desktop:`.
- ✅ API user-agent reports `nhentai/<version>`.
- ✅ Gallery cards use a uniform fixed 2:3 cover ratio with `object-fit: cover`; all
  paginated views (New / tag pages / Search / Popular) serve 28 galleries per page.

### 🐛 Fixed

- 🐛 Images failed to load: API v2 returns slash-less relative paths; URL join is now
  correct.
- 🐛 Sign-in chip showed "Sign in" even when an API key was registered but
  `getCurrentUser()` failed at startup.
- 🐛 Dev-mode close threw `Failed to unregister class Chrome_WidgetWin_0 (error 1412)`;
  quit paths now destroy the main window before `app.exit(0)`.

[Unreleased]: https://github.com/HELIX-Origin/nhentai-desktop/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/HELIX-Origin/nhentai-desktop/releases/tag/v0.2.0
[0.1.1]: https://github.com/HELIX-Origin/nhentai-desktop/releases/tag/v0.1.1
[0.1.0]: https://github.com/HELIX-Origin/nhentai-desktop/releases/tag/v0.1.0