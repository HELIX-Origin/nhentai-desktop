# Agent — <name>

<!-- Copy for each new primary or sub-agent. Primary = top-level domain folder under
     agents/. Sub-agent = folder under an existing primary. Start with the add-agent skill. -->

**Kind:** primary · sub-agent
**Parent (sub only):** <primary>
**Location:** `.agents/agents/<parent>/<name>/README.md` (or `<name>/README.md` for primary)

## 👤 Identity

```yaml
name: <kebab-case>
role: <one-line role>
parent: <primary, if sub-agent>
reads: <docs + rules it must read>
writes: <what it writes to source/docs>
verifies: <checks it runs>
```

## 🎯 Responsibility

<!-- 3–6 bullets: what the agent owns. Reference shared rules/skills by name; never
     duplicate their content here. -->

## 🔄 Operating loop

<!-- A small mermaid (≤10 nodes, TD) or a short numbered list: how this agent executes. -->

## ⚠️ Rules of the role

<!-- Non-negotiables. Point to rules/*.md where they live. -->

## ✅ Definition of done

<!-- What must be true before this agent's work is "done". -->

## 💡 Notes

<!-- Where it delegates, which sub-agents it spawns / belongs under, which shared assets
     (skills/*, templates/*) it reuses. -->