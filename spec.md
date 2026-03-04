# unifast — Full Design Spec (Rust-core, unified-independent)

**Project name:** `unifast`  
**Goal:** A high-performance Markdown/MDX compiler with an Oxc-style architecture (Rust core + built-in passes), **not** built on `unified`. It covers the mainstream `remark`/`rehype` use-cases by implementing the features directly (not by JS plugin compatibility). Node usage is first-class via **N-API (napi-rs)**. WASM is supported as a secondary target using the same Rust core.

This document is written so a coding agent can implement everything end-to-end.

---

## 0. Non-goals (explicit)

- No attempt to execute existing `remark`/`rehype` JS plugins inside core.
- No “API compatibility” with `unified`/`remark`/`rehype`. We target **use-case completeness**, not drop-in compatibility.
- No runtime dependency on Node’s module resolution or JS AST objects in the core path.

---

## 1. Core Use-cases (reverse-engineered)

unifast must support these production patterns:

1. **Docs/blog/SSG rendering**: Markdown/MDX → HTML
2. **Frontmatter extraction**: YAML/TOML/JSON
3. **GFM features**: tables, task list, strikethrough, autolink literals, footnotes
4. **Raw HTML policy**: disallow / allow dangerous / parse+sanitize
5. **Sanitization**: schema-based allowlist (safe defaults; configurable)
6. **Code highlighting**: pluggable engine; at least one builtin mode
7. **Diagnostics**: precise error spans, and line/column mapping

---

## 2. High-level Architecture

### 2.1 Compilation Pipeline

```
Input text
  -> Parse (Markdown or MDX)
  -> IR0: MdAst (Markdown structure)
  -> Normalize + Built-in MdAst passes
  -> Lower to IR1: HAst (HTML structure)
  -> HAst passes (sanitize, highlight, rewrites, etc.)
  -> Emit (HTML string) or return AST (HAst/MdAst)
```

### 2.2 Internal IRs

- **IR0: MdAst** — Markdown structure
- **IR1: HAst** — HTML structure
- **IR2: JsAst** — only for MDX ESM/JSX parsing and emission (when `outputKind = mdxJs`)

### 2.3 Key Design Properties

- **Span-first source mapping**: store byte spans everywhere; compute line/column lazily via `LineIndex`.
- **Pass registry**: Oxc-like passes, ordered by phase.
- **Built-in first**: “popular plugin behaviors” become builtin passes.
- **Extensibility**: Rust plugins are first-class. WASM plugins are optional and constrained.

---

## 3. Public Interfaces

## 3.1 Rust Core API

### 3.1.1 Types

```rust
pub enum InputKind { Markdown, Mdx }
pub enum OutputKind { Html, Hast, Mdast, MdxJs }

pub enum RawHtmlPolicy {
  Disallow,
  AllowDangerous,
  ParseAndSanitize,
}

pub struct CompileOptions {
  pub input_kind: InputKind,
  pub output_kind: OutputKind,

  pub gfm: GfmOptions,
  pub frontmatter: FrontmatterOptions,
  pub raw_html: RawHtmlPolicy,
  pub sanitize: SanitizeOptions,
  pub highlight: HighlightOptions,
  pub slug: SlugOptions,
  pub toc: TocOptions,

  pub extensions: Vec<Extension>,         // feature toggles / experimental flags
  pub plugins: Vec<Box<dyn Plugin>>,      // Rust plugins

  pub diagnostics: DiagnosticsOptions,
  pub cache: CacheOptions,
}

pub struct CompileResult {
  pub output: Output,
  pub ast: Option<AstBundle>,             // returned only if requested
  pub frontmatter: FrontmatterData,
  pub diagnostics: Vec<Diagnostic>,
  pub stats: CompileStats,
}

pub enum Output {
  Html(String),
  Hast(HAstDoc),
  Mdast(MdAstDoc),
  MdxJs { code: String, map: Option<String> },
}
```

### 3.1.2 Entry Point

```rust
pub fn compile(input: &str, opts: &CompileOptions) -> CompileResult;
```

---

## 3.2 Pass System (Oxc-style)

### 3.2.1 Pass Trait

```rust
pub trait Pass {
  fn name(&self) -> &'static str;
  fn phase(&self) -> Phase; // Parse|Lower|Transform|Optimize|Emit
  fn run(&mut self, ctx: &mut PassContext, ast: &mut AstBundle) -> PassResult;
}
```

### 3.2.2 Phases

- `Parse` (parser only; no passes run here)
- `Lower` (IR0 -> IR1 transitions)
- `Transform` (mutate IR0 or IR1)
- `Optimize` (optional: simplification/minify-like)
- `Emit` (stringify, sourcemap building)

