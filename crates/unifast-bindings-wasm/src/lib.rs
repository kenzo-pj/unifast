use unifast_core::api::compile;
use unifast_core::api::options::{
    AbbrOptions, AccessibleEmojiOptions, AddClassesOptions, AlertIconDef, AutolinkHeadingsBehavior,
    AutolinkHeadingsOptions, BreaksOptions, CjkOptions, CodeImportOptions, CodeMetaOptions,
    CommentRemovalOptions, CompileOptions, CustomHeadingIdOptions, DefinitionListOptions,
    DirectiveOptions, EmojiOptions, ExcerptOptions, ExternalLinksOptions, FigureOptions,
    FrontmatterOptions, GfmOptions, GithubAlertIconMode, GithubAlertOptions, HighlightEngine,
    HighlightOptions, ImgLazyLoadingOptions, InputKind, LineNumberOptions, MathOptions,
    MinifyOptions, OutputKind, RawHtmlPolicy, ReadingTimeOptions, RubyAnnotationOptions,
    SanitizeOptions, SanitizeSchema, SectionizeOptions, SlugMode, SlugOptions, SmartypantsOptions,
    TocOptions, WikiLinkOptions,
};
use unifast_core::api::result::Output;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[must_use]
pub fn compile_to_html(input: &str) -> String {
    let opts = CompileOptions::default();
    let result = compile::compile(input, &opts);
    match result.output {
        Output::Html(html) => html,
        _ => String::new(),
    }
}

#[wasm_bindgen]
#[must_use]
pub fn compile_with_options(input: &str, options_json: &str) -> String {
    let opts = parse_options_from_json(options_json);
    let result = compile::compile(input, &opts);

    let output_str = match &result.output {
        Output::Html(html) => html.clone(),
        Output::MdxJs { code, .. } => code.clone(),
        Output::Hast(root) => {
            serde_json::to_string(root).unwrap_or_else(|e| format!("{{\"error\":\"{e}\"}}"))
        }
        Output::Mdast(doc) => format!("{doc:#?}"),
    };

    let frontmatter_json = serde_json::to_string(&result.frontmatter).unwrap_or_default();
    let diagnostics_json: Vec<serde_json::Value> = result
        .diagnostics
        .iter()
        .map(|d| {
            serde_json::json!({
                "level": match d.level {
                    unifast_core::diagnostics::diagnostic::DiagLevel::Error => "error",
                    unifast_core::diagnostics::diagnostic::DiagLevel::Warning => "warn",
                },
                "message": d.message,
                "start": d.span.start,
                "end": d.span.end,
            })
        })
        .collect();

    let toc_json: Vec<serde_json::Value> = result
        .toc
        .iter()
        .map(|e| {
            serde_json::json!({
                "depth": e.depth,
                "text": e.text,
                "slug": e.slug,
            })
        })
        .collect();

    serde_json::json!({
        "output": output_str,
        "frontmatter": serde_json::from_str::<serde_json::Value>(&frontmatter_json).unwrap_or_default(),
        "diagnostics": diagnostics_json,
        "stats": {
            "parseMs": result.stats.parse_ms,
            "transformMs": result.stats.transform_ms,
            "emitMs": result.stats.emit_ms,
        },
        "toc": toc_json,
    })
    .to_string()
}

#[derive(serde::Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
struct WasmOpts {
    input_kind: Option<String>,
    output_kind: Option<String>,
    raw_html: Option<String>,
    highlight: Option<WasmHighlightOpts>,
    gfm: Option<WasmGfmOpts>,
    frontmatter: Option<WasmFrontmatterOpts>,
    sanitize: Option<WasmSanitizeOpts>,
    line_numbers: Option<WasmToggle>,
    slug: Option<WasmSlugOpts>,
    toc: Option<WasmTocOpts>,
    external_links: Option<WasmExternalLinksOpts>,
    autolink_headings: Option<WasmAutolinkHeadingsOpts>,
    sectionize: Option<WasmToggle>,
    breaks: Option<WasmToggle>,
    smartypants: Option<WasmSmartypantsOpts>,
    emoji: Option<WasmToggle>,
    github_alert: Option<WasmGithubAlertOpts>,
    math: Option<WasmToggle>,
    directive: Option<WasmToggle>,
    wiki_link: Option<WasmWikiLinkOpts>,
    definition_list: Option<WasmToggle>,
    ruby_annotation: Option<WasmToggle>,
    cjk: Option<WasmToggle>,
    code_import: Option<WasmCodeImportOpts>,
    code_meta: Option<WasmToggle>,
    figure: Option<WasmToggle>,
    custom_heading_id: Option<WasmToggle>,
    reading_time: Option<WasmReadingTimeOpts>,
    excerpt: Option<WasmExcerptOpts>,
    abbr: Option<WasmToggle>,
    comment_removal: Option<WasmToggle>,
    img_lazy_loading: Option<WasmImgLazyLoadingOpts>,
    accessible_emoji: Option<WasmToggle>,
    add_classes: Option<WasmAddClassesOpts>,
    minify: Option<WasmToggle>,
}

