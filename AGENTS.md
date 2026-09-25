# AGENTS.md

> **Entry point for AI agents working in this repository.** Read this file first, then
> traverse into `.agents/` for detailed rules, agent roles, and workflows.

## 👋 Project

**NH Desktop** — a lightweight, modern, cross-platform desktop client for **nhentai.net**
with its **own custom UI** and a materially better **search/filter + global blacklist**
experience than the site provides.

> ⚠️ **Identity trap:** the workspace folder is named `lewd-clips-app` but that is NOT the
> project name — the folder was simply misnamed and the user will fix it later. The product
> is **NH Desktop** (exe `NH Desktop.exe`, window title "NH Desktop"). `nhentai` / `nhentai.net`
> is the *website* this app is built for — never build, brand, or label the app with the
> website's name alone. See `.agents/rules/project-identity.md`.

- The site's filtering and blacklist are acknowledged weaknesses; our app replaces them.
- The app imports as much data as the site makes available (galleries, tags, languages,
  categories, artists, characters, parodies) and lets users slice it locally.
- **Installer:** Tauri-native unified installer/uninstaller (no NSIS/WiX MSI). Single
  binary is both app and installer; pattern ported from the lewdzone-launcher reference
  repo (reference only, never a product target).

## 🏗️ Stack (locked)

| Concern | Choice |
| --- | --- |
| Desktop shell | Tauri 2 (Rust) |
| Frontend | SvelteKit (static SPA), Svelte 5 runes, TypeScript `strict` |
| Styling | Plain modern CSS with design tokens (CSS custom properties) — **no** framework |
| Backend networking | Rust `reqwest` (nhentai API), exposed via Tauri commands |
| Local persistence | Browser `localStorage` (favorites, history, blacklist, settings) |
| Package manager | npm |

## 🏗️ Layout

```
AGENTS.md               # this file — start here
ROADMAP.md              # product direction & milestones — start here for "what/why"
TODO.md                 # actionable task ledger
BUGS.md                 # known issues & quirks
.agents/
  README.md             # gateway into the agent ecosystem (indexes are README.md, no INDEX.md)
  rules/                # standing conventions (read these before writing code)
  agents/               # role definitions ({primary-agent}/{sub-agent}/README.md)
  skills/               # repeatable workflows (implement-feature, fix-bug, ...)
  templates/            # pull-request, bug, feature, todo, requirement, agent templates
  tracking/             # optional per-item detail files (BUG-001.md, TODO-007.md, REQ-###.md)
src/                    # frontend (SvelteKit SPA)
src/lib/api/            # nhentai API types + client wrapper
src/lib/components/     # UI components
src/lib/stores/         # favorites, history, blacklist, settings (runes-based)
src-tauri/              # Rust backend
src-tauri/src/          # main.rs, lib.rs, nh_desktop.rs (API client), commands.rs
```

## 🚀 Commands

| Action | Command |
| --- | --- |
| Install deps | `npm install` |
| Run app (dev) | `npm run tauri dev` |
| Frontend type/lint check | `npm run check` (svelte-kit sync + svelte-check) |
| Frontend build | `npm run build` |
| Rust type check | `cargo check` (run in `src-tauri/`) |
| Rust unit tests | `cargo test` (run in `src-tauri/`) |
| Full release bundle | `npm run tauri build` |

Always run the relevant check before declaring a task done:
frontend changes → `npm run check`; Rust changes → `cargo check` + `cargo test`.

## ⚠️ Standing conventions (condensed — details in `.agents/rules/`)

- **Do not add comments to code unless explicitly asked.** Prefer self-documenting names.
- Frontend: kebab-case file/folder names (`gallery-card.svelte`), PascalCase component
  names (`GalleryCard`), use Svelte 5 runes (`$state`, `$derived`), strict TS, `$lib/` alias.
- Backend: snake_case, small focused modules (`nh_desktop.rs`, `commands.rs`, `error.rs`),
  no panics across the command boundary — return `Result`.
- Respect nhentai's public API; throttle requests; never hammer the site. See `.agents/rules/git-workflow.md` and `security.md`.
- **Keep tracking docs honest:** updating `TODO.md`, `BUGS.md`, or scope changes requires
  updating the doc in the same change.
- **Never commit, push, or open PRs unless the user explicitly asks.** This is non-negotiable.
- **Compress context early and tiny.** On DCP `dcp-system-reminder` nudges, run a compression
  pass per `skills/manage-context.md`. One `compress` call must stay under ~7,000 chars total
  JSON (it truncates ~8k → invalid JSON). Compress multiple small ranges, never one giant blob.

## 🛠️ Operating in a headless (non-interactive) shell

Agents run commands without a TTY. Follow these rules:

- No editors/pagers (`vim`, `nano`, `less`, `man`). Use `git --no-pager log`, `--no-edit` for merges.
- No interactive flags (`git add -p`, `git rebase -i`). Use non-interactive equivalents.
- If a command needs credentials or an interactive choice, stop and report — never pipe
  passwords (`yes | ...`, `echo pass | sudo -S`) and never blanket-approve prompts.
- Use `sudo -n` when elevation is required; it fails fast if a password is needed.
- Use `-y`/`--no-input` style flags where the tool authorizes them; do not force otherwise.

## 💡 Decision log (recent, authoritative)

- **Project identity:** the product is **NH Desktop**, a desktop client for **nhentai.net**
  (the website keeps its own name — the app never brands itself as "nhentai"). The folder
  `lewd-clips-app` is a misnomer, NOT the project name — never derive branding/naming from
  it. See `.agents/rules/project-identity.md`.
- **Name scheme:** the *user-facing* product name is **NH Desktop** (exe `NH Desktop.exe`,
  install dir `Programs\NH Desktop`, shortcuts/registry DisplayName "NH Desktop"). **All
  identifiers use the lowercase hyphenated name** `nh-desktop` (`nh_desktop_lib`,
  `nh_desktop` where Rust requires underscores): cargo package `nh-desktop`, lib crate
  `nh_desktop_lib`, module `nh_desktop` / type `NhDesktopClient`, npm package `nh-desktop`,
  bundle ID `net.nh-desktop.client`, cache prefix `nh-desktop:`, db file `nh-desktop.db`.
  The website `nhentai.net` (API, image hosts, accounts) keeps its own name, as do the
  repo's historical references to the site.
- **Stack:** Tauri 2 + SvelteKit SPA (adapter-static, `fallback: "index.html"`) + Svelte 5 runes.
- **Styling:** plain CSS with design tokens. Rationale: lightweight, full control for the
  custom UI, zero extra deps. Revisit only with a strong argument.
- **Networking model:**
  - Gallery/search/lists → Rust commands (`reqwest`) → JSON back to frontend.
  - Images → direct `<img>` loading from `t.nhentai.net` / `i.nhentai.net`; fallback to a
    Rust image-proxy command returning bytes when a direct load fails (blocked/CSP/404).
- **Blacklist:** global, persistent, applied **server-side** (query `-tag:` excludes) *and*
  **client-side** (hiding/blurring in grids), with a master toggle. Never breaks the grid.
- **Persistence:** `localStorage` for favorites/history/blacklist/settings (no server).
- **Context management:** DCP plugin (`@tarquinen/opencode-dcp`) drives the `compress` tool.
  Contract and size limits in `.agents/rules/context-management.md`; pass routine in
  `.agents/skills/manage-context.md`. Config stays at the global `~/.config/opencode/dcp.jsonc`.

For full detail, rules, and role definitions, continue to **`.agents/README.md`**.