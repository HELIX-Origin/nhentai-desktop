# Rule: Frontend (Svelte 5 · SvelteKit SPA · TypeScript)

## 🏗️ Stack notes

- SvelteKit in **SPA mode**: `adapter-static` with `fallback: "index.html"`,
  `export const ssr = false` in `+layout.ts`. No server routes, no SSR.
- Svelte **5 runes**. Use `$state`, `$derived`, `$effect`, `$props`. Avoid legacy
  reactivity (`let x = $state` everywhere; no `onMount` when `$effect` + runes suffice).
- TypeScript `strict`. No `any` unless unavoidable and then justify in code review.

## 🏗️ Structure

```
src/lib/
  api.ts        # nhentai API wrappers (typed invoke)
  client.ts     # typed invoke wrapper
  types.ts      # nhentai wire types + frontend shapes
  query.ts      # search query builder
  image.ts      # image URL derivation + host allow-list
  format.ts     # formatting helpers
  cache.ts      # SQLite-backed KV cache interface (prefix `nh-desktop:`)
  components/   # UI components (PascalCase files, PascalCase names)
  design/       # CSS tokens and base styles
  stores/       # runes-based state; one module per domain
```

- Views are SvelteKit routes under `src/routes/`.
- `$lib/...` alias is available; relative imports only within a feature folder when it
  helps readability. Prefer `$lib/` from route/component files.

## 🔄 State & data flow

- All app state lives in `src/lib/stores/` as exported runes or small classes of runes:
  `favorites`, `history`, `blacklist`, `settings`, `account`, `service`.
- Persist via `src/lib/cache.ts` → Tauri `db_*` commands → SQLite `kv` table in
  `nh-desktop.db`; write-through on mutation, hydrate via `cacheInit()`. Cache keys use the
  prefix `nh-desktop:`.
- Components receive props downward; do not import stores deep inside leaf components
  unless the store *is* the natural contract (e.g. a `FavoritesButton`).
- Data fetching happens in views/route modules, never inside presentational components.

## 🎨 Styling

- **Plain CSS** with design tokens as CSS custom properties (see `src/lib/design/tokens.css`
  and `src/lib/design/base.css`). No framework (no Tailwind), no CSS-in-JS.
- Semantic tokens (`--bg`, `--surface`, `--accent`, `--radius`, `--radius-sm`, `--text` …)
  — components reference tokens, never raw hex inside a component style block, excepting
  one-off component accents that belong to its identity.
- Dark + light via `[data-theme]` on `<html>`; default follows `prefers-color-scheme`.
- Scoped styles per component with BEM-lite class names (`gallery-card__title`).

## ♿ Accessibility

- Native controls first (`<button>`, `<input>`, `<select>`), real labels, visible focus
  rings via tokens, `prefers-reduced-motion` honored.
- The filter bar and reader must be fully keyboard-operable.

## ✅ Verification (frontend)

- Run `npm run check` (svelte-kit sync + svelte-check). Zero errors.
- Pure logic (query building in `src/lib/query.ts`, formatting in `src/lib/format.ts`,
  image URL derivation in `src/lib/image.ts`) lives in flat `src/lib/` modules so it can be
  audited in isolation; add tests where a regression would be painful and a test framework
  is present — otherwise keep the logic *so* pure it's trivially auditable.
- Validate the non-happy paths: empty results, API error, image 404, offline.