use super::options::{CompileOptions, InputKind, OutputKind};
use super::result::{CompileResult, CompileStats, Output};
use crate::ast::common::NodeIdGen;
use crate::emit::html::stringify;
use crate::parse;
use crate::transform::pass::{AstPayload, PassContext};
use crate::transform::passes::{
    autolink_headings_pass::AutolinkHeadingsPass, breaks_pass::BreaksPass, cjk_pass::CjkPass,
    cleanup_pass::CleanupPass, code_import_pass::CodeImportPass,
    definition_list_pass::DefinitionListPass, directive_pass::DirectivePass, emoji_pass::EmojiPass,
    external_links_pass::ExternalLinksPass, github_alert_pass::GithubAlertPass,
    highlight_pass::HighlightPass, line_number_pass::LineNumberPass, lower_pass::LowerPass,
    math_pass::MathPass, normalize_pass::NormalizePass, resolve_defs_pass::ResolveDefsPass,
    ruby_annotation_pass::RubyAnnotationPass, sanitize_pass::SanitizePass,
    sectionize_pass::SectionizePass, slug_pass::SlugPass, smartypants_pass::SmartypantsPass,
    toc_pass::TocPass, wiki_link_pass::WikiLinkPass,
};
use crate::transform::registry::PassRegistry;
use std::time::Instant;

