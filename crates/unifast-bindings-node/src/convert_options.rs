use napi_derive::napi;
use unifast_core::api::options::*;

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
pub struct JsCompileOptions {
    pub input_kind: Option<String>,
    pub output_kind: Option<String>,
    pub gfm: Option<JsGfmOptions>,
    pub frontmatter: Option<JsFrontmatterOptions>,
    pub raw_html: Option<String>,
    pub sanitize: Option<JsSanitizeOptions>,
    pub highlight: Option<JsHighlightOptions>,
    pub slug: Option<JsSlugOptions>,
    pub toc: Option<JsTocOptions>,
    pub diagnostics: Option<JsDiagnosticsOptions>,
    pub cache: Option<JsCacheOptions>,
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
                    Some("builtin") => HighlightEngine::Builtin,
                    _ => HighlightEngine::None,
                },
            }
        } else {
            HighlightOptions::default()
        },
        ..Default::default()
    }
}
