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
  despite valid metadata (CDN re-encoding, `webp` shifts). Proxy fallback + graceful placeholder required.
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

*(none yet)*