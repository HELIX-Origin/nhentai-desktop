# Skill: Add or Restructure an Agent

Trigger: the user asks to add a new agent/sub-agent or reorganize the ecosystem. Goal: extend
`.agents/agents/` correctly per `rules/agent-ecosystem.md` with zero duplication and no
dangling cross-references.

## Workflow

```mermaid
flowchart TD
    A[User asks for an agent] --> B{Primary or sub?}
    B -- sub --> C[Pick parent primary]
    B -- primary --> D[New top-level domain folder]
    C --> E[Write agents/{parent}/{name}/README.md<br/>from templates/agent.md]
    D --> F[Write agents/{name}/README.md<br/>from templates/agent.md]
    E --> G[Wire into parent README sub-list]
    F --> G2[Wire into .agents/README.md roles table]
    G --> H[Update routing table in .agents/README.md]
    G2 --> H
    H --> I[Greppable verify: no old refs, all new refs resolve]
    I --> J[Report structure + files touched]
```

## Steps

1. **Decide scope.** Default: a *sub-agent* under an existing primary. A new *primary*
   is justified only by a genuinely new top-level domain (functionality vs delivery vs
   quality vs intake — see the current primaries).
2. **Read the skeleton:** `templates/agent.md`, then mimic an existing sibling
   (e.g. `engineer/backend/README.md`).
3. **Fill identity:** `name` (kebab-case), `role`, `parent`, `reads`, `writes`, `verifies`.
4. **Share, don't duplicate.** Reference shared `rules/*` and `skills/*` by name; never
   copy their content into the agent file, and never inline a convention that belongs in
   `rules/`.
5. **Wire it in (same change):**
   - Sub-agent → list it in the parent primary's README.md.
   - Primary → add a row to the roles table in `.agents/README.md`.
   - Routing → add/update "Read first" row in `.agents/README.md`.
6. **Verify:** grep for the agent name across `.agents/` and `AGENTS.md`; confirm every
   reference resolves to a real path and no old path lingers.

## Checklist

| Gate | Pass |
| --- | --- |
| Primary vs sub correct? | no new fold for an existing domain |
| File at `agents/{primary}/{name}/README.md` | ✓ |
| Modeled on `templates/agent.md` | ✓ |
| Reads/writes/verifies realistic | ✓ |
| Reuses shared rules/skills (no copies) | ✓ |
| Wired into parent README + gateway routing | ✓ |
| Grep: no dangling refs | ✓ |

## Definition of done

- Structure matches `{primary}/{sub}/README.md`; indexes are `README.md`.
- The new agent's identity file is the *only* new file (everything else is an edit to
  existing refs). No duplicated skills/rules/templates exist.
- `AGENTS.md`-reachable references all resolve.