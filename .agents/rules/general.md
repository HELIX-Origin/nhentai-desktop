# Rule: General Conventions

Applies to every file and every agent, everywhere.

> Identity rule first: the workspace folder name is NOT the product name. This project is
> the **nhentai** desktop client; never build for "LewdClips"/"LewdZone" — see
> `rules/project-identity.md` before naming anything.

## Code

- **No explanatory comments.** Do not add comments to code unless the user asks. Prefer
  names that carry the intent (`strip_protocol_from_url` > `// remove protocol`).
  Exception: `// SAFETY:`-style notes for unsafe/unusual constructs are allowed.
- **Comments already in the codebase** (e.g. license headers, `DO NOT REMOVE!!` dirties)
  are not deleted casually; they were placed for a reason.
- Keep functions small and purposeful. If a function does two things, split it.
- Match surrounding style. When touching a module, stay inside its conventions.

## Files & naming

| Kind | Convention | Example |
| --- | --- | --- |
| Svelte component files | kebab-case `.svelte` | `gallery-card.svelte` |
| Svelte component names | PascalCase | `GalleryCard` |
| TS/JS modules | kebab-case | `nhentai-client.ts` |
| Rust files | snake_case | `nhentai.rs`, `commands.rs`, `error.rs` |
| Docs | `UPPER.md` for root, Title Case in `.agents/` | `BUGS.md`, `git-workflow.md` |

## Scope & honesty

- Finish what you start in a change: no half-wired features, no dead imports, no
  commented-out code left behind.
- **YAGNI.** Build for the current milestone; note future needs in `TODO.md`, don't build them.
- When a change affects product scope, status, or known issues, update `TODO.md`,
  `BUGS.md`, or `ROADMAP.md` in the **same change**.
- If unsure between options with real trade-offs, ask the user instead of guessing.

## Language

- All UI copy and docs are English unless a task says otherwise.
- Error messages shown to users are human-first ("Couldn't reach nhentai — try again in a
  moment"), while `TauriError` debug strings keep technical detail.

## Etiquette

- Never commit, push, or open PRs unless the user explicitly asks (see `git-workflow.md`).
- Never invent URLs, endpoints, credentials, or package versions. Verify before relying.
- Never leave secrets or local config in committed files.