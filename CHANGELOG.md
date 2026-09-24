# Changelog

All notable changes to the **NH Desktop** client are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this
project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### ✨ Added

- ✨ **Native unified installer/uninstaller** (Tauri-native wizard — no NSIS/WiX):
  - Install, reinstall/update, repair shortcuts, and uninstall flows in one binary.
  - Per-OS support split behind `src-tauri/src/platform/` (`windows.rs`, `macos.rs`,
    `linux.rs`) — Windows: shortcuts, PATH, HKCU uninstall key; macOS: `.app` bundle,
    Desktop alias via AppleScript; Linux: desktop entry + `~/.local/bin` symlink.
  - `scripts/build-installer.mjs` assembles `dist/installer/NH Desktop-Setup-<version>.exe`
    (plus generic `NH Desktop-Setup.exe`) from a `tauri build --no-bundle` release.
  - Maintenance-mode entry point: sidebar ⚙ Maintenance opens the installer window.
- ✨ **Content Security Policy** (`tauri.conf.json`): image/`connect-src` allow-list for
  nhentai hosts + IPC, separate `devCsp` for Vite HMR.
- ✨ Project documentation: `README.md`, `PRIVACY.md`, `TOS.md`, `SECURITY.md`,
  `CHANGELOG.md`, and a full `wiki/` (GitHub-wiki-ready: `_Sidebar.md` + `_Footer.md`).
- ✨ `.agents/` ecosystem (agent roles, rules, skills, templates) for development workflow.

### ✅ Changed

- ✅ Rebranded the user-facing product to **NH Desktop** (`NH Desktop.exe`, `Programs\NH Desktop`
  install dir, "NH Desktop" branding); backend identifiers kept (`nhentai_lib`,
  `net.nhentai.client`).
- ✅ SQLite database standardized to `nhentai.db`; cache prefix `nhentai:`.
- ✅ API user-agent reports `nhentai/<version>`.

### 🐛 Fixed

- (none yet)

## [0.1.0] - 2026-09-24

Initial release scaffold.

### ✨ Added

- ✨ Tauri 2 + SvelteKit static SPA shell; Svelte 5 runes, strict TypeScript, plain CSS design
  tokens (dark theme, violet `#7c5cff` accent).
- ✨ Rust backend: `nhentai.rs` API client (reqwest, throttled), `commands.rs` (26+ Tauri
  commands), `db.rs` (local SQLite: key/value cache + API key), `error.rs`.
- ✨ Core browse: New (home), Popular, tag pages, gallery grid/cards with lazy images + image
  proxy fallback.
- ✨ Search & filters: query builder (text, language, category, per-type tag include/exclude,
  page ranges, 6 sort modes) compiled to native nhentai syntax.
- ✨ Global blacklist: server-side `-tag:` excludes + client-side hide/blur, master toggle,
  quick-add from any gallery.
- ✨ Library: favorites + history (localStorage); account favorites/blacklist sync via optional
  nhentai API key.
- ✨ Reader: gallery detail, paged thumbnails, strip/paged modes, preload, fullscreen.
- ✨ Installer engine foundations: `installer.rs` status/disk-space/install/uninstall commands.

[Unreleased]: https://github.com/HELIX-Origin/nhentai-desktop/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/HELIX-Origin/nhentai-desktop/releases/tag/v0.1.0