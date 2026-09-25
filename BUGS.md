# BUGS.md

> Living index of known issues and quirks. One section per bug, most recent first.
> Entering a bug → use the template in `.agents/templates/bug.md`; large investigations
> get a detail file in `.agents/tracking/bugs/BUG-###.md` (link it here).

## 📖 Status legend

- 🚨 **open** — reproducible, needs fixing
- 🚧 **investigating** — repro/root-cause in progress
- ⚠️ **wontfix** — accepted limitation
- ✅ **fixed** — resolved (moved to this header, keep for history)

## ⚠️ Known quirks & external limitations (wontfix bucket)

- 🐢 **Rate limiting / 429s:** nhentai throttles rapid API access. The Rust client throttles
  requests; UI must back off and not spam-retry.
- **Image CDN 404s:** some legacy galleries return 404s on `t.nhentai.net`/`i.nhentai.net`
  despite valid metadata (CDN re-encoding, `webp` shifts). Proxy fallback (cache-backed
  `proxy_image`) + graceful placeholder required.
- **Single-instance is app-wide:** running the app again focuses the existing window — by
  design. The installer/maintenance mode builds its own Tauri app and is intentionally outside
  the single-instance plugin.
- **Search semantics are the site's:** query syntax is nhentai's (`-`, `,`, `:`, ranges).
  Our UI builds it, but edge semantics (e.g. OR-within-type) are inherited, not invented.
- **Upload-date "popular" ordering:** the site exposes no stable *popularity* sort in search;
  only recency. Surfaces limited accordingly.

## 🧠 Explicitly not bugs

- 💡 Galleries that legitimately contain blacklisted tags are still accessible from detail/reader
  (blacklist governs discovery lists, not direct links) — by design.

## 🚨 Open

*(no open issues)*

## ✅ Fixed

- ✅ **Images blank everywhere (URL join bug).** nhentai API v2 returns *relative* paths without
  a leading slash (e.g. `galleries/<id>/thumb.webp`); naive `host + path` concatenation produced
  invalid URLs (`https://t.nhentai.netgalleries/...`) and images never loaded. Fixed by deriving
  absolute URLs in `image.ts` (`joinUrl` via `pagePath`/`thumbPath`/`avatarUrl`). Old persisted
  history thumbnails from the buggy era are re-joined idempotently by `thumbPath`.
- ✅ **Sign-in chip showed "Sign in" with a registered key.** The chip derived from
  `account.user`, so a failed `getCurrentUser()` at startup (e.g. network) made a configured key
  look signed-out. Fixed: the chip now derives from `keyStatus.configured` — username when the
  user fetch succeeded, "Connected" when a key exists but the fetch failed, "Sign in" only when
  no key is stored.