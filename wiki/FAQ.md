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

- Windows: `%APPDATA%\nhentai` (database), `%LOCALAPPDATA%\Programs\NH Desktop` (install dir).
- macOS: `~/Library/Application Support/nhentai`.
- Linux: `~/.local/share/nhentai`.

Browser (localStorage) data lives in the WebView storage for the app origin.

## 🚫 Does the blacklist sync with my nhentai.net account?

Only if you add an API key and enable it. By default the blacklist is fully local
(localStorage, mirrored to `nhentai.db`). See [Blacklist](Blacklist.md).

## ⬜ Can I download galleries for offline reading?

Planned (stretch milestone). See [Roadmap](Roadmap.md).

## 🤝 I found a bug / want a feature.

Open an issue at [HELIX-Origin/nhentai-desktop](https://github.com/HELIX-Origin/nhentai-desktop),
or see [Development & Contributing](Development-and-Contributing.md).

## 🔗 Related

- [Getting Started](Getting-Started.md) · [Troubleshooting](Troubleshooting.md) ·
  [Privacy](Privacy.md) · [Roadmap](Roadmap.md)