# Rule: Security & API Etiquette

## 🔑 Secrets & credentials

- No secrets anywhere in the repo: no API keys, tokens, passwords, or personal config in
  code, docs, or commit history. This app is secret-free by design — keep it that way.
- Code that would *need* a secret is a design smell here; stop and ask.
- `.env` / local config is never committed. If added later, ensure `.gitignore` covers it.

## 🔒 The webview's attack surface

- **No arbitrary remote `fetch` from the frontend.** All nhentai API traffic goes through
  Rust commands. The frontend only ever talks to Tauri (`invoke`).
- **Images are remote and untrusted.** We load bytes and render them in `<img>`; we never
  `innerHTML` remote content, never execute remote scripts, never `eval`.
- **CSP:** keep it strict in `tauri.conf.json`. Allow only what the app needs:
  `default-src 'self'`, `img-src 'self' data: blob: https://nhentai.net https://*.nhentai.net`
  (covers `t.nhentai.net`, `i.nhentai.net`, and `static.nhentai.net` avatars),
  `style-src 'self' 'unsafe-inline'` (Svelte injects scoped styles in dev), script stays
  `'self'` (Tauri injects its own bootstrap). Re-verify after any CSP change.
- Window config: `navigate`/`open` always opens external links in the system browser via
  `tauri-plugin-opener` — never embed an untrusted webview.
- Sanitize only what we render ourselves (titles/tags from the API pass through Svelte
  text interpolation — safe by default; never `{@html trusted}` with API data unless a
  task explicitly sanitizes first).

## 🤝 nhentai API etiquette

- We are guests of a public API. Rules:
  1. Throttle: ≥250 ms between API calls (shared, backend-enforced).
  2. Dedupe: identical in-flight requests collapse into one network call.
  3. No scraping loops, no parallel page pre-fetch, no crawling the *entire* catalog in
     bulk. When e.g. a future "download library" feature needs bulk, add pacing + user
     confirmation.
  4. Cache aggressively on disk/memory where legal (metadata is public) to reduce hits.
- If a user action would generate heavy traffic (e.g. downloading a series of galleries),
  surface it and pace it. Never "emptied the catalog" silently.

## 🔗 External links & downloads

- Any URL opened externally goes through `tauri-plugin-opener` (system browser). Nothing
  opens inside the app window except first-party `localhost`/`tauri://` content.
- Future downloads (CBZ) write only to user-chosen paths via a native dialog; never guess
  or hardcode write locations.