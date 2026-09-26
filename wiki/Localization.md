# 🌍 Localization & Language Support

NH Desktop speaks your language. The interface defaults to your operating system language, and
you can pick a different one at any time — in the installer or in **Settings**.

> Only the **app interface** is translated. Gallery titles, tags, and artwork come from
> nhentai.net in whatever language they were published in — those are never machine-translated
> by the app, because that would corrupt search and filtering.

---

## 🎯 Choosing a language

**During installation** — the installer has a **Language** dropdown on the *Options* step. The
installer itself is already in your language at this point, so you can read the options before
you choose.

**After installation** — open **Settings → Appearance → Language** and pick from the list. The
change applies immediately; no restart needed.

**Automatic selection** — on first launch, NH Desktop asks the operating system for its locale
(`en-US` → English, `ja-JP` → Japanese, `zh-CN` → Simplified Chinese, and so on). If your
system language has no pack yet, NH Desktop falls back to English until you choose one.

---

## 📦 Available language packs

| Language | Code | Status |
| --- | --- | --- |
| English | `en` | Source language (always complete) |
| 日本語 Japanese | `ja` | ✅ Complete |
| 简体中文 Chinese (Simplified) | `zh-Hans` | ✅ Complete |
| 繁體中文 Chinese (Traditional) | `zh-Hant` | ✅ Complete |
| 한국어 Korean | `ko` | 🧩 Community needed |
| Español Spanish | `es` | 🧩 Community needed |
| Français French | `fr` | 🧩 Community needed |
| Deutsch German | `de` | 🧩 Community needed |
| Русский Russian | `ru` | 🧩 Community needed |
| Português Portuguese | `pt` | 🧩 Community needed |
| Italiano Italian | `it` | 🧩 Community needed |
| ไทย Thai | `th` | 🧩 Community needed |
| Tiếng Việt Vietnamese | `vi` | 🧩 Community needed |
| Bahasa Indonesia Indonesian | `id` | 🧩 Community needed |
| Polski Polish | `pl` | 🧩 Community needed |
| Nederlands Dutch | `nl` | 🧩 Community needed |
| Türkçe Turkish | `tr` | 🧩 Community needed |
| العربية Arabic | `ar` | 🧩 Community needed |

Languages marked **Community needed** are selectable in the app already; untranslated strings
fall back to English until a pack lands. Adding one is a small, self-contained pull request —
see below.

---

## 🤝 Contributing a translation

Everything you need lives in `src/lib/i18n/`. There is no build step, no compiled resource, and
no tooling to install.

### Improving an existing pack

1. Open the file, e.g. `src/lib/i18n/ja.json`.
2. Edit the strings you want to improve. Keep every key unchanged.
3. Run the validator:

   ```bash
   npm run i18n:check
   ```

   It reports missing keys, untranslated leftovers, and typos (unknown keys):

   ```
   ja          78/78 keys  OK
   ```

4. Open a pull request. That is the whole process.

### Adding a new language

1. **Pick a code.** Use a BCP-47 tag — `ko`, `es`, `fr`, `pt-BR`, `zh-Hant`, `ar`. Match an
   existing code in `src/lib/i18n/locales.ts` if one is already registered; if the language is
   new, add a new entry there too (see below).
2. **Copy the source file.**

   ```bash
   cp src/lib/i18n/en.json src/lib/i18n/ko.json
   ```

3. **Translate every value.** Leave the keys exactly as they are. Do not add or remove keys —
   that is what the validator is for.
4. **Register the locale** in `src/lib/i18n/locales.ts`:

   ```ts
   { code: 'ko', name: 'Korean', nativeName: '한국어' },
   ```

   `name` is the English name, `nativeName` is what users see in the dropdown.
5. **Import the pack** in `src/lib/i18n/index.ts`:

   ```ts
   import ko from './ko.json';

   const dictionaries: Partial<Record<LocaleCode, Record<string, unknown>>> = {
       en,
       ja,
       ko,
   };
   ```

6. **Verify.**

   ```bash
   npm run i18n:check     # pack completeness
   npm run check          # types
   cargo check            # backend, if you touched it
   ```

7. Open a pull request with a title like `feat(i18n): add Korean (ko) translation pack`.

### Style guidelines

- **Be concise.** These are short interface labels in a dense UI. Prefer the shortest form a
  native speaker would actually use — long sentences shrink the window and look wrong.
- **Keep placeholders intact.** Strings like `~128 MB` or `Enter` may embed literal text.
- **Use the app's own vocabulary.** The installer calls its steps *Welcome / Location / Options /
  Install*; the sidebar calls them *Latest / Popular / Favorites / History / Downloads /
  Blacklist / Settings*. Translate the concept, not word-for-word, and stay consistent within a
  language.
- **Mind the UI language.** `zh-Hans` and `zh-Hant` are genuinely different files. Arabic is
  right-to-left; keep the wording neutral so layout mirrors correctly.
- **Punctuation and spacing** follow the target language's own conventions (full-width
  punctuation in CJK, non-breaking spaces before `:` in French, and so on).

### Free resources for getting it right

If you want to be certain a phrase reads naturally, cross-check against these free sources:

- **Mozilla Fluent / DeepL glossary** — <https://www.deepl.com>
- **Wiktionary** for term consistency — <https://www.wiktionary.org>
- **translate-i18n** community terminology databases — <https://github.com/translate-i18n>
- **Microsoft terminology** (many UI terms defined per locale) — <https://www.microsoft.com>
- Your own OS: many desktop apps are already localized into your language. Inconsistent machine
  translation is easy to spot once you have seen how a competent app words the same button.

---

## 🧱 How localization works under the hood

| File | Role |
| --- | --- |
| `src/lib/i18n/en.json` | **Source of truth.** Every key must exist here first. |
| `src/lib/i18n/<locale>.json` | One file per language, same shape as `en.json`. |
| `src/lib/i18n/locales.ts` | Registry of supported codes, display names, and the default. |
| `src/lib/i18n/index.ts` | Registers packs and implements lookup with English fallback. |
| `src/lib/stores/locale.svelte.ts` | Reactive current locale, system detection, persistence. |
| `scripts/check-i18n.mjs` | `npm run i18n:check` validator. |

Lookup is `locale.t('installer.welcomeTitle')` — a dotted key resolved against the active
dictionary. Missing keys silently fall back to English rather than rendering a raw key, so a
partial pack degrades gracefully instead of breaking the UI.

Your choice is stored in the local database (`settings:locale`) and reused by both the installer
and the app, so they always agree.

---

## 🔗 Related

- [Settings & API Key](Settings-and-API-Key)
- [Installation & Maintenance](Installation-and-Maintenance)
- [Development & Contributing](Development-and-Contributing)
