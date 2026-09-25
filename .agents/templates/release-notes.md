# Release Notes: NH Desktop {{ version }}

**Release date:** {{ YYYY-MM-DD }}

## ✨ Highlights

{{ 2–3 sentence summary for users. }}

## 🚀 Key Improvements & Features

- {{ Feature / change }}
- {{ Feature / change }}

## 🛡️ Security & Governance

- {{ security fix / dependency bump / CSP note }}

## Fixed

- {{ Bug fix }}

## Removed / deprecated

- {{ Removed feature }}

## 📦 Install & Upgrading

Download from the Assets section below:

- Windows: `NH Desktop-Setup-{{ version }}.exe`
- macOS: `NH Desktop_{{ version }}_x64.dmg` / `_aarch64.dmg` (Apple Silicon)
- Linux: `nh-desktop_{{ version }}_amd64.deb` / `.AppImage` (or distro package)

Upgrade in place: run the new installer — it repairs/reinstalls over the
existing installation (Tauri-native unified installer/uninstaller).

## Verification

- `cargo check` + `cargo test` (src-tauri) — green
- `npm run check` (svelte-check) + `npm run build` — green
- `npm run tauri build` — green; installer smoke — passed

## 📄 Changes & Commits

See the commit range and [CHANGELOG.md](../CHANGELOG.md).