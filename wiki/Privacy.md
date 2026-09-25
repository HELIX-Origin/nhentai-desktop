# Privacy

What NH Desktop stores, what it sends, and what it never does.

> Full policy: [PRIVACY.md](https://github.com/HELIX-Origin/nhentai-desktop/blob/main/PRIVACY.md)
> (repo root). This wiki page is the short version.

## 💾 What stays on your device

| Data | Storage |
| --- | --- |
| Favorites, history, blacklist, settings | `localStorage` |
| API key + cache mirror (`nh-desktop:…` entries) | `nh-desktop.db` (SQLite in app data dir) |

All of it is local. There is no app server; you are never "logged in" to anything except
nhentai.net itself (optional, via API key).

## 🌐 What goes over the network

- Requests to **nhentai.net** (API) and its image **CDNs** (`t.nhentai.net`,
  `i.nhentai.net`) — only what you trigger by browsing, searching, or loading an image.
- If you supply an API key, authenticated requests go to nhentai.net so it can return your
  favorites/blacklist/settings. The key is used by nhentai.net's service, not by us.

## 🚫 What it does **not** do

- No analytics, no tracking, no crash reporters, no third-party SDKs.
- No data collection or phone-home endpoints.
- Does not read your files, scan your drives, or upload gallery content.
- Does not store your data on any server we run.

## 🗑️ Deleting your data

- Clear the cache from **Settings → Cache**.
- Clear favorites/history/blacklist from the corresponding views (or clear site data).
- Full removal: uninstall with **Remove user data** checked — wipes localStorage and
  `nh-desktop.db`.

## 🧩 Third-party

The app is built on Tauri (Rust), SvelteKit, and reuses only standard dependencies. See
`package.json` / `src-tauri/Cargo.toml` for the exact list.

## 📮 Contact

Questions/requests → GitHub issues on
[HELIX-Origin/nhentai-desktop](https://github.com/HELIX-Origin/nhentai-desktop) or the
maintainer contact in PRIVACY.md.

## 🔗 Related

- [Security](Security.md) · [Settings & API Key](Settings-and-API-Key.md) ·
  [Architecture](Architecture.md) · [Getting Started](Getting-Started.md)