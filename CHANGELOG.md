# 📜 NH Desktop Changelog

Historical record of every change to the NH Desktop client. Newer releases are added at the
top; the current development state lives under `Unreleased`.

## Unreleased

(No unreleased changes.)

## [v0.2.1](https://github.com/HELIX-Origin/nhentai-desktop/releases/tag/v0.2.1)

### ✨ Added

* **(none)**: No new features in this patch release.

### ✅ Changed

* **Agent ecosystem audit**: Corrected agent rules and project docs so AI agents have a single, non-contradictory source of truth. Fixes include the SQLite-backed KV cache persistence model, flat `src/lib/` frontend layout, PascalCase Svelte component files, CSP/image-host allowlist alignment, 43-command count, `win-x64` installer naming, and bundle-identifier data paths.
* **Milestone status sync**: Synchronized `TODO.md`, `ROADMAP.md`, and `wiki/Roadmap.md` to reflect shipped M7/M8 work and current focus.
* **Version bump**: Bumped `package.json`, `Cargo.toml`, and `tauri.conf.json` to `0.2.1`.

### 🐛 Fixed

* **Installer titlebar**: Simplified the installer titlebar and window controls so Finish/Close/Min/Max use direct `getCurrentWindow()` calls, the header stays draggable via `startDragging()`, and the custom titlebar renders without duplicated native chrome.

## [v0.2.0](https://github.com/HELIX-Origin/nhentai-desktop/releases/tag/v0.2.0)

### ✨ Added

* **Custom UI shell**: Sidebar navigation, frameless titlebar with macOS-style traffic lights, system tray integration, and single-instance support.
* **Search & filters**: Query builder compiling structured controls into native nhentai syntax.
* **Global blacklist**: Server-side `-tag:` excludes plus client-side hide/blur, with a master toggle.
* **Library**: Favorites and History views with SQLite-backed persistence.
* **Reader**: Gallery detail, paged thumbnails, strip mode, preload, fullscreen, and fit modes.
* **Background service**: Throttled worker for gallery downloads, image prefetch, cache maintenance, account sync, and Popular auto-refresh with live job events.
* **Unified installer**: Single-binary install/uninstall/maintenance wizard with shortcuts and PATH registration.
* **Disk image cache**: `cache/images` backing `proxy_image`, pruned by background maintenance.
* **Agent ecosystem**: `.agents/` rules, roles, skills, and templates.

### ✅ Changed

* **Rebrand to NH Desktop**: Unified product identity, bundle ID `net.nh-desktop.client`, and cache prefix `nh-desktop:`.
* **CI & docs**: Added packaging workflow and polished project documentation.
* **Version metadata**: Synchronized all manifests to `0.2.0`.

### 🐛 Fixed

* **(none)**: No explicit fixes in this milestone release.

## [v0.1.1](https://github.com/HELIX-Origin/nhentai-desktop/releases/tag/v0.1.1)

### ✨ Added

* **(none)**: No new features in this patch release.

### ✅ Changed

* **(none)**: No changes in this patch release.

### 🐛 Fixed

* **Installer filenames**: Added platform and architecture tags to download filenames so users can identify the correct download.

## [v0.1.0](https://github.com/HELIX-Origin/nhentai-desktop/releases/tag/v0.1.0)

### ✨ Added

* **Initial scaffold**: NH Desktop client foundation for nhentai.net.
* **Tech stack**: Tauri 2 + SvelteKit static SPA with a throttled nhentai.net API client.

### ✅ Changed

* **(none)**: No changes in initial release.

### 🐛 Fixed

* **(none)**: No fixes in initial release.

---

| Version | Title | Description | Status |
| :---: | :---: | :---: | :---: |
| 0.2.1 | v0.2.1 — installer titlebar polish + agent/docs audit | Patch release fixing installer titlebar/window controls and auditing agent ecosystem + project docs | Released |
| 0.2.0 | v0.2.0 — custom UI shell + background service | Milestone release with full SPA shell, search/filter, blacklist, reader, background service, unified installer, and agent ecosystem | Released |
| 0.1.1 | v0.1.1 — platform-tag installer names | Patch release adding platform/arch tags to installer download filenames | Released |
| 0.1.0 | v0.1.0 — initial release | Initial NH Desktop client scaffold for nhentai.net | Released |

---

**Last updated:** 2026-09-25
