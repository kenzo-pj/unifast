# Multilingual i18n Expansion (15 Languages, Full Coverage)

**Date:** 2026-04-14
**Status:** Approved (pending user review of this spec)
**Author:** Brainstorming session — Kenzo Wada × Claude Opus 4.6

## 1. Goal

Expand the unifast website from 2 supported locales (`en`, `ja`) to 15 supported locales, with full UI + LandingPage + docs content translated for every language. The language switcher must work seamlessly across all locales. Implementation is delivered as a single PR ("one-shot full"). Translation quality is enforced via Opus 4.6 subagents per language; routine code transformations may use Sonnet.

## 2. Scope

### In scope
- 13 new locales added: `zh-CN, zh-TW, ko, fr, it, es, pt-BR, de, ru, hi, id, tr, vi`
- Existing locales preserved: `en` (default), `ja`
- UI dictionary expansion: existing 5 sections (`nav, search, theme, notFound, i18n`) + new `landing` section
- Translation of all 15 UI dictionaries (incl. retranslating `ja` for the new `landing` section)
- Refactor of `LandingPage/index.tsx` to extract hardcoded strings into the i18n dictionary
- LanguageSwitcher updated to display all 15 native locale labels via a `LOCALE_LABELS` map
- `vite-plugin-translation-status.ts` generalized from hardcoded `ja` to all locales
- Per-locale landing page metadata (`[locale]/index.astro` and `index.astro`) read from dictionary
- ~70 docs `.md` / `.mdx` files mirrored into 13 new `content/<locale>/` directories with translated prose
- Cleanup of orphan file `content/ja/api/compile.md`

### Out of scope (deferred to future PRs)
- Arabic / RTL support
- Hreflang `<link rel="alternate">` SEO tags
- CJK / Devanagari web fonts
- Browser-locale auto-redirect
- Crowdin / Weblate integration
- README / CONTRIBUTING translation
- Search index multilingual reindexing (already works automatically via `flattenNav` labelKey)

## 3. Languages

| Code | Language | Native label | Notes |
|---|---|---|---|
| `en` | English | English | Default, existing |
| `ja` | Japanese | 日本語 | Existing, will be retranslated for landing section |
| `zh-CN` | Simplified Chinese | 简体中文 | New |
| `zh-TW` | Traditional Chinese | 繁體中文 | New, distinct from zh-CN |
| `ko` | Korean | 한국어 | New |
| `fr` | French | Français | New |
| `it` | Italian | Italiano | New |
| `es` | Spanish | Español | New |
| `pt-BR` | Portuguese (Brazil) | Português (Brasil) | New |
| `de` | German | Deutsch | New |
| `ru` | Russian | Русский | New |
| `hi` | Hindi | हिन्दी | New |
| `id` | Indonesian | Bahasa Indonesia | New |
| `tr` | Turkish | Türkçe | New |
| `vi` | Vietnamese | Tiếng Việt | New |

Total: **15 locales**, **14 non-default**.

## 4. Architecture

### 4.1 Module changes

| File | Change type | Description |
|---|---|---|
| `website/src/i18n/index.ts` | modify | Expand `LocaleCode` to 15, expand `SUPPORTED_LOCALES`, add `LOCALE_LABELS: Record<LocaleCode, string>`, expand `dictionaries` map |
| `website/src/i18n/locales/en.ts` | modify | Add `landing.*` section (~25 keys) extracted from LandingPage hardcoded strings |
| `website/src/i18n/locales/ja.ts` | modify | Add `landing.*` translations + remove `i18n.switchLabel` |
| `website/src/i18n/locales/{zh-CN,zh-TW,ko,fr,it,es,pt-BR,de,ru,hi,id,tr,vi}.ts` | create | New per-locale dictionaries, full translation of all sections |
| `website/src/components/LandingPage/index.tsx` | modify | Replace all hardcoded prose with `t("landing.*")` calls |
| `website/src/components/LanguageSwitcher/index.tsx` | modify | Replace `loc === "en" ? "English" : "日本語"` ternary with `LOCALE_LABELS[loc]` |
| `website/plugins/vite-plugin-translation-status.ts` | modify | Loop over `SUPPORTED_LOCALES`, change manifest type to `Record<LocaleCode, Record<string, TranslationStatusEntry>>` |
| `website/src/pages/[locale]/index.astro` | modify | Read `landing.metaTitle` / `landing.metaDescription` from dictionary instead of hardcoded `siteMetadata.ja` |
| `website/src/pages/index.astro` | modify | Same — use English dictionary's landing meta |
| `website/content/{zh-CN,zh-TW,ko,fr,it,es,pt-BR,de,ru,hi,id,tr,vi}/**/*.{md,mdx}` | create | ~70 files × 13 locales = ~910 new docs files |
| `website/content/ja/{introduction,guides,packages}/**` | create | Mirror of `content/en/` structure (~70 files) |
| `website/content/ja/api/compile.md` | delete | Orphan file, predates current navigation |
| `website/knip.json` | modify (conditional) | Add `src/i18n/locales/*.ts` to entry pattern if knip flags new locale files as unused |

