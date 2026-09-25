# Rule: Backend (Rust · Tauri 2)

## 🏗️ Module layout (`src-tauri/src/`)

```
main.rs        # bin entry, #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
lib.rs         # Builder::default() → plugins → invoke_handler → run()
nh_desktop.rs  # nhentai API wire types (serde) + client functions (reqwest): get_gallery,
               #   search, list_new, list_all, list_tagged, fetch_bytes
commands.rs    # #[tauri::command] wrappers — thin, typed, call client, no business logic
error.rs       # AppError enum -> String messages; no panics across the command boundary
```

## 🦀 Conventions

- snake_case functions/fields; `#[derive(Serialize, Deserialize)]` for wire types.
- `reqwest::Client` is constructed once (with a browser-like `User-Agent` and a sane
  timeout) and shared via `tauri::State`/managed state.
- **Errors:** commands return `Result<T, String>` (Tauri-friendly). `AppError` implements
  `Display`; map HTTP/network failures to friendly user text in `commands.rs`.
- No `unwrap()`/`expect()`/`panic!()` on any path reachable from a command. Parse with
  `?` / `ok_or`; log via `eprintln!`/`log` where it aids debugging.
- Keep requests to nhentai polite: a shared throttle (e.g. a `Mutex<(Instant,())>`
  enforcing ≥250 ms between API calls) plus in-flight dedupe (single-flight) for identical
  recent requests. Never fire-and-forget retry loops.

## ⚙️ Tauri specifics

- All app capabilities/permissions live in `src-tauri/capabilities/default.json`
  (core + what plugins we actually use). Adding a plugin requires adding its permission.
- The webview's CSP is defined in `tauri.conf.json` `app.security.csp`. Tighten, don't
  loosen: API access goes through `invoke`, so the CSP needs no `connect-src` to the
  nhentai API; images need `img-src https://t.nhentai.net https://i.nhentai.net data:`.
- `proxy_image` returns bytes (frontend builds a Blob URL) — keep an in-memory cap and
  cache in-flight lookups so the reader doesn't duplicate fetches.

## ✅ Verification (backend)

- `cargo check` clean; `cargo test` for pure logic (URL builders, query serialization,
  type mapping). Keep pure logic in free functions that don't touch a runtime so tests
  run fast and headless.
- Manual smoke: `npm run tauri dev`, hit search/detail from the UI, verify throttling and
  error surfacing.