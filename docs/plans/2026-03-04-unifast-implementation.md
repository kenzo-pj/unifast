# unifast Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Build a high-performance Markdown/MDX compiler with Rust core, N-API Node bindings, and WASM support.

**Architecture:** Oxc-style pipeline: Parse → MdAst → Built-in passes → Lower to HAst → HAst passes → Emit. Arena-allocated ASTs with span-first source mapping. Pass registry for extensibility.

**Tech Stack:** Rust 2024 edition, bumpalo (arena), oxc_parser (MDX), syntect (highlight), napi-rs (Node), wasm-bindgen (WASM), serde_yaml/toml/serde_json, insta (snapshots), vitest (TS tests), pnpm workspaces.

---

## Task 1: Project Scaffolding

**Files:**
- Create: `Cargo.toml` (workspace root)
- Create: `crates/unifast-core/Cargo.toml`
- Create: `crates/unifast-core/src/lib.rs`
- Create: `crates/unifast-bindings-node/Cargo.toml`
- Create: `crates/unifast-bindings-node/src/lib.rs`
- Create: `crates/unifast-bindings-wasm/Cargo.toml`
- Create: `crates/unifast-bindings-wasm/src/lib.rs`
- Create: `crates/unifast-cli/Cargo.toml`
- Create: `crates/unifast-cli/src/main.rs`
- Create: `.rustfmt.toml`
- Create: `.clippy.toml`
- Create: `rust-toolchain.toml`

**Step 1: Create workspace Cargo.toml**

```toml
[workspace]
resolver = "2"
members = [
    "crates/unifast-core",
    "crates/unifast-bindings-node",
    "crates/unifast-bindings-wasm",
    "crates/unifast-cli",
]

[workspace.package]
version = "0.1.0"
edition = "2024"
license = "MIT"
repository = "https://github.com/kenzwada/unifast"

[workspace.dependencies]
unifast-core = { path = "crates/unifast-core" }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
serde_yaml = "0.9"
toml = "0.8"
bumpalo = { version = "3", features = ["collections"] }
insta = { version = "1", features = ["yaml"] }
```

**Step 2: Create unifast-core crate**

`crates/unifast-core/Cargo.toml`:
```toml
[package]
name = "unifast-core"
version.workspace = true
edition.workspace = true

[dependencies]
serde = { workspace = true }
serde_json = { workspace = true }
serde_yaml = { workspace = true }
toml = { workspace = true }
bumpalo = { workspace = true }

[dev-dependencies]
insta = { workspace = true }
```

`crates/unifast-core/src/lib.rs`:
```rust
pub mod ast;
pub mod api;
pub mod parse;
pub mod transform;
pub mod emit;
pub mod diagnostics;
pub mod cache;
pub mod util;
```

**Step 3: Create stub crates for bindings and CLI**

Each with minimal `Cargo.toml` and stub `src/lib.rs` or `src/main.rs`.

**Step 4: Create tooling configs**

`.rustfmt.toml`, `rust-toolchain.toml`

**Step 5: Verify build**

Run: `cargo check --workspace`
Expected: compiles with no errors

**Step 6: Commit**

```bash
git add -A
git commit -m "feat: scaffold workspace with core, bindings, and CLI crates"
```

---

## Task 2: Core Types — Span, NodeId, Position (Milestone 1)

**Files:**
- Create: `crates/unifast-core/src/ast/mod.rs`
- Create: `crates/unifast-core/src/ast/common.rs`

**Step 1: Write failing tests for Span and NodeId**

In `crates/unifast-core/src/ast/common.rs`:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn span_contains_offset() {
        let span = Span::new(10, 20);
        assert!(span.contains(15));
        assert!(!span.contains(5));
        assert!(!span.contains(20));
    }

    #[test]
    fn span_merge() {
        let a = Span::new(5, 10);
        let b = Span::new(15, 25);
        let merged = a.merge(b);
        assert_eq!(merged.start, 5);
        assert_eq!(merged.end, 25);
    }

    #[test]
    fn node_id_generation() {
        let mut gen = NodeIdGen::new();
        let a = gen.next();
        let b = gen.next();
        assert_ne!(a, b);
        assert_eq!(a.0, 0);
        assert_eq!(b.0, 1);
    }
}
```

**Step 2: Run tests to verify they fail**

Run: `cargo test -p unifast-core`
Expected: compilation errors (types not defined)

**Step 3: Implement Span, NodeId, Position, NodeIdGen**

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Span {
    pub start: u32,
    pub end: u32,
}

impl Span {
    pub fn new(start: u32, end: u32) -> Self { Self { start, end } }
    pub fn empty() -> Self { Self { start: 0, end: 0 } }
    pub fn contains(&self, offset: u32) -> bool { offset >= self.start && offset < self.end }
    pub fn len(&self) -> u32 { self.end - self.start }
    pub fn is_empty(&self) -> bool { self.start == self.end }
    pub fn merge(self, other: Span) -> Span {
        Span::new(self.start.min(other.start), self.end.max(other.end))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(pub u32);

pub struct NodeIdGen(u32);

impl NodeIdGen {
    pub fn new() -> Self { Self(0) }
    pub fn next(&mut self) -> NodeId {
        let id = NodeId(self.0);
        self.0 += 1;
        id
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Position {
    pub line: u32,
    pub column: u32,
}
```