### 4.2 Type system

The `Locale` type derivation from `en.ts` (line 93) is preserved unchanged:

```typescript
export type Locale = { [K in keyof typeof en]: { [P in keyof (typeof en)[K]]: string } };
```

Adding the `landing` section to `en.ts` automatically requires every other locale dictionary file to provide `landing` keys, enforced at compile time. This is the central guarantee of the migration: TypeScript will refuse to build until all 14 locales are fully translated.

### 4.3 Translation status manifest

**Before:**
```typescript
export type TranslationManifest = Record<string, TranslationStatusEntry>;
// { "packages/node/compile": { status: "missing" } }
```

**After:**
```typescript
export type TranslationManifest = Partial<Record<LocaleCode, Record<string, TranslationStatusEntry>>>;
// {
//   "ja":    { "packages/node/compile": { status: "translated", ... } },
//   "zh-CN": { "packages/node/compile": { status: "translated", ... } },
//   ...
// }
```

The Astro consumer (`[locale]/docs/[...slug].astro`) currently uses `fs.existsSync` directly and does not read the manifest, so the manifest type change is internally consistent with no runtime breakage.

### 4.4 Routing

The existing routing strategy is unchanged:
- `/` → English (default, no prefix)
- `/<locale>/` → non-default locales
- `/<locale>/docs/<slug>` → non-default locale docs

The `parseLocaleFromPath` function (`i18n/index.ts:62`) iterates `SUPPORTED_LOCALES` already, so expanding the array to 15 entries gives correct routing for all new locales without code changes. The `getStaticPaths` in `[locale]/docs/[...slug].astro:83` similarly iterates `nonDefaultLocales` and works automatically.

## 5. Implementation order (7 stages)

```
Stage 1: i18n/index.ts — expand LocaleCode, SUPPORTED_LOCALES, LOCALE_LABELS, dictionaries map
   ↓
Stage 2: en.ts — add `landing` section (~25 keys)
   ↓ (TS compiler now flags ja.ts as missing landing)
Stage 3: ja.ts — add `landing` translations, remove `i18n.switchLabel`
   ↓
Stage 4: 13 new locale TS files — full translation, parallel via 13 Opus 4.6 subagents
   ↓
Stage 5: LandingPage.tsx — t() conversion of hardcoded strings
        LanguageSwitcher.tsx — LOCALE_LABELS reference
        [locale]/index.astro + index.astro — landing.meta* reference
   ↓
Stage 6: vite-plugin-translation-status.ts — generalize to all locales
   ↓
Stage 7: ~910 docs md/mdx files generated, parallel via 13 Opus 4.6 subagents
```

### 5.1 Subagent model strategy

| Stage | Execution mode | Model | Justification |
|---|---|---|---|
| 1, 2, 3, 5, 6 | Main thread (current Opus 4.6 session) | Opus 4.6 | Code changes require full project context |
| 4 (UI dictionaries) | 13 parallel subagents | **Opus 4.6** | UI strings are short but culturally sensitive (button labels, error messages, formal/informal tone) |
| 7 (docs prose) | 13 parallel subagents | **Opus 4.6** | Translation requires deep linguistic reasoning to avoid literal/word-for-word output |

Sonnet is explicitly **not used** for translation tasks — the user has flagged this as a quality risk, and translation across distant language pairs (CJK ↔ European) is where Opus 4.6's reasoning depth matters most.

### 5.2 Subagent prompt template (Stages 4 and 7)

Every translation subagent receives:
1. **Target locale and native language characteristics** — formality conventions, pluralization, sentence structure norms
2. **Absolute rules:**
   - Code blocks: do not modify code content
   - Function names, package names, API signatures, technical terms (MDX, AST, HAST, Markdown, Rust, Vite, React): preserve as-is in original language
   - Frontmatter `title` / `description`: must translate
   - Markdown structure (link URLs, image paths, heading levels): preserve
3. **Quality bar:** "Native-quality, idiomatic, technically accurate. Not literal/word-for-word."
4. **Self-verification:** After completing all files, re-read each one and check structure/spelling
5. **Reference example:** `content/ja/index.md` (existing manually-written translation) as quality benchmark

