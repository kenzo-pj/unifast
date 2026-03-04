use super::options::{CompileOptions, InputKind, OutputKind};
use super::result::{CompileResult, CompileStats, Output};
use crate::ast::common::NodeIdGen;
use crate::ast::hast::nodes::HNode;
use crate::emit::html::stringify;
use crate::parse;
use crate::transform::passes::{
    highlight, html_cleanup, mdast_to_hast, normalize, resolve_defs, sanitize as sanitize_pass,
    slug,
};
use std::time::Instant;

pub fn compile(input: &str, opts: &CompileOptions) -> CompileResult {
    let start = Instant::now();

    // 1. Parse — choose parser based on input kind.
    let parse_start = Instant::now();
    let parse_result = match opts.input_kind {
        InputKind::Markdown => parse::parse_markdown(input),
        InputKind::Mdx => parse::parse_mdx_input(input),
    };
    let mut document = parse_result.document;
    let mut diagnostics = parse_result.diagnostics;
    let frontmatter = parse_result.frontmatter;
    let parse_ms = parse_start.elapsed().as_secs_f64() * 1000.0;

    // 2. MdAst transforms
    let transform_start = Instant::now();

    // Normalize
    normalize::normalize(&mut document);

    // Slugs
    let slug_mode = match opts.slug.mode {
        crate::api::options::SlugMode::GitHub => slug::SlugMode::GitHub,
        crate::api::options::SlugMode::Unicode => slug::SlugMode::Unicode,
    };
    slug::apply_slugs(&mut document, slug_mode);

    // Resolve definitions (remove Definition nodes)
    let defs = std::collections::HashMap::new();
    resolve_defs::resolve_definitions(&mut document, &defs, &mut diagnostics);

    // 3. Lower MdAst -> HAst
    let mut id_gen = NodeIdGen::new();
    let hast_node = mdast_to_hast::lower(&document, &mut id_gen, opts.raw_html, &mut diagnostics);

    // Extract the HRoot from the HNode wrapper
    let mut hast_root = match hast_node {
        HNode::Root(root) => root,
        _ => {
            // Shouldn't happen, but wrap in a root if it does
            crate::ast::hast::nodes::HRoot {
                id: id_gen.next_id(),
                span: crate::ast::common::Span::empty(),
                children: vec![hast_node],
            }
        }
    };

    // 4. Sanitize (if enabled)
    if opts.sanitize.enabled {
        let schema = if let Some(ref api_schema) = opts.sanitize.schema {
            sanitize_pass::from_api_schema(api_schema)
        } else {
            sanitize_pass::default_safe_schema()
        };
        sanitize_pass::sanitize(&mut hast_root, &schema, &mut diagnostics);
    }

    // 5. Highlight (if enabled)
    if opts.highlight.enabled {
        let engine = highlight::BuiltinHighlighter;
        let mut highlight_id_gen = NodeIdGen::new();
        highlight::apply_highlight(&mut hast_root, &engine, &mut highlight_id_gen);
    }

    // 6. HTML cleanup (always — but with default options which are no-ops)
    html_cleanup::cleanup(&mut hast_root, &html_cleanup::CleanupOptions::default());

    let transform_ms = transform_start.elapsed().as_secs_f64() * 1000.0;

    // 5. Emit
    let emit_start = Instant::now();
    let output = match opts.output_kind {
        OutputKind::Html => {
            let html = stringify::stringify(&hast_root);
            Output::Html(html)
        }
        OutputKind::Hast => Output::Hast(hast_root),
        OutputKind::Mdast => Output::Mdast(document),
        OutputKind::MdxJs => {
            let mdx_output = crate::emit::mdx_js::printer::print_mdx_js(&document);
            let map = Some(crate::emit::mdx_js::sourcemap::generate_sourcemap(
                "output.js",
                input,
                &mdx_output.source_mappings,
            ));
            Output::MdxJs {
                code: mdx_output.code,
                map,
            }
        }
    };
    let emit_ms = emit_start.elapsed().as_secs_f64() * 1000.0;

    let _ = start; // suppress unused variable warning

    CompileResult {
        output,
        frontmatter,
        diagnostics: diagnostics.into_diagnostics(),
        stats: CompileStats {
            parse_ms,
            transform_ms,
            emit_ms,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::options::*;

    fn compile_html(input: &str) -> String {
        let result = compile(input, &CompileOptions::default());
        match result.output {
            Output::Html(html) => html,
            _ => panic!("expected HTML output"),
        }
    }

    #[test]
    fn e2e_empty_input() {
        let html = compile_html("");
        assert_eq!(html, "");
    }

    #[test]
    fn e2e_heading() {
        let html = compile_html("# Hello\n");
        assert!(html.contains("<h1"));
        assert!(html.contains("Hello"));
        assert!(html.contains("</h1>"));
    }

    #[test]
    fn e2e_paragraph() {
        let html = compile_html("Hello world\n");
        assert!(html.contains("<p>"));
        assert!(html.contains("Hello world"));
        assert!(html.contains("</p>"));
    }

    #[test]
    fn e2e_emphasis_and_strong() {
        let html = compile_html("Hello *world* and **bold**\n");
        assert!(html.contains("<em>world</em>"));
        assert!(html.contains("<strong>bold</strong>"));
    }

    #[test]
    fn e2e_code_fence() {
        let html = compile_html("```rust\nfn main() {}\n```\n");
        assert!(html.contains("<pre>"));
        assert!(html.contains("language-rust"));
        assert!(html.contains("fn main() {}"));
    }

    #[test]
    fn e2e_link() {
        let html = compile_html("[Click](http://example.com)\n");
        assert!(html.contains("href=\"http://example.com\""));
        assert!(html.contains("Click"));
    }

    #[test]
    fn e2e_image() {
        let html = compile_html("![alt](img.png)\n");
        assert!(html.contains("src=\"img.png\""));
        assert!(html.contains("alt=\"alt\""));
    }

    #[test]
    fn e2e_frontmatter() {
        let result = compile(
            "---\ntitle: Hello\n---\n\n# Content\n",
            &CompileOptions::default(),
        );
        assert_eq!(
            result.frontmatter.get("title").and_then(|v| v.as_str()),
            Some("Hello")
        );
        match result.output {
            Output::Html(html) => assert!(html.contains("<h1")),
            _ => panic!("expected HTML output"),
        }
    }

    #[test]
    fn e2e_gfm_table() {
        let html = compile_html("| a | b |\n|---|---|\n| 1 | 2 |\n");
        assert!(html.contains("<table>"));
        assert!(html.contains("<th>"));
        assert!(html.contains("<td>"));
    }

    #[test]
    fn e2e_task_list() {
        let html = compile_html("- [x] done\n- [ ] todo\n");
        assert!(html.contains("type=\"checkbox\""));
        assert!(html.contains("checked"));
    }

    #[test]
    fn e2e_compile_stats() {
        let result = compile("# Hello\n", &CompileOptions::default());
        assert!(result.stats.parse_ms >= 0.0);
        assert!(result.stats.transform_ms >= 0.0);
        assert!(result.stats.emit_ms >= 0.0);
    }

    #[test]
    fn e2e_diagnostics_empty_for_valid() {
        let result = compile("# Hello\n", &CompileOptions::default());
        assert!(result.diagnostics.is_empty());
    }

    #[test]
    fn e2e_html_escaping() {
        let html = compile_html("Hello <script>alert('xss')</script>\n");
        // With default Disallow policy, raw HTML is stripped/escaped
        assert!(!html.contains("<script>"));
    }

    #[test]
    fn e2e_output_hast() {
        let result = compile(
            "# Hello\n",
            &CompileOptions {
                output_kind: OutputKind::Hast,
                ..Default::default()
            },
        );
        assert!(matches!(result.output, Output::Hast(_)));
    }

    #[test]
    fn e2e_output_mdast() {
        let result = compile(
            "# Hello\n",
            &CompileOptions {
                output_kind: OutputKind::Mdast,
                ..Default::default()
            },
        );
        assert!(matches!(result.output, Output::Mdast(_)));
    }

    #[test]
    fn e2e_blockquote() {
        let html = compile_html("> quoted text\n");
        assert!(html.contains("<blockquote>"));
        assert!(html.contains("quoted text"));
        assert!(html.contains("</blockquote>"));
    }

    #[test]
    fn e2e_unordered_list() {
        let html = compile_html("- item 1\n- item 2\n");
        assert!(html.contains("<ul>"));
        assert!(html.contains("<li>"));
        assert!(html.contains("item 1"));
        assert!(html.contains("item 2"));
    }

    #[test]
    fn e2e_ordered_list() {
        let html = compile_html("1. first\n2. second\n");
        assert!(html.contains("<ol>"));
        assert!(html.contains("<li>"));
        assert!(html.contains("first"));
        assert!(html.contains("second"));
    }

    #[test]
    fn e2e_inline_code() {
        let html = compile_html("Use `println!` macro\n");
        assert!(html.contains("<code>println!</code>"));
    }

    #[test]
    fn e2e_horizontal_rule() {
        let html = compile_html("---\n");
        assert!(html.contains("<hr"));
    }

    #[test]
    fn e2e_heading_with_slug() {
        let html = compile_html("# My Title\n");
        // Slugs should produce id attribute on heading
        assert!(html.contains("id=\"my-title\""));
    }

    #[test]
    fn e2e_multiple_paragraphs() {
        let html = compile_html("First paragraph\n\nSecond paragraph\n");
        // Should have two separate <p> elements
        let p_count = html.matches("<p>").count();
        assert!(p_count >= 2, "expected 2+ paragraphs, got {}", p_count);
    }

    #[test]
    fn e2e_sanitize_disabled() {
        let result = compile(
            "# Hello\n",
            &CompileOptions {
                sanitize: SanitizeOptions {
                    enabled: false,
                    schema: None,
                },
                ..Default::default()
            },
        );
        match result.output {
            Output::Html(html) => assert!(html.contains("<h1")),
            _ => panic!("expected HTML output"),
        }
    }
}