**Step 4: Run tests**

Run: `cargo test -p unifast-core`
Expected: all pass

**Step 5: Commit**

```bash
git commit -m "feat: add core types — Span, NodeId, Position"
```

---

## Task 3: LineIndex (Milestone 1)

**Files:**
- Create: `crates/unifast-core/src/util/mod.rs`
- Create: `crates/unifast-core/src/util/line_index.rs`

**Step 1: Write failing tests**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::common::Position;

    #[test]
    fn line_index_simple() {
        let src = "hello\nworld\n";
        let idx = LineIndex::new(src);
        assert_eq!(idx.line_col(0), Position { line: 1, column: 1 });
        assert_eq!(idx.line_col(6), Position { line: 2, column: 1 });
        assert_eq!(idx.line_col(8), Position { line: 2, column: 3 });
    }

    #[test]
    fn line_index_empty() {
        let idx = LineIndex::new("");
        assert_eq!(idx.line_col(0), Position { line: 1, column: 1 });
    }
}
```

**Step 2: Implement LineIndex**

Binary search over newline offsets for O(log n) line/column lookup.

**Step 3: Run tests, verify pass**

Run: `cargo test -p unifast-core -- util`

**Step 4: Commit**

```bash
git commit -m "feat: add LineIndex for lazy line/column computation"
```

---

## Task 4: Diagnostics System (Milestone 1)

**Files:**
- Create: `crates/unifast-core/src/diagnostics/mod.rs`
- Create: `crates/unifast-core/src/diagnostics/diagnostic.rs`
- Create: `crates/unifast-core/src/diagnostics/sink.rs`
- Create: `crates/unifast-core/src/diagnostics/render.rs`

**Step 1: Write failing tests**

```rust
#[test]
fn diagnostic_creation() {
    let d = Diagnostic::error("missing closing bracket", Span::new(5, 10))
        .with_code("E001");
    assert_eq!(d.level, DiagLevel::Error);
    assert_eq!(d.span.start, 5);
}