### 3.2.3 Context

```rust
pub struct PassContext<'a> {
  pub arena: &'a mut Arena,
  pub interner: &'a mut Interner,
  pub source: &'a SourceText,
  pub line_index: &'a LineIndex,
  pub options: &'a CompileOptions,
  pub diagnostics: &'a mut DiagnosticSink,
  pub cache: &'a mut dyn CacheStore,
  pub fs: Option<&'a dyn VirtualFs>, // optional
}
```

---

## 3.3 Plugin Interface (Rust)

### 3.3.1 Plugin Trait

```rust
pub trait Plugin: Send + Sync {
  fn name(&self) -> &'static str;
  fn apply(&self, registry: &mut PassRegistry);
}
```

### 3.3.2 Pass Registry

- Plugins register passes into phases.
- Conflicts resolve by:
  - Phase order fixed
  - Within a phase: builtin passes first, then plugin passes in registration order, unless a pass declares `Before/After` constraints.

---

## 3.4 Node API (N-API via napi-rs)

### 3.4.1 JS/TS Types

```ts
export type CompileOptions = {
  inputKind?: "md" | "mdx";
  outputKind?: "html" | "hast" | "mdast" | "mdxJs";

  gfm?: {
    tables?: boolean;
    taskList?: boolean;
    strikethrough?: boolean;
    footnotes?: boolean;
    autolink?: boolean;
  };

  frontmatter?: { yaml?: boolean; toml?: boolean; json?: boolean };

  rawHtml?: "disallow" | "allowDangerous" | "parseAndSanitize";

  sanitize?: { enabled?: boolean; schema?: SanitizeSchema };

  highlight?: { enabled?: boolean; engine?: "none" | "builtin" };

  slug?: { mode?: "github" | "unicode" };

  toc?: { enabled?: boolean; maxDepth?: number };

  diagnostics?: { format?: "compact" | "verbose" };

  cache?: { enabled?: boolean; dir?: string };

  // Rust plugin selection by name (no JS plugin execution in core)
  plugins?: Array<{ name: string; options?: unknown }>;
};

export type CompileResult = {
  output: string | object;
  frontmatter: Record<string, unknown>;
  diagnostics: Array<{
    level: "error" | "warn";
    message: string;
    start?: number;
    end?: number;
    line?: number;
    column?: number;
  }>;
  stats: { parseMs: number; transformMs: number; emitMs: number };
};

export function compile(input: string, options?: CompileOptions): CompileResult;
```

---

## 4. Concrete Feature Requirements (Complete Edition)

## 4.1 Parsing

### 4.1.1 Markdown (CommonMark core)

Must parse at least:

- headings, paragraphs, emphasis/strong, inline code, code fences, blockquote
- ordered/unordered lists, tight/loose lists, thematic breaks
- links, images, reference definitions
- inline HTML and block HTML (subject to `rawHtml` policy)
- escaped characters, entities

### 4.1.2 GFM

- tables
- task list items
- strikethrough
- autolink literals
- footnotes (inline/definitions)

### 4.1.3 Frontmatter

- YAML frontmatter (required)
- TOML frontmatter (required)
- JSON frontmatter (required)

Behavior:

- frontmatter removed from MdAst document body
- parsed metadata returned via `frontmatter` field in result
- also attach to `AstBundle` as document-level metadata

### 4.1.4 MDX (Complete Edition, **OXc-based**)

MDX support is required and must be implemented **without SWC**.

#### Required capabilities

- JSX in flow and text positions
- ESM blocks: `import`/`export`
- MDX expressions: `{...}`

#### Parsing requirement

- Parse JS/JSX using **Oxc** (Rust) as the JS/TS parser backend.
- Preserve byte spans for all MDX nodes (JSX/ESM/expression) and connect them to the parent MdAst spans.

#### Notes

- unifast core must treat embedded JS/JSX as structured nodes, not opaque strings, when `outputKind = mdxJs`.
- When `outputKind = html`, embedded JSX/ESM nodes must be handled via a documented policy:
  - default: keep as raw markers and emit diagnostics (since HTML cannot execute JSX)
  - optional: strip or escape (configurable)

---

## 4.2 Built-in Passes (MdAst)

### 4.2.1 Normalize

- normalize list tightness rules
- normalize whitespace nodes
- ensure all nodes have valid spans
- canonicalize heading depth boundaries (1..6)

### 4.2.2 Slug

- generate heading slugs
- GitHub-like dedup strategy (e.g., `a`, `a-1`, `a-2`)
- attach slug to heading node data

### 4.2.3 TOC

- collect headings up to `maxDepth`
- generate a TOC structure (returned in metadata or AST)
- optional: insert TOC node at placeholder directive (if enabled)

