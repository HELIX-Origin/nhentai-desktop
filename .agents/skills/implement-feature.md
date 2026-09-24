# Skill: Implement a Feature

Trigger: a `TODO.md` item (or user ask) describing a feature. Goal: ship it verified, with
tracking docs honest.

## Workflow

```mermaid
flowchart TD
    A[Read TODO item + skill] --> B[Read relevant rules]
    B --> C[Read existing code patterns]
    C --> D[Plan: files touched, contract]
    D --> E{Scope > item?}
    E -- yes --> F[Update TODO/ROADMAP first]
    E -- no --> G[Implement backend or frontend-first per plan]
    F --> G
    G --> H{Pure logic involved?}
    H -- yes --> I[Write unit tests for it]
    H -- no --> J[Skip tested-with-UI note]
    I --> K[Run checks: check / cargo test]
    J --> K
    K --> L[Review pass]
    L --> M{Pass?}
    M -- no --> G
    M -- yes --> N[Update TODO status]
    N --> O[Report: what, verified, left-to-smoke]
```

## Checklist

| Phase | Gate |
| --- | --- |
| Rules read | `rules/general.md` + the side's rule (`frontend`/`backend`) + `security.md` + `testing.md` |
| Existing patterns | Search for nearest equivalent before writing anything new |
| Contract | Find where frontend↔backend data crosses; types match exactly (`types.ts` ↔ serde) |
| Checks | `npm run check` (FE) · `cargo check`+`cargo test` (BE) |
| Tracking | `TODO.md` status + `ROADMAP.md` milestone if it completes one |
| Report | What changed · what was verified · what still needs a human smoke test |

## Frontend-first vs backend-first

```mermaid
flowchart LR
    F[Feature type?] --> S{Needs new command?}
    S -- yes --> P[Backend-first: command + types + tests]
    P --> A[Then FE view + client.ts]
    S -- no --> B[Frontend-first: util + stores + view]
    B --> C[Run check; wire if invoke signature changed]
    A --> D[End-to-end check]
```

## Definition of done (hard)

- Checks green; regression tests for any pure logic.
- Empty/error/loading states visible in the UI.
- Tracking docs updated in the same change.
- No comments added unless the task asked for them; no dead code; names self-document.