use super::options::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ─── JSON-deserializable CompileOptions ───────────────────────────────
// Single source of truth for JSON → CompileOptions conversion.
// Used by WASM binding directly; Node binding keeps napi types for TS DX
// but should mirror this logic.

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct JsonCompileOptions {
    pub input_kind: Option<String>,
    pub output_kind: Option<String>,
    pub raw_html: Option<String>,
    pub gfm: Option<JsonGfmOptions>,
    pub frontmatter: Option<JsonFrontmatterOptions>,
    pub sanitize: Option<JsonSanitizeOptions>,
    pub highlight: Option<JsonHighlightOptions>,
    pub line_numbers: Option<JsonToggle>,
    pub slug: Option<JsonSlugOptions>,
    pub toc: Option<JsonTocOptions>,
    pub external_links: Option<JsonExternalLinksOptions>,
    pub autolink_headings: Option<JsonAutolinkHeadingsOptions>,
    pub sectionize: Option<JsonToggle>,
    pub breaks: Option<JsonToggle>,
    pub smartypants: Option<JsonSmartypantsOptions>,
    pub emoji: Option<JsonToggle>,
    pub github_alert: Option<JsonGithubAlertOptions>,
    pub math: Option<JsonToggle>,
    pub directive: Option<JsonToggle>,
    pub wiki_link: Option<JsonWikiLinkOptions>,
    pub definition_list: Option<JsonToggle>,
    pub ruby_annotation: Option<JsonToggle>,
    pub cjk: Option<JsonToggle>,
    pub code_import: Option<JsonCodeImportOptions>,
    pub code_meta: Option<JsonToggle>,
    pub figure: Option<JsonToggle>,
    pub custom_heading_id: Option<JsonToggle>,
    pub reading_time: Option<JsonReadingTimeOptions>,
    pub excerpt: Option<JsonExcerptOptions>,
    pub abbr: Option<JsonToggle>,
    pub comment_removal: Option<JsonToggle>,
    pub img_lazy_loading: Option<JsonImgLazyLoadingOptions>,
    pub accessible_emoji: Option<JsonToggle>,
    pub add_classes: Option<JsonAddClassesOptions>,
    pub html_cleanup: Option<JsonHtmlCleanupOptions>,
    pub minify: Option<JsonToggle>,
    pub diagnostics: Option<JsonDiagnosticsOptions>,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct JsonToggle {
    pub enabled: Option<bool>,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct JsonGfmOptions {
    pub tables: Option<bool>,
    pub task_list: Option<bool>,
    pub strikethrough: Option<bool>,
    pub footnotes: Option<bool>,
    pub autolink: Option<bool>,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct JsonFrontmatterOptions {
    pub yaml: Option<bool>,
    pub toml: Option<bool>,
    pub json: Option<bool>,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct JsonSanitizeOptions {
    pub enabled: Option<bool>,
    pub schema: Option<JsonSanitizeSchema>,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct JsonSanitizeSchema {
    pub allowed_tags: Option<Vec<String>>,
    pub allowed_attributes: Option<HashMap<String, Vec<String>>>,
    pub allowed_protocols: Option<HashMap<String, Vec<String>>>,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct JsonHighlightOptions {
    pub enabled: Option<bool>,
    pub engine: Option<String>,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct JsonSlugOptions {
    pub mode: Option<String>,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct JsonTocOptions {
    pub enabled: Option<bool>,
    pub max_depth: Option<u32>,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct JsonExternalLinksOptions {
    pub enabled: Option<bool>,
    pub rel: Option<String>,
    pub target: Option<String>,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct JsonAutolinkHeadingsOptions {
    pub enabled: Option<bool>,
    pub behavior: Option<String>,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct JsonSmartypantsOptions {
    pub enabled: Option<bool>,
    pub quotes: Option<bool>,
    pub dashes: Option<bool>,
    pub ellipses: Option<bool>,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct JsonGithubAlertOptions {
    pub enabled: Option<bool>,
    pub icons: Option<String>,
    pub custom_icons: Option<HashMap<String, JsonAlertIconDef>>,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct JsonAlertIconDef {
    pub svg: Option<String>,
    pub import_source: Option<String>,
    pub import_name: Option<String>,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct JsonWikiLinkOptions {
    pub enabled: Option<bool>,
    pub href_template: Option<String>,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct JsonCodeImportOptions {
    pub enabled: Option<bool>,
    pub root_dir: Option<String>,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct JsonReadingTimeOptions {
    pub enabled: Option<bool>,
    pub words_per_minute: Option<u32>,
    pub cjk_chars_per_minute: Option<u32>,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct JsonExcerptOptions {
    pub enabled: Option<bool>,
    pub separator: Option<String>,
    pub fallback_paragraphs: Option<u32>,
    pub fallback_characters: Option<u32>,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct JsonImgLazyLoadingOptions {
    pub enabled: Option<bool>,
    pub skip_first: Option<u32>,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct JsonAddClassesRule {
    pub selector: String,
    pub classes: String,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct JsonAddClassesOptions {
    pub enabled: Option<bool>,
    pub rules: Option<Vec<JsonAddClassesRule>>,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct JsonHtmlCleanupOptions {
    pub remove_empty_nodes: Option<bool>,
    pub minify_whitespace: Option<bool>,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct JsonDiagnosticsOptions {
    pub format: Option<String>,
}

// ─── JsonCompileOptions → CompileOptions ──────────────────────────────

impl From<JsonCompileOptions> for CompileOptions {
    fn from(j: JsonCompileOptions) -> Self {
        Self {
            input_kind: match j.input_kind.as_deref() {
                Some("mdx") => InputKind::Mdx,
                _ => InputKind::Markdown,
            },
            output_kind: match j.output_kind.as_deref() {
                Some("hast") => OutputKind::Hast,
                Some("mdast") => OutputKind::Mdast,
                Some("mdxJs") => OutputKind::MdxJs,
                _ => OutputKind::Html,
            },
            raw_html: match j.raw_html.as_deref() {
                Some("allowDangerous") => RawHtmlPolicy::AllowDangerous,
                Some("parseAndSanitize") => RawHtmlPolicy::ParseAndSanitize,
                _ => RawHtmlPolicy::Disallow,
            },
            gfm: if let Some(g) = j.gfm {
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
            frontmatter: if let Some(f) = j.frontmatter {
                FrontmatterOptions {
                    yaml: f.yaml.unwrap_or(false),
                    toml: f.toml.unwrap_or(false),
                    json: f.json.unwrap_or(false),
                }
            } else {
                FrontmatterOptions::default()
            },
            sanitize: if let Some(s) = j.sanitize {
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
            highlight: if let Some(h) = j.highlight {
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
            line_numbers: LineNumberOptions {
                enabled: j.line_numbers.and_then(|l| l.enabled).unwrap_or(false),
            },
            slug: if let Some(s) = j.slug {
                SlugOptions {
                    mode: match s.mode.as_deref() {
                        Some("unicode") => SlugMode::Unicode,
                        _ => SlugMode::GitHub,
                    },
                }
            } else {
                SlugOptions::default()
            },
            toc: if let Some(t) = j.toc {
                TocOptions {
                    enabled: t.enabled.unwrap_or(false),
                    max_depth: t.max_depth.map_or(6, |d| d as u8),
                }
            } else {
                TocOptions::default()
            },
            external_links: if let Some(el) = j.external_links {
                ExternalLinksOptions {
                    enabled: el.enabled.unwrap_or(false),
                    rel: el.rel.unwrap_or_else(|| "noopener noreferrer".to_string()),
                    target: el.target,
                }
            } else {
                ExternalLinksOptions::default()
            },
            autolink_headings: if let Some(ah) = j.autolink_headings {
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
                enabled: j.sectionize.and_then(|s| s.enabled).unwrap_or(false),
            },
            breaks: BreaksOptions {
                enabled: j.breaks.and_then(|b| b.enabled).unwrap_or(false),
            },
            smartypants: if let Some(sp) = j.smartypants {
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
                enabled: j.emoji.and_then(|e| e.enabled).unwrap_or(false),
            },
            github_alert: if let Some(ga) = j.github_alert {
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
                enabled: j.math.and_then(|m| m.enabled).unwrap_or(false),
            },
            directive: DirectiveOptions {
                enabled: j.directive.and_then(|d| d.enabled).unwrap_or(false),
            },
            wiki_link: if let Some(wl) = j.wiki_link {
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
                enabled: j.definition_list.and_then(|d| d.enabled).unwrap_or(false),
            },
            ruby_annotation: RubyAnnotationOptions {
                enabled: j.ruby_annotation.and_then(|r| r.enabled).unwrap_or(false),
            },
            cjk: CjkOptions {
                enabled: j.cjk.and_then(|c| c.enabled).unwrap_or(false),
            },
            code_import: if let Some(ci) = j.code_import {
                CodeImportOptions {
                    enabled: ci.enabled.unwrap_or(false),
                    root_dir: ci.root_dir,
                }
            } else {
                CodeImportOptions::default()
            },
            code_meta: CodeMetaOptions {
                enabled: j.code_meta.and_then(|c| c.enabled).unwrap_or(false),
            },
            figure: FigureOptions {
                enabled: j.figure.and_then(|f| f.enabled).unwrap_or(false),
            },
            custom_heading_id: CustomHeadingIdOptions {
                enabled: j.custom_heading_id.and_then(|c| c.enabled).unwrap_or(false),
            },
            reading_time_opts: if let Some(rt) = j.reading_time {
                ReadingTimeOptions {
                    enabled: rt.enabled.unwrap_or(false),
                    words_per_minute: rt.words_per_minute.unwrap_or(200),
                    cjk_chars_per_minute: rt.cjk_chars_per_minute.unwrap_or(500),
                }
            } else {
                ReadingTimeOptions::default()
            },
            excerpt_opts: if let Some(ex) = j.excerpt {
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
                enabled: j.abbr.and_then(|a| a.enabled).unwrap_or(false),
            },
            comment_removal: CommentRemovalOptions {
                enabled: j.comment_removal.and_then(|c| c.enabled).unwrap_or(false),
            },
            img_lazy_loading: if let Some(il) = j.img_lazy_loading {
                ImgLazyLoadingOptions {
                    enabled: il.enabled.unwrap_or(false),
                    skip_first: il.skip_first.unwrap_or(0),
                }
            } else {
                ImgLazyLoadingOptions::default()
            },
            accessible_emoji: AccessibleEmojiOptions {
                enabled: j.accessible_emoji.and_then(|a| a.enabled).unwrap_or(false),
            },
            add_classes: if let Some(ac) = j.add_classes {
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
            html_cleanup: if let Some(hc) = j.html_cleanup {
                HtmlCleanupOptions {
                    remove_empty_nodes: hc.remove_empty_nodes.unwrap_or(false),
                    minify_whitespace: hc.minify_whitespace.unwrap_or(false),
                }
            } else {
                HtmlCleanupOptions::default()
            },
            minify: MinifyOptions {
                enabled: j.minify.and_then(|m| m.enabled).unwrap_or(false),
            },
            diagnostics: if let Some(d) = j.diagnostics {
                DiagnosticsOptions {
                    format: match d.format.as_deref() {
                        Some("verbose") => DiagnosticsFormat::Verbose,
                        _ => DiagnosticsFormat::Compact,
                    },
                }
            } else {
                DiagnosticsOptions::default()
            },
            ..Default::default()
        }
    }
}

impl CompileOptions {
    /// Deserialize `CompileOptions` from a JSON string.
    /// Fields not present in JSON get their default values.
    /// `plugins` and `extensions` are always empty (not representable in JSON).
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        let json_opts: JsonCompileOptions = serde_json::from_str(json)?;
        Ok(json_opts.into())
    }
}

// ─── JSON-serializable CompileResult ──────────────────────────────────

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonCompileResult {
    pub output: String,
    pub frontmatter: serde_json::Value,
    pub diagnostics: Vec<JsonDiagnostic>,
    pub stats: JsonCompileStats,
    pub toc: Vec<JsonTocEntry>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reading_time: Option<JsonReadingTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excerpt: Option<String>,
}

#[derive(Serialize)]
pub struct JsonDiagnostic {
    pub level: String,
    pub message: String,
    pub start: u32,
    pub end: u32,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonCompileStats {
    pub parse_ms: f64,
    pub transform_ms: f64,
    pub emit_ms: f64,
}

#[derive(Serialize)]
pub struct JsonTocEntry {
    pub depth: u8,
    pub text: String,
    pub slug: String,
}

#[derive(Serialize)]
pub struct JsonReadingTime {
    pub words: u32,
    pub minutes: f64,
}

impl super::result::CompileResult {
    /// Convert this result into a JSON-serializable struct.
    #[must_use]
    pub fn into_json(self) -> JsonCompileResult {
        use super::result::Output;
        use crate::diagnostics::diagnostic::DiagLevel;

        let output = match &self.output {
            Output::Html(html) => html.clone(),
            Output::MdxJs { code, .. } => code.clone(),
            Output::Hast(root) => {
                serde_json::to_string(root).unwrap_or_else(|e| format!("{{\"error\":\"{e}\"}}"))
            }
            Output::Mdast(doc) => {
                serde_json::to_string(doc).unwrap_or_else(|e| format!("{{\"error\":\"{e}\"}}"))
            }
        };

        let frontmatter = serde_json::to_value(&self.frontmatter)
            .unwrap_or_else(|_| serde_json::Value::Object(serde_json::Map::new()));

        JsonCompileResult {
            output,
            frontmatter,
            diagnostics: self
                .diagnostics
                .iter()
                .map(|d| JsonDiagnostic {
                    level: match d.level {
                        DiagLevel::Error => "error".to_string(),
                        DiagLevel::Warning => "warn".to_string(),
                    },
                    message: d.message.clone(),
                    start: d.span.start,
                    end: d.span.end,
                })
                .collect(),
            stats: JsonCompileStats {
                parse_ms: self.stats.parse_ms,
                transform_ms: self.stats.transform_ms,
                emit_ms: self.stats.emit_ms,
            },
            toc: self
                .toc
                .iter()
                .map(|e| JsonTocEntry {
                    depth: e.depth,
                    text: e.text.clone(),
                    slug: e.slug.clone(),
                })
                .collect(),
            reading_time: self.reading_time.map(|rt| JsonReadingTime {
                words: rt.words,
                minutes: rt.minutes,
            }),
            excerpt: self.excerpt.clone(),
        }
    }

    /// Serialize this result directly to a JSON string.
    #[must_use]
    pub fn to_json_string(self) -> String {
        let json_result = self.into_json();
        serde_json::to_string(&json_result).unwrap_or_else(|_| "{}".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_json_gives_defaults() {
        let opts = CompileOptions::from_json("{}").unwrap();
        assert_eq!(opts.input_kind, InputKind::Markdown);
        assert_eq!(opts.output_kind, OutputKind::Html);
        assert!(opts.gfm.tables);
        assert!(!opts.toc.enabled);
    }

    #[test]
    fn camel_case_fields_work() {
        let opts = CompileOptions::from_json(
            r#"{"inputKind":"mdx","outputKind":"hast","readingTime":{"enabled":true}}"#,
        )
        .unwrap();
        assert_eq!(opts.input_kind, InputKind::Mdx);
        assert_eq!(opts.output_kind, OutputKind::Hast);
        assert!(opts.reading_time_opts.enabled);
    }

    #[test]
    fn all_feature_toggles() {
        let opts = CompileOptions::from_json(
            r#"{"toc":{"enabled":true},"emoji":{"enabled":true},"math":{"enabled":true}}"#,
        )
        .unwrap();
        assert!(opts.toc.enabled);
        assert!(opts.emoji.enabled);
        assert!(opts.math.enabled);
    }

    #[test]
    fn highlight_engine_parsing() {
        let opts =
            CompileOptions::from_json(r#"{"highlight":{"enabled":true,"engine":"syntect"}}"#)
                .unwrap();
        assert!(opts.highlight.enabled);
        assert_eq!(opts.highlight.engine, HighlightEngine::Syntect);
    }

    #[test]
    fn invalid_json_returns_error() {
        assert!(CompileOptions::from_json("{invalid").is_err());
    }

    #[test]
    fn unknown_fields_ignored() {
        let opts = CompileOptions::from_json(r#"{"unknownField": 42}"#).unwrap();
        assert_eq!(opts.input_kind, InputKind::Markdown);
    }

    #[test]
    fn diagnostics_format_parsing() {
        let opts = CompileOptions::from_json(r#"{"diagnostics":{"format":"verbose"}}"#).unwrap();
        assert_eq!(opts.diagnostics.format, DiagnosticsFormat::Verbose);

        let opts2 = CompileOptions::from_json(r#"{"diagnostics":{"format":"compact"}}"#).unwrap();
        assert_eq!(opts2.diagnostics.format, DiagnosticsFormat::Compact);

        let opts3 = CompileOptions::from_json(r"{}").unwrap();
        assert_eq!(opts3.diagnostics.format, DiagnosticsFormat::Compact);
    }

    #[test]
    fn result_serialization_round_trip() {
        use crate::api::result::{CompileResult, CompileStats, Output};

        let result = CompileResult {
            output: Output::Html("<p>hello</p>".to_string()),
            frontmatter: {
                let mut m = std::collections::HashMap::new();
                m.insert(
                    "title".to_string(),
                    serde_json::Value::String("Test".to_string()),
                );
                m
            },
            diagnostics: vec![],
            stats: CompileStats {
                parse_ms: 1.0,
                transform_ms: 2.0,
                emit_ms: 0.5,
            },
            toc: vec![],
            reading_time: None,
            excerpt: None,
        };

        let json_str = result.to_json_string();
        let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();
        assert_eq!(parsed["output"], "<p>hello</p>");
        assert_eq!(parsed["frontmatter"]["title"], "Test");
        assert_eq!(parsed["stats"]["parseMs"], 1.0);
    }
}
