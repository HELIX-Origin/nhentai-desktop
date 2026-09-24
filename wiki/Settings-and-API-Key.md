# Settings & API Key

**Route:** Settings · component `src/lib/components/SettingsView.svelte`

Settings are kept deliberately small and local. All settings persist in `localStorage`
(dark theme active; a light theme is reserved behind `[data-theme='light']`).

## Appearance

- **Theme:** dark (default), light (reserved, not yet shipped — see [Roadmap](Roadmap.md)).
- Accent color and image-quality/reader-preference options are planned (Settings backlog).

## API key (optional)

The only "online" setting. Get a key from nhentai.net → *Settings → API Key*.

| Control | Behavior |
| --- | --- |
| **Add API key** | Stores the key locally in `nhentai.db` (the `api_key` table). The UI shows only a 4-character prefix after saving. |
| **Verify** | Calls nhentai.net's `/api/users/me` with the stored key and shows your username if valid. |
| **Clear** | Removes the stored key from the local DB immediately. |

### What the key enables

- Account favorites sync (`check_favorite`, `add_favorite`, `remove_favorite`,
  `fetch_favorites`).
- Account blacklist sync (`fetch_account_blacklist`, `update_account_blacklist`).
- Downloading galleries with your account's privileges (`download_gallery`).

### Security notes

- Keys are stored **only locally** (SQLite), never logged, never sent anywhere except
  nhentai.net over HTTPS (as the standard `Authorization: Bearer <key>` header).
- The Rust commands `require_key`/`optional_key` (`commands.rs`) ensure commands that need a
  key fail with a friendly error when missing — the UI never silently passes an empty key.
- Clearing the key is immediate; future command calls then fall back to anonymous mode or
  return "no API key configured".

## Cache

- **Clear cache** — flushes the in-memory Redis-style cache and the mirrored SQLite table
  (`cacheFlush()` → `clear_db`). Safe: it only drops cached lists/images, never favorites,
  history, blacklist, settings, or the API key.
- The cache stores recent gallery/list payloads under the `nhentai:` prefix and rehydrates on
  launch (`cacheInit()`), keeping repeat navigation instant and reducing load on the site.

## Other settings surfaces

Beyond the Settings page, related persistent state lives in:

- `src/lib/stores/settings.svelte.ts` — user preferences
- `src/lib/stores/library.svelte.ts` — favorites/history persistence
- `src/lib/stores/blacklist.svelte.ts` — the global blacklist + master toggle
- `src/lib/stores/account.svelte.ts` — account/API-key state

## Related

- [Search & Filters](Search-and-Filters.md) · [Blacklist](Blacklist.md)
- [Backend (Rust)](Backend-Rust.md) — the commands behind these toggles