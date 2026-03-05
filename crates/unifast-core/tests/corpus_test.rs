/// Golden corpus integration tests.
/// Each test compiles a corpus file and verifies the output is non-empty and valid.
use unifast_core::api::compile::compile;
use unifast_core::api::options::*;

fn read_corpus(name: &str) -> String {
    let path = format!("{}/tests/corpus/{}", env!("CARGO_MANIFEST_DIR"), name);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("Failed to read corpus file {}: {}", path, e))
}

#[test]
fn corpus_commonmark_to_html() {
    let input = read_corpus("commonmark.md");
    let opts = CompileOptions::default();
    let result = compile(&input, &opts);
    let html = match &result.output {
        unifast_core::api::result::Output::Html(h) => h,
        other => panic!("Expected HTML output, got {:?}", other),
    };
    assert!(
        !html.is_empty(),
        "CommonMark HTML output should not be empty"
    );
    assert!(html.contains("<h1"), "Should contain h1 heading");
    assert!(html.contains("<h2"), "Should contain h2 heading");
    assert!(html.contains("<strong>"), "Should contain strong");
    assert!(html.contains("<em>"), "Should contain emphasis");
    assert!(html.contains("<code>"), "Should contain inline code");
    assert!(html.contains("<pre>"), "Should contain code block");
    assert!(html.contains("<blockquote>"), "Should contain blockquote");
    assert!(html.contains("<ul>"), "Should contain unordered list");
    assert!(html.contains("<ol>"), "Should contain ordered list");
    assert!(html.contains("<hr"), "Should contain thematic break");
    assert!(html.contains("<a "), "Should contain links");
    assert!(html.contains("<img "), "Should contain images");
}

#[test]
fn corpus_gfm_to_html() {
    let input = read_corpus("gfm.md");
    let opts = CompileOptions {
        gfm: GfmOptions::default(),
        ..Default::default()
    };
    let result = compile(&input, &opts);
    let html = match &result.output {
        unifast_core::api::result::Output::Html(h) => h,
        other => panic!("Expected HTML output, got {:?}", other),
    };
    assert!(!html.is_empty(), "GFM HTML output should not be empty");
    assert!(html.contains("<table>"), "Should contain table");
    assert!(html.contains("<th"), "Should contain table headers");
    assert!(html.contains("<td"), "Should contain table cells");
    assert!(html.contains("<del>"), "Should contain strikethrough");
    assert!(
        html.contains("type=\"checkbox\"") || html.contains("checkbox"),
        "Should contain task list checkboxes"
    );
}

#[test]
fn corpus_frontmatter() {
    let input = read_corpus("frontmatter.md");
    let opts = CompileOptions {
        frontmatter: FrontmatterOptions {
            yaml: true,
            ..Default::default()
        },
        ..Default::default()
    };
    let result = compile(&input, &opts);

    // FrontmatterData is HashMap<String, Value> — non-empty means parsed
    assert!(
        !result.frontmatter.is_empty(),
        "Frontmatter should be parsed"
    );
    assert_eq!(
        result
            .frontmatter
            .get("title")
            .and_then(|v: &serde_json::Value| v.as_str()),
        Some("Test Document")
    );
    assert_eq!(
        result
            .frontmatter
            .get("author")
            .and_then(|v: &serde_json::Value| v.as_str()),
        Some("unifast")
    );

    // Verify HTML output doesn't contain frontmatter delimiters
    let html = match &result.output {
        unifast_core::api::result::Output::Html(h) => h,
        other => panic!("Expected HTML output, got {:?}", other),
    };
    assert!(
        !html.contains("---\ntitle"),
        "Frontmatter should not appear in HTML"
    );
    assert!(html.contains("<h1"), "Content heading should be present");
}

#[test]
fn corpus_highlight() {
    let input = read_corpus("highlight.md");
    let opts = CompileOptions {
        highlight: HighlightOptions {
            enabled: true,
            engine: HighlightEngine::Syntect,
        },
        ..Default::default()
    };
    let result = compile(&input, &opts);
    let html = match &result.output {
        unifast_core::api::result::Output::Html(h) => h,
        other => panic!("Expected HTML output, got {:?}", other),
    };
    assert!(
        !html.is_empty(),
        "Highlight HTML output should not be empty"
    );
    assert!(html.contains("<pre>"), "Should contain code blocks");
    // Highlighted code should have span elements with hljs classes
    assert!(
        html.contains("hljs-keyword") || html.contains("<span"),
        "Should contain highlighted tokens"
    );
}

#[test]
fn corpus_sanitize_disallow() {
    let input = read_corpus("sanitize.md");
    let opts = CompileOptions {
        raw_html: RawHtmlPolicy::Disallow,
        ..Default::default()
    };
    let result = compile(&input, &opts);
    let html = match &result.output {
        unifast_core::api::result::Output::Html(h) => h,
        other => panic!("Expected HTML output, got {:?}", other),
    };
    // With Disallow policy, script/iframe tags should not appear
    assert!(!html.contains("<script"), "Script tags should be stripped");
    assert!(!html.contains("<iframe"), "Iframe tags should be stripped");
    // Safe markdown content should be present
    assert!(html.contains("<strong>"), "Safe markdown should render");
}

#[test]
fn corpus_compile_stats() {
    let input = read_corpus("commonmark.md");
    let opts = CompileOptions::default();
    let result = compile(&input, &opts);
    // Stats should be populated
    assert!(
        result.stats.parse_ms >= 0.0,
        "parse_ms should be non-negative"
    );
    assert!(
        result.stats.transform_ms >= 0.0,
        "transform_ms should be non-negative"
    );
    assert!(
        result.stats.emit_ms >= 0.0,
        "emit_ms should be non-negative"
    );
}

#[test]
fn corpus_mdast_output() {
    let input = read_corpus("commonmark.md");
    let opts = CompileOptions {
        output_kind: OutputKind::Mdast,
        ..Default::default()
    };
    let result = compile(&input, &opts);
    match &result.output {
        unifast_core::api::result::Output::Mdast(doc) => {
            assert!(!doc.children.is_empty(), "MdAst should have children");
        }
        other => panic!("Expected Mdast output, got {:?}", other),
    }
}

#[test]
fn corpus_hast_output() {
    let input = read_corpus("commonmark.md");
    let opts = CompileOptions {
        output_kind: OutputKind::Hast,
        ..Default::default()
    };
    let result = compile(&input, &opts);
    match &result.output {
        unifast_core::api::result::Output::Hast(root) => {
            assert!(!root.children.is_empty(), "HAst root should have children");
        }
        other => panic!("Expected Hast output, got {:?}", other),
    }
}
