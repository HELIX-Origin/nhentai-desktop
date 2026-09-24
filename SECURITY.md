# Security Policy

The nhentai desktop client is an independent client for nhentai.net. We take security
seriously — both for the app itself and for the privacy of your local data.

## Supported versions

| Version | Supported |
| --- | --- |
| 0.1.x | ✅ Active |
| < 0.1 | ❌ |

Only the latest release receives security fixes. Check the
[Releases page](https://github.com/HELIX-Origin/nhentai-desktop/releases) for the current
version.

## Reporting a vulnerability

**Do not open a public issue** for a security vulnerability.

Report privately:

- **GitHub:** use the repository's **private vulnerability reporting** feature
  (Repository → `Security` → `Report a vulnerability`) at
  `https://github.com/HELIX-Origin/nhentai-desktop/security`.
- **Email:** if needed, reach a maintainer via the discussion board or a maintainer's public
  profile — we will respond as soon as possible.

In your report, please include:

1. The affected version(s) and platform.
2. A clear description of the vulnerability.
3. Steps to reproduce (minimal, if possible).
4. Impact assessment and — if you have one — a suggested fix.

### What happens next

We aim to acknowledge reports within **5 business days**, and to ship a fix for confirmed
issues within a reasonable window depending on severity. Security fixes are noted in
[CHANGELOG.md](CHANGELOG.md).

## Scope

Issues we consider in scope:

- Code execution, path traversal, or arbitrary writes via the installer/uninstaller
  (`installer.rs`, `platform/` powering install & uninstall).
- Injection or data corruption via the Rust API client, SQLite persistence (`db.rs`), or the
  WebView bridge (Tauri commands).
- Malicious gallery metadata (titles, tags, filenames) that breaks the app or leaks data —
  remember remote content is **not trusted input**.
- Privacy leaks in the image proxy (`proxy_image`) or external-link opener.
- Insecure release pipeline (unsigned/mutated binaries, malicious installers).

Out of scope:

- Denial-of-service against nhentai.net, or issues that fundamentally stem from nhentai.net's
  own website/app.
- Phishing or social engineering against individual nhentai users.
- Vulnerability reports about the site's Web API itself — report those to nhentai.net.

## Security design notes (for auditors)

- **CSP is set** in `src-tauri/tauri.conf.json`: images are allow-listed to `t.nhentai.net`,
  `i.nhentai.net`, `data:`, `blob:`; IPC connects are limited to the app's local IPC endpoints.
  A separate `devCsp` permits the Vite dev-server (HMR) endpoints.
- **No remote code paths**: the Rust backend fetches only from nhentai.net API + CDN hosts
  (`nhentai.rs`), rendering content is done in a WebView with the CSP above, and external links
  open in the system browser via the OS opener.
- **Rate limiting**: the API client throttles requests (`THROTTLE` sleep in `nhentai.rs`) to
  respect nhentai.net's public API; the CI/CLI never bypasses it.
- **The installer runs a single binary** (the app itself is the installer, see `main.rs`
  argv routing). Uninstall strings, Start Menu/Desktop shortcuts, PATH, and the HKCU uninstall
  key are all managed through the `platform/` module with PowerShell/AppleScript desktop APIs.
- **User data** (favorites, history, blacklist, settings, API key) is stored locally only
  (SQLite + `localStorage`). See [PRIVACY.md](PRIVACY.md).

## Security hardening wishlist

These are tracked in the Wiki's [Roadmap](wiki/Roadmap.md):

- Code-signing / notarization for Windows + macOS release binaries.
- Installer signature verification prior to launch-any-later-updates.
- Additional sanitization hardening for HTML/URL output in gallery detail pages.
- A formal threat-model document.