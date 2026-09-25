# Rule: Git Workflow

## ⚠️ Non-negotiable

- **Never commit, push, or open a PR unless the user explicitly asks.** Re-read this when
  tempted. The user reviews and drives git.
- Never rewrite history, force-push, amend others' commits, or change git config/hooks.

## 🤝 When git is used (user-sanctioned workflow)

1. `git status` and `git --no-pager diff` first; stage **only intended files**; never
   commit secrets, temp files, `node_modules/`, `target/`, or build output.
2. Commit message: conventional, imperative, ≤ ~72 chars subject, body explains *why*.
   Match repo style (see `git --no-pager log --oneline -10`). Full type/scope/emoji
   guide in `templates/commit-message.md`.
3. Small logical commits per change (feature atomically, fix atomically, docs atomically).
4. Branch naming: `feat/`, `fix/`, `docs/`, `chore/` + short slug (`feat/blacklist-ui`).
5. Issue/PR tracking and release flow follow `rules/issue-protocol.md` and
   `rules/release-standards.md` (roadmap-first, `gh` non-interactive with
   `--body-file`, SemVer + verification gate).
6. A change that moves `TODO.md`/`BUGS.md`/`ROADMAP.md`/`CHANGELOG.md` state should
   carry that doc update in the same commit when practical.

## 🖥️ Headless (non-interactive) shell rules

This repo is often operated by agents without a TTY:

- No editors/pagers: use `git --no-pager log`, `--no-edit` on merges.
- No interactive prompts: `git add -p`, `git rebase -i`, empty `git commit` (editor) are
  banned. Use `git commit -m "..."` with full message.
- If a command would block on credentials, use the prompt-safe variants
  (`GIT_TERMINAL_PROMPT=0`, `ssh -o BatchMode=yes`), or stop and report — never pipe
  passwords, never blanket-`yes`.
- Non-interactive equivalents are preferred for anything prompty (`-y`, `--no-input`,
  `--force`) where case-specific and justified.

## 🤝 PR hygiene (when user asks for a PR)

- Target default branch; diff against base; summarize what/why/risks/test results in the
  description using `.agents/templates/pr.md`.
- Reference TODO/BUG/ROADMAP ids when they exist.
- No draft-forever PRs; PRs are requested to merge.

## 🔄 Review loop

- Before declaring work done, run the affected checks (see `testing.md`). A reviewer pass
  uses `.agents/skills/review-code.md`.