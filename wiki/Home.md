# What is NH Desktop?

**NH Desktop** is a lightweight, modern, cross-platform **desktop client** for the adult manga
archive nhentai.net. It is not a website clone: it ships a custom UI and a materially better
**search/filter** and **global blacklist** experience than the site can offer, backed by the
site's own public API and imported metadata (galleries, tags, languages, categories, artists,
characters, parodies).

- 👋 **Browser & discover** — New (home), Popular, tag pages, everything lazy-loaded.
- **Search that actually works** — a filter drawer (text, language, category, per-type tag
  include/exclude, page ranges, six sorts) compiled into native nhentai query syntax, so you
  can cut a giant haystack down to exactly what you want.
- **A blacklist you can trust** — global, persistent, applied server-side (`-tag:` excludes)
  *and* client-side (hide/blur), with a master toggle. It never silently breaks the grid.
- **Private by default** — no accounts, no servers, no telemetry. Favorites, history,
  blacklist, and settings are stored only on your device. An optional nhentai.net API key
  unlocks account favorites/blacklist sync.
- **A real installer** — a Tauri-native unified setup/uninstall wizard (no NSIS/WiX MSI),
  single binary is both the app and the installer.

> **18+ only.** This software is for adults. You confirm you are of legal age to view adult
> content. See the [Terms of Service](https://github.com/HELIX-Origin/nhentai-desktop/blob/main/TOS.md).

## 📦 Platform support

| Platform | Status |
| --- | --- |
| Windows 10+ | ✅ Supported |
| macOS 10.13+ | ✅ Supported |
| Linux (x86_64) | ✅ Supported |
| Mobile | ⬜ Not supported (by design) |

## 🧭 Getting around

- **[Getting Started](Getting-Started.md)** — first run, API key, what to expect.
- **[Installation & Maintenance](Installation-and-Maintenance.md)** — installing, updating,
  uninstalling, shortcuts/PATH.
- **[Search & Filters](Search-and-Filters.md)** — the query engine, full reference.
- **[Blacklist](Blacklist.md)** — the blacklist, in depth.
- **[Readers & Galleries](Reader-and-Galleries.md)** — detail pages and the reader.
- **[Architecture](Architecture.md)** — how the pieces fit together.

## 💡 Quick facts

- Stack: **Tauri 2 (Rust `reqwest`)** backend + **SvelteKit static SPA** (Svelte 5 runes,
  strict TypeScript) + **plain CSS tokens** (no UI framework).
- Persistence: browser `localStorage` + a local SQLite database (`nh-desktop.db`).
- Rebranded/verified: this is the **NH Desktop** product — `NH Desktop.exe`, window title
  "NH Desktop", identifier `net.nh-desktop.client`. The old scaffold names are gone.

## 🔗 Links

- [Repository](https://github.com/HELIX-Origin/nhentai-desktop)
- [Releases](https://github.com/HELIX-Origin/nhentai-desktop/releases)
- [Changelog](https://github.com/HELIX-Origin/nhentai-desktop/blob/main/CHANGELOG.md)
- [Privacy](https://github.com/HELIX-Origin/nhentai-desktop/blob/main/PRIVACY.md)
- [Terms](https://github.com/HELIX-Origin/nhentai-desktop/blob/main/TOS.md)
- [Security](https://github.com/HELIX-Origin/nhentai-desktop/blob/main/SECURITY.md)
- [nhentai.net](https://nhentai.net) — the site this client talks to.