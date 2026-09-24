# Getting Started

This page walks you through your first minutes with **NH Desktop**. Everything is local-first:
no account required, nothing to sign up for.

## 📦 1. Install

Download the installer for your OS from
[Releases](https://github.com/HELIX-Origin/nhentai-desktop/releases) and run the wizard. See
[Installation & Maintenance](Installation-and-Maintenance.md) for the full walkthrough,
including uninstall and repair options.

## 🏁 2. First launch

The app opens on the **New** (home) feed — recently published galleries, paginated. The
sidebar on the left navigates everywhere:

- **New** — latest galleries
- **Popular** — the site's popular list
- **Search** — the [filter engine](Search-and-Filters.md)
- **Favorites** / **History** — your local library
- **Blacklist** — global [blacklist](Blacklist.md) management
- **Settings** — appearance + optional [API key](Settings-and-API-Key.md)

## 🔑 3. Do you need an API key?

**No.** Browsing, searching, the reader, favorites, history, and blacklist all work without
one. An nhentai.net API key is **optional** and only unlocks one thing in this app:

- **Account sync:** view/manage your nhentai.net account *favorites* and *blacklist* from
  within the app (see [Favorites & History](Favorites-and-History.md)).

Get one from nhentai.net → *Settings → API Key*. Add it in the app under **Settings → API
Key**. It is stored locally in `nhentai.db` and sent only to nhentai.net over HTTPS — see
[Privacy](Privacy.md).

## 🔍 4. Search something real

Try the **Search** page. Enter raw nhentai syntax (`english`, `-lolicon`, `artist:hana`), or
open the **Filters** drawer for structured controls. Example that mixes both:

```
blue archive language:english -tag:lolicon pages:>100
```

See [Search & Filters](Search-and-Filters.md) for the full language reference.

## 🚫 5. Set up your blacklist

Go to **Blacklist** and add tags or text patterns you never want to see. The blacklist is:

- applied **server-side** (tag excludes are baked into every search's query — the site itself
  filters),
- applied **client-side** too (hide/blur in grids),
- toggleable globally (master switch) — flip it off if a search is "too empty".

Details: [Blacklist](Blacklist.md).

## 🖼️ 6. Reading

Open any gallery to see its detail page, then hit the reader. Paged thumbnails, strip mode,
preload, and fullscreen are all supported — see [Reader & Galleries](Reader-and-Galleries.md).

## ❓ 7. Unsure about something?

Check [Troubleshooting](Troubleshooting.md) and the [FAQ](FAQ.md). For behavior/errors you
still can't resolve, open an issue at the [repository](https://github.com/HELIX-Origin/nhentai-desktop).

---

- Next: [Search & Filters](Search-and-Filters.md) · [Blacklist](Blacklist.md)