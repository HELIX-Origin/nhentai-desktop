# Rule: Context Management (DCP / compress)

Applies to every agent session that runs under **opencode with the DCP plugin**
(`@tarquinen/opencode-dcp`, AGPL-3.0, auto-installed globally). This is meta-infrastructure:
get it wrong and sessions degrade, lose intent, or waste the user's time. **Read before doing
anything context-related.**

## 🧠 What DCP does

- Renders a `compress` tool into the session. It replaces **closed**, stale spans of the
  conversation with high-fidelity summaries (never edits the raw session history).
- Runs automatic cleanup strategies: deduplication of repeated tool calls (same tool+args)
  and purge of error tool inputs after N turns.
- Injects `dcp-system-reminder` blocks nudging compression when the context is approaching
  `minContextLimit`/`maxContextLimit` (defaults 50k / 100k tokens, `compress.summaryBuffer`
  counts active summaries toward the limit).
- Manual controls: `/dcp` panel and `/dcp-compress [focus]`.

Active settings on this machine follow the plugin defaults (local global config at
`~/.config/opencode/dcp.jsonc` contains only `$schema`). Config search order is global
`~/.config/opencode/dcp.jsonc` → `$OPENCODE_CONFIG_DIR/dcp.jsonc` → project
`.opencode/dcp.jsonc` (project wins). **Restart required** after config edits. Per repo
doctrine (`.agents/README.md` principle 5) we keep DCP config global, not in `.opencode/`.

## 🧠 The compress tool — input contract (exact)

Call shape:

```json
{
  "topic": "Short label, 3-5 words",
  "content": [
    { "startId": "m0002", "endId": "m0012", "summary": "..." },
    { "startId": "m0015", "endId": "m0020", "summary": "..." }
  ]
}
```

Requirements:

| Item | Requirement |
| --- | --- |
| `topic` | 3–5 word label of the whole call |
| `startId`/`endId` | Exact IDs visible in `<dcp-message-id>` tags. Raw messages use `mNNNN`; previously compressed blocks use `bN`. Never invent IDs. `startId` must precede `endId` in the transcript. |
| Non-overlap | Entries inside one call must NOT overlap one another. |
| Placeholders | If the selected range includes previously compressed blocks, reference them in the summary as `(bN)` placeholders (each exactly once, in order, nothing after them that reads wrong once expanded). Only for blocks genuinely inside the range. |
| `summary` | Replace the whole span; must read as a stand-alone record: state, decisions, file paths + line numbers, signatures, constraints, user intent, and next moves. Lead with a recap of the *oldest* content in the span. |

### 🚨 HARD SIZE LIMIT — the failure we hit

**Total JSON payload of one `compress` call is truncated around ~8,000–8,200 characters.**
Crossing it returns `Expected ',' or ']' after array element in JSON` at that position and
the call fails, eating turns and patience.

- Keep **every** `compress` call well under ~7,000 characters (topic + all summaries + JSON
  syntax). Treat ~6,500 as the safety target.
- Prefer **multiple small calls** over one large call. A single call can carry many ranges,
  but each summary must be short.
- Do not paste years of specifics into one summary. Compress by chapter, not by novel.
- If a summary would be huge, split the range into successive smaller ranges across calls,
  or shorten it. Loss is acceptable when it keeps the tool working.

### ✅ Summary quality bar

- **Be exhaustive for what you keep:** paths, function signatures, decisions, constraints,
  verification state (which check passed/failed), user intent — then be *lean*: strip
  dead-end exploration, verbose tool dumps, and fix history.
- **Preserve user intent faithfully.** Quote short user directives verbatim (with
  `<protect>`-free plain quoting); never paraphrase a constraint away.
- **Protected tools' output lives independently** under DCP (`task`, `skill`, `todowrite`,
  `todoread`, plus `compress`, `batch`, `plan_enter`, `plan_exit`, `write`, `edit`). Do NOT
  duplicate those outputs inside summaries.
- Skip `bN` placeholder mention for blocks outside the chosen range.

### 🕑 When to compress

| Compress | Do NOT compress |
| --- | --- |
| A chapter completed and verified (exploration done, impl green, checks run) | Content still needed verbatim for edits/references |
| Closed research whose conclusions are now captured | Actively in-progress work |
| Dead-end noise, failed attempts that led nowhere, redundant dumps | Exact code/errors/file content needed in the immediate next steps |
| Voluminous tool output (large dir listings, huge file reads) | Anything already summarized plus its env-reminder |

When a `dcp-system-reminder` says context is at/near the limit, **stop exploring and
compress older, resolved spans first** — finish only a critical atomic step first, then run
a compression pass (see `skills/manage-context.md`).

## ⚙️ Automatic strategies

- `deduplication`: repeated identical tool calls keep only the last output. Recalculated on
  each `compress` run — batch tidy-ups into the same pass.
- `purgeErrors`: errored tool inputs pruned after 4 turns; error text kept.
- Don't "fake" either: don't re-run identical calls to trigger dedup, and never prune an
  errored input whose details matter before its time.

## 💾 Prompt-cache trade-off

DCP changes messages, which invalidates provider prompt-cache prefixes from that point on.
Compression that is too frequent raises cache misses. Prefer compressing in fewer, bigger
(but still size-capped) passes over many tiny ones — except when nudged hard, where any
compression beats none.

## 🚨 Failure script (if a `compress` call errors)

1. Truncation-style JSON error → payload too long. Split or shorten, retry with a smaller
   call. Never retry the same oversized payload.
2. Unknown/invalid boundary ID → re-read the transcript tags; use only IDs present there.
3. Overlapping ranges → merge them into one range (safe, non-overlapping) and retry.
4. Compress is unavailable → do not thrash; continue minimizing output, keep needed facts
   in a `.agents/tracking/` scratch file for the user.

## 🔗 Related

- `skills/manage-context.md` — the routine for a compression pass.
- `rules/general.md` — scope honesty; a compressed-away decision must survive in summaries.
- `rules/git-workflow.md`, `rules/security.md` — unchanged by DCP.