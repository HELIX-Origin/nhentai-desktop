# Changelog Entry (per release)

NH Desktop follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) +
SemVer. `CHANGELOG.md` is the single historical record; release notes and the
changelog entry are authored from the same commit list. Newest releases on top,
living state under `## [Unreleased]`.

## Template

```md
## [{{ version }}]({{ release-url }}) - {{ YYYY-MM-DD }}

### ✨ Added

- **{{ <emoji> <type>(<scope>): <subject> }}** [{{ short sha }}]({{ commit-url }}) — {{ what/why }}

### ✅ Changed

- ...

### 🐛 Fixed

- ...

### 🔒 Security

- ...

### 🧹 Removed / Deprecated

- ...
```

## Fill rules

1. **`{{ version }}` = `vX.Y.Z`** — exact SemVer tag, release URL =
   `https://github.com/HELIX-Origin/nhentai-desktop/releases/tag/vX.Y.Z`.
2. **Category headings** — emoji headings as above: `✨ Added` / `✅ Changed` /
   `🐛 Fixed` / `🔒 Security` / `🧹 Removed | Deprecated`. Use those that apply;
   omit the rest.
3. **Commit entries** — subject mirrors the
   [commit-message template](./commit-message.md):
   `**{{ emoji type(scope): subject }}** [short sha](full-url)` + one-line
   recap of the user-visible effect.
4. **Grouped bullets** — separate blank line between grouped blocks; keep
   entries scannable.
5. **Unreleased** section always sits above the newest tagged release and is
   drained (moved into the release section) at ship time.

## Verify

- [ ] newest tagged section is at the top, directly below `## [Unreleased]`
- [ ] every commit since the last release appears under exactly one category
- [ ] release URL matches the tag; hash links point at real commits
- [ ] `package.json`, `src-tauri/Cargo.toml`, `tauri.conf.json` all bumped
  (`release-standards` rule)