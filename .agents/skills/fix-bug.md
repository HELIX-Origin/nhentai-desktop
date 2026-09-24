# Skill: Fix a Bug

Trigger: a `BUGS.md` entry (or a report). Goal: recording, fix, regression coverage, tidy
paper trail.

## Workflow

```mermaid
flowchart TD
    A[Read BUGS entry / repro] --> B[Confirm reproduction path]
    B --> C{Existing entry?}
    C -- no --> D[Create BUG entry + tracking file]
    C -- yes --> E[Set investigating]
    D --> E
    E --> F[Root-cause: read code + fixtures]
    F --> G[Fix smallest surface, follow rules]
    G --> H[Add regression check for pure logic]
    H --> I[Run checks]
    I --> J[Reviewer pass]
    J --> K{Pass?}
    K -- no --> G
    K -- yes --> L[Update BUG status: fixed]
    L --> M[Close tracking file]
    M --> N[Report root cause + fix + proof]
```

## Sequence (with docs)

```mermaid
sequenceDiagram
    participant T as Triager
    participant E as Engineer
    participant R as Reviewer
    participant D as BUGS.md

    T->>D: open entry (repro, area, severity)
    T->>E: route fix
    E->>E: root cause + patch + regression test
    E->>T: patch + checks result
    R->>E: review pass (APPROVE / CHANGES)
    E->>D: status fixed + root cause summary
    E->>T: close out
```

## Rules

- **Reproduce before fixing.** If you can't reproduce, record environment/steps and stop
  rather than blind-patching.
- **Smallest surface.** One bug, one patch. Don't refactor adjacent code in the same diff.
- **Regression coverage** wherever the logic is pure (query builder, blacklist matcher,
  storage migration) — this is the repo's permanent invariant.
- A fix must also answer "why did the tests/checks not catch this" and close that gap.
- Update `BUGS.md` honestly: root cause (not speculation), verification evidence.