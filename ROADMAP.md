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
| M8 | Polish / release | 🚧 In progress (0.2.0 shipped) | Release 0.2.0 shipped 2026-09-24 (installer + verified gates green, light theme, configurable downloads folder); still backlog: accent picker, image quality, reader preload distance, installer E2E smoke via CI |
| M9 | Localization & language packs | 🚧 In progress (0.2.1+) | i18n infrastructure and locale-aware UI; support all nhentai.net content languages; default to system language; selectors in installer + settings; staged translation pass (core languages first, community review for remainder) |

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
- 🚧 M9 active: i18n infrastructure, system-language default, installer + settings language selectors, staged translation of language packs for all nhentai.net content languages

## 🔁 Recurring themes (applies to every milestone)

- 🐢 Rate-limit and cache requests; single-flight duplicate requests.
- Images always lazy; degraded gracefully when the CDN 404s.
- Accessibility: keyboard navigation for filters, focus states, reduced motion.
- Testing: svelte-check strictness + Rust unit tests on query building and URL mapping;
  filter-query round-trip is a permanent invariant.

## 🌍 M9 — Localization & language packs

Goal: full UI localization with language packs for every language nhentai.net tags on content.

- **Phase 1 (infrastructure)**: locale store, `t()` helper, system-language detection,
  language selector in installer options and Settings page.
- **Phase 2 (core packs)**: complete, reviewed packs for English, Japanese, Chinese
  (Simplified + Traditional), Korean, Spanish, French, German, Russian, Portuguese.
- **Phase 3 (extended packs)**: Italian, Thai, Vietnamese, Indonesian, Polish, Dutch,
  Turkish, Arabic — draft translations with community-review markers; English fallback
  for any missing string.
- **Phase 4 (polish)**: RTL layout support for Arabic, date/number formatting per locale,
  translator contribution docs.

## 📅 Review cadence

- 📝 ROADMAP.md is updated whenever scope changes, a milestone completes, or a decision log
  entry lands in `AGENTS.md`.
- Everything in ROADMAP should trace to `TODO.md` items; orphaned items get cleaned up.