#[derive(serde::Deserialize, Default)]
#[serde(default)]
struct WasmToggle {
    enabled: Option<bool>,
}

#[derive(serde::Deserialize, Default)]
#[serde(default)]
struct WasmHighlightOpts {
    enabled: Option<bool>,
    engine: Option<String>,
}

#[derive(serde::Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
struct WasmGfmOpts {
    tables: Option<bool>,
    task_list: Option<bool>,
    strikethrough: Option<bool>,
    footnotes: Option<bool>,
    autolink: Option<bool>,
}

#[derive(serde::Deserialize, Default)]
#[serde(default)]
struct WasmFrontmatterOpts {
    yaml: Option<bool>,
    toml: Option<bool>,
    json: Option<bool>,
}

#[derive(serde::Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
struct WasmSanitizeOpts {
    enabled: Option<bool>,
    schema: Option<WasmSanitizeSchema>,
}

#[derive(serde::Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
struct WasmSanitizeSchema {
    allowed_tags: Option<Vec<String>>,
    allowed_attributes: Option<std::collections::HashMap<String, Vec<String>>>,
    allowed_protocols: Option<std::collections::HashMap<String, Vec<String>>>,
}

#[derive(serde::Deserialize, Default)]
#[serde(default)]
struct WasmSlugOpts {
    mode: Option<String>,
}

#[derive(serde::Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
struct WasmTocOpts {
    enabled: Option<bool>,
    max_depth: Option<u32>,
}

#[derive(serde::Deserialize, Default)]
#[serde(default)]
struct WasmExternalLinksOpts {
    enabled: Option<bool>,
    rel: Option<String>,
    target: Option<String>,
}

#[derive(serde::Deserialize, Default)]
#[serde(default)]
struct WasmAutolinkHeadingsOpts {
    enabled: Option<bool>,
    behavior: Option<String>,
}

#[derive(serde::Deserialize, Default)]
#[serde(default)]
struct WasmSmartypantsOpts {
    enabled: Option<bool>,
    quotes: Option<bool>,
    dashes: Option<bool>,
    ellipses: Option<bool>,
}

#[derive(serde::Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
struct WasmGithubAlertOpts {
    enabled: Option<bool>,
    icons: Option<String>,
    custom_icons: Option<std::collections::HashMap<String, WasmAlertIconDef>>,
}

#[derive(serde::Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
struct WasmAlertIconDef {
    svg: Option<String>,
    import_source: Option<String>,
    import_name: Option<String>,
}

#[derive(serde::Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
struct WasmWikiLinkOpts {
    enabled: Option<bool>,
    href_template: Option<String>,
}

#[derive(serde::Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
struct WasmCodeImportOpts {
    enabled: Option<bool>,
    root_dir: Option<String>,
}

#[derive(serde::Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
struct WasmReadingTimeOpts {
    enabled: Option<bool>,
    words_per_minute: Option<u32>,
    cjk_chars_per_minute: Option<u32>,
}

#[derive(serde::Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
struct WasmExcerptOpts {
    enabled: Option<bool>,
    separator: Option<String>,
    fallback_paragraphs: Option<u32>,
    fallback_characters: Option<u32>,
}

#[derive(serde::Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
struct WasmImgLazyLoadingOpts {
    enabled: Option<bool>,
    skip_first: Option<u32>,
}

#[derive(serde::Deserialize, Default)]
#[serde(default)]
struct WasmAddClassesRule {
    selector: String,
    classes: String,
}

#[derive(serde::Deserialize, Default)]
#[serde(default)]
struct WasmAddClassesOpts {
    enabled: Option<bool>,
    rules: Option<Vec<WasmAddClassesRule>>,
}

