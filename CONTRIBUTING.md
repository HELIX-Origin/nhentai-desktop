# Contributing to NH Desktop

Thank you for your interest in **NH Desktop**. This document covers how to set up a development environment, follow the project's conventions, open issues, and submit changes.

If you have not read it yet, start with [`AGENTS.md`](./AGENTS.md) for the high-level project overview, stack decisions, and layout.

---

## Table of Contents

1. [Code of Conduct](#code-of-conduct)
2. [Ways to Contribute](#ways-to-contribute)
3. [Development Setup](#development-setup)
4. [Project Layout](#project-layout)
5. [Coding Conventions](#coding-conventions)
6. [Commit Message Format](#commit-message-format)
7. [Testing](#testing)
8. [Pull Request Process](#pull-request-process)
9. [Release Process](#release-process)
10. [Security](#security)
11. [Getting Help](#getting-help)

---

## Code of Conduct

- Be respectful and constructive.
- Keep criticism focused on the code, not the person.
- Respect project boundaries: NH Desktop is a desktop-only client for nhentai.net. Mobile support and server-side hosting are out of scope.
- Follow the nhentai.net terms of service and do not build features designed to abuse the site or its API.

---

## Ways to Contribute

- **Report bugs** via [GitHub Issues](../../issues) using the bug-report template.
- **Propose features** via [GitHub Issues](../../issues) using the feature-proposal template.
- **Improve documentation** in `README.md`, `wiki/`, or `.agents/`.
- **Submit code changes** via pull request.
- **Review pull requests** from other contributors.

Before starting significant work, open an issue or comment on an existing one so the approach can be discussed. See `.agents/rules/issue-protocol.md` for issue conventions.

---

## Development Setup

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable)
- [Node.js](https://nodejs.org/) LTS
- [npm](https://www.npmjs.com/)
- Platform-specific Tauri dependencies:
  - **Windows**: Microsoft Visual C++ Build Tools, WebView2 runtime.
  - **macOS**: Xcode Command Line Tools.
  - **Linux**: `libwebkit2gtk-4.1-dev`, `build-essential`, `libssl-dev`, `libappindicator3-dev`, `librsvg2-dev` (see [Tauri prerequisites](https://tauri.app/start/prerequisites/)).

### Install dependencies

```bash
npm install
```

### Run the app in development mode

```bash
npm run dev:tauri
```

This starts Vite on the fixed port `14440` (HMR on `14441`) and launches the Tauri window.

### Useful commands

| Command | Purpose |
| --- | --- |
| `npm run check` | Frontend type/lint check (`svelte-kit sync` + `svelte-check`) |
| `npm run build` | Build the static frontend site |
| `npm run tauri build` | Full release bundle |
| `cargo check` | Rust type check (run in `src-tauri/`) |
| `cargo test` | Rust unit tests (run in `src-tauri/`) |

Always run the relevant checks before declaring a task done:

- Frontend changes → `npm run check`
- Rust changes → `cargo check` + `cargo test`
- Full release → `npm run tauri build`

---

## Project Layout

```
AGENTS.md                    # Agent entry point and project overview
README.md                    # Human-facing quick start and overview
ROADMAP.md                   # Product direction and milestones
TODO.md                      # Actionable task ledger
BUGS.md                      # Known issues and quirks
CHANGELOG.md                 # Release changelog
LICENSE.md                   # BSD 3-Clause license
CITATION.cff                 # Citation metadata
.agents/                     # Agent ecosystem (rules, skills, templates)
.github/                     # Issue templates and CI workflows
src/                         # Frontend SvelteKit SPA
  lib/
    api/                     # nhentai API types + client wrapper
    components/              # UI components
    design/                  # CSS tokens and base styles
    stores/                  # Runes-based stores
  routes/                    # SvelteKit routes
src-tauri/                   # Rust backend
  src/
    commands.rs              # Tauri command layer
    lib.rs                   # App bootstrap + window/tray/service wiring
    main.rs                  # Entry point / installer routing
    nh_desktop.rs            # nhentai API client
    service.rs               # Background worker queue
    image_cache.rs           # Disk image cache
    db.rs                    # SQLite persistence
    installer.rs             # Unified installer/uninstaller logic
    platform/                # Platform-specific installer helpers
wiki/                        # GitHub wiki pages
```

---

## Coding Conventions

Detailed rules live in `.agents/rules/`:

- `.agents/rules/frontend.md`
- `.agents/rules/backend.md`
- `.agents/rules/git-workflow.md`
- `.agents/rules/issue-protocol.md`
- `.agents/rules/release-standards.md`

The short version:

### Frontend

- **Svelte 5 runes** (`$state`, `$derived`, `$effect`, `$props`).
- **Strict TypeScript** enabled.
- **Plain CSS** with design tokens (`src/lib/design/tokens.css`). No CSS frameworks.
- File/folder names: `kebab-case`.
- Component names: `PascalCase`.
- Prefer `$lib/` aliases for imports.
- Do not add comments unless explicitly asked; prefer self-documenting names.

### Backend

- **Rust** with `snake_case` modules and functions.
- Small, focused modules (`nh_desktop.rs`, `commands.rs`, `error.rs`, etc.).
- No panics across the Tauri command boundary — return `Result`.
- Use `thiserror` / `anyhow` patterns for error handling.
- Respect nhentai's public API: throttle requests and never hammer the site.

### General

- Keep changes minimal and focused.
- Update tracker docs (`TODO.md`, `BUGS.md`, `CHANGELOG.md`, `ROADMAP.md`) when scope changes.
- Match the existing code style.

---

## Commit Message Format

We use [Conventional Commits](https://www.conventionalcommits.org/) with project scopes. See `.agents/templates/commit-message.md` for the full guide.

```
<type>(<scope>): <short summary in imperative mood>

<body: what changed and why. Keep lines ≤ 72 chars.>

Refs: #<issue> | Closes #<sub-issue>
```

Common types:

| Emoji | Type | Use |
| --- | --- | --- |
| ✨ | `feat` | New user-facing capability |
| 🐛 | `fix` | Bug fix |
| 📝 | `docs` | Documentation, wiki, `.agents/` |
| 🧪 | `test` | Tests only |
| ♻️ | `refactor` | No behavior change |
| ⚡ | `perf` | Performance improvement |
| 🔧 | `chore` | Build, deps, tooling, CI |
| 🔒 | `security` | Security hardening |
| 🏗️ | `build` | Builds / packaging / installer |

Common scopes include `ui`, `gallery`, `reader`, `search`, `blacklist`, `account`, `library`, `settings`, `installer`, `tray`, `service`, `cache`, `commands`, `api`, `docs`, `agents`, `release`, `ci`.

---

## Testing

### Frontend

```bash
npm run check
```

This runs `svelte-kit sync` and `svelte-check` with strict TypeScript.

### Backend

```bash
cd src-tauri
cargo check
cargo test
```

All nine unit tests must pass:

- `image_cache::tests::round_trip_and_distinct_names`
- `image_cache::tests::prune_removes_old_only`
- `installer::tests::check_disk_space_reports_space`
- `installer::tests::detect_status_returns_valid_info`
- `installer::tests::executable_name_matches_platform`
- `nh_desktop::tests::deserializes_favorite_and_user_fixtures`
- `nh_desktop::tests::deserializes_gallery_detail_from_v2_fixture`
- `nh_desktop::tests::deserializes_gallery_list_from_v2_fixture`
- `nh_desktop::tests::parses_only_https_allowlisted_image_hosts`

### Manual / release testing

- `npm run build` must complete.
- `npm run tauri build` must produce a working installer.
- On Windows, run the installer, verify install/uninstall, and confirm the app launches.

---

## Pull Request Process

1. **Open an issue first** for non-trivial changes.
2. **Create a branch** from `main`.
3. **Make focused commits** following the commit-message format.
4. **Run checks** (`npm run check`, `cargo check`, `cargo test`).
5. **Update docs** if behavior changed (`CHANGELOG.md`, `README.md`, `wiki/`, `.agents/` as appropriate).
6. **Open a PR** and fill out the template. Use `--body-file` for non-interactive submission if needed.
7. **Respond to review feedback** and keep the branch up to date with `main`.

Do not push directly to `main`. Releases are managed via annotated tags per `.agents/rules/release-standards.md`.

---

## Release Process

Releases follow `.agents/rules/release-standards.md`:

1. Sync version strings in `package.json`, `src-tauri/Cargo.toml`, and `src-tauri/tauri.conf.json`.
2. Run the verification gate: `cargo check`, `cargo test`, `npm run check`, `npm run build`, installer smoke test.
3. Update `CHANGELOG.md`.
4. Create an annotated tag: `git tag -a vX.Y.Z -m "vX.Y.Z — summary"`.
5. Push tags: `git push origin main --tags`.
6. Create a GitHub release with detailed notes (never just "See CHANGELOG.md").
7. The `v*` tag push triggers the Cross-Platform Packaging workflow, which attaches installers.

Only maintainers cut releases.

---

## Security

- Do not commit API keys, passwords, or personal paths.
- Report sensitive security issues privately per `SECURITY.md`.
- Keep networking in the Rust backend; the frontend talks to nhentai only through Tauri commands.
- Follow `.agents/rules/security.md` for dependency, CSP, and API-key handling guidance.

---

## Getting Help

- Read [`AGENTS.md`](./AGENTS.md) and `.agents/README.md`.
- Check `wiki/` and existing [Discussions](../../discussions).
- Open a [GitHub Issue](../../issues) if something is unclear or broken.

Thank you for contributing.
