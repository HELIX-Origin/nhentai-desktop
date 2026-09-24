# Rule: Frontend (Svelte 5 · SvelteKit SPA · TypeScript)

## Stack notes

- SvelteKit in **SPA mode**: `adapter-static` with `fallback: "index.html"`,
  `export const ssr = false` in `+layout.ts`. No server routes, no SSR.
- Svelte **5 runes**. Use `$state`, `$derived`, `$effect`, `$props`. Avoid legacy
  reactivity (`let x = $state` everywhere; no `onMount` when `$effect` + runes suffice).
- TypeScript `strict`. No `any` unless unavoidable and then justify in code review.

## Structure

```
src/lib/
  api/          # types.ts (nhentai wire types) + client.ts (invoke wrapper)
  components/   # UI components (kebab-case files, PascalCase names)
  stores/       # runes-based state; one module per domain
  util/         # pure helpers (formatting, query building) — unit-testable
```

- Views are SvelteKit routes under `src/routes/`.
- `$lib/...` alias is available; relative imports only within a feature folder when it
  helps readability. Prefer `$lib/` from route/component files.

## State & data flow

- All app state lives in `src/lib/stores/` as exported runes or small classes of runes:
  `favorites`, `history`, `blacklist`, `settings`.
- Persist via `localStorage` through a tiny helper in `src/lib/util/`; write-through on
  mutation, hydrate on module load. Version the storage keys (`lewdclips:favorites:v1`).
- Components receive props downward; do not import stores deep inside leaf components
  unless the store *is* the natural contract (e.g. a `FavoritesButton`).
- Data fetching happens in views/route modules, never inside presentational components.

## Styling

- **Plain CSS** with design tokens as CSS custom properties (see `src/lib/styles/tokens.css`).
  No framework (no Tailwind), no CSS-in-JS.
- Semantic tokens (`--color-bg`, `--color-surface`, `--color-accent`, `--radius-md`,
  `--space-4`, `--fs-sm` …) — components reference tokens, never raw hex inside a
  component style block, excepting one-off component accents that belong to its identity.
- Dark + light via `[data-theme]` on `<html>`; default follows `prefers-color-scheme`.
- Scoped styles per component with BEM-lite class names (`gallery-card__title`).

## Accessibility

- Native controls first (`<button>`, `<input>`, `<select>`), real labels, visible focus
  rings via tokens, `prefers-reduced-motion` honored.
- The filter bar and reader must be fully keyboard-operable.

## Verification (frontend)

- Run `npm run check` (svelte-kit sync + svelte-check). Zero errors.
- Pure logic (query building, blacklist matching) lives in `src/lib/util/` so it can be
  tested in isolation; add tests where a regression would be painful and a test framework
  is present — otherwise keep the logic *so* pure it's trivially auditable.
- Validate the non-happy paths: empty results, API error, image 404, offline.