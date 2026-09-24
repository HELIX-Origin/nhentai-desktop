# Rule: Testing & "Done" Definition

## Make it green

Before any task counts as done:

| Change side | Required |
| --- | --- |
| Any frontend change | `npm run check` — zero errors |
| Any Rust change | `cargo check` and `cargo test` in `src-tauri/` |

The user may run `npm run tauri dev` for interactive smoke tests; agents cannot rely on a
running GUI, so signal correctness through checks + reasoning, and state explicitly what
was verified via automated checks vs. not manually smoked.

## What to actually test

- **Pure logic = unit-testable.** Query building (filter model → nhentai search string),
  image-URL mapping, blacklist matching, storage serialization. Keep these in free
  functions/modules with no I/O so tests are trivial (Rust `#[cfg(test)]` modules, or a
  Tinyest/Vitest-style test in frontend if introduced).
- **Wire shapes.** Add Rust tests asserting deserialized JSON fixtures match the types
  (guards against silent breaks when nhentai shifts fields).
- **Error paths.** Every command returns `Result`; verify the failure variants surface a
  friendly message and the UI's error state, not a crash.

## Coverage bar (pragmatic)

- No blanket coverage targets. Prioritize: query building, blacklist matching, image URL
  mapping, storage migration edge cases.
- A bug fix ships with (a) the fix, (b) a regression check that'd have caught it (unit
  test whenever the logic is pure), and (c) a `BUGS.md` status update.

## Test hygiene

- Tests must be deterministic and offline (fixtures, no live nhentai).
- Keep `npm run check` fast; it runs on every frontend change. Rust `cargo test` runs
  only when Rust changed.
- Tests live next to code: Rust `#[cfg(test)] mod tests` in the same module file;
  frontend `.test.ts` beside the util it covers.

## When "done" is not done

- `npm run check` errors → not done.
- A changed behavior with no `TODO.md`/`BUGS.md`/`ROADMAP.md` update → not done.
- Types that don't round-trip against a real API fixture → not done.
- Anything that could panic across the command boundary → not done.