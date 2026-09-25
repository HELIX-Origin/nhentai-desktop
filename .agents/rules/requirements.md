# Rule: Requirements & Functional Understanding

Applies to every agent that needs to know **what the product does, why, and what a change
must satisfy** before acting. This is the anti-guessing rule: requirements are sourced and
recorded, never invented.

## 🎯 Where understanding comes from

Consult in this order — higher wins on conflict:

| Source | Holds | Authority |
| --- | --- | --- |
| `AGENTS.md` decision log | settled facts & rationale | highest |
| `.agents/README.md` | ecosystem map, routing | high |
| `.agents/rules/*` | standing conventions | high |
| `ROADMAP.md` | product direction & milestones | high |
| `requirements` docs (`.agents/tracking/requirements/*` or wiki) | written requirements | medium |
| `TODO.md` / `BUGS.md` | current work & known issues | medium |
| source code + fixtures | ground truth of behavior | verifiable fact |

## 📝 What a requirement is

A requirement is a written, traceable statement of **problem → behavior → constraints →
verification**. It answers "what & why", never "how" (the engineer owns how).

- **Problem** — the user/product-facing need.
- **Behavior** — observable, testable effects.
- **Constraints** — identifiers, stack locks, performance/etiquette limits (nhentai API
  respect), non-goals.
- **Verification** — how we prove it's met (a check, a test, a manual step).

## 🚫 Anti-guessing rules

- **Never invent facts.** If `AGENTS.md`/rules/docs conflict with your assumption, the docs
  win — and if a doc is stale, flag it rather than silently improvising.
- **Never invent requirements.** Ambiguous ask → ask the user; never half-guess scope.
- **Record, then build.** A requirement for a small feature may live as one block in a
  `templates/feature.md` doc; a big one gets a full `templates/requirement.md` in
  `.agents/tracking/requirements/`. Either way it's written before the build.
- **Traceability.** Every requirement traces to milestone + TODO id(s); every TODO item
  traces back to a requirement, milestone, or backlog tag.
- **Identity discipline.** Product/project names come from `rules/project-identity.md`.
  `NH Desktop` is the user-facing name; `nh-desktop`/`nh_desktop_lib` are identifiers;
  `nhentai.net` (and `t./i.nhentai.net`) are the *website* — never rebrand them.

## 🔄 Who does what

- **project-context primary agent** — the canonical map; routes questions.
- **requirements sub-agent** (`agents/project-context/requirements/`) — authors requirement
  docs from `templates/requirement.md`, records user corrections verbatim.
- **planner / roadmap sub-agent** — decomposes requirements into verified TODO items.
- **engineer sub-agents** — implement, using the requirement doc as the contract.

## ✅ Definition of done (any task)

- The requirement that drove the work is written and traceable.
- The change satisfies the requirement's verification plan (checks green, tracking docs
  updated in the same change).
- No invented requirement, name, or project fact in the diff.

## Related

- `rules/project-identity.md` — identity & identifier source of truth.
- `rules/agent-ecosystem.md` — how agents own these docs.
- `skills/understand-project.md` — the ramp-up routine for new contexts.