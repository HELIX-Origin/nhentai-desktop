# Skill: Review Code

Trigger: a change is presented as done. Goal: verifiable "approved" outcome with evidence.

## Review passes (in order)

```mermaid
flowchart TD
    D[Diff] --> P1[Correctness]
    P1 --> P2[Rules conformance]
    P2 --> P3[Docs & diagrams]
    P3 --> P4[Edge cases]
    P4 --> P5[Security]
    P5 --> P6[Tracking docs]
    P6 --> V{Verdict}
    V -- APPROVE --> OUT[Pass + evidence list]
    V -- CHANGES --> RC[File:line findings]
    RC --> P1
```

| Pass | Questions |
| --- | --- |
| Correctness | Does it do what the task said? Are the types right? Could it panic? |
| Rules | Naming, runes vs legacy, tokens vs hard-coded color, snake_case, no invented deps |
| Docs/diagrams | Emoji per `docs-format.md`; every `mermaid` fence valid, ≤10 nodes, TD-default, legible at GitHub scale |
| Edge cases | Empty results, error path, refresh, rapid double-click, offline, 404 image |
| Security | CSP still tight? API only via invoke? No `{@html}` of remote data? No secrets? |
| Tracking | `TODO.md`/`BUGS.md`/`ROADMAP.md` reflect the change? |

## Sequence

```mermaid
sequenceDiagram
    participant E as Engineer
    participant R as Reviewer
    participant C as Checks (check / cargo test)

    E->>R: here's the diff + what I verified
    R->>C: run applicable checks
    C-->>R: results
    R->>E: APPROVE / CHANGES with file:line notes
    E->>R: fixes (loop as needed)
```

## Hard requirements on the verdict

- **Do not approve** if: checks error, pure logic lacks regression coverage, tracking docs
  are stale, or any remote/untrusted data is rendered unsafely.
- **Request changes** with `path:line` evidence for every finding; never vague.
- A reviewer approves *what was run*, and states what could still only be human-smoked
  (interactive flows).