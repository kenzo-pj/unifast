use napi_derive::napi;
use unifast_core::api::options::{
    AutolinkHeadingsBehavior, AutolinkHeadingsOptions, BreaksOptions, CjkOptions,
    CodeImportOptions, CompileOptions, DefinitionListOptions, DirectiveOptions, EmojiOptions,
    ExternalLinksOptions, FrontmatterOptions, GfmOptions, GithubAlertOptions, HighlightEngine,
    HighlightOptions, InputKind, LineNumberOptions, MathOptions, OutputKind, RawHtmlPolicy,
    RubyAnnotationOptions, SanitizeOptions, SectionizeOptions, SlugMode, SlugOptions,
    SmartypantsOptions, TocOptions, WikiLinkOptions,
};

#[napi(object)]
pub struct JsGfmOptions {
    pub tables: Option<bool>,
    pub task_list: Option<bool>,
    pub strikethrough: Option<bool>,
    pub footnotes: Option<bool>,
    pub autolink: Option<bool>,
}

#[napi(object)]
pub struct JsFrontmatterOptions {
    pub yaml: Option<bool>,
    pub toml: Option<bool>,
    pub json: Option<bool>,
}

#[napi(object)]
pub struct JsSanitizeOptions {
    pub enabled: Option<bool>,
}

#[napi(object)]
pub struct JsHighlightOptions {
    pub enabled: Option<bool>,
    pub engine: Option<String>,
}

#[napi(object)]
pub struct JsSlugOptions {
    pub mode: Option<String>,
}

#[napi(object)]
pub struct JsLineNumberOptions {
    pub enabled: Option<bool>,
}

#[napi(object)]
pub struct JsTocOptions {
    pub enabled: Option<bool>,
    pub max_depth: Option<u32>,
}

#[napi(object)]
pub struct JsDiagnosticsOptions {
    pub format: Option<String>,
}

#[napi(object)]
pub struct JsCacheOptions {
    pub enabled: Option<bool>,
    pub dir: Option<String>,
}

#[napi(object)]
pub struct JsExternalLinksOptions {
    pub enabled: Option<bool>,
    pub rel: Option<String>,
    pub target: Option<String>,
}

#[napi(object)]
pub struct JsAutolinkHeadingsOptions {
    pub enabled: Option<bool>,
    pub behavior: Option<String>,
}

#[napi(object)]
pub struct JsSmartypantsOptions {
    pub enabled: Option<bool>,
    pub quotes: Option<bool>,
    pub dashes: Option<bool>,
    pub ellipses: Option<bool>,
}

#[napi(object)]
pub struct JsWikiLinkOptions {
    pub enabled: Option<bool>,
    pub href_template: Option<String>,
}

#[napi(object)]
pub struct JsCodeImportOptions {
    pub enabled: Option<bool>,
    pub root_dir: Option<String>,
}

#[napi(object)]
pub struct JsFeatureToggle {
    pub enabled: Option<bool>,
}

#[napi(object)]
pub struct JsCompileOptions {
    pub input_kind: Option<String>,
    pub output_kind: Option<String>,
    pub gfm: Option<JsGfmOptions>,
    pub frontmatter: Option<JsFrontmatterOptions>,
    pub raw_html: Option<String>,
    pub sanitize: Option<JsSanitizeOptions>,
    pub highlight: Option<JsHighlightOptions>,
    pub line_numbers: Option<JsLineNumberOptions>,
    pub slug: Option<JsSlugOptions>,
    pub toc: Option<JsTocOptions>,
    pub diagnostics: Option<JsDiagnosticsOptions>,
    pub cache: Option<JsCacheOptions>,
    pub external_links: Option<JsExternalLinksOptions>,
    pub autolink_headings: Option<JsAutolinkHeadingsOptions>,
    pub sectionize: Option<JsFeatureToggle>,
    pub breaks: Option<JsFeatureToggle>,
    pub smartypants: Option<JsSmartypantsOptions>,
    pub emoji: Option<JsFeatureToggle>,
    pub github_alert: Option<JsFeatureToggle>,
    pub math: Option<JsFeatureToggle>,
    pub directive: Option<JsFeatureToggle>,
    pub wiki_link: Option<JsWikiLinkOptions>,
    pub definition_list: Option<JsFeatureToggle>,
    pub ruby_annotation: Option<JsFeatureToggle>,
    pub cjk: Option<JsFeatureToggle>,
    pub code_import: Option<JsCodeImportOptions>,
}

