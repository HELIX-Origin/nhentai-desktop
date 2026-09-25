# Security

Security posture, reporting, and hardening notes for NH Desktop.

> The full project policy lives at
> [SECURITY.md](https://github.com/HELIX-Origin/nhentai-desktop/blob/main/SECURITY.md) in the
> repo root — this page is the wiki summary.

## 🏗️ Design principles

- **Local-first.** Favorites, history, blacklist, settings and the cache live on-device
  (localStorage / `nh-desktop.db`). No app server, no telemetry.
- **No remote code paths.** The WebView never executes remote scripts; CSP restricts
  connections to nhentai.net + its CDNs.
- **One network path.** All API traffic goes through `NhDesktopClient` (reqwest), throttled
  (`THROTTLE`) to respect nhentai.net and avoid hammering the service.
- **API key stays local.** The nhentai.net API key is stored in `nh-desktop.db` only and is used
  solely to authenticate requests you trigger; it is not uploaded anywhere else. The UI shows
  only a 4-character prefix.

## 🛡️ Hardenings in place

| Area | Where |
| --- | --- |
| CSP (`default-src 'self'` + image hosts + IPC only) | `src-tauri/tauri.conf.json` |
| Dev CSP (Vite HMR) isolated from production | `security.devCsp` in the same config |
| Result-based errors (no panics across the bridge) | `src-tauri/src/error.rs` |
| Request throttle | `nh_desktop.rs` |
| Single unsigned binary signature | installer engine (see below) |

## 🚨 Supported versions / reporting

- Supported: current 0.1.x.
- Report vulnerabilities via **GitHub private vulnerability reporting** on the repository, or
  to the maintainer directly (contacts in SECURITY.md).
- Scope: the app code, installer, and build scripts. Out of scope: nhentai.net itself and
  its CDNs.

## ⬜ Hardening wishlist (future)

- Code signing (Windows) / notarization (macOS).
- Optional verify-on-first-run of the downloaded installer.
- Per-host network policy beyond CSP (e.g. platform-level connect-restr).

## 🔗 Related

- [Privacy](Privacy.md) · [Installer Engine](Installer-Engine.md) ·
  [Architecture](Architecture.md) · [Settings & API Key](Settings-and-API-Key.md)