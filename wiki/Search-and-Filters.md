# Search & Filters

The search page is the heart of nhentai.desktop. It compiles your structured filters and raw
query text into **native nhentai.net search syntax** and runs it through the site's API —
server-side filtering (plus your [blacklist](Blacklist.md) on top).

## The search box

Type anything nhentai's own search understands. The full syntax you can use here:

| Pattern | Meaning |
| --- | --- |
| `blue archive` | Free-text search (titles, tags) |
| `-lolicon` | Exclude all-tags matching `lolicon` |
| `tag:lolicon` | Include the `lolicon` tag explicitly |
| `artist:hana` | Only galleries by artist `hana` |
| `character:yuki` / `parody:fate` / `group:circle` / `language:japanese` / `category:doujinshi` | Type-scoped filters |
| `pages:>100` / `pages:<50` | Page-count bounds |
| `uploaded:2025` | Date-ish filter (site-supported syntax) |
| `sort:popular-today` | Sort order (also available in the drawer) |

Rules nhentai enforces (you cannot bypass them through the UI, we inherit them):

- Excluded terms use `-`.
- Multiple tokens are implicitly AND.
- There is no cross-token OR in the public search — block queries overlap instead.

## The filter drawer (structured filters)

Click **Filters** (top-right of the Search page) to open a side drawer with structured
controls. The URI built from the drawer is **the same query language** as the box above —
they compose:

- **Query** — the free-text query.
- **Included tags** — per-type (artist, character, parody, group, tag, …) — added as
  `type:name` tokens.
- **Excluded tags** — per-type — added as `-type:name` tokens.
- **Language** — e.g. `language:english`.
- **Category** — e.g. `category:doujinshi`.
- **Page count** — `pages:>N` and/or `pages:<M`.
- **Sort** — `date`, `popular-today`, `popular-week`, `popular-month`, `popular` (all time).
  Nonexistent: a stable "total popularity" sort (the site only exposes recency).

The current composed query is shown as a pill next to the search title, so you always know
exactly what was sent.

## How your blacklist is combined

Every search automatically appends your global blacklist tag-exclusions
(`-tag:...`, etc.) to the query via `buildServerExcludes()`. So **Search results never show
blacklisted tags even if the site would return them.** Your blacklist is built in
[Blacklist](Blacklist.md).

## Pagination

Results paginate via the site's page model. The **Pager** at the bottom gives Previous / Next
plus page numbers. Page transitions re-issue the query with `page=N`.

## Behavior details

- Typing in the search box does **not** auto-search; press **Search** (or Enter). The drawer
  applies instantly.
- The URL shows `?q=...`; the app re-runs the current query if you navigate with a query
  present.
- **Cancellation:** an in-flight search is cancelled when you change the query — the app never
  shows a stale result for a fast retype.
- Loading spinner and an inline error banner (with retry) handle the network path.

## Query builder source-of-truth

The exact query-string compilation lives in `src/lib/query.ts`
(`buildQuery`, `tagQueryPart`, `SORT_OPTIONS`). The backend search command is
`search_galleries` in `src-tauri/src/commands.rs`.

---

- Previous: [Getting Started](Getting-Started.md) · Next: [Blacklist](Blacklist.md)