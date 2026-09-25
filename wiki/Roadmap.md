# Roadmap

Where the project is headed. Source of truth: the repo-root
[ROADMAP.md](https://github.com/HELIX-Origin/nhentai-desktop/blob/main/ROADMAP.md) and
`TODO.md`.

## 🏗️ Guiding principles

1. **Filtering first.** Every view shares the same filter engine.
2. **Blacklist without compromise.** Global, persistent, instant, toggle-able — never breaks
   the grid or reader.
3. **Fast and light.** SPA on Tauri; images lazy; requests throttled.
4. **Local & private.** Favorites, history, blacklist, settings stay on-device.

## 🚫 Non-goals

- **Mobile support (Android/iOS).** Desktop-only, deliberately. Android already has a good
  third-party client ([NClientV3](https://github.com/maxwai/NClientV3)); iOS rejects NSFW apps
  and its developer license is prohibitively expensive. Use NClientV3 on Android or the site in
  a browser.

## 🗺️ Milestones

| # | Milestone | Status | Scope |
| --- | --- | --- | --- |
| M1 | Foundation | ✅ Shipped (core) | Scaffold, Rust client + 37 commands, CSP, shell/tokens/stores/views, unified installer |
| M2 | Browse & discover | ✅ Shipped (core) | Home (new), popular, grid/cards, pagination, lazy images with proxy fallback |
| M3 | Search & filters | ✅ Shipped (core) | Query builder, filter drawer (text/language/category/tags/pages/sort) |
| M4 | Global blacklist | ✅ Shipped (core) | Manage panel, server-side `-tag:` excludes, client-side hide/blur, master toggle |
| M5 | Library | ✅ Shipped (core) | Favorites, history, local persistence; export/import still backlog |
| M6 | Reader | ✅ Shipped (core) | Detail, paged thumbnails, strip mode, preload, fullscreen |
| M7 | Downloads & background services | 🚧 In progress | Background-service core shipped: zip downloads to disk with progress, image prefetch, cache/image maintenance, account sync, Popular auto-refresh, job events. Backlog: per-gallery Download button, CBZ/other formats, storage management, cache consumers |
| M8 | Polish / release | 🚧 In progress | Installer (built), light theme, accent picker, image quality, reader preload distance, end-to-end installer smoke test |

## 🚧 Current focus

M7's background-service core shipped (downloads, prefetch, maintenance, sync, auto-refresh) and
the reader/image pipeline was fixed. Now closing out M1 polish items (clean-clone baseline,
initial tagged release) and M8 hardening (light theme, themed accents, installer end-to-end
verification on Windows/macOS/Linux), plus the M7 backlog: per-gallery download buttons, CBZ
and other formats, downloads-folder/storage management.

See [Backend (Rust)](Backend-Rust.md), [Frontend (SvelteKit)](Frontend-SvelteKit.md) and
[Installer Engine](Installer-Engine.md) for implementation details.

## 🔗 Related

- [Home](Home.md) · [FAQ](FAQ.md) · [Development & Contributing](Development-and-Contributing.md)