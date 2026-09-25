# Reader & Galleries

The gallery detail page and the reader are where you actually consume content. Both use the
Rust backend for metadata and direct image loading with a Rust-side fallback.

## 🖼️ Detail page

**Route:** `/gallery/[id]`

- 🖼️ **Header:** cover, title (original English/pretty joined), artist(s), parody, language,
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

## 🖼️ Reader

**Route:** `/gallery/[id]/reader`

One page at a time; arrow keys, click zones (page back when clicking in the left ~28%, forward
in the right ~72%), and the thumbnail strip change pages. The toolbar's **fit mode** cycles how
a single page is sized:

| Fit mode | Behavior |
| --- | --- |
| **Width** | Page fills the stage width; scroll vertically (stage overflow) when taller than the stage. |
| **Height** | Page fills the stage height; scroll horizontally when wider than the stage. |
| **Contain** | Whole page fit inside both axes (letterboxed; nothing is cropped). |

Other reader features:

- 🖼️ **Thumbnail strip** — jump to any page.
- **Preload** — upcoming pages are prefetched so scrolling is smooth.
- **Fullscreen** — expand the reader to fill the window.
- **Progress** remembered — the reader resumes where you left off (local persistence; part of
  history tracking).
- **Image fallback** — each page tries the CDN directly; on a 404/failure it re-fetches via
  the Rust `proxy_image` command (served from the disk image cache `cache/images` when present)
  and renders a `blob:` URL. Legacy galleries whose CDN re-encoded images (a known quirk) are
  handled gracefully with a placeholder + retry.

### 🖼️ Reader data

- 🖼️ Image URLs are derived from the API's relative path fragments — the API returns paths
  like `galleries/<id>/thumb.webp` *without* a leading slash, and `image.ts` joins them onto
  the right host (`https://i.nhentai.net` pages, `https://t.nhentai.net` thumbs,
  `https://static.nhentai.net` avatars).
- Hosts are always loadable because of the [CSP](../README.md) allow-list
  (`img-src 'self' data: blob: https://nhentai.net https://*.nhentai.net`).

## 🖼️ Image loading pipeline

Each image follows a three-hop fallback chain:

```mermaid
flowchart TD
    A[Direct CDN load] -->|"success"| B[Render image]
    A -->|"404 or error"| C[proxy_image fallback]
    C -->|"bytes"| D[Render Blob URL]
    C -->|"fails too"| E[Placeholder + retry]
```

1. 🖼️ **Direct load** — `<img src="https://t.nhentai.net/...">` or `https://i.nhentai.net/...`.
2. **Proxy fallback** — `CoverImage`/`ReaderImage` (`src/lib/image.ts`) catch load errors and
   call `backend.proxyImage(url)` → Rust `proxy_image` command → bytes (served from the disk
   image cache first, else fetched and cached) → `Blob` URL.
3. **Placeholder** — a quiet placeholder keeps layout stable while a card/reader image loads.

## 🤝 Related

- [Architecture](Architecture.md) — where these pieces live
- [Favorites & History](Favorites-and-History.md) — opening a gallery records history