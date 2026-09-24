# nhentai desktop client — Wiki

This folder mirrors the [project Wiki](https://github.com/HELIX-Origin/nhentai-desktop/wiki)
as plain Markdown so it can be reviewed, contributed to, and synced to GitHub Wiki.

- To **publish** to GitHub Wiki, copy this folder's contents into the wiki repository
  (`https://github.com/HELIX-Origin/nhentai-desktop.wiki.git`) — the wiki root is the folder
  root. GitHub automatically renders `_Sidebar.md` (right sidebar in the wiki UI) and
  `_Footer.md` (footer).
- Files are named for direct wiki-page mapping: `Home.md`, `Getting-Started.md`, and so on.
  GitHub wiki page names use hyphens in URLs; keep file names hyphenated (no spaces).

## Page index (mirrors `_Sidebar.md`)

- **Start here:** [Home](Home.md) · [Getting Started](Getting-Started.md) · [Installation & Maintenance](Installation-and-Maintenance.md)
- **Using the app:** [Search & Filters](Search-and-Filters.md) · [Blacklist](Blacklist.md) · [Reader & Galleries](Reader-and-Galleries.md) · [Favorites & History](Favorites-and-History.md) · [Settings & API Key](Settings-and-API-Key.md)
- **Under the hood:** [Architecture](Architecture.md) · [Backend (Rust)](Backend-Rust.md) · [Frontend (SvelteKit)](Frontend-SvelteKit.md) · [Installer Engine](Installer-Engine.md)
- **Policy & development:** [Security](Security.md) · [Privacy](Privacy.md) · [Troubleshooting](Troubleshooting.md) · [FAQ](FAQ.md) · [Roadmap](Roadmap.md) · [Development & Contributing](Development-and-Contributing.md)

---

Pull requests against `wiki/` are welcome. Keep each page focused and link-heavy — the wiki
routes between pages, so prefer cross-links over duplicating content.