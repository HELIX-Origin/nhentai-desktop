# ROADMAP.md

> Product direction and milestones for **nhentai**. This is the "what / why" doc —
> pair it with `TODO.md` (executable tasks) and `BUGS.md` (known issues).

## 🔭 Vision

A fast, desktop-native client for nhentai.net that makes the site's weakest areas its
strengths: **search that actually works** and a **blacklist you can trust**. The app is
deliberately lightweight (Tauri + local storage, no server, no accounts) and respects the
public API — we fetch data, we don't scrape aggressively.

## 🧭 Positioning principles

1. 🔍 **Filtering first.** Every view shares the same powerful filter engine.
2. **Blacklist without compromise.** Global, persistent, instant, and toggle-able; it must
   never silently break the grid or the reader.
3. **Fast and light.** SPA on Tauri; images load lazily; requests throttled.
4. **Local & private.** Favorites, history, blacklist, and settings live only on-device.

## 🚫 Non-goals

- **Mobile support (Android/iOS).** Desktop-only, deliberately. Android already has a good
  third-party client ([NClientV3](https://github.com/maxwai/NClientV3)); iOS rejects NSFW apps
  and its developer license is prohibitively expensive. See the decision log in `AGENTS.md`.

## 🗺️ Milestones

| # | Milestone | Status | Scope |
| --- | --- | --- | --- |
| M1 | Foundation | ✅ Shipped (core) | Scaffold, green toolchain, Rust client + 37 commands, CSP, app shell, tokens, stores, all views; remaining: clean-clone baseline + initial commit |
| M2 | Browse & discover | ✅ Shipped (core) | Home (new releases), popular, gallery grid/cards, pagination, lazy images with proxy fallback |
| M3 | Search & filters | ✅ Shipped (core) | Query builder, filter drawer (text, language, category, per-type tag include/exclude, page ranges, sort), results + count |
| M4 | Global blacklist | ✅ Shipped (core) | Manage panel, server-side `-tag:` excludes, client-side hide/blur, master toggle |
| M5 | Library | ✅ Shipped (core) | Favorites, history, local persistence; import/export JSON still backlog |
| M6 | Reader | ✅ Shipped (core) | Gallery detail, paged thumbnails, strip mode, preload, fullscreen |
| M7 | Downloads & background services | 🚧 In progress | Background-service core shipped (zip downloads to disk with progress, image prefetch, cache/image maintenance, account sync, Popular auto-refresh with job events); still backlog: per-gallery Download button, CBZ/other formats, storage management, cache consumers |
| M8 | Polish / release | 🚧 In progress | Release 0.1.0 shipped 2026-09-24 (installer + verified gates green); still backlog: light theme, accent picker, image quality, reader preload distance, installer E2E smoke via CI |

## 🎯 Current focus

**M7 Downloads & background services** core pipeline is shipped (see `TODO.md`); **M1
Foundation** closeout and **M8 hardening** remain:

- ⬜ `npm install` + `npm run check` + `cargo check`/`test` all green on a clean clone
- ✅ Rust client returns real nhentai data (search, newest, detail) through Tauri commands
- ✅ SPA chrome navigates between Home / Search / Favorites / History / Blacklist / Settings
- ✅ Design tokens wired (dark theme live, light reserved); responsive grid
- ✅ CSP hardened (image hosts allowlisted, IPC only, devCsp for Vite HMR)
- ✅ Background service: enqueued downloads/prefetch/maintenance/sync/auto-refresh + live job events
- ✅ Single-instance: second launch focuses the running window
- ✅ Images verified loading from API v2 relative paths; disk image cache backing `proxy_image`
- ⬜ M7 polish: Download button, CBZ/other formats, downloads storage management
- ⬜ M8: light theme, accent picker, image quality, reader preload distance; installer E2E smoke now runs via CI packaging workflow on release tags

## 🔁 Recurring themes (applies to every milestone)

- 🐢 Rate-limit and cache requests; single-flight duplicate requests.
- Images always lazy; degraded gracefully when the CDN 404s.
- Accessibility: keyboard navigation for filters, focus states, reduced motion.
- Testing: svelte-check strictness + Rust unit tests on query building and URL mapping;
  filter-query round-trip is a permanent invariant.

## 📅 Review cadence

- 📝 ROADMAP.md is updated whenever scope changes, a milestone completes, or a decision log
  entry lands in `AGENTS.md`.
- Everything in ROADMAP should trace to `TODO.md` items; orphaned items get cleaned up.