### 4.2.4 Link/Image Rewrite

- base URL resolution
- optional: rewrite relative links to absolute
- optional: asset pipeline hints (record assets encountered)

### 4.2.5 Definition Resolution

- resolve reference links/images
- warn on missing definitions

---

## 4.3 Lowering (MdAst -> HAst)

- Convert Markdown structure into HAst
- Respect `rawHtml` policy:
  - `Disallow`: convert raw HTML nodes into escaped text and emit warning
  - `AllowDangerous`: preserve as raw nodes
  - `ParseAndSanitize`: parse raw HTML into HAst, then sanitize (later)

---

## 4.4 Built-in Passes (HAst)

### 4.4.1 Sanitize

- schema-driven allowlist
- safe defaults
- option to provide custom schema
- report warnings for removed tags/attributes when diagnostics enabled

### 4.4.2 Highlight

- at least one builtin engine
- apply to fenced code blocks:
  - add language class
  - wrap output HTML nodes as needed
- engine must be pluggable behind a trait so alternative engines can be added later

### 4.4.3 HTML Policy / Cleanup

- normalize attribute ordering (stable output for snapshots)
- optionally remove empty nodes
- optional minify-like whitespace normalization (off by default)

---

## 4.5 Emit

### 4.5.1 HTML stringify

- correct escaping rules
- void elements handling
- raw node emission only if allowed
- deterministic output (stable snapshots)

### 4.5.2 MDX JS emission (when outputKind = mdxJs)

- compile MDX AST to JS module string
- optionally produce sourcemap
- preserve component usage / ESM blocks
- deterministic and testable via snapshots

---

## 4.6 Diagnostics

- `Diagnostic { level, message, span, code?, notes? }`
- Rendered to Node:
  - include start/end span
  - include line/column if requested or for verbose mode
- Parsing errors must be precise and include local excerpts in verbose format.

---

## 4.7 Caching / Incremental

- keyed by:
  - input content hash
  - options hash
  - unifast version hash (to avoid stale cache across versions)
- memory cache in core
- disk cache optional in Node layer (`cache.dir`)

---

## 5. Repository Layout (Final)

```text
repo/
  Cargo.toml
  crates/
    unifast-core/
      src/
        lib.rs

        api/
          compile.rs
          options.rs
          result.rs

        ast/
          mod.rs
          common.rs                 # NodeId, Span, Position, data
          mdast/
            mod.rs
            nodes.rs
            builder.rs
            visitor.rs
          hast/
            mod.rs
            nodes.rs
            builder.rs
            visitor.rs
          jsast/
            mod.rs
            nodes.rs

        parse/
          mod.rs
          markdown/
            mod.rs
            parser.rs
          gfm/
            mod.rs
            tables.rs
            footnotes.rs
            autolink.rs
            task_list.rs
            strikethrough.rs
          frontmatter/
            mod.rs
            yaml.rs
            toml.rs
            json.rs
          mdx/
            mod.rs
            oxc_jsx.rs               # Oxc parser integration
            esm.rs
            expr.rs

        transform/
          mod.rs
          registry.rs
          pass.rs
          phases.rs
          passes/
            normalize.rs
            slug.rs
            toc.rs
            rewrite_links.rs
            resolve_defs.rs
            mdast_to_hast.rs
            raw_html.rs
            sanitize.rs
            highlight.rs
            html_cleanup.rs

        emit/
          mod.rs
          html/
            mod.rs
            stringify.rs
            escape.rs
            void_elements.rs
          mdx_js/
            mod.rs
            printer.rs
            sourcemap.rs

        diagnostics/
          mod.rs
          diagnostic.rs
          sink.rs
          render.rs

        cache/
          mod.rs
          memory.rs
          disk.rs

        util/
          interner.rs
          line_index.rs
          hash.rs
          small_map.rs

    unifast-bindings-node/
      src/
        lib.rs                      # napi entry
        convert_options.rs
        convert_result.rs
      Cargo.toml

    unifast-bindings-wasm/
      src/
        lib.rs                      # wasm-bindgen entry
      Cargo.toml

    unifast-cli/
      src/
        main.rs
        args.rs
      Cargo.toml

  packages/
    core/                           # @unifast/core (TS only)
      src/
        index.ts
        options.ts
        result.ts
        errors.ts
      package.json
      tsconfig.json

    node/                           # @unifast/node (TS + native loader)
      src/
        index.ts
        native.ts
      package.json
      tsconfig.json

    mdx/                            # @unifast/mdx (optional split)
      src/
        index.ts
      package.json
      tsconfig.json

    plugin-gfm/
      src/index.ts
      package.json

    plugin-frontmatter/
      src/index.ts
      package.json

    plugin-sanitize/
      src/index.ts
      package.json

    plugin-highlight/
      src/index.ts
      package.json

  tests/
    corpus/
      markdown/
      gfm/
      frontmatter/
      raw_html/
      sanitize/
      mdx/
    snapshots/
      html/
      mdx_js/
    fixtures/

  benches/
    compile.rs
    parse.rs
    sanitize.rs
    highlight.rs

  tools/
    gen-corpus/
    release/
```