pub fn convert_options(js_opts: Option<JsCompileOptions>) -> CompileOptions {
    let js = match js_opts {
        Some(o) => o,
        None => return CompileOptions::default(),
    };

    CompileOptions {
        input_kind: match js.input_kind.as_deref() {
            Some("mdx") => InputKind::Mdx,
            _ => InputKind::Markdown,
        },
        output_kind: match js.output_kind.as_deref() {
            Some("hast") => OutputKind::Hast,
            Some("mdast") => OutputKind::Mdast,
            Some("mdxJs") => OutputKind::MdxJs,
            _ => OutputKind::Html,
        },
        gfm: if let Some(g) = js.gfm {
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
        frontmatter: if let Some(f) = js.frontmatter {
            FrontmatterOptions {
                yaml: f.yaml.unwrap_or(false),
                toml: f.toml.unwrap_or(false),
                json: f.json.unwrap_or(false),
            }
        } else {
            FrontmatterOptions::default()
        },
        raw_html: match js.raw_html.as_deref() {
            Some("allowDangerous") => RawHtmlPolicy::AllowDangerous,
            Some("parseAndSanitize") => RawHtmlPolicy::ParseAndSanitize,
            _ => RawHtmlPolicy::Disallow,
        },
        sanitize: if let Some(s) = js.sanitize {
            SanitizeOptions {
                enabled: s.enabled.unwrap_or(true),
                schema: None,
            }
        } else {
            SanitizeOptions::default()
        },
        highlight: if let Some(h) = js.highlight {
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
        line_numbers: if let Some(ln) = js.line_numbers {
            LineNumberOptions {
                enabled: ln.enabled.unwrap_or(false),
            }
        } else {
            LineNumberOptions::default()
        },
        slug: if let Some(s) = js.slug {
            SlugOptions {
                mode: match s.mode.as_deref() {
                    Some("unicode") => SlugMode::Unicode,
                    _ => SlugMode::GitHub,
                },
            }
        } else {
            SlugOptions::default()
        },
        toc: if let Some(t) = js.toc {
            TocOptions {
                enabled: t.enabled.unwrap_or(false),
                max_depth: t.max_depth.map_or(6, |d| d as u8),
            }
        } else {
            TocOptions::default()
        },
        external_links: if let Some(el) = js.external_links {
            ExternalLinksOptions {
                enabled: el.enabled.unwrap_or(false),
                rel: el.rel.unwrap_or_else(|| "noopener noreferrer".to_string()),
                target: el.target,
            }
        } else {
            ExternalLinksOptions::default()
        },
        autolink_headings: if let Some(ah) = js.autolink_headings {
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
            enabled: js.sectionize.and_then(|s| s.enabled).unwrap_or(false),
        },
        breaks: BreaksOptions {
            enabled: js.breaks.and_then(|b| b.enabled).unwrap_or(false),
        },
        smartypants: if let Some(sp) = js.smartypants {
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
            enabled: js.emoji.and_then(|e| e.enabled).unwrap_or(false),
        },
        github_alert: GithubAlertOptions {
            enabled: js.github_alert.and_then(|g| g.enabled).unwrap_or(false),
        },
        math: MathOptions {
            enabled: js.math.and_then(|m| m.enabled).unwrap_or(false),
        },
        directive: DirectiveOptions {
            enabled: js.directive.and_then(|d| d.enabled).unwrap_or(false),
        },
        wiki_link: if let Some(wl) = js.wiki_link {
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
            enabled: js.definition_list.and_then(|d| d.enabled).unwrap_or(false),
        },
        ruby_annotation: RubyAnnotationOptions {
            enabled: js.ruby_annotation.and_then(|r| r.enabled).unwrap_or(false),
        },
        cjk: CjkOptions {
            enabled: js.cjk.and_then(|c| c.enabled).unwrap_or(false),
        },
        code_import: if let Some(ci) = js.code_import {
            CodeImportOptions {
                enabled: ci.enabled.unwrap_or(false),
                root_dir: ci.root_dir,
            }
        } else {
            CodeImportOptions::default()
        },
        ..Default::default()
    }
}
