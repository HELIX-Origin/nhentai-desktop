# Rule: Project Identity (never the folder name)

Applies to every agent, every session, every task. This rule exists because a past failure
cost real time: the workspace directory name was mistaken for the product name, leading to
wasted work and nonsense branding.

## The project

We build **a modern desktop client for nhentai.net**, named **nhentai** in UI and
installer branding (exe `nhentai.exe`, window title "nhentai", product name "nhentai",
install dir `Programs\nhentai`). The backend talks to the nhentai.net public API; the
frontend is a SvelteKit static SPA.

## The rule

- **The project folder name is NOT the project name.** This workspace currently sits in a
  directory named `lewd-clips-app`, and it is *not* a "LewdClips" product. The directory was
  simply misnamed; the user will fix it later. Never derive product identity, exe names,
  window titles, branding, install-dir names, registry keys, or labels from the folder name.
- **Never invent or revive alternate product names.** "LewdClips", "LewdZone", and any other
  leaked names belong to *other* projects or are dead ends. If the user mentions them, treat
  them as references/examples only — never as something to build or brand for.
- **Pin identity to the authoritative source:** `AGENTS.md` decision log and this rule. When
  in doubt about what something is called, re-read those before writing a single name.
- **If a task's file paths or code reference a stale brand** (e.g. `tauri-app`, `LewdClips`,
  `app.lewdclips.client`), rename to the real identity in the same change where practical,
  and flag anything you left behind in `TODO.md`.
- **Keep the user's corrections verbatim in tracking docs.** If the user corrects a name or
  scope fact, the correction goes into the relevant doc in the same change so it cannot be
  un-learned.

## Checklist (before naming anything)

1. Product/brand name → from `AGENTS.md` + this rule, never from the workspace folder.
2. Exe / window / registry / install-dir names → derived from the product name.
3. Any appearance of `LewdClips`, `LewdZone`, or `tauri-app` in active code → rename or flag.
4. Reference projects (e.g. the lewdzone installer pattern) → consulted for *how*, never used
   for *what* to build.

## Related

- `rules/general.md` — scope honesty, no dead naming, keep docs truthful.
- `AGENTS.md` — project facts and decision log live at the root.