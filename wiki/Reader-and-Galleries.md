# Reader & Galleries

The gallery detail page and the reader are where you actually consume content. Both use the
Rust backend for metadata and direct image loading with a Rust-side fallback.

## Detail page

**Route:** `/gallery/[id]`

- **Header:** cover, title (original English/pretty joined), artist(s), parody, language,
  category, page count, upload age, score.
- **Actions:**
  - **Open reader** — start paging through the gallery.
  - **Favorite / Unfavorite** — see [Favorites & History](Favorites-and-History.md).
  - **Open on nhentai.net** — external browser (system opener).
  - **Quick-add blacklist** — blacklist a tag or the artist right from the page
    (see [Blacklist](Blacklist.md)).
- **Tag clouds:** grouped by type (tag, artist, character, parody, group, language, category).
  Clicking a tag navigates to that tag's gallery list — as anywhere, **the [blacklist](Blacklist.md)
  is applied**.
- **Related galleries:** the site's "related" list renders as a compact grid.

## Reader

**Route:** `/gallery/[id]/reader`

Two modes, switchable in the reader toolbar:

| Mode | Behavior |
| --- | --- |
| **Paged** | One page at a time; arrow keys / click zones / toolbar change pages. |
| **Strip (long-strip)** | Continuous vertical scroll through all pages. |

Other reader features:

- **Thumbnail strip** — jump to any page.
- **Preload** — upcoming pages are prefetched so scrolling is smooth.
- **Fullscreen** — expand the reader to fill the window.
- **Progress** remembered — the reader resumes where you left off (local persistence; part of
  history tracking).
- **Image fallback** — each page tries the CDN directly; on a 404/failure it re-fetches via
  the Rust `proxy_image` command and renders a `blob:` URL. Legacy galleries whose CDN re-encoded
  images (a known quirk) are handled gracefully with a placeholder + retry.

### Reader data

- Page URLs are built from gallery metadata: cover thumb at `https://t.nhentai.net`,
  page images at `https://i.nhentai.net`.
- Both hosts are always loadable because of the [CSP](../README.md) allow-list
  (`img-src 'self' data: blob: https://t.nhentai.net https://i.nhentai.net`).

## Image loading pipeline

1. **Direct load** — `<img src="https://t.nhentai.net/...">` or `https://i.nhentai.net/...`.
2. **Proxy fallback** — `CoverImage`/`ReaderImage` (`src/lib/image.ts`) catch load errors and
   call `backend.proxyImage(url)` → Rust `proxy_image` command → returns bytes → `Blob` URL.
3. **Placeholder** — a quiet placeholder keeps layout stable while a card/reader image loads.

## Related

- [Architecture](Architecture.md) — where these pieces live
- [Favorites & History](Favorites-and-History.md) — opening a gallery records history