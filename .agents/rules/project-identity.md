# Rule: Project Identity (never the folder name)

Applies to every agent, every session, every task. This rule exists because a past failure
cost real time: the workspace directory name was mistaken for the product name, leading to
wasted work and nonsense branding.

## The project

We build **a modern desktop client for nhentai.net**, named **NH Desktop**. The app talks to
the nhentai.net public API; the nhentai.net website and its hosts (`t.nhentai.net`,
`i.nhentai.net`, `nhentai.net/g/…`) keep their own names and are referenced as-is.

**Name scheme (authoritative):**

| Scope | Value |
| --- | --- |
| User-facing product (exe, title, install dir `Programs\NH Desktop`, shortcuts, registry DisplayName) | **NH Desktop** |
| Rust crate package / npm package / bundle identifier / file prefixes | `nh-desktop` · `net.nh-desktop.client` |
| Rust lib crate / Rust module / cache prefix / db file | `nh_desktop_lib` · `nh_desktop` · `nh-desktop:` · `nh-desktop.db` |
| Website / API / image hosts | `nhentai.net` · `nhentai.net/api/v2` · `t.nhentai.net` · `i.nhentai.net` (never renamed) |
| Repo | misnamed folder `lewd-clips-app` is NOT the name; product is NH Desktop |

Identifiers use `nh-desktop` (hyphenated, lowercase) or `nh_desktop_lib` where Rust requires
underscores; a bare `nhentai` string in the app is stale unless it refers to the website.

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
- **The app is not the website.** `nhentai.net` is the site the app is built for; the app is
  a distinct product named **NH Desktop**. A bare "nhentai" string inside the app often should
  read "NH Desktop"; keep "nhentai"/"nhentai.net" only where it refers to the website, the
  API, an account, or the API key. When unsure which, re-read `AGENTS.md`.

## Checklist (before naming anything)

1. Product/brand name → from `AGENTS.md` + this rule, never from the workspace folder.
2. Exe / window / registry / install-dir names → derived from the product name.
3. Any appearance of `LewdClips`, `LewdZone`, or `tauri-app` in active code → rename or flag.
4. Reference projects (e.g. the lewdzone installer pattern) → consulted for *how*, never used
   for *what* to build.

## Related

- `rules/general.md` — scope honesty, no dead naming, keep docs truthful.
- `AGENTS.md` — project facts and decision log live at the root.