### 5.3 Parallelism plan

- **Stage 4:** Single message containing 13 `Agent` tool calls in parallel, one per non-default locale, `model: "opus"`
- **Stage 7:** Single message containing 13 `Agent` tool calls in parallel, one per non-default locale, `model: "opus"`
- Total subagent count: **26** (split across two parallel batches)
- Estimated wall-clock time: bounded by the slowest single-locale completion in each batch (not the sum)

## 6. Verification (definition of done)

| # | Command | Expected |
|---|---|---|
| 1 | `pnpm --filter @unifast/website type-check` (or `tsc --noEmit`) | 0 errors |
| 2 | `pnpm --filter @unifast/website lint` | 0 errors (warnings allowed at baseline) |
| 3 | `pnpm --filter @unifast/website fmt:check` | no diff |
| 4 | `pnpm --filter @unifast/website knip` | no new issues vs baseline |
| 5 | `pnpm --filter @unifast/website build` | SSG build succeeds, ~1050 pages generated |
| 6 | Spot check `dist/zh-CN/docs/packages/node/compile/index.html` | UI labels in Simplified Chinese |
| 7 | Spot check `dist/ko/index.html` | LandingPage section titles in Korean |
| 8 | Spot check `dist/fr/docs/introduction/quick-start/index.html` | French docs prose visible |

Browser/Playwright validation is **not** part of this session's completion criteria — performed post-merge by the user.

## 7. Risks and mitigations

| Risk | Severity | Mitigation |
|---|---|---|
| Machine translation accuracy in technical contexts | Medium | Strict rule: code, API names, technical jargon stay in original. Native review deferred to OSS contributors. |
| Removal of `i18n.switchLabel` is a breaking change | Low | Pre-grep confirms only LanguageSwitcher uses it. |
| Manifest type change in translation-status plugin | Low | Astro consumer uses `fs.existsSync` directly, manifest is internal. |
| Subagent translation consistency drift | Medium | Identical prompt template + identical reference example for all 13 subagents. |
| Build time exceeds CI timeout (15× page count) | Medium | Measure post-Stage-7. If >60min, tune `astro.config.mjs` `build.concurrency`. |
| ~944-file PR is hard to review | High | User commits manually with stage-level granularity recommended. |
| `knip` flags new locale files as unused | Low | Add `src/i18n/locales/*.ts` to `knip.json` entry pattern if needed. |
| Long session context exhaustion | Medium | Subagent parallelism in Stages 4 and 7 keeps main-thread context spend minimal. |
| CJK font rendering fallback quality | Low | System fonts adequate for default render. |

## 8. Translation rules (docs prose)

| Element | Translate? |
|---|---|
| Prose paragraphs, headings | Yes |
| Frontmatter `title` / `description` | Yes |
| Code block contents | No |
| Code comments inside code blocks | No (English convention) |
| Function names, package names, API signatures (`compile()`, `@unifast/node`) | No |
| URLs and file paths | No |
| Markdown link/list/table structure | Yes — preserve structure, translate inner text |
| Technical jargon (Markdown, MDX, AST, HAST, Rust) | No — preserve in original |
| Mermaid diagrams, math expressions | No — preserve syntax |

## 9. Side effects

1. `pnpm-lock.yaml`: no change (no new dependencies)
2. `.astro/types.d.ts`: regenerated automatically (already gitignored)
3. `knip.json`: may need entry pattern update (verify during Stage 4)
4. CI build duration: ~15× longer; verify against CI timeout

## 10. Rollback plan

Each stage produces independently-revertible files. If translation quality is judged inadequate post-merge:
1. Per-locale rollback: delete `content/<locale>/` and remove from `SUPPORTED_LOCALES`
2. Per-section rollback: revert `landing.*` keys; LandingPage falls back to literal strings (requires Stage 5 partial revert)
3. Full rollback: revert the merge commit

## 11. Open questions resolved during brainstorming

| Question | Decision |
|---|---|
| Coverage scope (UI only / UI+landing / + docs) | D — Full machine-translated docs |
| Language list (RTL inclusion) | 15 locales, no Arabic / no RTL |
| File format (TS / JSON / YAML / TOML) | TS (preserve type safety) |
| Execution chunking (one-shot / phased) | One-shot full PR |
| Subagent model assignment | Opus 4.6 for all translation work |

## 12. Completion criteria summary

- Code changes: Stages 1, 2, 3, 5, 6 committed
- 14 non-default UI dictionaries created and type-checked: Stage 4
- ~910 docs files created: Stage 7
- All 8 verification commands pass
- Spec document committed alongside implementation