fn parse_options_from_json(json: &str) -> CompileOptions {
    let parsed: WasmOpts = serde_json::from_str(json).unwrap_or_default();

    CompileOptions {
        input_kind: match parsed.input_kind.as_deref() {
            Some("mdx") => InputKind::Mdx,
            _ => InputKind::Markdown,
        },
        output_kind: match parsed.output_kind.as_deref() {
            Some("hast") => OutputKind::Hast,
            Some("mdast") => OutputKind::Mdast,
            Some("mdxJs") => OutputKind::MdxJs,
            _ => OutputKind::Html,
        },
        raw_html: match parsed.raw_html.as_deref() {
            Some("allowDangerous") => RawHtmlPolicy::AllowDangerous,
            Some("parseAndSanitize") => RawHtmlPolicy::ParseAndSanitize,
            _ => RawHtmlPolicy::Disallow,
        },
        highlight: if let Some(h) = parsed.highlight {
            HighlightOptions {
                enabled: h.enabled.unwrap_or(false),
                engine: match h.engine.as_deref() {
                    Some("syntect") => HighlightEngine::Syntect,
                    Some("treeSitter") => HighlightEngine::TreeSitter,
                    _ => HighlightEngine::None,
                },
            }
        } else {
            HighlightOptions::default()
        },
        gfm: if let Some(g) = parsed.gfm {
            GfmOptions {
                tables: g.tables.unwrap_or(true),
                task_list: g.task_list.unwrap_or(true),
                strikethrough: g.strikethrough.unwrap_or(true),
                footnotes: g.footnotes.unwrap_or(true),
                autolink: g.autolink.unwrap_or(true),
            }
        } else {
            GfmOptions::default()
        },
        frontmatter: if let Some(f) = parsed.frontmatter {
            FrontmatterOptions {
                yaml: f.yaml.unwrap_or(false),
                toml: f.toml.unwrap_or(false),
                json: f.json.unwrap_or(false),
            }
        } else {
            FrontmatterOptions::default()
        },
        sanitize: if let Some(s) = parsed.sanitize {
            SanitizeOptions {
                enabled: s.enabled.unwrap_or(true),
                schema: s.schema.map(|sc| SanitizeSchema {
                    allowed_tags: sc.allowed_tags.unwrap_or_default(),
                    allowed_attributes: sc.allowed_attributes.unwrap_or_default(),
                    allowed_protocols: sc.allowed_protocols.unwrap_or_default(),
                }),
            }
        } else {
            SanitizeOptions::default()
        },
        line_numbers: LineNumberOptions {
            enabled: parsed.line_numbers.and_then(|l| l.enabled).unwrap_or(false),
        },
        slug: if let Some(s) = parsed.slug {
            SlugOptions {
                mode: match s.mode.as_deref() {
                    Some("unicode") => SlugMode::Unicode,
                    _ => SlugMode::GitHub,
                },
            }
        } else {
            SlugOptions::default()
        },
        toc: if let Some(t) = parsed.toc {
            TocOptions {
                enabled: t.enabled.unwrap_or(false),
                max_depth: t.max_depth.map_or(6, |d| d as u8),
            }
        } else {
            TocOptions::default()
        },
        external_links: if let Some(el) = parsed.external_links {
            ExternalLinksOptions {
                enabled: el.enabled.unwrap_or(false),
                rel: el.rel.unwrap_or_else(|| "noopener noreferrer".to_string()),
                target: el.target,
            }
        } else {
            ExternalLinksOptions::default()
        },
        autolink_headings: if let Some(ah) = parsed.autolink_headings {
            AutolinkHeadingsOptions {
                enabled: ah.enabled.unwrap_or(false),
                behavior: match ah.behavior.as_deref() {
                    Some("append") => AutolinkHeadingsBehavior::Append,
                    Some("wrap") => AutolinkHeadingsBehavior::Wrap,
                    _ => AutolinkHeadingsBehavior::Prepend,
                },
            }
        } else {
            AutolinkHeadingsOptions::default()
        },
        sectionize: SectionizeOptions {
            enabled: parsed.sectionize.and_then(|s| s.enabled).unwrap_or(false),
        },
        breaks: BreaksOptions {
            enabled: parsed.breaks.and_then(|b| b.enabled).unwrap_or(false),
        },
        smartypants: if let Some(sp) = parsed.smartypants {
            SmartypantsOptions {
                enabled: sp.enabled.unwrap_or(false),
                quotes: sp.quotes.unwrap_or(true),
                dashes: sp.dashes.unwrap_or(true),
                ellipses: sp.ellipses.unwrap_or(true),
            }
        } else {
            SmartypantsOptions::default()
        },
        emoji: EmojiOptions {
            enabled: parsed.emoji.and_then(|e| e.enabled).unwrap_or(false),
        },
        github_alert: if let Some(ga) = parsed.github_alert {
            let icons = match ga.icons.as_deref() {
                Some("none") => GithubAlertIconMode::None,
                Some("octicon") | None => {
                    if let Some(custom) = ga.custom_icons {
                        let map = custom
                            .into_iter()
                            .map(|(k, v)| {
                                (
                                    k,
                                    AlertIconDef {
                                        svg: v.svg,
                                        import_source: v.import_source,
                                        import_name: v.import_name,
                                    },
                                )
                            })
                            .collect();
                        GithubAlertIconMode::Custom(map)
                    } else {
                        GithubAlertIconMode::Octicon
                    }
                }
                _ => GithubAlertIconMode::Octicon,
            };
            GithubAlertOptions {
                enabled: ga.enabled.unwrap_or(false),
                icons,
            }
        } else {
            GithubAlertOptions::default()
        },
        math: MathOptions {
            enabled: parsed.math.and_then(|m| m.enabled).unwrap_or(false),
        },
        directive: DirectiveOptions {
            enabled: parsed.directive.and_then(|d| d.enabled).unwrap_or(false),
        },
        wiki_link: if let Some(wl) = parsed.wiki_link {
            WikiLinkOptions {
                enabled: wl.enabled.unwrap_or(false),
                href_template: wl
                    .href_template
                    .unwrap_or_else(|| "/wiki/{slug}".to_string()),
            }
        } else {
            WikiLinkOptions::default()
        },
        definition_list: DefinitionListOptions {
            enabled: parsed
                .definition_list
                .and_then(|d| d.enabled)
                .unwrap_or(false),
        },
        ruby_annotation: RubyAnnotationOptions {
            enabled: parsed
                .ruby_annotation
                .and_then(|r| r.enabled)
                .unwrap_or(false),
        },
        cjk: CjkOptions {
            enabled: parsed.cjk.and_then(|c| c.enabled).unwrap_or(false),
        },
        code_import: if let Some(ci) = parsed.code_import {
            CodeImportOptions {
                enabled: ci.enabled.unwrap_or(false),
                root_dir: ci.root_dir,
            }
        } else {
            CodeImportOptions::default()
        },
        code_meta: CodeMetaOptions {
            enabled: parsed.code_meta.and_then(|c| c.enabled).unwrap_or(false),
        },
        figure: FigureOptions {
            enabled: parsed.figure.and_then(|f| f.enabled).unwrap_or(false),
        },
        custom_heading_id: CustomHeadingIdOptions {
            enabled: parsed
                .custom_heading_id
                .and_then(|c| c.enabled)
                .unwrap_or(false),
        },
        reading_time_opts: if let Some(rt) = parsed.reading_time {
            ReadingTimeOptions {
                enabled: rt.enabled.unwrap_or(false),
                words_per_minute: rt.words_per_minute.unwrap_or(200),
                cjk_chars_per_minute: rt.cjk_chars_per_minute.unwrap_or(500),
            }
        } else {
            ReadingTimeOptions::default()
        },
        excerpt_opts: if let Some(ex) = parsed.excerpt {
            ExcerptOptions {
                enabled: ex.enabled.unwrap_or(false),
                separator: ex.separator.unwrap_or_else(|| "<!-- more -->".to_string()),
                fallback_paragraphs: ex.fallback_paragraphs.or(Some(1)),
                fallback_characters: ex.fallback_characters,
            }
        } else {
            ExcerptOptions::default()
        },
        abbr: AbbrOptions {
            enabled: parsed.abbr.and_then(|a| a.enabled).unwrap_or(false),
        },
        comment_removal: CommentRemovalOptions {
            enabled: parsed
                .comment_removal
                .and_then(|c| c.enabled)
                .unwrap_or(false),
        },
        img_lazy_loading: if let Some(il) = parsed.img_lazy_loading {
            ImgLazyLoadingOptions {
                enabled: il.enabled.unwrap_or(false),
                skip_first: il.skip_first.unwrap_or(0),
            }
        } else {
            ImgLazyLoadingOptions::default()
        },
        accessible_emoji: AccessibleEmojiOptions {
            enabled: parsed
                .accessible_emoji
                .and_then(|a| a.enabled)
                .unwrap_or(false),
        },
        add_classes: if let Some(ac) = parsed.add_classes {
            AddClassesOptions {
                enabled: ac.enabled.unwrap_or(false),
                rules: ac
                    .rules
                    .map(|r| {
                        r.into_iter()
                            .map(|rule| (rule.selector, rule.classes))
                            .collect()
                    })
                    .unwrap_or_default(),
            }
        } else {
            AddClassesOptions::default()
        },
        minify: MinifyOptions {
            enabled: parsed.minify.and_then(|m| m.enabled).unwrap_or(false),
        },
        ..Default::default()
    }
}
