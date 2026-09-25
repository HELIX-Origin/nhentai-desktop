# 📜 NH Desktop Changelog

Historical record of every change to the NH Desktop client. Newer releases are added at the
top; the current development state lives under `Unreleased`.

## Unreleased

(No unreleased changes.)

## [v0.2.1](https://github.com/HELIX-Origin/nhentai-desktop/releases/tag/v0.2.1)

### ✨ Added

* (No new features in this patch release.)

### ✅ Changed

* Audited and corrected the agent ecosystem and project documentation so AI agents have a single, non-contradictory source of truth. Fixes include the SQLite-backed KV cache persistence model, flat `src/lib/` frontend layout, PascalCase Svelte component files, CSP/image-host allowlist alignment, 43-command count, `win-x64` installer naming, and bundle-identifier data paths.
* Synchronized milestone status across `TODO.md`, `ROADMAP.md`, and `wiki/Roadmap.md` to reflect shipped M7/M8 work and current focus.
* Bumped version metadata to `0.2.1` across `package.json`, `Cargo.toml`, and `tauri.conf.json`.

### 🐛 Fixed

* Simplified the installer titlebar and window controls: Finish/Close/Min/Max now use direct `getCurrentWindow()` calls, the header stays draggable via `startDragging()`, and the custom titlebar renders without duplicated native chrome.

## [v0.2.0](https://github.com/HELIX-Origin/nhentai-desktop/releases/tag/v0.2.0)

### ✨ Added

* Shipped the full custom UI shell including sidebar navigation, frameless titlebar with macOS-style traffic lights, system tray integration, and single-instance support.
* Implemented search and filters with a query builder compiling structured controls into native nhentai syntax.
* Added a global blacklist with server-side `-tag:` excludes and client-side hide/blur, plus a master toggle.
* Added Favorites and History library views with SQLite-backed persistence.
* Added the gallery reader with paged thumbnails, strip mode, preload, fullscreen, and fit modes.
* Added a background service for gallery downloads, image prefetch, cache maintenance, account sync, and Popular auto-refresh with live job events.
* Added the unified installer/uninstaller wizard in a single binary with shortcuts and PATH registration.
* Added a disk image cache backing `proxy_image` and pruned by background maintenance.
* Introduced the `.agents/` agent ecosystem with rules, roles, skills, and templates.

### ✅ Changed

* Rebranded the product to NH Desktop with unified identifiers, bundle ID `net.nh-desktop.client`, and cache prefix `nh-desktop:`.
* Added CI packaging workflow and polished project documentation.
* Synchronized version metadata to `0.2.0` across all manifest files.

### 🐛 Fixed

* (No explicit fixes in this milestone release.)

## [v0.1.1](https://github.com/HELIX-Origin/nhentai-desktop/releases/tag/v0.1.1)

### ✨ Added

* (No new features in this patch release.)

### ✅ Changed

* (No changes in this patch release.)

### 🐛 Fixed

* Added platform and architecture tags to installer download filenames so users can identify the correct download.

## [v0.1.0](https://github.com/HELIX-Origin/nhentai-desktop/releases/tag/v0.1.0)

### ✨ Added

* Initial NH Desktop client scaffold for nhentai.net.
* Tauri 2 + SvelteKit static SPA foundation with a throttled nhentai.net API client.

### ✅ Changed

* (No changes in initial release.)

### 🐛 Fixed

* (No fixes in initial release.)

---

| Version | Title | Description | Status |
| :---: | :---: | :---: | :---: |
| 0.2.1 | v0.2.1 — installer titlebar polish + agent/docs audit | Patch release fixing installer titlebar/window controls and auditing agent ecosystem + project docs | Released |
| 0.2.0 | v0.2.0 — custom UI shell + background service | Milestone release with full SPA shell, search/filter, blacklist, reader, background service, unified installer, and agent ecosystem | Released |
| 0.1.1 | v0.1.1 — platform-tag installer names | Patch release adding platform/arch tags to installer download filenames | Released |
| 0.1.0 | v0.1.0 — initial release | Initial NH Desktop client scaffold for nhentai.net | Released |

---

**Last updated:** 2026-09-25