#[must_use]
pub fn compile(input: &str, opts: &CompileOptions) -> CompileResult {
    let start = Instant::now();

    let parse_start = Instant::now();
    let parse_result = match opts.input_kind {
        InputKind::Markdown => parse::parse_markdown(input),
        InputKind::Mdx => parse::parse_mdx_input(input),
    };
    let document = parse_result.document;
    let mut diagnostics = parse_result.diagnostics;
    let frontmatter = parse_result.frontmatter;
    let parse_ms = parse_start.elapsed().as_secs_f64() * 1000.0;

    let transform_start = Instant::now();
    let mut registry = PassRegistry::new();

    registry.register(Box::new(NormalizePass));
    registry.register(Box::new(BreaksPass));
    registry.register(Box::new(CjkPass));
    registry.register(Box::new(SmartypantsPass));
    registry.register(Box::new(EmojiPass));
    registry.register(Box::new(MathPass));
    registry.register(Box::new(DirectivePass));
    registry.register(Box::new(WikiLinkPass));
    registry.register(Box::new(DefinitionListPass));
    registry.register(Box::new(RubyAnnotationPass));
    registry.register(Box::new(GithubAlertPass));
    registry.register(Box::new(CodeImportPass));
    registry.register(Box::new(SlugPass));
    registry.register(Box::new(TocPass));
    registry.register(Box::new(ResolveDefsPass));

    if opts.output_kind != OutputKind::Mdast {
        registry.register(Box::new(LowerPass));
        if opts.sanitize.enabled {
            registry.register(Box::new(SanitizePass));
        }
        if opts.highlight.enabled {
            registry.register(Box::new(HighlightPass));
        }
        if opts.line_numbers.enabled {
            registry.register(Box::new(LineNumberPass));
        }
        registry.register(Box::new(ExternalLinksPass));
        registry.register(Box::new(AutolinkHeadingsPass));
        registry.register(Box::new(SectionizePass));
        registry.register(Box::new(CleanupPass));
    }

    for plugin in &opts.plugins {
        plugin.apply(&mut registry);
    }

    let mut id_gen = NodeIdGen::new();
    let mut payload = AstPayload::Mdast(document);

    let toc;
    {
        let mut ctx = PassContext {
            source: input,
            diagnostics: &mut diagnostics,
            options: opts,
            id_gen: &mut id_gen,
            toc: Vec::new(),
        };

        for pass in registry.ordered_passes() {
            if let Err(_e) = pass.run(&mut ctx, &mut payload) {
                break;
            }
        }
        toc = std::mem::take(&mut ctx.toc);
    }

    let transform_ms = transform_start.elapsed().as_secs_f64() * 1000.0;

    let emit_start = Instant::now();
    let output = match opts.output_kind {
        OutputKind::Html => {
            let html = match &payload {
                AstPayload::Hast(root) => stringify::stringify(root),
                AstPayload::Both { hast, .. } => stringify::stringify(hast),
                _ => String::new(),
            };
            Output::Html(html)
        }
        OutputKind::Hast => match payload {
            AstPayload::Hast(root) => Output::Hast(root),
            AstPayload::Both { hast, .. } => Output::Hast(hast),
            _ => Output::Html(String::new()),
        },
        OutputKind::Mdast => match payload {
            AstPayload::Mdast(doc) => Output::Mdast(doc),
            _ => Output::Mdast(crate::ast::mdast::nodes::Document {
                id: crate::ast::common::NodeId(0),
                span: crate::ast::common::Span::empty(),
                children: vec![],
            }),
        },
        OutputKind::MdxJs => {
            let mdx_result = parse::parse_mdx_input(input);
            let mut mdx_doc = mdx_result.document;
            let slug_mode = match opts.slug.mode {
                crate::api::options::SlugMode::GitHub => {
                    crate::transform::passes::slug::SlugMode::GitHub
                }
                crate::api::options::SlugMode::Unicode => {
                    crate::transform::passes::slug::SlugMode::Unicode
                }
            };
            crate::transform::passes::slug::apply_slugs(&mut mdx_doc, slug_mode);
            let mdx_output = crate::emit::mdx_js::printer::print_mdx_js(&mdx_doc);
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

    let _ = start;

    CompileResult {
        output,
        frontmatter,
        diagnostics: diagnostics.into_diagnostics(),
        stats: CompileStats {
            parse_ms,
            transform_ms,
            emit_ms,
        },
        toc,
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
        assert!(html.contains("id=\"my-title\""));
    }

    #[test]
    fn e2e_multiple_paragraphs() {
        let html = compile_html("First paragraph\n\nSecond paragraph\n");
        let p_count = html.matches("<p>").count();
        assert!(p_count >= 2, "expected 2+ paragraphs, got {p_count}");
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

    #[test]
    fn e2e_custom_plugin_adds_pass() {
        use crate::transform::pass::{AstPayload, Pass, PassContext, PassResult, Phase};
        use crate::transform::plugin::Plugin;
        use crate::transform::registry::PassRegistry;

        struct UppercasePass;
        impl Pass for UppercasePass {
            fn name(&self) -> &'static str {
                "uppercase"
            }
            fn phase(&self) -> Phase {
                Phase::Optimize
            }
            fn run(&self, _ctx: &mut PassContext, ast: &mut AstPayload) -> PassResult {
                fn uppercase_hast(node: &mut crate::ast::hast::nodes::HNode) {
                    match node {
                        crate::ast::hast::nodes::HNode::Text(t) => {
                            t.value = t.value.to_uppercase();
                        }
                        crate::ast::hast::nodes::HNode::Element(e) => {
                            for child in &mut e.children {
                                uppercase_hast(child);
                            }
                        }
                        crate::ast::hast::nodes::HNode::Root(r) => {
                            for child in &mut r.children {
                                uppercase_hast(child);
                            }
                        }
                        _ => {}
                    }
                }
                if let AstPayload::Hast(root) = ast {
                    for child in &mut root.children {
                        uppercase_hast(child);
                    }
                }
                Ok(())
            }
        }

        struct UppercasePlugin;
        impl Plugin for UppercasePlugin {
            fn name(&self) -> &'static str {
                "uppercase_plugin"
            }
            fn apply(&self, registry: &mut PassRegistry) {
                registry.register(Box::new(UppercasePass));
            }
        }

        let result = compile(
            "hello world\n",
            &CompileOptions {
                plugins: vec![Box::new(UppercasePlugin)],
                ..Default::default()
            },
        );

        match result.output {
            Output::Html(html) => {
                assert!(
                    html.contains("HELLO WORLD"),
                    "Expected uppercase, got: {html}"
                );
            }
            _ => panic!("expected HTML"),
        }
    }

    #[test]
    fn e2e_toc_extraction() {
        let result = compile(
            "# First\n\n## Second\n\n### Third\n",
            &CompileOptions {
                toc: TocOptions {
                    enabled: true,
                    max_depth: 6,
                },
                ..Default::default()
            },
        );
        assert_eq!(result.toc.len(), 3);
        assert_eq!(result.toc[0].depth, 1);
        assert_eq!(result.toc[0].text, "First");
        assert_eq!(result.toc[0].slug, "first");
        assert_eq!(result.toc[1].depth, 2);
        assert_eq!(result.toc[1].text, "Second");
        assert_eq!(result.toc[2].depth, 3);
        assert_eq!(result.toc[2].text, "Third");
    }

    #[test]
    fn e2e_line_numbers() {
        let result = compile(
            "```rust\nfn main() {\n    println!(\"hi\");\n}\n```\n",
            &CompileOptions {
                line_numbers: LineNumberOptions { enabled: true },
                ..Default::default()
            },
        );
        match result.output {
            Output::Html(html) => {
                assert!(html.contains("data-line=\"1\""), "missing line 1: {html}");
                assert!(html.contains("data-line=\"2\""), "missing line 2: {html}");
                assert!(html.contains("data-line=\"3\""), "missing line 3: {html}");
                assert!(
                    html.contains("class=\"line\""),
                    "missing line class: {html}"
                );
            }
            _ => panic!("expected HTML output"),
        }
    }

    #[test]
    fn e2e_line_numbers_disabled_by_default() {
        let html = compile_html("```\nhello\n```\n");
        assert!(!html.contains("data-line"));
    }

    #[test]
    fn e2e_highlight_with_line_numbers() {
        let result = compile(
            "```rust\nfn main() {\n    println!(\"hi\");\n}\n```\n",
            &CompileOptions {
                highlight: HighlightOptions {
                    enabled: true,
                    engine: HighlightEngine::Syntect,
                },
                line_numbers: LineNumberOptions { enabled: true },
                ..Default::default()
            },
        );
        match result.output {
            Output::Html(html) => {
                assert!(
                    html.contains("sy-"),
                    "missing syntax highlight spans: {html}"
                );
                assert!(html.contains("data-line=\"1\""), "missing line 1: {html}");
                assert!(html.contains("data-line=\"2\""), "missing line 2: {html}");
                assert!(html.contains("data-line=\"3\""), "missing line 3: {html}");
                assert!(
                    html.contains("class=\"line\""),
                    "missing line class: {html}"
                );
            }
            _ => panic!("expected HTML output"),
        }
    }

    #[test]
    fn e2e_highlight_without_line_numbers() {
        let result = compile(
            "```rust\nfn main() {}\n```\n",
            &CompileOptions {
                highlight: HighlightOptions {
                    enabled: true,
                    engine: HighlightEngine::Syntect,
                },
                ..Default::default()
            },
        );
        match result.output {
            Output::Html(html) => {
                assert!(html.contains("sy-"), "missing syntax highlight: {html}");
                assert!(!html.contains("data-line"), "should not have line numbers");
            }
            _ => panic!("expected HTML output"),
        }
    }

    #[test]
    fn e2e_toc_disabled_by_default() {
        let result = compile("# Heading\n", &CompileOptions::default());
        assert!(result.toc.is_empty());
    }

    #[test]
    fn e2e_toc_max_depth() {
        let result = compile(
            "# H1\n\n## H2\n\n### H3\n",
            &CompileOptions {
                toc: TocOptions {
                    enabled: true,
                    max_depth: 2,
                },
                ..Default::default()
            },
        );
        assert_eq!(result.toc.len(), 2);
    }

    #[test]
    fn e2e_highlight_with_tree_sitter() {
        let result = compile(
            "```rust\nfn main() {\n    println!(\"hi\");\n}\n```\n",
            &CompileOptions {
                highlight: HighlightOptions {
                    enabled: true,
                    engine: HighlightEngine::TreeSitter,
                },
                ..Default::default()
            },
        );
        match result.output {
            Output::Html(html) => {
                assert!(
                    html.contains("ts-"),
                    "expected tree-sitter ts- classes in output: {html}"
                );
                assert!(
                    html.contains("<span"),
                    "expected span elements in output: {html}"
                );
            }
            _ => panic!("expected HTML output"),
        }
    }

    #[test]
    fn e2e_tree_sitter_with_line_numbers() {
        let result = compile(
            "```rust\nfn main() {\n    println!(\"hi\");\n}\n```\n",
            &CompileOptions {
                highlight: HighlightOptions {
                    enabled: true,
                    engine: HighlightEngine::TreeSitter,
                },
                line_numbers: LineNumberOptions { enabled: true },
                ..Default::default()
            },
        );
        match result.output {
            Output::Html(html) => {
                assert!(
                    html.contains("ts-"),
                    "missing tree-sitter highlight: {html}"
                );
                assert!(html.contains("data-line=\"1\""), "missing line 1: {html}");
            }
            _ => panic!("expected HTML output"),
        }
    }
}
