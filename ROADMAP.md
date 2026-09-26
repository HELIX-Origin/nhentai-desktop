# ROADMAP.md

> Product direction and milestones for **NH Desktop**. This is the "what / why" doc —
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
| M1 | Foundation | ✅ Shipped (core); closeout pending | Scaffold, green toolchain, Rust client + 43 commands, CSP, app shell, tokens, stores, all views; remaining: clean-clone baseline verification |
| M2 | Browse & discover | ✅ Shipped (core) | Home (new releases), popular, gallery grid/cards, pagination, lazy images with proxy fallback |
| M3 | Search & filters | ✅ Shipped (core) | Query builder, filter drawer (text, language, category, per-type tag include/exclude, page ranges, sort), results + count |
| M4 | Global blacklist | ✅ Shipped (core) | Manage panel, server-side `-tag:` excludes, client-side hide/blur, master toggle |
| M5 | Library | ✅ Shipped (core) | Favorites, history, local persistence; import/export JSON still backlog |
| M6 | Reader | ✅ Shipped (core) | Gallery detail, paged thumbnails, strip mode, preload, fullscreen |
| M7 | Downloads & background services | 🚧 In progress (core + UI shipped) | Background-service core shipped (zip/cbz/torrent downloads to disk with progress, image prefetch, cache/image maintenance, account sync, Popular auto-refresh with job events); per-gallery Download button + Downloads page shipped; still backlog: queue resume/persist, cache consumers, storage-management polish |
| M8 | Polish / release | 🚧 In progress (0.3.0 shipped) | Release 0.3.0 shipped 2026-09-25 (localization, installer launch fix, centered titlebar search); still backlog: accent picker, image quality, reader preload distance |
| M9 | Localization & language packs | ✅ Core shipped (M9.1) | System-locale default, installer + Settings language pickers, English fallback per key, `en`/`ja`/`zh-Hans`/`zh-Hant` packs complete. **M9.2** (14 remaining nhentai language packs + RTL pass for Arabic) is planned in `TODO.md`, not started |

## 🎯 Current focus

**M7 Downloads & background services** core pipeline is shipped (see `TODO.md`); **M1
Foundation** closeout and **M8 hardening** remain:

- ⬜ `npm install` + `npm run check` + `cargo check`/`test` all green on a clean clone
- ✅ Rust client returns real nhentai data (search, newest, detail) through Tauri commands
- ✅ SPA chrome navigates between Home / Search / Favorites / History / Blacklist / Settings
- ✅ Design tokens wired (dark theme live, light theme shipped); responsive grid
- ✅ CSP hardened (image hosts allowlisted, IPC only, devCsp for Vite HMR)
- ✅ Background service: enqueued downloads/prefetch/maintenance/sync/auto-refresh + live job events
- ✅ Single-instance: second launch focuses the running window
- ✅ Images verified loading from API v2 relative paths; disk image cache backing `proxy_image`
- ✅ M7 polish: per-gallery Download button, ZIP/CBZ/torrent formats, configurable downloads folder, open-downloads-folder
- ⬜ M7 remaining: queue resume/persist, cache consumers, storage-management polish
- ⬜ M8 remaining: accent picker, image quality, reader preload distance; installer E2E smoke now runs via CI packaging workflow on release tags
- ✅ M9 shipped: system-locale default, installer + Settings language pickers, `en`/`ja`/`zh-Hans`/`zh-Hant` packs, English fallback, contributor guide (`wiki/Localization.md`); remaining packs deferred

## 🔁 Recurring themes (applies to every milestone)

- 🐢 Rate-limit and cache requests; single-flight duplicate requests.
- Images always lazy; degraded gracefully when the CDN 404s.
- Accessibility: keyboard navigation for filters, focus states, reduced motion.
- Testing: svelte-check strictness + Rust unit tests on query building and URL mapping;
  filter-query round-trip is a permanent invariant.

## 🌍 M9 — Localization & language packs

Goal: full UI localization with language packs for every language nhentai.net tags on content.

- **Phase 1 — M9.1 core (shipped)**: locale store, `t()` helper, system-language detection,
  language selector in installer options and Settings page, `npm run i18n:check` validator, and
  complete packs for English (source), Japanese, Chinese (Simplified + Traditional).
- **Phase 2 — M9.2 remaining packs (planned, not started)**: the other nhentai content languages
  (`ko`, `es`, `fr`, `de`, `ru`, `pt`, `it`, `th`, `vi`, `id`, `pl`, `nl`, `tr`, `ar`). All are
  already registered and selectable with per-key English fallback. Itemised in `TODO.md`;
  contributed opportunistically or via community pull requests. See `wiki/Localization.md`.
- **Phase 3 — M9.2 RTL (planned, not started)**: RTL layout for Arabic, date/number formatting
  per locale, extension of `en.json` to the remaining views.

## 📅 Review cadence

- 📝 ROADMAP.md is updated whenever scope changes, a milestone completes, or a decision log
  entry lands in `AGENTS.md`.
- Everything in ROADMAP should trace to `TODO.md` items; orphaned items get cleaned up.