# Troubleshooting

Common problems and fixes. If the same issue persists after these steps, open a GitHub issue
with the steps you took and the relevant log/error text.

## 📦 Installation

### 🚨 "The app won't install / shows nothing"

- On Windows, run the uninstall/repair from **Settings → Apps → NH Desktop → Modify** (or the
  maintenance window in-app via the sidebar → **Modify installation**).
- On macOS, drag the `.app` into `/Applications`, then double-click.
- On Linux, make sure `~/.local/share/NH Desktop` is writable and the `.desktop` entry was
  created.

### 🚨 "Setup disappears after running"

The installer binary is the app binary. If the file is named `NH Desktop-Setup.exe`, double
clicking it opens the wizard. If it was renamed to just `NH Desktop.exe`, it starts the main
app instead. Re-run via a correctly-suffixed copy, or pass `--installer`.

## 🖼️ Loading / images

### 🚨 Galleries load but images are blank

- Check the network: `t.nhentai.net` / `i.nhentai.net` must be reachable (CSP already
  allows them).
- Some legacy galleries 404 on the CDN. The app auto-falls back to its image proxy
  (`proxy_image` → `blob:`); if that still fails you'll see an error notice with a retry.
- Try clearing the cache (**Settings → Cache**), then reload.

Stuck on blank images? Follow this decision tree:

```mermaid
flowchart TD
    A[Image is blank] --> B{CDN reachable?}
    B -->|No| C[Check network - CDN hosts blocked]
    B -->|Yes| D{Direct load worked?}
    D -->|Yes| E[Image displays]
    D -->|No| F[Auto proxy_image fallback]
    F -->|Loaded| E
    F -->|Fails| G[Clear cache and reload]
    G --> H{Still blank?}
    H -->|No| E
    H -->|Yes| I[Error notice with retry]
```

### 🚨 "Network error" or timeouts

- nhentai.net rate-limits. The app throttles requests, but rapid pagination can still hit a
  wall — wait a moment and retry (the UI shows a retry button on notices).
- Restart the app; the client rebuilds its connection.

## 🔍 Search & filters

### 🚨 "Search returns wrong / no results"

- Check the nhentai query syntax (see [Search & Filters](Search-and-Filters.md)): exclusion
  needs the `-` prefix (`-tag:loli`), `tag:`, `artist:`, `language:english`, etc.
- Note the blacklist is applied **server-side** to every query. If a gallery is hidden but
  search says "no results", a matching blacklisted tag may be excluding it. Toggle the
  blacklist master switch off to confirm.
- Sorts other than date fetch from specific popular lists; a "popular" result page may differ
  from a free-text search.

## ⭐ Favorites / history / blacklist

### 🚨 "My favorites disappeared"

- Favorites are stored in `localStorage`. Browser-data clears, app reinstalls with
  "Remove user data", or storage eviction can wipe them. Export is planned; for now back up
  early.
- If you use an API key, **in-app favorites** and **account favorites on nhentai.net** are
  separate — in-app favorites are local-only.

## 🔗 Related

- [Getting Started](Getting-Started.md) · [FAQ](FAQ.md) ·
  [Search & Filters](Search-and-Filters.md) · [Privacy](Privacy.md)