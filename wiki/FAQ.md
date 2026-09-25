# FAQ

Short answers to common questions.

## ❓ Is this an official nhentai.net app?

No. It's an independent client that uses nhentai.net's public API. It is not affiliated with
or endorsed by nhentai.net.

## 🔑 Do I need an API key?

No. Everything works without one. A key adds account features (synced favorites,
authenticated blacklist, downloads) via nhentai.net's API.

## 🕵️ Is my data private?

Yes. See [Privacy](Privacy.md). Everything is stored on-device; the app only talks to
nhentai.net and its CDNs. No telemetry, no app server.

## ⚠️ How is adult content handled?

This is a client for an adult-adjacent site; you must be 18+ to use it. See
[TOS](https://github.com/HELIX-Origin/nhentai-desktop/blob/main/TOS.md).

## 📦 Why is the app one binary and not an MSI?

We build the installer with Tauri itself (a single binary routes between app and
installer/uninstaller by its filename/args). It avoids outdated MSI/NSIS tooling and keeps a
small footprint. See [Installer Engine](Installer-Engine.md).

## 💾 Where is my data stored on disk?

- Windows: `%APPDATA%\net.nh-desktop.client` (database/cache), `%LOCALAPPDATA%\Programs\NH Desktop` (install dir).
- macOS: `~/Library/Application Support/net.nh-desktop.client`.
- Linux: `~/.local/share/net.nh-desktop.client`.

The app does not use browser `localStorage`; all persistent data is in the SQLite database or
on-disk caches above.

## 🚫 Does the blacklist sync with my nhentai.net account?

Only if you add an API key and enable it. By default the blacklist is fully local
(`nh-desktop.db`). See [Blacklist](Blacklist.md).

## ⬜ Can I download galleries for offline reading?

Yes — gallery **zip** downloads run in the background service and stream progress to the
Settings panel. They need an API key. There's no in-app **Download** button yet (the queue and
commands are wired; the UI is in the M7 backlog). CBZ/other formats and downloads-folder
management are planned. See [Roadmap](Roadmap.md).

## 📱 Is there a mobile version?

No. NH Desktop is desktop-only (Windows, macOS, Linux), **by design**:

- **Android** already has a good third-party client: [`NClientV3`](https://github.com/maxwai/NClientV3).
- **iOS** rejects adult/NSFW apps, and an Apple developer license is prohibitively expensive
  for an app that wouldn't be allowed anyway.

Use NClientV3 on Android or the nhentai.net site in a mobile browser.

## 🤝 I found a bug / want a feature.

Open an issue at [HELIX-Origin/nhentai-desktop](https://github.com/HELIX-Origin/nhentai-desktop),
or see [Development & Contributing](Development-and-Contributing.md).

## 🔗 Related

- [Getting Started](Getting-Started.md) · [Troubleshooting](Troubleshooting.md) ·
  [Privacy](Privacy.md) · [Roadmap](Roadmap.md)