#[test]
fn sink_collects_diagnostics() {
    let mut sink = DiagnosticSink::new();
    sink.error("err1", Span::new(0, 1));
    sink.warn("warn1", Span::new(2, 3));
    assert_eq!(sink.diagnostics().len(), 2);
    assert!(sink.has_errors());
}
```

**Step 2: Implement Diagnostic, DiagLevel, DiagnosticSink, render functions**

**Step 3: Run tests, verify pass**

**Step 4: Commit**

```bash
git commit -m "feat: add diagnostics system with Diagnostic, DiagnosticSink, render"
```

---

## Task 5: Interner and Hash Utilities (Milestone 1)

**Files:**
- Create: `crates/unifast-core/src/util/interner.rs`
- Create: `crates/unifast-core/src/util/hash.rs`
- Create: `crates/unifast-core/src/util/small_map.rs`

**Step 1: Write failing tests for Interner**

```rust
#[test]
fn interner_roundtrip() {
    let mut interner = Interner::new();
    let sym = interner.intern("hello");
    assert_eq!(interner.resolve(sym), "hello");
    let sym2 = interner.intern("hello");
    assert_eq!(sym, sym2); // same string => same symbol
}
```

**Step 2: Implement Interner (FxHashMap-based intern table)**

**Step 3: Implement content_hash and options_hash utilities**

**Step 4: Run tests, verify pass**

**Step 5: Commit**

```bash
git commit -m "feat: add Interner, hash utilities, SmallMap"
```

---

## Task 6: MdAst Node Definitions (Milestone 1)

**Files:**
- Create: `crates/unifast-core/src/ast/mdast/mod.rs`
- Create: `crates/unifast-core/src/ast/mdast/nodes.rs`
- Create: `crates/unifast-core/src/ast/mdast/builder.rs`
- Create: `crates/unifast-core/src/ast/mdast/visitor.rs`

**Step 1: Define MdAst node types**

All CommonMark + GFM nodes: Document, Heading, Paragraph, Text, Emphasis, Strong, InlineCode, Code (fenced), Blockquote, List, ListItem, ThematicBreak, Link, Image, Definition, Html, Table, TableRow, TableCell, Delete (strikethrough), FootnoteDefinition, FootnoteReference, TaskListItem, Yaml, Toml, Json (frontmatter), MdxJsxFlowElement, MdxJsxTextElement, MdxjsEsm, MdxFlowExpression, MdxTextExpression.

Each node has:
- `span: Span`
- `id: NodeId`
- node-specific fields
- `children: Vec<MdNode>` where applicable

**Step 2: Write tests for builder and visitor**

```rust
#[test]
fn build_simple_document() {
    let mut gen = NodeIdGen::new();
    let doc = MdNode::Document(Document {
        id: gen.next(),
        span: Span::new(0, 12),
        children: vec![
            MdNode::Paragraph(Paragraph {
                id: gen.next(),
                span: Span::new(0, 12),
                children: vec![
                    MdNode::Text(Text {
                        id: gen.next(),
                        span: Span::new(0, 12),
                        value: "Hello world!".into(),
                    }),
                ],
            }),
        ],
    });
    assert!(matches!(doc, MdNode::Document(_)));
}
```

**Step 3: Implement Visitor trait**

```rust
pub trait MdVisitor {
    fn visit_document(&mut self, node: &Document) { self.visit_children(&node.children); }
    fn visit_heading(&mut self, node: &Heading) { self.visit_children(&node.children); }
    // ... for each node type
    fn visit_children(&mut self, children: &[MdNode]) {
        for child in children { self.visit_node(child); }
    }
    fn visit_node(&mut self, node: &MdNode) { /* dispatch */ }
}
```

**Step 4: Run tests, verify pass**

**Step 5: Commit**

```bash
git commit -m "feat: define MdAst node types, builder, and visitor"
```

---

## Task 7: HAst Node Definitions (Milestone 1)

**Files:**
- Create: `crates/unifast-core/src/ast/hast/mod.rs`
- Create: `crates/unifast-core/src/ast/hast/nodes.rs`
- Create: `crates/unifast-core/src/ast/hast/builder.rs`
- Create: `crates/unifast-core/src/ast/hast/visitor.rs`

**Step 1: Define HAst node types**

Element, Text, Comment, Doctype, Root. Each Element has tag, attributes (BTreeMap for stable ordering), children, span.

**Step 2: Write tests for HAst builder**

**Step 3: Implement Visitor trait for HAst**

**Step 4: Run tests, commit**

```bash
git commit -m "feat: define HAst node types, builder, and visitor"
```

---

## Task 8: JsAst Node Definitions (Milestone 1)

**Files:**
- Create: `crates/unifast-core/src/ast/jsast/mod.rs`
- Create: `crates/unifast-core/src/ast/jsast/nodes.rs`

**Step 1: Define JsAst node stubs**

JsAstNode (Program, ImportDecl, ExportDecl, JsxElement, JsxFragment, Expression). Minimal — detailed parsing delegated to oxc_parser.

**Step 2: Write tests, implement, commit**

```bash
git commit -m "feat: define JsAst node stubs for MDX support"
```

---

## Task 9: API Types — CompileOptions, CompileResult (Milestone 1)

**Files:**
- Create: `crates/unifast-core/src/api/mod.rs`
- Create: `crates/unifast-core/src/api/options.rs`
- Create: `crates/unifast-core/src/api/result.rs`
- Create: `crates/unifast-core/src/api/compile.rs`

**Step 1: Define types from spec Section 3.1.1**

InputKind, OutputKind, RawHtmlPolicy, CompileOptions (with all sub-options), CompileResult, Output enum, AstBundle, CompileStats.

**Step 2: Implement compile() stub that returns empty result**

**Step 3: Write test for compile stub**

```rust
#[test]
fn compile_empty_input() {
    let result = compile("", &CompileOptions::default());
    assert!(result.diagnostics.is_empty());
    assert!(matches!(result.output, Output::Html(ref s) if s.is_empty()));
}
```

**Step 4: Run tests, commit**

```bash
git commit -m "feat: add API types and compile() entry point stub"
```

---

## Task 10: Pass System — Trait, Registry, Phases (Milestone 1)

**Files:**
- Create: `crates/unifast-core/src/transform/mod.rs`
- Create: `crates/unifast-core/src/transform/pass.rs`
- Create: `crates/unifast-core/src/transform/registry.rs`
- Create: `crates/unifast-core/src/transform/phases.rs`

**Step 1: Define Pass trait, Phase enum, PassContext, PassResult**

As specified in Section 3.2.

**Step 2: Implement PassRegistry**

- Register passes by phase
- Order: builtin first, then plugins, respect Before/After constraints
- `run_all()` method that runs passes in order

**Step 3: Write tests**

```rust
#[test]
fn registry_orders_by_phase() {
    let mut reg = PassRegistry::new();
    reg.register(Box::new(MockPass::new("emit_pass", Phase::Emit)));
    reg.register(Box::new(MockPass::new("transform_pass", Phase::Transform)));
    let ordered = reg.ordered_passes();
    assert_eq!(ordered[0].name(), "transform_pass");
    assert_eq!(ordered[1].name(), "emit_pass");
}
```

**Step 4: Run tests, commit**

```bash
git commit -m "feat: add Pass trait, PassRegistry, and Phase system"
```

---

## Task 11: Plugin Interface (Milestone 1)

**Files:**
- Modify: `crates/unifast-core/src/transform/mod.rs`

**Step 1: Define Plugin trait**

As specified in Section 3.3.

**Step 2: Write test for plugin registration**

**Step 3: Commit**

```bash
git commit -m "feat: add Plugin trait for Rust plugin extensibility"
```

---

## Task 12: Cache System (Milestone 1)

**Files:**
- Create: `crates/unifast-core/src/cache/mod.rs`
- Create: `crates/unifast-core/src/cache/memory.rs`
- Create: `crates/unifast-core/src/cache/disk.rs`

**Step 1: Define CacheStore trait, CacheKey, CacheOptions**

**Step 2: Implement MemoryCache (HashMap-based)**

**Step 3: Implement DiskCache stub (file-system backed, optional)**

**Step 4: Write tests for cache hit/miss**

```rust
#[test]
fn memory_cache_hit() {
    let mut cache = MemoryCache::new();
    let key = CacheKey::new("abc123", "opts456", "v0.1.0");
    cache.put(key.clone(), b"<p>Hello</p>".to_vec());
    assert_eq!(cache.get(&key), Some(&b"<p>Hello</p>".to_vec()[..]));
}
```

**Step 5: Run tests, commit**

```bash
git commit -m "feat: add cache system with MemoryCache and DiskCache"
```

---

## Task 13: Markdown Parser — Core Block Parsing (Milestone 2)

**Files:**
- Create: `crates/unifast-core/src/parse/mod.rs`
- Create: `crates/unifast-core/src/parse/markdown/mod.rs`
- Create: `crates/unifast-core/src/parse/markdown/parser.rs`
- Create: `tests/corpus/markdown/` directory with test fixtures

**Step 1: Write corpus test fixtures**

Create `tests/corpus/markdown/headings.md`, `paragraphs.md`, `emphasis.md`, etc. with expected HTML output.

**Step 2: Write integration test harness**

```rust
#[test]
fn test_heading_parsing() {
    let input = "# Hello\n\n## World\n";
    let ast = parse_markdown(input);
    // Assert AST shape
    assert!(matches!(&ast.children[0], MdNode::Heading(h) if h.depth == 1));
    assert!(matches!(&ast.children[1], MdNode::Heading(h) if h.depth == 2));
}
```

**Step 3: Implement block-level parser**

- State machine for: headings (ATX + setext), paragraphs, code fences, blockquotes, thematic breaks, lists (ordered/unordered, tight/loose), HTML blocks.
- Each block produces MdNode with correct Span.

**Step 4: Run tests, commit**

```bash
git commit -m "feat: implement Markdown block-level parser"
```

---

## Task 14: Markdown Parser — Inline Parsing (Milestone 2)

**Files:**
- Modify: `crates/unifast-core/src/parse/markdown/parser.rs`

**Step 1: Write tests for inline elements**

```rust
#[test]
fn test_emphasis() {
    let ast = parse_markdown("Hello *world* and **bold**\n");
    // Assert emphasis and strong nodes
}

