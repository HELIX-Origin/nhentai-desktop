# NH Desktop v0.3.0

**Release date:** 2026-09-25

## ✨ Highlights

NH Desktop now speaks your language. The interface detects your operating system locale on first
launch and ships complete translations for **English, Japanese, and Chinese (Simplified +
Traditional)** — switchable from the installer's Options step or from Settings, with the change
applying instantly. This release also fixes the installer launching the app before you clicked
Finish, and makes the titlebar search box act on the page you are currently viewing.

## 🚀 Key Improvements & Features

- **Interface localization (M9)** — full i18n infrastructure with system-locale detection, plus
  complete language packs for Japanese (`ja`), Chinese Simplified (`zh-Hans`), and Chinese
  Traditional (`zh-Hant`) — 78 keys each. Fourteen further nhentai content languages are already
  registered and selectable, degrading gracefully to English key-by-key until their packs land.
- **Language picker in the installer** — the setup wizard's Options step now includes a Language
  dropdown. The installer itself renders in your system language first, so you can read the
  options *before* you choose one.
- **Language picker in Settings** — `Settings → Appearance → Language` switches the interface
  immediately, no restart required. Your choice is shared with the installer, so the two never
  disagree.
- **Context-aware titlebar search** — the quick-search box now operates on the active page. On
  Search it updates the query in place; on Favorites and History it filters the local list by
  title; everywhere else it falls back to a global search.
- **Refined titlebar layout** — the search box is centred, and the account chip sits adjacent to
  the window controls (left on Windows/Linux, right on macOS) rather than pinned to the far edge.
- **Free, offline translation tooling** — `npm run i18n:check` validates every language pack for
  missing keys, untranslated leftovers, and typos, with no network access and no cost. Shipped
  strings were hand-written, not machine-translated.

## ✅ Changed

- `ROADMAP.md` gains **M9 — Localization & language packs** with a three-phase plan; `TODO.md` is
  split into **M9.1** (shipped core) and **M9.2** (the 14 remaining language packs, itemised with
  per-language notes but deliberately not started).
- New `wiki/Localization.md` contributor guide documents both improving an existing pack and
  adding a brand-new language, including free terminology cross-reference resources. Linked from
  Home, Settings, Installation, the wiki sidebar, and the wiki index.
- Version synchronized to `0.3.0` across `package.json`, `src-tauri/Cargo.toml`,
  `src-tauri/tauri.conf.json`, and `src-tauri/Cargo.lock`.
- Corrected the sidebar version readout, which still displayed the stale `v0.2.0`.

## 🐛 Fixed

- **The installer no longer launches the app before you finish.** The setup wizard used to start
  NH Desktop while installation was still running, so the app would appear on top of the
  installer mid-install. Launching is now an explicit step: the new `installer_launch_app` command
  fires only when you click **Finish** with "Launch after finish" checked, and a launch failure is
  non-fatal — the wizard still closes cleanly.

## 📦 Install & Upgrading

Download the installer for your platform from the Assets section below:

- Windows: `NH Desktop-Setup-0.3.0-win-x64.exe`
- macOS: `NH Desktop-Setup-0.3.0-macos-arm64` (or `macos-x64` if available)
- Linux: `NH Desktop-Setup-0.3.0-linux-x64`

Upgrade in place: run the new installer over your existing installation. The unified installer will
repair/replace files and update shortcuts. Your favorites, history, blacklist, and settings are
preserved — the new language preference is stored in the same local database and carries forward.

## Verification

- `cargo check` + `cargo test` (src-tauri) — passed (9/9)
- `npm run check` (svelte-check) — 0 errors, 0 warnings
- `npm run i18n:check` — 3/3 language packs OK
- `npm run build` — adapter-static site generated successfully
- `npm run build:installer` — produced `NH Desktop-Setup-0.3.0-win-x64.exe`

## 📄 Changes & Commits

- `feat(i18n): add localization, language packs, and system-locale default (M9)`
- `fix(installer): defer launch until Finish click; feat(ui): context-aware titlebar search`
- `docs: audit and correct agent ecosystem + project docs, finish installer titlebar simplification`
- `docs: add CHANGELOG.md following project template`

Full commit history: `git log --oneline v0.2.1..v0.3.0`
