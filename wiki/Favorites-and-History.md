# Favorites & History

The client keeps two personal libraries locally: **Favorites** and **History**. Both persist
in browser `localStorage` (with a mirrored copy in the SQLite-backed cache), so they survive
restarts and live only on your machine — see [Privacy](Privacy.md).

## ⭐ Favorites

### 🛠️ Adding / removing

- From a gallery **detail page**: the **Favorite** toggle (heart) adds or removes the gallery.
  It works **with or without** an nhentai.net API key:
  - **No API key:** the favorite is stored **locally only** in `localStorage`.
  - **API key configured:** the toggle also calls `check_favorite` / `add_favorite` /
    `remove_favorite` against your nhentai.net account (streaming sync).
- From any grid card: the card's favorite badge/button.

### 🖼️ Viewing

The **Favorites** page lists your favorited galleries. With an API key it can also fetch your
remote nhentai.net favorites (`fetch_favorites`) and merge/sync them into the local view.

### 💡 What counts as a favorite?

A gallery is considered a favorite if either:

- it exists in your local favorites list, **or**
- (with API key) your nhentai.net account marks it favorite.

The detail page's favorite state reflects both sources. A favorite removed remote-side is
reflected next time the local list syncs.

## 🕑 History

### 📝 Recording

Every gallery you **open** (reader or detail) is recorded with a timestamp. History tracks
**which gallery, when**, nothing more.

### 🖼️ In the app

- **History** page lists recently-viewed galleries (most recent first), letting you jump back
  to anything you've read without re-searching.
- History persists locally and is **not** synced anywhere (no account-history feature).

### 🗑️ Clearing

History can be cleared as a batch from the History page. (Individual-entry removal is on the
roadmap — see [Roadmap](Roadmap.md).)

## 💾 Storage & privacy

- `src/lib/stores/library.svelte.ts` — favorites + history (localStorage, runes-based store).
- A mirrored copy reaches `nhentai.db` through the cache layer (`src/lib/cache.ts`) — still
  local.
- No favorites/history telemetry. Ever.

## 🔗 Related

- [Reader & Galleries](Reader-and-Galleries.md) — opening galleries (which populates history)
- [Settings & API Key](Settings-and-API-Key.md) — account-sync enablement
- [Privacy](Privacy.md)