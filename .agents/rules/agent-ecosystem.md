# Rule: Agent Ecosystem Structure

Applies to every file under `.agents/`. This rule defines how agents, sub-agents, skills,
rules, and templates are **organized, shared, and extended** — with zero duplication.

## 🏗️ Structure

```
.agents/
  README.md            # gateway; the ONLY index (no INDEX.md). Index files are README.md
  rules/               # standing conventions (flat; shared by all agents)
  agents/              # {primary-agent}/{sub-agent}/README.md
  skills/              # reusable workflows (flat; shared, never duplicated per agent)
  templates/           # fill-in-the-blank docs (flat; shared)
  tracking/            # per-item detail files (bugs, todos, requirements)
```

- **Primaries** are folders under `agents/`: `project-context`, `planner`, `engineer`,
  `reviewer`, `triager`. Each primary's README.md defines the agent and lists its sub-agents.
- **Sub-agents** are folders under a primary: e.g. `engineer/backend/README.md`. Path form:
  `agents/{primary-agent}/{sub-agent}/README.md`.
- **Index rule:** every index file is named `README.md` so GitHub renders it online. There
  is no `INDEX.md` anywhere in this tree.

## 🔄 Sharing (no duplication)

| Asset | Home | Rule |
| --- | --- | --- |
| Standing conventions | `rules/*.md` | One definition global; agents *read* them |
| Workflows | `skills/*.md` | One workflow global; agents *invoke* them |
| Fill-in docs | `templates/*.md` | One template global; agents *copy* them |
| Role definitions | `agents/*/README.md` | Owned by that agent; sub-agents reuse parent's skills/rules |

- **Never duplicate** a skill, rule, or template into an agent folder. If two agents need
  the same routine, it belongs in `skills/` (or `rules/` if it's a convention), and both
  reference it.
- Sub-agents reuse the parent's `reads:`/`writes:` and the shared rule set rather than
  restating them.
- A behavior that's a *judgement* goes in `rules/`; a behavior that's a *procedure* goes in
  `skills/`. When in doubt: judge → rule, procedure → skill.

## ➕ Adding/editing an agent

1. Read `skills/add-agent.md` (the procedure) and `templates/agent.md` (the skeleton).
2. Decide: is this a **primary** (new top-level domain) or a **sub-agent** (belongs under
   an existing primary)? Default: sub-agent.
3. Create `agents/{primary}/{name}/README.md` modeled on `templates/agent.md`.
4. Wire it in: update the primary's README (sub-agent list) and the routing table in
   `.agents/README.md` — in the same change.
5. Name files `kebab-case.md`; sub-agent owns only its identity file, not shared assets.

## ⚠️ Governance

- **Agents never mutate policy.** Rules/agent roles/skills are changed only when the *user*
  asks; implementer agents follow them as written (`.agents/README.md` principle 4).
- Keep files dense and scannable (tables before prose, mermaid ≤10 nodes, emoji per
  `rules/docs-format.md`).
- Any restructure that moves or renames files must update cross-references in the same
  change (gateway README, parent README, `AGENTS.md` if it links into this tree).

## ✅ Definition of done (an ecosystem change)

- Structure matches `{primary}/{sub}/README.md`; all indexes are `README.md`.
- No dangling cross-references (greppable); no duplicated skills/rules/templates.
- `AGENTS.md` references still resolve (`README.md`, not `INDEX.md`).

## Related

- `.agents/README.md` — the gateway.
- `skills/add-agent.md` — the procedure for adding agents.
- `templates/agent.md` — the skeleton for a new (sub-)agent.