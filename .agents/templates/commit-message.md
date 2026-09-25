# Commit Message Format

Follow [Conventional Commits](https://www.conventionalcommits.org/) with the
project's scopes.

## Format

```
<type>(<scope>): <short summary in imperative mood>

<body: what changed and why. Keep lines ≤ 72 chars.>

Refs: #<issue> | Closes #<sub-issue>
```

Keep the subject ≤ ~72 chars and imperative ("add", "fix", "wire" — not
"added", "fixes"). Body explains *why*, not just *what*.

## Types

| Emoji | Type | Use |
| --- | --- | --- |
| ✨ | feat | new user-facing capability |
| 🐛 | fix | bug fix |
| 📝 | docs | documentation, wiki, `.agents/` |
| 🧪 | test | tests only |
| ♻️ | refactor | no behavior change |
| ⚡ | perf | performance |
| 🔧 | chore | build, deps, tooling, CI |
| 🔒 | security | security hardening |
| 🏗️ | build | builds / packaging / installer |

Emoji is optional but consistent with repo history; `type` is always present.

## Scopes

- `api` — nhentai API client, `nh_desktop.rs`, `api.ts`, types
- `commands` — Tauri command layer (`commands.rs`)
- `service` — background worker queue (`service.rs`)
- `cache` — disk image cache (`image_cache.rs`) / `cacheInit`
- `gallery` — gallery grid, cards, cover images
- `reader` — gallery detail / reader views
- `search` — query builder, filter drawer, search results
- `blacklist` — blacklist store, `-tag:` excludes, hide/blur modes
- `account` — sign-in, account store, key status
- `library` — favorites, history
- `settings` — settings store, Settings view, service panel
- `installer` — unified installer/uninstaller, maintenance window
- `tray` — system tray, window controls
- `ui` — app shell, titlebar, sidebar, styling/tokens
- `docs` — README/ROADMAP/TODO/BUGS/wiki
- `agents` — `.agents/` rules/skills/templates
- `release` — version bumps, changelogs
- `ci` — workflows

## Examples

```
✨ feat(reader): add preload for next gallery page

Reader now fetches the following page while the current one is being
viewed, so paging feels instant on slow connections.

Closes #4
```

```
📝 docs(agents): rewrite rule-04 for roadmap-first issues

Mirrors the issue/commit standards used in the reference launcher
repo so both projects stay consistent.
```

## Don'ts

- No "wip", "stuff", "misc", or body-less mega-commits mixing unrelated
  concerns.
- No secrets, no personal paths, no binary blobs.
- A commit that moves `TODO.md` / `BUGS.md` / `ROADMAP.md` state carries that
  doc update in the same commit (or calls it out).