#[test]
fn test_links_and_images() {
    let ast = parse_markdown("[link](http://example.com)\n![alt](img.png)\n");
    // Assert Link and Image nodes with correct URLs
}
```

**Step 2: Implement inline parser**

- Delimiter-based algorithm for emphasis/strong
- Link/image parsing with title support
- Inline code, escaped characters, HTML entities
- Reference link resolution

**Step 3: Run tests, commit**

```bash
git commit -m "feat: implement Markdown inline parser"
```

---

## Task 15: GFM Parser Extensions (Milestone 2)

**Files:**
- Create: `crates/unifast-core/src/parse/gfm/mod.rs`
- Create: `crates/unifast-core/src/parse/gfm/tables.rs`
- Create: `crates/unifast-core/src/parse/gfm/footnotes.rs`
- Create: `crates/unifast-core/src/parse/gfm/autolink.rs`
- Create: `crates/unifast-core/src/parse/gfm/task_list.rs`
- Create: `crates/unifast-core/src/parse/gfm/strikethrough.rs`
- Create: `tests/corpus/gfm/` directory

**Step 1: Write tests for each GFM feature**

Tables, task lists, strikethrough, autolink literals, footnotes — each with test fixtures.

**Step 2: Implement each GFM extension**

- Tables: pipe table parsing, alignment support
- Task list: `[ ]` and `[x]` parsing
- Strikethrough: `~~text~~`
- Autolink: URL and email detection
- Footnotes: `[^id]` reference and `[^id]: content` definitions

**Step 3: Run tests, commit**

```bash
git commit -m "feat: implement GFM extensions (tables, tasks, strikethrough, autolink, footnotes)"
```

---

## Task 16: Frontmatter Parsing (Milestone 3)

**Files:**
- Create: `crates/unifast-core/src/parse/frontmatter/mod.rs`
- Create: `crates/unifast-core/src/parse/frontmatter/yaml.rs`
- Create: `crates/unifast-core/src/parse/frontmatter/toml.rs`
- Create: `crates/unifast-core/src/parse/frontmatter/json.rs`
- Create: `tests/corpus/frontmatter/` directory

**Step 1: Write tests**

```rust
#[test]
fn yaml_frontmatter() {
    let input = "---\ntitle: Hello\ndate: 2024-01-01\n---\n\n# Content\n";
    let result = parse_markdown(input);
    assert_eq!(result.frontmatter["title"], "Hello");
    // frontmatter removed from document body
    assert!(matches!(&result.doc.children[0], MdNode::Heading(_)));
}

#[test]
fn toml_frontmatter() {
    let input = "+++\ntitle = \"Hello\"\n+++\n\n# Content\n";
    let result = parse_markdown(input);
    assert_eq!(result.frontmatter["title"], "Hello");
}

