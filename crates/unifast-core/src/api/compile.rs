use super::options::{CompileOptions, InputKind, OutputKind};
use super::result::{CompileResult, CompileStats, Output};
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
    abbr, accessible_emoji, add_classes, autolink_headings, breaks, cjk, code_import, code_meta,
    comment_removal, custom_heading_id, definition_list, emoji, excerpt, external_links,
    github_alert,
    html_cleanup::{self, CleanupOptions},
    img_lazy_loading, line_number, math, mdast_to_hast, minify, normalize, raw_html, reading_time,
    resolve_defs, ruby_annotation, sanitize, sectionize, slug, smartypants, toc, wiki_link,
};
use crate::transform::registry::PassRegistry;
use std::time::Instant;

#[must_use]
pub fn compile(input: &str, opts: &CompileOptions) -> CompileResult {
    let start = Instant::now();

    let parse_start = Instant::now();
    let parse_result = match opts.input_kind {
        InputKind::Markdown => parse::parse_markdown(input, &opts.gfm, &opts.frontmatter),
        InputKind::Mdx => parse::parse_mdx_input(input, &opts.gfm, &opts.frontmatter),
    };
    let document = parse_result.document;
    let mut diagnostics = parse_result.diagnostics;
    let frontmatter = parse_result.frontmatter;
    let mut id_gen = parse_result.id_gen;
    let parse_ms = parse_start.elapsed().as_secs_f64() * 1000.0;

    let transform_start = Instant::now();
    let mut registry = PassRegistry::new();

    register_builtin_passes(&mut registry, opts);

    let mut payload = AstPayload::Mdast(document);

    let toc;
    let reading_time;
    let excerpt;
    let mut pass_failed = false;
    {
        let mut ctx = PassContext {
            source: input,
            diagnostics: &mut diagnostics,
            options: opts,
            id_gen: &mut id_gen,
            toc: Vec::new(),
            reading_time: None,
            excerpt: None,
        };

        for pass in registry.ordered_passes() {
            if let Err(e) = pass.run(&mut ctx, &mut payload) {
                ctx.diagnostics.error(
                    format!("pass '{}' failed: {e}", pass.name()),
                    Span::new(0, 0),
                );
                pass_failed = true;
                break;
            }
        }
        toc = std::mem::take(&mut ctx.toc);
        reading_time = ctx.reading_time.take();
        excerpt = ctx.excerpt.take();
    }

    let transform_ms = transform_start.elapsed().as_secs_f64() * 1000.0;

    let emit_start = Instant::now();
    let output = if pass_failed {
        Output::Html(String::new())
    } else {
        match opts.output_kind {
            OutputKind::Html => {
                let html = match &payload {
                    AstPayload::Hast(root) => {
                        if opts.minify.enabled {
                            stringify::stringify_minified(root)
                        } else {
                            stringify::stringify(root)
                        }
                    }
                    AstPayload::Both { hast, .. } => {
                        if opts.minify.enabled {
                            stringify::stringify_minified(hast)
                        } else {
                            stringify::stringify(hast)
                        }
                    }
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
                let mdx_doc = match payload {
                    AstPayload::Mdast(doc) => doc,
                    AstPayload::Both { mdast, .. } => mdast,
                    _ => parse::parse_mdx_input(input, &opts.gfm, &opts.frontmatter).document,
                };

                #[cfg(feature = "highlight")]
                let highlight_fn: Option<
                    Box<crate::emit::mdx_js::printer::HighlightFn>,
                > = if opts.highlight.enabled {
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
                let highlight_fn: Option<
                    Box<crate::emit::mdx_js::printer::HighlightFn>,
                > = None;

                let mdx_output = crate::emit::mdx_js::printer::print_mdx_js_full(
                    &mdx_doc,
                    highlight_fn.as_deref(),
                    &opts.github_alert.icons,
                    opts.line_numbers.enabled,
                    &opts.wiki_link.href_template,
                );
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
        reading_time,
        excerpt,
    }
}

fn register_builtin_passes(registry: &mut PassRegistry, opts: &CompileOptions) {
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
        if ctx.options.directive.enabled {
            return Ok(());
        }
        if let Some(doc) = ast.mdast_mut() {
            strip_directives(&mut doc.children);
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

    registry.register_fn("abbr", Phase::Transform, |ctx, ast| {
        if !ctx.options.abbr.enabled {
            return Ok(());
        }
        if let Some(doc) = ast.mdast_mut() {
            abbr::apply_abbr(doc, ctx.id_gen);
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
            code_import::apply_code_import(&mut doc.children, root_dir, ctx.diagnostics);
        }
        Ok(())
    });

    registry.register_fn("custom_heading_id", Phase::Transform, |ctx, ast| {
        if !ctx.options.custom_heading_id.enabled {
            return Ok(());
        }
        if let Some(doc) = ast.mdast_mut() {
            custom_heading_id::apply_custom_heading_ids(doc);
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

    registry.register_fn("reading_time", Phase::Transform, |ctx, ast| {
        if !ctx.options.reading_time_opts.enabled {
            return Ok(());
        }
        if let Some(doc) = ast.mdast_mut() {
            ctx.reading_time = Some(reading_time::calculate(
                doc,
                ctx.options.reading_time_opts.words_per_minute,
                ctx.options.reading_time_opts.cjk_chars_per_minute,
            ));
        }
        Ok(())
    });

    registry.register_fn("excerpt", Phase::Transform, |ctx, ast| {
        if !ctx.options.excerpt_opts.enabled {
            return Ok(());
        }
        if let Some(doc) = ast.mdast_mut() {
            ctx.excerpt = excerpt::extract_excerpt(
                doc,
                &ctx.options.excerpt_opts.separator,
                ctx.options.excerpt_opts.fallback_paragraphs,
                ctx.options.excerpt_opts.fallback_characters,
            );
        }
        Ok(())
    });

    registry.register_fn("comment_removal", Phase::Transform, |ctx, ast| {
        if !ctx.options.comment_removal.enabled {
            return Ok(());
        }
        if let Some(doc) = ast.mdast_mut() {
            comment_removal::remove_comments(&mut doc.children);
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

    if opts.output_kind != OutputKind::Mdast && opts.output_kind != OutputKind::MdxJs {
        registry.register_fn("mdast_to_hast", Phase::Lower, |ctx, ast| {
            let doc = match ast {
                AstPayload::Mdast(doc) => doc,
                AstPayload::Both { mdast, .. } => mdast,
                AstPayload::Hast(_) => return Ok(()),
            };
            let hast_node = mdast_to_hast::lower_with_icons(
                doc,
                ctx.id_gen,
                ctx.options.raw_html,
                ctx.diagnostics,
                &ctx.options.github_alert.icons,
                ctx.options.figure.enabled,
                &ctx.options.wiki_link.href_template,
            );
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

        registry.register_fn("raw_html", Phase::Optimize, |ctx, ast| {
            if let Some(root) = ast.hast_mut() {
                raw_html::process_raw_html(root, ctx.options.raw_html, ctx.id_gen, ctx.diagnostics);
            }
            Ok(())
        });

        if opts.sanitize.enabled {
            registry.register_fn("sanitize", Phase::Optimize, |ctx, ast| {
                let custom_schema;
                let schema = if let Some(ref api_schema) = ctx.options.sanitize.schema {
                    custom_schema = sanitize::from_api_schema(api_schema);
                    &custom_schema
                } else {
                    sanitize::default_safe_schema()
                };
                if let Some(root) = ast.hast_mut() {
                    sanitize::sanitize(root, schema, ctx.diagnostics);
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

        registry.register_fn("code_meta", Phase::Optimize, |ctx, ast| {
            if !ctx.options.code_meta.enabled {
                return Ok(());
            }
            if let Some(root) = ast.hast_mut() {
                code_meta::apply_code_meta(root, ctx.id_gen);
            }
            Ok(())
        });

        registry.register_fn("img_lazy_loading", Phase::Optimize, |ctx, ast| {
            if !ctx.options.img_lazy_loading.enabled {
                return Ok(());
            }
            if let Some(root) = ast.hast_mut() {
                img_lazy_loading::apply_img_lazy_loading(
                    root,
                    ctx.options.img_lazy_loading.skip_first,
                );
            }
            Ok(())
        });

        registry.register_fn("accessible_emoji", Phase::Optimize, |ctx, ast| {
            if !ctx.options.accessible_emoji.enabled {
                return Ok(());
            }
            if let Some(root) = ast.hast_mut() {
                accessible_emoji::apply_accessible_emoji(root, ctx.id_gen);
            }
            Ok(())
        });

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

        registry.register_fn("add_classes", Phase::Optimize, |ctx, ast| {
            if !ctx.options.add_classes.enabled {
                return Ok(());
            }
            if let Some(root) = ast.hast_mut() {
                add_classes::apply_add_classes(root, &ctx.options.add_classes.rules);
            }
            Ok(())
        });

        registry.register_fn("html_cleanup", Phase::Optimize, |ctx, ast| {
            let cleanup_opts = &ctx.options.html_cleanup;
            if !cleanup_opts.remove_empty_nodes && !cleanup_opts.minify_whitespace {
                return Ok(());
            }
            if let Some(root) = ast.hast_mut() {
                html_cleanup::cleanup(
                    root,
                    &CleanupOptions {
                        remove_empty_nodes: cleanup_opts.remove_empty_nodes,
                        minify_whitespace: cleanup_opts.minify_whitespace,
                    },
                );
            }
            Ok(())
        });

        registry.register_fn("minify", Phase::Optimize, |ctx, ast| {
            if !ctx.options.minify.enabled {
                return Ok(());
            }
            if let Some(root) = ast.hast_mut() {
                minify::minify_hast(root);
            }
            Ok(())
        });
    }

    for plugin in &opts.plugins {
        plugin.apply(registry);
    }
}

fn strip_directives(children: &mut Vec<crate::ast::mdast::nodes::MdNode>) {
    use crate::ast::mdast::nodes::MdNode;
    children.retain(|node| {
        !matches!(
            node,
            MdNode::ContainerDirective(_) | MdNode::LeafDirective(_) | MdNode::TextDirective(_)
        )
    });
    for child in children.iter_mut() {
        if let Some(kids) = child.children_mut() {
            strip_directives(kids);
        }
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
        assert!(html.contains("<pre"), "missing <pre: {html}");
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
            &CompileOptions {
                frontmatter: crate::api::options::FrontmatterOptions::yaml_only(),
                ..CompileOptions::default()
            },
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

    #[test]
    fn e2e_github_alert_with_octicon_icons() {
        let result = compile(
            "> [!NOTE]\n> This is a note.\n",
            &CompileOptions {
                github_alert: GithubAlertOptions {
                    enabled: true,
                    icons: crate::api::options::GithubAlertIconMode::Octicon,
                },
                raw_html: RawHtmlPolicy::AllowDangerous,
                ..Default::default()
            },
        );
        match result.output {
            Output::Html(html) => {
                assert!(
                    html.contains("alert alert-note"),
                    "missing alert class: {html}"
                );
                assert!(html.contains("alert-title"), "missing alert-title: {html}");
                assert!(
                    html.contains("alert-icon"),
                    "missing alert-icon svg: {html}"
                );
                assert!(html.contains("<svg"), "missing svg element: {html}");
                assert!(html.contains("This is a note"), "missing content: {html}");
            }
            _ => panic!("expected HTML output"),
        }
    }

    #[test]
    fn e2e_github_alert_no_icons() {
        let result = compile(
            "> [!WARNING]\n> Be careful.\n",
            &CompileOptions {
                github_alert: GithubAlertOptions {
                    enabled: true,
                    icons: crate::api::options::GithubAlertIconMode::None,
                },
                raw_html: RawHtmlPolicy::AllowDangerous,
                ..Default::default()
            },
        );
        match result.output {
            Output::Html(html) => {
                assert!(
                    html.contains("alert alert-warning"),
                    "missing alert class: {html}"
                );
                assert!(!html.contains("<svg"), "should not contain svg: {html}");
            }
            _ => panic!("expected HTML output"),
        }
    }

    #[test]
    fn e2e_github_alert_custom_svg_icon() {
        use std::collections::HashMap;
        let mut custom = HashMap::new();
        custom.insert(
            "note".to_string(),
            crate::api::options::AlertIconDef {
                svg: Some("<svg class=\"custom-icon\"><circle r=\"5\"/></svg>".to_string()),
                import_source: None,
                import_name: None,
            },
        );
        let result = compile(
            "> [!NOTE]\n> Custom icon test.\n",
            &CompileOptions {
                github_alert: GithubAlertOptions {
                    enabled: true,
                    icons: crate::api::options::GithubAlertIconMode::Custom(custom),
                },
                raw_html: RawHtmlPolicy::AllowDangerous,
                sanitize: SanitizeOptions {
                    enabled: false,
                    schema: None,
                },
                ..Default::default()
            },
        );
        match result.output {
            Output::Html(html) => {
                assert!(html.contains("custom-icon"), "missing custom icon: {html}");
                assert!(
                    html.contains("<circle"),
                    "missing custom svg content: {html}"
                );
            }
            _ => panic!("expected HTML output"),
        }
    }

    #[test]
    fn e2e_github_alert_mdx_js_output() {
        let result = compile(
            "> [!TIP]\n> A helpful tip.\n",
            &CompileOptions {
                input_kind: InputKind::Mdx,
                output_kind: OutputKind::MdxJs,
                github_alert: GithubAlertOptions {
                    enabled: true,
                    icons: crate::api::options::GithubAlertIconMode::Octicon,
                },
                ..Default::default()
            },
        );
        match result.output {
            Output::MdxJs { code, .. } => {
                assert!(
                    code.contains("alert alert-tip"),
                    "missing alert class: {code}"
                );
                assert!(code.contains("alert-title"), "missing alert-title: {code}");
                assert!(code.contains("alert-icon"), "missing alert-icon: {code}");
            }
            _ => panic!("expected MdxJs output"),
        }
    }

    #[test]
    fn e2e_container_directive() {
        let result = compile(
            ":::note\nThis is a note.\n:::\n",
            &CompileOptions {
                directive: DirectiveOptions { enabled: true },
                sanitize: SanitizeOptions {
                    enabled: false,
                    schema: None,
                },
                ..Default::default()
            },
        );
        match result.output {
            Output::Html(html) => {
                assert!(
                    html.contains("directive directive-note"),
                    "missing class: {html}"
                );
                assert!(
                    html.contains("data-directive=\"note\""),
                    "missing data attr: {html}"
                );
                assert!(html.contains("This is a note."), "missing content: {html}");
            }
            _ => panic!("expected HTML output"),
        }
    }

    #[test]
    fn e2e_container_directive_with_title() {
        let result = compile(
            ":::warning title=\"Deprecation Notice\"\nThis API is deprecated.\n:::\n",
            &CompileOptions {
                directive: DirectiveOptions { enabled: true },
                sanitize: SanitizeOptions {
                    enabled: false,
                    schema: None,
                },
                ..Default::default()
            },
        );
        match result.output {
            Output::Html(html) => {
                assert!(html.contains("directive-warning"), "missing class: {html}");
                assert!(
                    html.contains("directive-title"),
                    "missing title class: {html}"
                );
                assert!(
                    html.contains("Deprecation Notice"),
                    "missing title text: {html}"
                );
                assert!(
                    html.contains("This API is deprecated."),
                    "missing content: {html}"
                );
            }
            _ => panic!("expected HTML output"),
        }
    }

    #[test]
    fn e2e_github_alert_mdx_js_npm_import() {
        use std::collections::HashMap;
        let mut custom = HashMap::new();
        custom.insert(
            "note".to_string(),
            crate::api::options::AlertIconDef {
                svg: None,
                import_source: Some("lucide-react".to_string()),
                import_name: Some("Info".to_string()),
            },
        );
        let result = compile(
            "> [!NOTE]\n> Import test.\n",
            &CompileOptions {
                input_kind: InputKind::Mdx,
                output_kind: OutputKind::MdxJs,
                github_alert: GithubAlertOptions {
                    enabled: true,
                    icons: crate::api::options::GithubAlertIconMode::Custom(custom),
                },
                ..Default::default()
            },
        );
        match result.output {
            Output::MdxJs { code, .. } => {
                assert!(
                    code.contains("import { Info as _AlertIcon_note } from 'lucide-react'"),
                    "missing import statement: {code}"
                );
                assert!(
                    code.contains("_AlertIcon_note"),
                    "missing icon component reference: {code}"
                );
            }
            _ => panic!("expected MdxJs output"),
        }
    }

    #[test]
    fn e2e_code_meta_title() {
        let mut opts = CompileOptions::default();
        opts.code_meta.enabled = true;
        let result = compile("```js title=\"app.ts\"\nconsole.log('hi')\n```\n", &opts);
        let html = match result.output {
            Output::Html(h) => h,
            _ => panic!("expected HTML output"),
        };
        assert!(
            html.contains("data-title=\"app.ts\""),
            "missing data-title: {html}"
        );
    }

    #[test]
    fn e2e_code_meta_word_wrap() {
        let mut opts = CompileOptions::default();
        opts.code_meta.enabled = true;
        let result = compile("```js wordWrap\nconsole.log('hi')\n```\n", &opts);
        let html = match result.output {
            Output::Html(h) => h,
            _ => panic!("expected HTML output"),
        };
        assert!(
            html.contains("data-word-wrap"),
            "missing data-word-wrap: {html}"
        );
    }

    #[test]
    fn e2e_code_meta_removes_data_meta() {
        let mut opts = CompileOptions::default();
        opts.code_meta.enabled = true;
        let result = compile("```js title=\"test\"\nhello\n```\n", &opts);
        let html = match result.output {
            Output::Html(h) => h,
            _ => panic!("expected HTML output"),
        };
        assert!(
            !html.contains("data-meta"),
            "data-meta should be removed: {html}"
        );
    }

    #[test]
    fn e2e_code_meta_disabled_by_default() {
        let result = compile(
            "```js title=\"app.ts\"\nconsole.log('hi')\n```\n",
            &CompileOptions::default(),
        );
        let html = match result.output {
            Output::Html(h) => h,
            _ => panic!("expected HTML output"),
        };
        assert!(
            !html.contains("data-title"),
            "data-title should not be present when code_meta is disabled: {html}"
        );
    }

    #[test]
    fn e2e_code_meta_data_lang() {
        let mut opts = CompileOptions::default();
        opts.code_meta.enabled = true;
        let result = compile("```rust title=\"main.rs\"\nfn main() {}\n```\n", &opts);
        let html = match result.output {
            Output::Html(h) => h,
            _ => panic!("expected HTML output"),
        };
        assert!(
            html.contains("data-lang=\"rust\""),
            "missing data-lang: {html}"
        );
        assert!(
            html.contains("data-title=\"main.rs\""),
            "missing data-title: {html}"
        );
    }

    #[test]
    fn e2e_custom_heading_id() {
        let mut opts = CompileOptions::default();
        opts.custom_heading_id.enabled = true;
        let result = compile("## My Heading {#custom-id}\n", &opts);
        let html = match result.output {
            Output::Html(h) => h,
            _ => panic!("expected HTML output"),
        };
        assert!(
            html.contains("id=\"custom-id\""),
            "missing custom id: {html}"
        );
    }

    #[test]
    fn e2e_custom_heading_class() {
        let mut opts = CompileOptions::default();
        opts.custom_heading_id.enabled = true;
        let result = compile("## Heading {.note}\n", &opts);
        let html = match result.output {
            Output::Html(h) => h,
            _ => panic!("expected HTML output"),
        };
        assert!(html.contains("class=\"note\""), "missing class: {html}");
    }

    #[test]
    fn e2e_custom_heading_id_with_attrs() {
        let mut opts = CompileOptions::default();
        opts.custom_heading_id.enabled = true;
        opts.sanitize = SanitizeOptions {
            enabled: false,
            schema: None,
        };
        let result = compile("## Title {#my-id .warning data-level=2}\n", &opts);
        let html = match result.output {
            Output::Html(h) => h,
            _ => panic!("expected HTML output"),
        };
        assert!(html.contains("id=\"my-id\""), "missing custom id: {html}");
        assert!(html.contains("class=\"warning\""), "missing class: {html}");
        assert!(
            html.contains("data-level=\"2\""),
            "missing data attr: {html}"
        );
    }

    #[test]
    fn e2e_custom_heading_id_disabled_by_default() {
        let result = compile("## My Heading {#custom-id}\n", &CompileOptions::default());
        let html = match result.output {
            Output::Html(h) => h,
            _ => panic!("expected HTML output"),
        };
        assert!(
            !html.contains("id=\"custom-id\""),
            "custom id should not be set when disabled: {html}"
        );
    }

    #[test]
    fn e2e_custom_heading_id_preserves_text() {
        let mut opts = CompileOptions::default();
        opts.custom_heading_id.enabled = true;
        let result = compile("## My Heading {#custom-id}\n", &opts);
        let html = match result.output {
            Output::Html(h) => h,
            _ => panic!("expected HTML output"),
        };
        assert!(
            html.contains("My Heading"),
            "heading text should be preserved: {html}"
        );
        assert!(
            !html.contains("{#custom-id}"),
            "brace block should be stripped: {html}"
        );
    }

    #[test]
    fn e2e_figure_with_alt() {
        let mut opts = CompileOptions::default();
        opts.figure.enabled = true;
        let result = compile("![sunset](sunset.jpg)\n", &opts);
        let html = match result.output {
            Output::Html(h) => h,
            _ => panic!(),
        };
        assert!(html.contains("<figure>"), "missing figure: {html}");
        assert!(
            html.contains("<figcaption>sunset</figcaption>"),
            "missing figcaption: {html}"
        );
        assert!(html.contains("</figure>"), "missing closing figure: {html}");
    }

    #[test]
    fn e2e_figure_without_alt() {
        let mut opts = CompileOptions::default();
        opts.figure.enabled = true;
        let result = compile("![](logo.png)\n", &opts);
        let html = match result.output {
            Output::Html(h) => h,
            _ => panic!(),
        };
        assert!(html.contains("<figure>"), "missing figure: {html}");
        assert!(
            !html.contains("<figcaption>"),
            "should not have figcaption: {html}"
        );
    }

    #[test]
    fn e2e_figure_disabled_by_default() {
        let result = compile("![sunset](sunset.jpg)\n", &CompileOptions::default());
        let html = match result.output {
            Output::Html(h) => h,
            _ => panic!(),
        };
        assert!(
            !html.contains("<figure>"),
            "figure should not appear by default: {html}"
        );
    }

    #[test]
    fn e2e_reading_time() {
        let mut opts = CompileOptions::default();
        opts.reading_time_opts.enabled = true;
        let result = compile("Hello world this is a test with some words.\n", &opts);
        assert!(result.reading_time.is_some());
        let rt = result.reading_time.unwrap();
        assert!(rt.words > 0);
        assert!(rt.minutes >= 1.0);
    }

    #[test]
    fn e2e_reading_time_disabled_by_default() {
        let result = compile("Hello world\n", &CompileOptions::default());
        assert!(result.reading_time.is_none());
    }

    #[test]
    fn e2e_excerpt_with_marker() {
        let mut opts = CompileOptions::default();
        opts.excerpt_opts.enabled = true;
        let result = compile(
            "This is the intro.\n\n<!-- more -->\n\nThis is the rest.\n",
            &opts,
        );
        assert_eq!(result.excerpt.as_deref(), Some("This is the intro."));
    }

    #[test]
    fn e2e_excerpt_fallback() {
        let mut opts = CompileOptions::default();
        opts.excerpt_opts.enabled = true;
        let result = compile("First paragraph.\n\nSecond paragraph.\n", &opts);
        assert_eq!(result.excerpt.as_deref(), Some("First paragraph."));
    }

    #[test]
    fn e2e_excerpt_disabled_by_default() {
        let result = compile(
            "This is the intro.\n\n<!-- more -->\n\nThis is the rest.\n",
            &CompileOptions::default(),
        );
        assert!(result.excerpt.is_none());
    }

    #[test]
    fn e2e_excerpt_fallback_characters() {
        let mut opts = CompileOptions::default();
        opts.excerpt_opts.enabled = true;
        opts.excerpt_opts.fallback_paragraphs = None;
        opts.excerpt_opts.fallback_characters = Some(20);
        let result = compile("Hello world this is a long text without marker.\n", &opts);
        let excerpt = result.excerpt.unwrap();
        assert!(excerpt.len() <= 20, "excerpt too long: {excerpt}");
        assert_eq!(excerpt, "Hello world this is");
    }

    #[test]
    fn e2e_abbr() {
        let mut opts = CompileOptions::default();
        opts.abbr.enabled = true;
        let input = "*[HTML]: Hyper Text Markup Language\n\nHTML is great.\n";
        let result = compile(input, &opts);
        let html = match result.output {
            Output::Html(h) => h,
            _ => panic!("expected HTML output"),
        };
        assert!(
            html.contains("<abbr title=\"Hyper Text Markup Language\">HTML</abbr>"),
            "missing abbr: {html}"
        );
        assert!(
            !html.contains("*[HTML]"),
            "definition should be removed: {html}"
        );
    }

    #[test]
    fn e2e_abbr_disabled_by_default() {
        let input = "*[HTML]: Hyper Text Markup Language\n\nHTML is great.\n";
        let result = compile(input, &CompileOptions::default());
        let html = match result.output {
            Output::Html(h) => h,
            _ => panic!("expected HTML output"),
        };
        assert!(
            !html.contains("<abbr"),
            "abbr should not appear when disabled: {html}"
        );
    }

    #[test]
    fn e2e_comment_removal() {
        let mut opts = CompileOptions::default();
        opts.comment_removal.enabled = true;
        let result = compile("Hello\n\n<!-- this is a comment -->\n\nWorld\n", &opts);
        let html = match result.output {
            Output::Html(h) => h,
            _ => panic!("expected HTML output"),
        };
        assert!(
            !html.contains("comment"),
            "comment should be removed: {html}"
        );
        assert!(html.contains("Hello"));
        assert!(html.contains("World"));
    }

    #[test]
    fn e2e_comment_removal_disabled_by_default() {
        let result = compile(
            "Hello\n\n<!-- this is a comment -->\n\nWorld\n",
            &CompileOptions::default(),
        );
        let html = match result.output {
            Output::Html(h) => h,
            _ => panic!("expected HTML output"),
        };
        assert!(html.contains("Hello"));
        assert!(html.contains("World"));
    }

    #[test]
    fn e2e_comment_removal_preserves_non_comment_html() {
        let mut opts = CompileOptions::default();
        opts.comment_removal.enabled = true;
        opts.raw_html = crate::api::options::RawHtmlPolicy::AllowDangerous;
        let result = compile(
            "Hello\n\n<div>keep me</div>\n\n<!-- remove me -->\n\nWorld\n",
            &opts,
        );
        let html = match result.output {
            Output::Html(h) => h,
            _ => panic!("expected HTML output"),
        };
        assert!(
            !html.contains("remove me"),
            "comment should be removed: {html}"
        );
        assert!(
            html.contains("keep me"),
            "non-comment HTML should remain: {html}"
        );
        assert!(html.contains("Hello"));
        assert!(html.contains("World"));
    }

    #[test]
    fn e2e_abbr_multiple_terms() {
        let mut opts = CompileOptions::default();
        opts.abbr.enabled = true;
        let input = "*[HTML]: Hyper Text Markup Language\n\n*[CSS]: Cascading Style Sheets\n\nHTML and CSS are great.\n";
        let result = compile(input, &opts);
        let html = match result.output {
            Output::Html(h) => h,
            _ => panic!("expected HTML output"),
        };
        assert!(
            html.contains("<abbr title=\"Hyper Text Markup Language\">HTML</abbr>"),
            "missing HTML abbr: {html}"
        );
        assert!(
            html.contains("<abbr title=\"Cascading Style Sheets\">CSS</abbr>"),
            "missing CSS abbr: {html}"
        );
    }

    #[test]
    fn e2e_img_lazy_loading() {
        let mut opts = CompileOptions::default();
        opts.img_lazy_loading.enabled = true;
        let result = compile("![alt](test.jpg)\n", &opts);
        let html = match result.output {
            Output::Html(h) => h,
            _ => panic!("expected HTML output"),
        };
        assert!(html.contains("loading=\"lazy\""), "missing lazy: {html}");
        assert!(html.contains("decoding=\"async\""), "missing async: {html}");
    }

    #[test]
    fn e2e_img_lazy_loading_skip_first() {
        let mut opts = CompileOptions::default();
        opts.img_lazy_loading.enabled = true;
        opts.img_lazy_loading.skip_first = 1;
        let result = compile("![first](a.jpg)\n\n![second](b.jpg)\n", &opts);
        let html = match result.output {
            Output::Html(h) => h,
            _ => panic!("expected HTML output"),
        };
        assert_eq!(
            html.matches("loading=\"lazy\"").count(),
            1,
            "should have exactly one lazy: {html}"
        );
    }

    #[test]
    fn e2e_img_lazy_loading_disabled_by_default() {
        let opts = CompileOptions::default();
        let result = compile("![alt](test.jpg)\n", &opts);
        let html = match result.output {
            Output::Html(h) => h,
            _ => panic!("expected HTML output"),
        };
        assert!(
            !html.contains("loading=\"lazy\""),
            "lazy loading should not be present when disabled: {html}"
        );
    }

    #[test]
    fn e2e_accessible_emoji() {
        let mut opts = CompileOptions::default();
        opts.accessible_emoji.enabled = true;
        let result = compile("Hello \u{1F680} world\n", &opts);
        let html = match result.output {
            Output::Html(h) => h,
            _ => panic!("expected HTML output"),
        };
        assert!(html.contains("role=\"img\""), "missing role=img: {html}");
        assert!(
            html.contains("aria-label=\"rocket\""),
            "missing aria-label: {html}"
        );
    }

    #[test]
    fn e2e_accessible_emoji_disabled_by_default() {
        let opts = CompileOptions::default();
        let result = compile("Hello \u{1F680} world\n", &opts);
        let html = match result.output {
            Output::Html(h) => h,
            _ => panic!("expected HTML output"),
        };
        assert!(
            !html.contains("role=\"img\""),
            "accessible emoji should not be present when disabled: {html}"
        );
    }

    #[test]
    fn e2e_add_classes_tag() {
        let mut opts = CompileOptions::default();
        opts.add_classes.enabled = true;
        opts.add_classes.rules = vec![("h1".to_string(), "text-3xl font-bold".to_string())];
        let result = compile("# Hello\n", &opts);
        let html = match result.output {
            Output::Html(h) => h,
            _ => panic!("expected HTML output"),
        };
        assert!(
            html.contains("text-3xl font-bold"),
            "missing classes: {html}"
        );
    }

    #[test]
    fn e2e_add_classes_disabled_by_default() {
        let opts = CompileOptions::default();
        let result = compile("# Hello\n", &opts);
        let html = match result.output {
            Output::Html(h) => h,
            _ => panic!("expected HTML output"),
        };
        assert!(
            !html.contains("text-3xl"),
            "add_classes should not be active when disabled: {html}"
        );
    }

    #[test]
    fn e2e_add_classes_multiple_selectors() {
        let mut opts = CompileOptions::default();
        opts.add_classes.enabled = true;
        opts.add_classes.rules = vec![
            ("h1, h2".to_string(), "heading".to_string()),
            ("p".to_string(), "prose".to_string()),
        ];
        let result = compile("# Title\n\n## Subtitle\n\nParagraph\n", &opts);
        let html = match result.output {
            Output::Html(h) => h,
            _ => panic!("expected HTML output"),
        };
        assert!(html.contains("heading"), "missing heading class: {html}");
        assert!(html.contains("prose"), "missing prose class: {html}");
    }

    #[test]
    fn e2e_minify() {
        let mut opts = CompileOptions::default();
        opts.minify.enabled = true;
        let result = compile("# Hello\n\nWorld\n", &opts);
        let html = match result.output {
            Output::Html(h) => h,
            _ => panic!("expected HTML output"),
        };
        assert!(!html.contains("</p>"), "should omit </p>: {html}");
        assert!(html.contains("Hello"), "missing content: {html}");
        assert!(html.contains("World"), "missing content: {html}");
    }

    #[test]
    fn e2e_minify_boolean_attr() {
        let mut opts = CompileOptions::default();
        opts.minify.enabled = true;
        let result = compile("- [x] Done\n", &opts);
        let html = match result.output {
            Output::Html(h) => h,
            _ => panic!("expected HTML output"),
        };
        assert!(html.contains("checked"), "missing checked: {html}");
        assert!(
            !html.contains("checked=\""),
            "should not have checked= with value: {html}"
        );
    }

    #[test]
    fn e2e_minify_preserves_pre_content() {
        let mut opts = CompileOptions::default();
        opts.minify.enabled = true;
        let result = compile("```\n  indented\n    more\n```\n", &opts);
        let html = match result.output {
            Output::Html(h) => h,
            _ => panic!("expected HTML output"),
        };
        assert!(
            html.contains("  indented"),
            "pre whitespace should be preserved: {html}"
        );
    }

    #[test]
    fn e2e_minify_omits_comments() {
        let mut opts = CompileOptions::default();
        opts.minify.enabled = true;
        opts.raw_html = RawHtmlPolicy::AllowDangerous;
        let result = compile("Hello\n\n<!-- comment -->\n\nWorld\n", &opts);
        let html = match result.output {
            Output::Html(h) => h,
            _ => panic!("expected HTML output"),
        };
        assert!(!html.contains("<!--"), "comments should be removed: {html}");
    }

    #[test]
    fn e2e_minify_disabled_by_default() {
        let result = compile("# Hello\n\nWorld\n", &CompileOptions::default());
        let html = match result.output {
            Output::Html(h) => h,
            _ => panic!("expected HTML output"),
        };
        assert!(
            html.contains("</p>"),
            "non-minified should have </p>: {html}"
        );
    }

    #[test]
    fn pass_failure_emits_diagnostic_and_suppresses_output() {
        let mut opts = CompileOptions::default();
        opts.plugins.push(Box::new(FailingPlugin));
        let result = compile("# Hello\n", &opts);
        assert!(
            result
                .diagnostics
                .iter()
                .any(|d| d.message.contains("failing_pass") && d.message.contains("boom")),
            "should have diagnostic from failing pass: {:?}",
            result.diagnostics
        );
        assert!(
            result
                .diagnostics
                .iter()
                .any(|d| { d.level == crate::diagnostics::diagnostic::DiagLevel::Error }),
            "diagnostic should be error level"
        );
        match &result.output {
            Output::Html(html) => assert!(
                html.is_empty(),
                "should produce empty output on pass failure, got: {html}"
            ),
            _ => panic!("expected HTML output"),
        }
    }

    struct FailingPlugin;
    impl crate::transform::plugin::Plugin for FailingPlugin {
        fn name(&self) -> &'static str {
            "failing_plugin"
        }
        fn apply(&self, registry: &mut crate::transform::registry::PassRegistry) {
            registry.register_fn(
                "failing_pass",
                crate::transform::pass::Phase::Transform,
                |_ctx, _ast| Err(crate::transform::pass::PassError::new("boom")),
            );
        }
    }

    #[test]
    fn frontmatter_yaml_only_rejects_toml() {
        let input = "+++\ntitle = \"Hello\"\n+++\n\n# Content\n";
        let result = compile(
            input,
            &CompileOptions {
                frontmatter: crate::api::options::FrontmatterOptions::yaml_only(),
                ..CompileOptions::default()
            },
        );
        assert!(
            result.frontmatter.is_empty(),
            "TOML frontmatter should not be extracted when only yaml is enabled"
        );
    }

    #[test]
    fn frontmatter_disabled_extracts_nothing() {
        let input = "---\ntitle: Hello\n---\n\n# Content\n";
        let result = compile(input, &CompileOptions::default());
        assert!(
            result.frontmatter.is_empty(),
            "frontmatter should not be extracted when all formats are disabled"
        );
    }
}
