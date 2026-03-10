use napi_derive::napi;
use std::collections::HashMap;
use unifast_core::api::json_compat::*;
use unifast_core::api::options::CompileOptions;

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
pub struct JsSanitizeSchemaOptions {
    pub allowed_tags: Option<Vec<String>>,
    pub allowed_attributes: Option<HashMap<String, Vec<String>>>,
    pub allowed_protocols: Option<HashMap<String, Vec<String>>>,
}

#[napi(object)]
pub struct JsSanitizeOptions {
    pub enabled: Option<bool>,
    pub schema: Option<JsSanitizeSchemaOptions>,
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
pub struct JsAlertIconDef {
    pub svg: Option<String>,
    pub import_source: Option<String>,
    pub import_name: Option<String>,
}

#[napi(object)]
pub struct JsGithubAlertOptions {
    pub enabled: Option<bool>,
    pub icons: Option<String>,
    pub custom_icons: Option<HashMap<String, JsAlertIconDef>>,
}

#[napi(object)]
pub struct JsReadingTimeOptions {
    pub enabled: Option<bool>,
    pub words_per_minute: Option<u32>,
    pub cjk_chars_per_minute: Option<u32>,
}

#[napi(object)]
pub struct JsExcerptOptions {
    pub enabled: Option<bool>,
    pub separator: Option<String>,
    pub fallback_paragraphs: Option<u32>,
    pub fallback_characters: Option<u32>,
}

#[napi(object)]
pub struct JsImgLazyLoadingOptions {
    pub enabled: Option<bool>,
    pub skip_first: Option<u32>,
}

#[napi(object)]
pub struct JsAddClassesRule {
    pub selector: String,
    pub classes: String,
}

#[napi(object)]
pub struct JsAddClassesOptions {
    pub enabled: Option<bool>,
    pub rules: Option<Vec<JsAddClassesRule>>,
}

#[napi(object)]
pub struct JsHtmlCleanupOptions {
    pub remove_empty_nodes: Option<bool>,
    pub minify_whitespace: Option<bool>,
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
    pub external_links: Option<JsExternalLinksOptions>,
    pub autolink_headings: Option<JsAutolinkHeadingsOptions>,
    pub sectionize: Option<JsFeatureToggle>,
    pub breaks: Option<JsFeatureToggle>,
    pub smartypants: Option<JsSmartypantsOptions>,
    pub emoji: Option<JsFeatureToggle>,
    pub github_alert: Option<JsGithubAlertOptions>,
    pub math: Option<JsFeatureToggle>,
    pub directive: Option<JsFeatureToggle>,
    pub wiki_link: Option<JsWikiLinkOptions>,
    pub definition_list: Option<JsFeatureToggle>,
    pub ruby_annotation: Option<JsFeatureToggle>,
    pub cjk: Option<JsFeatureToggle>,
    pub code_import: Option<JsCodeImportOptions>,
    pub code_meta: Option<JsFeatureToggle>,
    pub figure: Option<JsFeatureToggle>,
    pub custom_heading_id: Option<JsFeatureToggle>,
    pub reading_time: Option<JsReadingTimeOptions>,
    pub excerpt: Option<JsExcerptOptions>,
    pub abbr: Option<JsFeatureToggle>,
    pub comment_removal: Option<JsFeatureToggle>,
    pub img_lazy_loading: Option<JsImgLazyLoadingOptions>,
    pub accessible_emoji: Option<JsFeatureToggle>,
    pub add_classes: Option<JsAddClassesOptions>,
    pub html_cleanup: Option<JsHtmlCleanupOptions>,
    pub minify: Option<JsFeatureToggle>,
}

// ─── Js* → Json* conversions (field mapping only, no business logic) ──

impl From<JsFeatureToggle> for JsonToggle {
    fn from(js: JsFeatureToggle) -> Self {
        Self {
            enabled: js.enabled,
        }
    }
}

impl From<JsGfmOptions> for JsonGfmOptions {
    fn from(js: JsGfmOptions) -> Self {
        Self {
            tables: js.tables,
            task_list: js.task_list,
            strikethrough: js.strikethrough,
            footnotes: js.footnotes,
            autolink: js.autolink,
        }
    }
}

impl From<JsFrontmatterOptions> for JsonFrontmatterOptions {
    fn from(js: JsFrontmatterOptions) -> Self {
        Self {
            yaml: js.yaml,
            toml: js.toml,
            json: js.json,
        }
    }
}

impl From<JsSanitizeSchemaOptions> for JsonSanitizeSchema {
    fn from(js: JsSanitizeSchemaOptions) -> Self {
        Self {
            allowed_tags: js.allowed_tags,
            allowed_attributes: js.allowed_attributes,
            allowed_protocols: js.allowed_protocols,
        }
    }
}

impl From<JsSanitizeOptions> for JsonSanitizeOptions {
    fn from(js: JsSanitizeOptions) -> Self {
        Self {
            enabled: js.enabled,
            schema: js.schema.map(Into::into),
        }
    }
}

impl From<JsHighlightOptions> for JsonHighlightOptions {
    fn from(js: JsHighlightOptions) -> Self {
        Self {
            enabled: js.enabled,
            engine: js.engine,
        }
    }
}

impl From<JsSlugOptions> for JsonSlugOptions {
    fn from(js: JsSlugOptions) -> Self {
        Self { mode: js.mode }
    }
}

impl From<JsTocOptions> for JsonTocOptions {
    fn from(js: JsTocOptions) -> Self {
        Self {
            enabled: js.enabled,
            max_depth: js.max_depth,
        }
    }
}

impl From<JsDiagnosticsOptions> for JsonDiagnosticsOptions {
    fn from(js: JsDiagnosticsOptions) -> Self {
        Self { format: js.format }
    }
}

impl From<JsExternalLinksOptions> for JsonExternalLinksOptions {
    fn from(js: JsExternalLinksOptions) -> Self {
        Self {
            enabled: js.enabled,
            rel: js.rel,
            target: js.target,
        }
    }
}

impl From<JsAutolinkHeadingsOptions> for JsonAutolinkHeadingsOptions {
    fn from(js: JsAutolinkHeadingsOptions) -> Self {
        Self {
            enabled: js.enabled,
            behavior: js.behavior,
        }
    }
}

impl From<JsSmartypantsOptions> for JsonSmartypantsOptions {
    fn from(js: JsSmartypantsOptions) -> Self {
        Self {
            enabled: js.enabled,
            quotes: js.quotes,
            dashes: js.dashes,
            ellipses: js.ellipses,
        }
    }
}

impl From<JsAlertIconDef> for JsonAlertIconDef {
    fn from(js: JsAlertIconDef) -> Self {
        Self {
            svg: js.svg,
            import_source: js.import_source,
            import_name: js.import_name,
        }
    }
}

impl From<JsGithubAlertOptions> for JsonGithubAlertOptions {
    fn from(js: JsGithubAlertOptions) -> Self {
        Self {
            enabled: js.enabled,
            icons: js.icons,
            custom_icons: js
                .custom_icons
                .map(|m| m.into_iter().map(|(k, v)| (k, v.into())).collect()),
        }
    }
}

impl From<JsWikiLinkOptions> for JsonWikiLinkOptions {
    fn from(js: JsWikiLinkOptions) -> Self {
        Self {
            enabled: js.enabled,
            href_template: js.href_template,
        }
    }
}

impl From<JsCodeImportOptions> for JsonCodeImportOptions {
    fn from(js: JsCodeImportOptions) -> Self {
        Self {
            enabled: js.enabled,
            root_dir: js.root_dir,
        }
    }
}

impl From<JsReadingTimeOptions> for JsonReadingTimeOptions {
    fn from(js: JsReadingTimeOptions) -> Self {
        Self {
            enabled: js.enabled,
            words_per_minute: js.words_per_minute,
            cjk_chars_per_minute: js.cjk_chars_per_minute,
        }
    }
}

impl From<JsExcerptOptions> for JsonExcerptOptions {
    fn from(js: JsExcerptOptions) -> Self {
        Self {
            enabled: js.enabled,
            separator: js.separator,
            fallback_paragraphs: js.fallback_paragraphs,
            fallback_characters: js.fallback_characters,
        }
    }
}

impl From<JsImgLazyLoadingOptions> for JsonImgLazyLoadingOptions {
    fn from(js: JsImgLazyLoadingOptions) -> Self {
        Self {
            enabled: js.enabled,
            skip_first: js.skip_first,
        }
    }
}

impl From<JsAddClassesRule> for JsonAddClassesRule {
    fn from(js: JsAddClassesRule) -> Self {
        Self {
            selector: js.selector,
            classes: js.classes,
        }
    }
}

impl From<JsAddClassesOptions> for JsonAddClassesOptions {
    fn from(js: JsAddClassesOptions) -> Self {
        Self {
            enabled: js.enabled,
            rules: js.rules.map(|r| r.into_iter().map(Into::into).collect()),
        }
    }
}

impl From<JsHtmlCleanupOptions> for JsonHtmlCleanupOptions {
    fn from(js: JsHtmlCleanupOptions) -> Self {
        Self {
            remove_empty_nodes: js.remove_empty_nodes,
            minify_whitespace: js.minify_whitespace,
        }
    }
}

impl From<JsCompileOptions> for JsonCompileOptions {
    fn from(js: JsCompileOptions) -> Self {
        Self {
            input_kind: js.input_kind,
            output_kind: js.output_kind,
            raw_html: js.raw_html,
            gfm: js.gfm.map(Into::into),
            frontmatter: js.frontmatter.map(Into::into),
            sanitize: js.sanitize.map(Into::into),
            highlight: js.highlight.map(Into::into),
            line_numbers: js.line_numbers.map(|l| JsonToggle { enabled: l.enabled }),
            slug: js.slug.map(Into::into),
            toc: js.toc.map(Into::into),
            external_links: js.external_links.map(Into::into),
            autolink_headings: js.autolink_headings.map(Into::into),
            sectionize: js.sectionize.map(Into::into),
            breaks: js.breaks.map(Into::into),
            smartypants: js.smartypants.map(Into::into),
            emoji: js.emoji.map(Into::into),
            github_alert: js.github_alert.map(Into::into),
            math: js.math.map(Into::into),
            directive: js.directive.map(Into::into),
            wiki_link: js.wiki_link.map(Into::into),
            definition_list: js.definition_list.map(Into::into),
            ruby_annotation: js.ruby_annotation.map(Into::into),
            cjk: js.cjk.map(Into::into),
            code_import: js.code_import.map(Into::into),
            code_meta: js.code_meta.map(Into::into),
            figure: js.figure.map(Into::into),
            custom_heading_id: js.custom_heading_id.map(Into::into),
            reading_time: js.reading_time.map(Into::into),
            excerpt: js.excerpt.map(Into::into),
            abbr: js.abbr.map(Into::into),
            comment_removal: js.comment_removal.map(Into::into),
            img_lazy_loading: js.img_lazy_loading.map(Into::into),
            accessible_emoji: js.accessible_emoji.map(Into::into),
            add_classes: js.add_classes.map(Into::into),
            html_cleanup: js.html_cleanup.map(Into::into),
            minify: js.minify.map(Into::into),
            diagnostics: js.diagnostics.map(Into::into),
        }
    }
}

pub fn convert_options(js_opts: Option<JsCompileOptions>) -> CompileOptions {
    match js_opts {
        Some(js) => {
            let json_opts: JsonCompileOptions = js.into();
            json_opts.into()
        }
        None => CompileOptions::default(),
    }
}