---

## 6. Tooling, Linting, Formatting (Mandatory)

## 6.1 Rust

- **rustfmt**: required
- **clippy**: required (deny warnings in CI)
- Edition: latest stable
- Use `cargo fmt`, `cargo clippy --all-targets --all-features`

## 6.2 TypeScript / JavaScript

- **Oxc tooling**:
  - `oxlint` as the linter
  - `oxfmt` as the formatter
- “Parser-related” enforcement:
  - Use Oxc-based lint rules to enforce import/order, unused, complexity limits, etc.
- No ESLint/Prettier unless a hard blocker is discovered.

## 6.3 CI Gate

CI must fail if any of these fail:

- formatting (Rust + TS)
- linting (clippy + oxlint)
- tests (Rust + Node)
- snapshot drift (unless explicitly updated with a command)

---

## 7. TDD Policy (Strict)

## 7.1 General Rules

- Every feature begins with a failing test.
- No production implementation without:
  - unit tests (module level)
  - integration tests (pipeline level)
  - snapshot tests for final outputs (HTML and MDX JS)

## 7.2 Test Layers

### 7.2.1 Rust Unit Tests

- Parser unit tests:
  - AST shape checks
  - span correctness checks
- Pass unit tests:
  - input AST -> output AST
  - deterministic transforms

### 7.2.2 Rust Integration Tests (Golden Corpus)

- Each corpus item:
  - `input.md` / `input.mdx`
  - `options.json` (optional)
  - `expected.html` OR `expected.mdx.js`
  - `expected.diagnostics.json` (optional)
- Tests compile input and compare exact output (stable stringify and stable attribute ordering are required)

### 7.2.3 Node Binding Tests

- Validate:
  - options conversion correctness
  - diagnostics mapping (span -> line/column)
  - output stability
- Snapshot tests in Node as well (especially for JS consumers)

## 7.3 Snapshot Discipline

- Snapshots must be deterministic:
  - stable ordering
  - stable whitespace normalization rules
- Provide explicit update commands:
  - `pnpm test -u` for TS snapshots
  - `cargo test -- --nocapture` with a `UNIFAST_UPDATE_SNAPSHOTS=1` gate if needed

---

## 8. Commands (Expected)

Examples (adjust to pnpm/workspace setup):

### 8.1 Rust

- `cargo fmt --all`
- `cargo clippy --all-targets --all-features -D warnings`
- `cargo test --all --all-features`
- `cargo bench`

### 8.2 Node/TS

- `pnpm lint` (oxlint)
- `pnpm fmt` (oxfmt)
- `pnpm test` (vitest/jest; choose one and stick to it)
- `pnpm build`

### 8.3 End-to-End

- `pnpm -C packages/node test`
- `cargo test -p unifast-core`

---

## 9. Implementation Notes (Hard requirements)

- **Deterministic output** is mandatory: all ordering and emission rules must be stable.
- **Span correctness** is mandatory: every AST node must have a valid span.
- **LineIndex must be the single source of truth** for line/column mapping.
- No silent lossy behavior:
  - raw HTML dropped => diagnostic warning unless explicitly suppressed
  - sanitization removal => warning (configurable)

---

## 10. Milestones (Complete Edition)

1. Core AST + Span + LineIndex + Diagnostics (with tests)
2. Markdown parse + GFM parse (with corpus + snapshots)
3. Frontmatter parse (yaml/toml/json)
4. MdAst normalize + built-in passes (slug/toc/defs)
5. Lowering MdAst -> HAst with rawHtml policies
6. Sanitize + HTML stringify (stable snapshots)
7. Highlight pass + emit
8. MDX parsing (JSX/ESM/expr) via **Oxc** + HTML emission path
9. MDX JS emission + sourcemaps (snapshots)
10. Node binding polish + packaging + CI gates

---

## 11. Deliverables

- `@unifast/node` provides the primary `compile()` API for Node.
- `unifast-cli` provides `unifast compile input.md --html` etc.
- A `tests/corpus` directory that demonstrates completeness across:
  - markdown core
  - gfm
  - frontmatter
  - raw html modes
  - sanitize schema behaviors
  - mdx compilation paths
  - span + diagnostics correctness
- CI that enforces TDD and all linters/formatters.

---

End of spec.
