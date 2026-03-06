use super::options::{CompileOptions, InputKind, OutputKind};
use super::result::{CompileResult, CompileStats, Output};
use crate::ast::common::NodeIdGen;
use crate::ast::common::Span;
use crate::ast::hast::nodes::{HNode, HRoot};
use crate::emit::html::stringify;
use crate::parse;
use crate::transform::pass::{AstPayload, PassContext, Phase};
#[cfg(feature = "highlight")]
use crate::transform::passes::highlight;
#[cfg(feature = "highlight")]
use crate::transform::passes::highlight::HighlightEngine as _;
use crate::transform::passes::{
    autolink_headings, breaks, cjk, code_import, definition_list, directive, emoji, external_links,
    github_alert,
    html_cleanup::{self, CleanupOptions},
    line_number, math, mdast_to_hast, normalize, resolve_defs, ruby_annotation, sanitize,
    sectionize, slug, smartypants, toc, wiki_link,
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

    registry.register_fn("normalize", Phase::Transform, |_ctx, ast| {
        if let Some(doc) = ast.mdast_mut() {
            normalize::normalize(doc);
        }
        Ok(())
    });

    registry.register_fn("breaks", Phase::Transform, |ctx, ast| {
        if !ctx.options.breaks.enabled {
            return Ok(());
        }
        if let Some(doc) = ast.mdast_mut() {
            breaks::apply_breaks(&mut doc.children, ctx.id_gen);
        }
        Ok(())
    });

    registry.register_fn("cjk", Phase::Transform, |ctx, ast| {
        if !ctx.options.cjk.enabled {
            return Ok(());
        }
        if let Some(doc) = ast.mdast_mut() {
            cjk::apply_cjk(&mut doc.children);
        }
        Ok(())
    });

    registry.register_fn("smartypants", Phase::Transform, |ctx, ast| {
        if !ctx.options.smartypants.enabled {
            return Ok(());
        }
        if let Some(doc) = ast.mdast_mut() {
            smartypants::apply_smartypants(
                doc,
                ctx.options.smartypants.quotes,
                ctx.options.smartypants.dashes,
                ctx.options.smartypants.ellipses,
            );
        }
        Ok(())
    });

    registry.register_fn("emoji", Phase::Transform, |ctx, ast| {
        if !ctx.options.emoji.enabled {
            return Ok(());
        }
        if let Some(doc) = ast.mdast_mut() {
            emoji::apply_emoji(doc);
        }
        Ok(())
    });

    registry.register_fn("math", Phase::Transform, |ctx, ast| {
        if !ctx.options.math.enabled {
            return Ok(());
        }
        if let Some(doc) = ast.mdast_mut() {
            math::apply_math(&mut doc.children, ctx.id_gen);
        }
        Ok(())
    });

    registry.register_fn("directive", Phase::Transform, |ctx, ast| {
        if !ctx.options.directive.enabled {
            return Ok(());
        }
        if let Some(doc) = ast.mdast_mut() {
            directive::apply_directives(&mut doc.children, ctx.id_gen);
        }
        Ok(())
    });

    registry.register_fn("wiki_link", Phase::Transform, |ctx, ast| {
        if !ctx.options.wiki_link.enabled {
            return Ok(());
        }
        if let Some(doc) = ast.mdast_mut() {
            wiki_link::apply_wiki_links(&mut doc.children, ctx.id_gen);
        }
        Ok(())
    });

    registry.register_fn("definition_list", Phase::Transform, |ctx, ast| {
        if !ctx.options.definition_list.enabled {
            return Ok(());
        }
        if let Some(doc) = ast.mdast_mut() {
            definition_list::apply_definition_lists(&mut doc.children, ctx.id_gen);
        }
        Ok(())
    });

    registry.register_fn("ruby_annotation", Phase::Transform, |ctx, ast| {
        if !ctx.options.ruby_annotation.enabled {
            return Ok(());
        }
        if let Some(doc) = ast.mdast_mut() {
            ruby_annotation::apply_ruby(&mut doc.children, ctx.id_gen);
        }
        Ok(())
    });

    registry.register_fn("github_alert", Phase::Transform, |ctx, ast| {
        if !ctx.options.github_alert.enabled {
            return Ok(());
        }
        if let Some(doc) = ast.mdast_mut() {
            github_alert::apply_github_alerts(doc, ctx.id_gen);
        }
        Ok(())
    });

    registry.register_fn("code_import", Phase::Transform, |ctx, ast| {
        if !ctx.options.code_import.enabled {
            return Ok(());
        }
        let root_dir = ctx.options.code_import.root_dir.as_deref();
        if let Some(doc) = ast.mdast_mut() {
            code_import::apply_code_import(&mut doc.children, root_dir);
        }
        Ok(())
    });

    registry.register_fn("slug", Phase::Transform, |ctx, ast| {
        if let Some(doc) = ast.mdast_mut() {
            let mode = match ctx.options.slug.mode {
                crate::api::options::SlugMode::GitHub => slug::SlugMode::GitHub,
                crate::api::options::SlugMode::Unicode => slug::SlugMode::Unicode,
            };
            slug::apply_slugs(doc, mode);
        }
        Ok(())
    });

    registry.register_fn("toc", Phase::Transform, |ctx, ast| {
        if !ctx.options.toc.enabled {
            return Ok(());
        }
        let max_depth = ctx.options.toc.max_depth;
        if let Some(doc) = ast.mdast_mut() {
            ctx.toc = toc::generate_toc(doc, max_depth);
        }
        Ok(())
    });

    registry.register_fn("resolve_defs", Phase::Transform, |ctx, ast| {
        if let Some(doc) = ast.mdast_mut() {
            let defs = std::collections::HashMap::new();
            resolve_defs::resolve_definitions(doc, &defs, ctx.diagnostics);
        }
        Ok(())
    });

    if opts.output_kind != OutputKind::Mdast {
        registry.register_fn("mdast_to_hast", Phase::Lower, |ctx, ast| {
            let doc = match ast {
                AstPayload::Mdast(doc) => doc,
                AstPayload::Both { mdast, .. } => mdast,
                AstPayload::Hast(_) => return Ok(()),
            };
            let hast_node =
                mdast_to_hast::lower(doc, ctx.id_gen, ctx.options.raw_html, ctx.diagnostics);
            let hast_root = match hast_node {
                HNode::Root(root) => root,
                other => HRoot {
                    id: ctx.id_gen.next_id(),
                    span: Span::empty(),
                    children: vec![other],
                },
            };
            *ast = AstPayload::Hast(hast_root);
            Ok(())
        });

        if opts.sanitize.enabled {
            registry.register_fn("sanitize", Phase::Optimize, |ctx, ast| {
                let schema = if let Some(ref api_schema) = ctx.options.sanitize.schema {
                    sanitize::from_api_schema(api_schema)
                } else {
                    sanitize::default_safe_schema()
                };
                if let Some(root) = ast.hast_mut() {
                    sanitize::sanitize(root, &schema, ctx.diagnostics);
                }
                Ok(())
            });
        }

        #[cfg(feature = "highlight")]
        if opts.highlight.enabled {
            registry.register_fn("highlight", Phase::Optimize, |ctx, ast| {
                let engine: Box<dyn highlight::HighlightEngine> = match ctx.options.highlight.engine
                {
                    crate::api::options::HighlightEngine::Syntect => {
                        Box::new(highlight::SyntectHighlighter::new())
                    }
                    crate::api::options::HighlightEngine::TreeSitter => {
                        Box::new(highlight::TreeSitterHighlighter)
                    }
                    crate::api::options::HighlightEngine::None => return Ok(()),
                };
                if let Some(root) = ast.hast_mut() {
                    highlight::apply_highlight(root, engine.as_ref(), ctx.id_gen);
                }
                Ok(())
            });
        }

        if opts.line_numbers.enabled {
            registry.register_fn("line_number", Phase::Optimize, |ctx, ast| {
                if let Some(root) = ast.hast_mut() {
                    line_number::apply_line_numbers(root, ctx.id_gen);
                }
                Ok(())
            });
        }

        registry.register_fn("external_links", Phase::Optimize, |ctx, ast| {
            if !ctx.options.external_links.enabled {
                return Ok(());
            }
            if let Some(root) = ast.hast_mut() {
                external_links::apply_external_links(
                    root,
                    &ctx.options.external_links.rel,
                    ctx.options.external_links.target.as_deref(),
                );
            }
            Ok(())
        });

        registry.register_fn("autolink_headings", Phase::Optimize, |ctx, ast| {
            if !ctx.options.autolink_headings.enabled {
                return Ok(());
            }
            if let Some(root) = ast.hast_mut() {
                autolink_headings::apply_autolink_headings(
                    root,
                    ctx.options.autolink_headings.behavior,
                    ctx.id_gen,
                );
            }
            Ok(())
        });

        registry.register_fn("sectionize", Phase::Optimize, |ctx, ast| {
            if !ctx.options.sectionize.enabled {
                return Ok(());
            }
            if let Some(root) = ast.hast_mut() {
                sectionize::apply_sectionize(root, ctx.id_gen);
            }
            Ok(())
        });

        registry.register_fn("html_cleanup", Phase::Optimize, |_ctx, ast| {
            if let Some(root) = ast.hast_mut() {
                html_cleanup::cleanup(root, &CleanupOptions::default());
            }
            Ok(())
        });
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

            #[cfg(feature = "highlight")]
            let highlight_fn: Option<Box<crate::emit::mdx_js::printer::HighlightFn>> =
                if opts.highlight.enabled {
                    match opts.highlight.engine {
                        crate::api::options::HighlightEngine::Syntect => {
                            let engine = highlight::SyntectHighlighter::new();
                            Some(Box::new(move |code: &str, lang: &str| {
                                engine.highlight(code, lang)
                            }))
                        }
                        crate::api::options::HighlightEngine::TreeSitter => {
                            let engine = highlight::TreeSitterHighlighter;
                            Some(Box::new(move |code: &str, lang: &str| {
                                engine.highlight(code, lang)
                            }))
                        }
                        crate::api::options::HighlightEngine::None => None,
                    }
                } else {
                    None
                };
            #[cfg(not(feature = "highlight"))]
            let highlight_fn: Option<Box<crate::emit::mdx_js::printer::HighlightFn>> = None;

            let mdx_output =
                crate::emit::mdx_js::printer::print_mdx_js(&mdx_doc, highlight_fn.as_deref());
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
