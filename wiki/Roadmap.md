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
| M1 | Foundation | ✅ Shipped (core); closeout pending | Scaffold, Rust client + 43 commands, CSP, shell/tokens/stores/views, unified installer; remaining: clean-clone baseline verification |
| M2 | Browse & discover | ✅ Shipped (core) | Home (new), popular, grid/cards, pagination, lazy images with proxy fallback |
| M3 | Search & filters | ✅ Shipped (core) | Query builder, filter drawer (text/language/category/tags/pages/sort) |
| M4 | Global blacklist | ✅ Shipped (core) | Manage panel, server-side `-tag:` excludes, client-side hide/blur, master toggle |
| M5 | Library | ✅ Shipped (core) | Favorites, history, local persistence; export/import still backlog |
| M6 | Reader | ✅ Shipped (core) | Detail, paged thumbnails, strip mode, preload, fullscreen |
| M7 | Downloads & background services | 🚧 In progress (core + UI shipped) | Background-service core shipped: zip/cbz/torrent downloads to disk with progress, image prefetch, cache/image maintenance, account sync, Popular auto-refresh, job events; per-gallery Download button + Downloads page shipped. Backlog: queue resume/persist, cache consumers, storage-management polish |
| M8 | Polish / release | 🚧 In progress (0.2.0 shipped) | Release 0.2.0 shipped 2026-09-24. Backlog: accent picker, image quality, reader preload distance, end-to-end installer smoke test |

## 🚧 Current focus

M7's background-service core and downloads UI shipped (per-gallery Download button, ZIP/CBZ/
torrent formats, Downloads page, configurable downloads folder). M8's 0.2.0 release shipped
with light theme and the configurable downloads folder. Current focus: M1 closeout
(clean-clone baseline verification) and remaining M8 hardening (accent picker, image quality,
reader preload distance, end-to-end installer smoke test).

See [Backend (Rust)](Backend-Rust.md), [Frontend (SvelteKit)](Frontend-SvelteKit.md) and
[Installer Engine](Installer-Engine.md) for implementation details.

## 🔗 Related

- [Home](Home.md) · [FAQ](FAQ.md) · [Development & Contributing](Development-and-Contributing.md)