#[test]
fn json_frontmatter() {
    let input = ";;;\n{\"title\": \"Hello\"}\n;;;\n\n# Content\n";
    let result = parse_markdown(input);
    assert_eq!(result.frontmatter["title"], "Hello");
}
```

**Step 2: Implement frontmatter detection and parsing**

- YAML: `---` delimiters, parse with serde_yaml
- TOML: `+++` delimiters, parse with toml crate
- JSON: `;;;` delimiters (or `{` at start), parse with serde_json
- Strip from document, attach to metadata

**Step 3: Run tests, commit**

```bash
git commit -m "feat: implement frontmatter parsing (YAML, TOML, JSON)"
```

---

## Task 17: MdAst Normalize Pass (Milestone 4)

**Files:**
- Create: `crates/unifast-core/src/transform/passes/mod.rs`
- Create: `crates/unifast-core/src/transform/passes/normalize.rs`

**Step 1: Write tests**

```rust
#[test]
fn normalize_heading_depth_clamp() {
    // heading depth 7 => clamped to 6
}

#[test]
fn normalize_whitespace() {
    // consecutive whitespace text nodes merged
}

#[test]
fn normalize_list_tightness() {
    // tight vs loose list detection
}
```

**Step 2: Implement NormalizePass**

- Clamp heading depths 1..6
- Merge adjacent text nodes
- Normalize whitespace
- Validate all spans

**Step 3: Run tests, commit**

```bash
git commit -m "feat: implement MdAst normalize pass"
```

---

## Task 18: Slug Pass (Milestone 4)

**Files:**
- Create: `crates/unifast-core/src/transform/passes/slug.rs`

**Step 1: Write tests**

```rust
#[test]
fn slug_github_style() {
    assert_eq!(generate_slug("Hello World!", SlugMode::GitHub), "hello-world");
}

#[test]
fn slug_dedup() {
    let mut seen = SlugSet::new();
    assert_eq!(seen.unique_slug("hello"), "hello");
    assert_eq!(seen.unique_slug("hello"), "hello-1");
    assert_eq!(seen.unique_slug("hello"), "hello-2");
}
```

**Step 2: Implement SlugPass**

- GitHub-like slugification: lowercase, strip non-alphanum (keep hyphens), dedup
- Unicode mode option
- Attach slug to heading node data

**Step 3: Run tests, commit**

```bash
git commit -m "feat: implement slug pass with GitHub-style dedup"
```

---

## Task 19: TOC Pass (Milestone 4)

**Files:**
- Create: `crates/unifast-core/src/transform/passes/toc.rs`

**Step 1: Write tests**

```rust
#[test]
fn toc_collects_headings() {
    let md = "# One\n## Two\n### Three\n## Four\n";
    let toc = generate_toc(parse(md), 3);
    assert_eq!(toc.len(), 4);
    assert_eq!(toc[0].depth, 1);
    assert_eq!(toc[0].text, "One");
}
```

**Step 2: Implement TocPass**

- Collect headings up to maxDepth
- Build TocEntry tree with slug, depth, text
- Optionally insert TOC node at placeholder

**Step 3: Run tests, commit**

```bash
git commit -m "feat: implement TOC pass"
```

---

## Task 20: Definition Resolution Pass (Milestone 4)

**Files:**
- Create: `crates/unifast-core/src/transform/passes/resolve_defs.rs`

**Step 1: Write tests**

```rust
#[test]
fn resolve_reference_link() {
    let md = "[hello][hw]\n\n[hw]: http://example.com\n";
    let ast = parse_and_resolve(md);
    // Reference link resolved to full Link node
}

#[test]
fn warn_missing_definition() {
    let md = "[hello][missing]\n";
    let result = parse_and_resolve(md);
    assert!(result.diagnostics.iter().any(|d| d.message.contains("missing")));
}
```

**Step 2: Implement ResolveDefsPass**

- Collect definitions
- Resolve reference links/images
- Emit warnings for missing definitions

**Step 3: Run tests, commit**

```bash
git commit -m "feat: implement definition resolution pass"
```

---

## Task 21: Link/Image Rewrite Pass (Milestone 4)

**Files:**
- Create: `crates/unifast-core/src/transform/passes/rewrite_links.rs`

**Step 1: Write tests**

```rust
#[test]
fn rewrite_relative_to_absolute() {
    // "./image.png" with base "https://example.com/docs/" => "https://example.com/docs/image.png"
}
```

**Step 2: Implement RewriteLinksPass**

- Base URL resolution
- Optional relative-to-absolute rewriting
- Asset recording

**Step 3: Run tests, commit**

```bash
git commit -m "feat: implement link/image rewrite pass"
```

---

## Task 22: Lowering MdAst → HAst (Milestone 5)

**Files:**
- Create: `crates/unifast-core/src/transform/passes/mdast_to_hast.rs`
- Create: `crates/unifast-core/src/transform/passes/raw_html.rs`

**Step 1: Write tests**

```rust
#[test]
fn lower_heading_to_h1() {
    let md = "# Hello\n";
    let hast = lower(parse(md));
    // <h1>Hello</h1>
    assert!(matches!(&hast.children[0], HNode::Element(e) if e.tag == "h1"));
}

#[test]
fn lower_paragraph_to_p() {
    let md = "Hello world\n";
    let hast = lower(parse(md));
    assert!(matches!(&hast.children[0], HNode::Element(e) if e.tag == "p"));
}

#[test]
fn raw_html_disallow_policy() {
    // Raw HTML => escaped text + warning
}

#[test]
fn raw_html_allow_dangerous() {
    // Raw HTML preserved as raw node
}

#[test]
fn raw_html_parse_and_sanitize() {
    // Raw HTML parsed into HAst then sanitized
}
```

**Step 2: Implement MdAstToHAstPass**

Convert every MdAst node to its HAst equivalent:
- Heading → h1-h6
- Paragraph → p
- Emphasis → em
- Strong → strong
- InlineCode → code
- Code → pre>code
- Blockquote → blockquote
- List → ul/ol
- ListItem → li
- Link → a
- Image → img
- Table → table>thead/tbody>tr>td/th
- ThematicBreak → hr
- Delete → del
- FootnoteReference → sup>a
- FootnoteDefinition → section.footnotes
- TaskListItem → li with checkbox input

**Step 3: Implement RawHtmlPass with 3 policies**

**Step 4: Run tests, commit**

```bash
git commit -m "feat: implement MdAst to HAst lowering with raw HTML policies"
```

---

## Task 23: Sanitize Pass (Milestone 6)

**Files:**
- Create: `crates/unifast-core/src/transform/passes/sanitize.rs`
- Create: `tests/corpus/sanitize/` directory

**Step 1: Write tests**

```rust
#[test]
fn sanitize_removes_script() {
    // <script>alert('xss')</script> => removed, warning emitted
}

#[test]
fn sanitize_allows_safe_tags() {
    // <p>, <a>, <strong>, <em> => kept
}

#[test]
fn sanitize_custom_schema() {
    // Custom schema allows <div class="..."> but not <div onclick="...">
}
```

**Step 2: Implement SanitizePass**

- Schema-driven allowlist (default safe schema)
- Tag allowlist, attribute allowlist per tag
- Protocol allowlist for href/src
- Warning emission for removed elements

**Step 3: Run tests, commit**

```bash
git commit -m "feat: implement sanitize pass with schema-driven allowlist"
```

---

## Task 24: HTML Stringify / Emit (Milestone 6)

**Files:**
- Create: `crates/unifast-core/src/emit/mod.rs`
- Create: `crates/unifast-core/src/emit/html/mod.rs`
- Create: `crates/unifast-core/src/emit/html/stringify.rs`
- Create: `crates/unifast-core/src/emit/html/escape.rs`
- Create: `crates/unifast-core/src/emit/html/void_elements.rs`
- Create: `tests/snapshots/html/` directory

**Step 1: Write snapshot tests**

```rust
#[test]
fn html_emit_heading() {
    let html = compile_to_html("# Hello\n");
    insta::assert_snapshot!(html, @"<h1>Hello</h1>");
}

#[test]
fn html_emit_paragraph_with_inline() {
    let html = compile_to_html("Hello *world* and **bold**\n");
    insta::assert_snapshot!(html, @"<p>Hello <em>world</em> and <strong>bold</strong></p>");
}
```

**Step 2: Implement HTML stringify**

- Recursive HAst traversal
- Correct escaping (& < > " ')
- Void element handling (br, hr, img, input, etc.)
- Stable attribute ordering (alphabetical via BTreeMap)
- Deterministic output

**Step 3: Run tests with insta snapshots**

Run: `cargo test -p unifast-core -- emit`

**Step 4: Commit**

```bash
git commit -m "feat: implement HTML stringify with escaping and void elements"
```

---

## Task 25: Wire Up Full Pipeline — compile() (Milestone 6)

**Files:**
- Modify: `crates/unifast-core/src/api/compile.rs`

**Step 1: Write E2E integration tests**

```rust
#[test]
fn e2e_simple_markdown() {
    let result = compile("# Hello\n\nWorld\n", &CompileOptions::default());
    assert_eq!(result.output, Output::Html("<h1>Hello</h1>\n<p>World</p>".into()));
}

#[test]
fn e2e_gfm_table() {
    let result = compile("| a | b |\n|---|---|\n| 1 | 2 |\n", &opts_with_gfm());
    // snapshot the HTML table output
}

#[test]
fn e2e_frontmatter_extraction() {
    let result = compile("---\ntitle: Hi\n---\n\n# Content\n", &CompileOptions::default());
    assert_eq!(result.frontmatter["title"], "Hi");
}
```

**Step 2: Wire up full pipeline in compile()**

1. Parse frontmatter (if enabled)
2. Parse markdown/MDX to MdAst
3. Register builtin passes + plugin passes
4. Run MdAst passes (normalize, slug, toc, resolve_defs, rewrite_links)
5. Lower MdAst → HAst
6. Run HAst passes (raw_html, sanitize)
7. Emit (HTML or AST)
8. Collect diagnostics, stats, frontmatter

**Step 3: Run tests, commit**

```bash
git commit -m "feat: wire up full compile() pipeline"
```

---

## Task 26: Highlight Pass (Milestone 7)

**Files:**
- Create: `crates/unifast-core/src/transform/passes/highlight.rs`

Add `syntect` to workspace dependencies.

**Step 1: Write tests**

```rust
#[test]
fn highlight_fenced_code() {
    let md = "```rust\nfn main() {}\n```\n";
    let html = compile_to_html_with_highlight(md);
    assert!(html.contains("class=\"language-rust\""));
    // syntect adds span elements for tokens
}

#[test]
fn highlight_unknown_language_fallback() {
    let md = "```unknownlang\ncode\n```\n";
    let html = compile_to_html_with_highlight(md);
    // Falls back to plain text, still wrapped in pre>code
}
```

**Step 2: Implement HighlightPass**

- Trait `HighlightEngine` for pluggability
- `SyntectEngine` as builtin
- Apply to fenced code blocks: add language class, wrap tokens
- Handle unknown language gracefully

**Step 3: Run tests, commit**

```bash
git commit -m "feat: implement highlight pass with syntect engine"
```

---

## Task 27: HTML Cleanup Pass (Milestone 7)

**Files:**
- Create: `crates/unifast-core/src/transform/passes/html_cleanup.rs`

**Step 1: Write tests**

```rust
#[test]
fn stable_attribute_ordering() {
    // class="foo" id="bar" => alphabetical: class, id
}

#[test]
fn remove_empty_nodes_when_enabled() {
    // <p></p> => removed
}
```

**Step 2: Implement HtmlCleanupPass**

**Step 3: Run tests, commit**

```bash
git commit -m "feat: implement HTML cleanup pass"
```

---

## Task 28: MDX Parsing via Oxc (Milestone 8)

**Files:**
- Create: `crates/unifast-core/src/parse/mdx/mod.rs`
- Create: `crates/unifast-core/src/parse/mdx/oxc_jsx.rs`
- Create: `crates/unifast-core/src/parse/mdx/esm.rs`
- Create: `crates/unifast-core/src/parse/mdx/expr.rs`
- Create: `tests/corpus/mdx/` directory

Add `oxc_parser`, `oxc_ast`, `oxc_allocator`, `oxc_span` to dependencies.

**Step 1: Write tests**

```rust
#[test]
fn mdx_jsx_flow() {
    let input = "<MyComponent prop=\"value\">\n  Content\n</MyComponent>\n";
    let ast = parse_mdx(input);
    assert!(matches!(&ast.children[0], MdNode::MdxJsxFlowElement(_)));
}

#[test]
fn mdx_esm_import() {
    let input = "import { Button } from './Button'\n\n# Hello\n";
    let ast = parse_mdx(input);
    assert!(matches!(&ast.children[0], MdNode::MdxjsEsm(_)));
}

#[test]
fn mdx_expression() {
    let input = "Hello {name}\n";
    let ast = parse_mdx(input);
    // Text + MdxTextExpression
}
```

**Step 2: Implement MDX parser**

- Detect JSX tags in flow and text positions
- Parse JS/JSX using oxc_parser
- Parse ESM blocks (import/export)
- Parse MDX expressions `{...}`
- Preserve byte spans for all MDX nodes

**Step 3: Handle MDX in HTML output mode**

- Default: keep as raw markers, emit diagnostic
- Optional: strip or escape (configurable)

**Step 4: Run tests, commit**

```bash
git commit -m "feat: implement MDX parsing with Oxc backend"
```

---

## Task 29: MDX JS Emission + Sourcemaps (Milestone 9)

**Files:**
- Create: `crates/unifast-core/src/emit/mdx_js/mod.rs`
- Create: `crates/unifast-core/src/emit/mdx_js/printer.rs`
- Create: `crates/unifast-core/src/emit/mdx_js/sourcemap.rs`
- Create: `tests/snapshots/mdx_js/` directory

**Step 1: Write snapshot tests**

```rust
#[test]
fn mdx_js_emit_simple() {
    let input = "import { X } from 'x'\n\n# Hello\n\n<X />\n";
    let result = compile(input, &CompileOptions {
        input_kind: InputKind::Mdx,
        output_kind: OutputKind::MdxJs,
        ..Default::default()
    });
    insta::assert_snapshot!(match result.output {
        Output::MdxJs { code, .. } => code,
        _ => panic!("expected MdxJs output"),
    });
}
```

**Step 2: Implement MDX JS printer**

- Compile MDX AST to JS module string
- Generate createElement/jsx calls for JSX elements
- Preserve ESM imports/exports
- Build sourcemap (VLQ encoding)

**Step 3: Run tests, commit**

```bash
git commit -m "feat: implement MDX JS emission with sourcemaps"
```

---

## Task 30: CLI Tool (Milestone 10)

**Files:**
- Modify: `crates/unifast-cli/Cargo.toml`
- Create: `crates/unifast-cli/src/main.rs`
- Create: `crates/unifast-cli/src/args.rs`

Add `clap` to dependencies.

**Step 1: Implement CLI args parsing**

```rust
#[derive(Parser)]
struct Cli {
    /// Input file path
    input: PathBuf,
    /// Output format
    #[arg(long, default_value = "html")]
    format: String,
    /// Enable GFM
    #[arg(long)]
    gfm: bool,
    // ... other options
}
```

**Step 2: Wire CLI to compile()**

**Step 3: Write integration test**

```rust
#[test]
fn cli_compile_markdown() {
    let output = Command::new(env!("CARGO_BIN_EXE_unifast-cli"))
        .args(["test.md", "--format", "html"])
        .output();
    // verify output
}
```

**Step 4: Commit**

```bash
git commit -m "feat: implement unifast CLI tool"
```

---

## Task 31: Node Binding — N-API (Milestone 10)

**Files:**
- Modify: `crates/unifast-bindings-node/Cargo.toml`
- Modify: `crates/unifast-bindings-node/src/lib.rs`
- Create: `crates/unifast-bindings-node/src/convert_options.rs`
- Create: `crates/unifast-bindings-node/src/convert_result.rs`

**Step 1: Set up napi-rs**

Add `napi` and `napi-derive` dependencies.

**Step 2: Implement JS ↔ Rust option conversion**

Convert JS CompileOptions (camelCase) to Rust CompileOptions.

**Step 3: Implement result conversion**

Convert Rust CompileResult to JS object with diagnostics including line/column.

**Step 4: Export compile function**

```rust
#[napi]
pub fn compile(input: String, options: Option<JsCompileOptions>) -> napi::Result<JsCompileResult> {
    let opts = convert_options(options)?;
    let result = unifast_core::api::compile::compile(&input, &opts);
    Ok(convert_result(result))
}
```

**Step 5: Build and test**

Run: `pnpm build` in packages/node

**Step 6: Commit**

```bash
git commit -m "feat: implement N-API Node binding via napi-rs"
```

---

## Task 32: WASM Binding (Milestone 10)

**Files:**
- Modify: `crates/unifast-bindings-wasm/Cargo.toml`
- Modify: `crates/unifast-bindings-wasm/src/lib.rs`

**Step 1: Set up wasm-bindgen**

**Step 2: Implement compile function with JsValue conversion**

**Step 3: Build**

Run: `wasm-pack build crates/unifast-bindings-wasm`

**Step 4: Commit**

```bash
git commit -m "feat: implement WASM binding via wasm-bindgen"
```

---

## Task 33: TypeScript Packages (Milestone 10)

**Files:**
- Create: `packages/core/package.json`
- Create: `packages/core/src/index.ts`
- Create: `packages/core/src/options.ts`
- Create: `packages/core/src/result.ts`
- Create: `packages/core/src/errors.ts`
- Create: `packages/core/tsconfig.json`
- Create: `packages/node/package.json`
- Create: `packages/node/src/index.ts`
- Create: `packages/node/src/native.ts`
- Create: `packages/node/tsconfig.json`
- Create: `packages/mdx/package.json`
- Create: `packages/mdx/src/index.ts`
- Create: `packages/mdx/tsconfig.json`
- Create: `packages/plugin-gfm/`, `plugin-frontmatter/`, `plugin-sanitize/`, `plugin-highlight/`
- Create: `pnpm-workspace.yaml`
- Create: `package.json` (root)

**Step 1: Set up pnpm workspace**

**Step 2: Create @unifast/core with TS type definitions**

Mirror Rust API types in TypeScript (Section 3.4.1).

**Step 3: Create @unifast/node that loads native binary**

**Step 4: Create plugin packages (thin wrappers around native config)**

**Step 5: Write vitest tests**

```ts
import { compile } from '@unifast/node';

test('compile markdown to html', () => {
  const result = compile('# Hello\n');
  expect(result.output).toBe('<h1>Hello</h1>');
});
```

**Step 6: Commit**

```bash
git commit -m "feat: set up TypeScript packages with pnpm workspace"
```

---

## Task 34: Golden Corpus Integration Tests (Milestone 10)

**Files:**
- Create: test fixtures in `tests/corpus/` for every feature
- Create: `tests/snapshots/` for HTML and MDX JS output

**Step 1: Create comprehensive corpus**

- `tests/corpus/markdown/`: headings, paragraphs, lists, emphasis, code, links, images, blockquotes, html, entities
- `tests/corpus/gfm/`: tables, tasks, strikethrough, autolink, footnotes
- `tests/corpus/frontmatter/`: yaml, toml, json
- `tests/corpus/raw_html/`: disallow, allow, sanitize
- `tests/corpus/sanitize/`: script removal, safe tags, custom schema
- `tests/corpus/mdx/`: jsx flow, jsx text, esm, expressions

**Step 2: Write test runner that processes all corpus items**

**Step 3: Generate and verify snapshots**

**Step 4: Commit**

```bash
git commit -m "feat: add comprehensive golden corpus test suite"
```

---

## Task 35: Benchmarks (Milestone 10)

**Files:**
- Create: `benches/compile.rs`
- Create: `benches/parse.rs`
- Create: `benches/sanitize.rs`
- Create: `benches/highlight.rs`

**Step 1: Set up criterion benchmarks**

Add `criterion` to workspace dev-dependencies.

**Step 2: Write benchmarks for key paths**

```rust
fn bench_parse_large_markdown(c: &mut Criterion) {
    let input = include_str!("../tests/fixtures/large.md");
    c.bench_function("parse_large_markdown", |b| {
        b.iter(|| parse_markdown(black_box(input)))
    });
}
```

**Step 3: Run benchmarks**

Run: `cargo bench`

**Step 4: Commit**

```bash
git commit -m "feat: add criterion benchmarks"
```

---

## Task 36: CI Configuration (Milestone 10)

**Files:**
- Create: `.github/workflows/ci.yml`

**Step 1: Create CI workflow**

```yaml
name: CI
on: [push, pull_request]
jobs:
  rust:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo fmt --all -- --check
      - run: cargo clippy --all-targets --all-features -- -D warnings
      - run: cargo test --all --all-features
  node:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: pnpm/action-setup@v4
      - uses: actions/setup-node@v4
      - run: pnpm install
      - run: pnpm lint
      - run: pnpm test
```

**Step 2: Commit**

```bash
git commit -m "feat: add CI configuration"
```

---

## Task 37: Final Integration + Polish (Milestone 10)

**Files:**
- Create: `CLAUDE.md` with project conventions and commands

**Step 1: Run full test suite**

Run: `cargo test --all --all-features && pnpm test`

**Step 2: Fix any remaining issues**

**Step 3: Create CLAUDE.md with project conventions**

**Step 4: Final commit**

```bash
git commit -m "chore: final integration polish and CLAUDE.md"
```
