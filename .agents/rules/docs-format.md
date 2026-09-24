# Rule: Documentation Format (readability-first)

Applies to every `.md` file in the repo: root docs, `wiki/`, `.agents/`.

> **Why this exists.** The user has significant eyesight limitation. Emoji headings and
> **legible mermaid diagrams** are accessibility features here, not decoration. Every added
> emoji or diagram must make a document *easier to scan at a glance*. If it doesn't help, it
> doesn't belong.

## 1. Emoji conventions

Use emoji as **visual anchors**: section headings, status markers, and key-list bullets only.
Never bulk-decorate prose, code fences, or every bullet in a paragraph.

- **Headings:** prefix `##`/`###` headings with one relevant emoji. `#` (H1) keeps its plain
  title (the page title is already the anchor). Reuse the same emoji for the same concept
  across the whole repo so users learn the map fast.
- **Recommended map** (use consistently):
  - 👋 Intro / what-is · 🚀 Quick start · 🏁 First run
  - 📦 Install / uninstall / platform
  - 🔍 Search & filters · 🚫 Blacklist
  - ⭐ Favorites · 🕑 History
  - 🖼️ Reader & galleries (images)
  - ⚙️ Settings · 🔑 API key · 💾 Persistence / storage
  - 🏗️ Architecture · 🦀 Backend · ⚡ Frontend · 🧩 Components
  - 🛠️ Maintain / repair · 🔧 Troubleshoot · ❓ FAQ
  - 🔒 Security · 🕵️ Privacy · 🗺️ Roadmap · 🤝 Contribute
  - ✅ Done/shipped · 🚧 In progress · ⬜ Planned/backlog · ⚠️ Warning/caution
  - 💡 Tip/note · 🚨 Error/attention
- **Status tables** (milestones, platform support, checklists): put ✅ / 🚧 / ⬜ in the status
  cell.
- **Lists:** emoji the *first bullet* of a semantic group to mark the group. Do **not** prefix
  every single bullet — that becomes visual noise again.
- **Language rule:** keep the emoji to a maximum of ONE per heading and ONE per bullet. Two or
  more makes scanability worse, not better.

## 2. Mermaid diagram conventions

GitHub renders mermaid from fenced blocks. This is a Tauri/Svelte repo — no mermaid CLI in CI,
so every diagram must be **correct by construction** and **legible when rendered at GitHub's
default scale**.

### Legibility (the hard requirement)

GitHub renders wide diagrams at full width and they get **shrunken down** — a user with poor
eyesight cannot read a shrunken diagram. Rules that keep them big enough to read:

- **Max ~8–10 nodes per diagram.** If a flow needs more, split it into two linked diagrams
  ("Part 1: … / Part 2: …") or compress detail into node text.
- **Prefer `flowchart TD` (top-down)** over `LR`. Top-down flows give each node more
  horizontal room, so labels render large. Reserve `LR` for genuinely horizontal, *short*
  chains (≤5 nodes).
- **Short labels.** Keep node text ≤ ~50 characters. Put the rest in prose below the diagram.
- **No subgraphs** unless they genuinely group states (they add nesting height and
  horizontal shrink). Favor flat diagrams.
- **One idea per diagram.** A diagram that shows 3 different things fails for everyone.

### Correctness (GitHub will render it or it silently errors)

- Always use the exact fence: a code block with language `mermaid`.
- Use standard flow shapes:
  - `A[text]` — process/box
  - `A{text}` — decision/diamond
  - `A==>` — thick arrow for a critical path
  - `A-.->B` — dotted for async/deferred
- Node IDs: `A`, `B`, `C` or short `case` names. No spaces in IDs.
- Edge labels: `A -->|"text"| B` or `A --> B`. Short labels only.
- Start/end nodes optional but helpful: `_start([Start])`.
- No markdown inside labels. No backticks. Escape `()` and `[]` only when required — prefer
  avoiding them in node text wholesale.
- Quote anything with special characters: `A["text (v1)"]`.

### Where diagrams are actually useful (don't force them)

- Architecture/flow: yes (component graph, request lifecycle).
- Installer/upgrade flow: yes.
- Search query build / blacklist pipeline: yes.
- Troubleshooting decision trees: yes (a `flowchart TD` with yes/no edges).
- Lists of facts, tables, policy prose (TOS/PRIVACY): **no diagram**.

## 3. Balance with existing style

- Root docs keep `UPPER.md` names; `wiki/` keeps Title-Case page names; `.agents/` stays as is.
- Do not reflow prose into emoji spam. Readability first: bigger, simpler, scanable.
- A heading pair like `## 🔍 Search` reads as `## 🔍 Search` — one emoji, space, title text.