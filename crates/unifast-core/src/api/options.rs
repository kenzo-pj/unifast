use crate::transform::plugin::Plugin;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InputKind {
    #[default]
    Markdown,
    Mdx,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OutputKind {
    #[default]
    Html,
    Hast,
    Mdast,
    MdxJs,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RawHtmlPolicy {
    #[default]
    Disallow,
    AllowDangerous,
    ParseAndSanitize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SlugMode {
    #[default]
    GitHub,
    Unicode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DiagnosticsFormat {
    #[default]
    Compact,
    Verbose,
}

#[derive(Debug, Clone, Copy)]
pub struct GfmOptions {
    pub tables: bool,
    pub task_list: bool,
    pub strikethrough: bool,
    pub footnotes: bool,
    pub autolink: bool,
}

impl Default for GfmOptions {
    fn default() -> Self {
        Self {
            tables: true,
            task_list: true,
            strikethrough: true,
            footnotes: true,
            autolink: true,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct FrontmatterOptions {
    pub yaml: bool,
    pub toml: bool,
    pub json: bool,
}

impl FrontmatterOptions {
    #[must_use]
    pub const fn yaml_only() -> Self {
        Self {
            yaml: true,
            toml: false,
            json: false,
        }
    }

    #[must_use]
    pub const fn all() -> Self {
        Self {
            yaml: true,
            toml: true,
            json: true,
        }
    }

    #[must_use]
    pub const fn any_enabled(&self) -> bool {
        self.yaml || self.toml || self.json
    }
}

#[derive(Debug, Clone, Default)]
pub struct SanitizeSchema {
    pub allowed_tags: Vec<String>,
    pub allowed_attributes: HashMap<String, Vec<String>>,
    pub allowed_protocols: HashMap<String, Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct SanitizeOptions {
    pub enabled: bool,
    pub schema: Option<SanitizeSchema>,
}

impl Default for SanitizeOptions {
    fn default() -> Self {
        Self {
            enabled: true,
            schema: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HighlightEngine {
    #[default]
    None,
    Syntect,
    TreeSitter,
}

#[derive(Debug, Clone)]
pub struct HighlightOptions {
    pub enabled: bool,
    pub engine: HighlightEngine,
}

impl Default for HighlightOptions {
    fn default() -> Self {
        Self {
            enabled: false,
            engine: HighlightEngine::None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SlugOptions {
    pub mode: SlugMode,
}

impl Default for SlugOptions {
    fn default() -> Self {
        Self {
            mode: SlugMode::GitHub,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TocOptions {
    pub enabled: bool,
    pub max_depth: u8,
}

impl Default for TocOptions {
    fn default() -> Self {
        Self {
            enabled: false,
            max_depth: 6,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct DiagnosticsOptions {
    pub format: DiagnosticsFormat,
}

#[derive(Debug, Clone, Default)]
pub struct LineNumberOptions {
    pub enabled: bool,
}

#[derive(Debug, Clone, Default)]
pub struct Extension {
    pub name: String,
    pub enabled: bool,
}

#[derive(Debug, Clone)]
pub struct ExternalLinksOptions {
    pub enabled: bool,
    pub rel: String,
    pub target: Option<String>,
}

impl Default for ExternalLinksOptions {
    fn default() -> Self {
        Self {
            enabled: false,
            rel: "noopener noreferrer".to_string(),
            target: Some("_blank".to_string()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AutolinkHeadingsBehavior {
    #[default]
    Prepend,
    Append,
    Wrap,
}

#[derive(Debug, Clone)]
pub struct AutolinkHeadingsOptions {
    pub enabled: bool,
    pub behavior: AutolinkHeadingsBehavior,
}

impl Default for AutolinkHeadingsOptions {
    fn default() -> Self {
        Self {
            enabled: false,
            behavior: AutolinkHeadingsBehavior::Prepend,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct SectionizeOptions {
    pub enabled: bool,
}

#[derive(Debug, Clone, Default)]
pub struct BreaksOptions {
    pub enabled: bool,
}

#[derive(Debug, Clone, Default)]
pub struct SmartypantsOptions {
    pub enabled: bool,
    pub quotes: bool,
    pub dashes: bool,
    pub ellipses: bool,
}

impl SmartypantsOptions {
    #[must_use]
    pub const fn all() -> Self {
        Self {
            enabled: true,
            quotes: true,
            dashes: true,
            ellipses: true,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct EmojiOptions {
    pub enabled: bool,
}

#[derive(Debug, Clone, Default)]
pub struct GithubAlertOptions {
    pub enabled: bool,
    pub icons: GithubAlertIconMode,
}

#[derive(Debug, Clone, Default)]
pub enum GithubAlertIconMode {
    None,
    #[default]
    Octicon,
    Custom(std::collections::HashMap<String, AlertIconDef>),
}

#[derive(Debug, Clone)]
pub struct AlertIconDef {
    pub svg: Option<String>,
    pub import_source: Option<String>,
    pub import_name: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct MathOptions {
    pub enabled: bool,
}

#[derive(Debug, Clone, Default)]
pub struct DirectiveOptions {
    pub enabled: bool,
}

#[derive(Debug, Clone)]
pub struct WikiLinkOptions {
    pub enabled: bool,
    pub href_template: String,
}

impl Default for WikiLinkOptions {
    fn default() -> Self {
        Self {
            enabled: false,
            href_template: "/wiki/{slug}".to_string(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct DefinitionListOptions {
    pub enabled: bool,
}

#[derive(Debug, Clone, Default)]
pub struct RubyAnnotationOptions {
    pub enabled: bool,
}

#[derive(Debug, Clone, Default)]
pub struct CjkOptions {
    pub enabled: bool,
}

#[derive(Debug, Clone, Default)]
pub struct CodeImportOptions {
    pub enabled: bool,
    pub root_dir: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct CodeMetaOptions {
    pub enabled: bool,
}

#[derive(Debug, Clone, Default)]
pub struct FigureOptions {
    pub enabled: bool,
}

#[derive(Debug, Clone, Default)]
pub struct CustomHeadingIdOptions {
    pub enabled: bool,
}

#[derive(Debug, Clone)]
pub struct ReadingTimeOptions {
    pub enabled: bool,
    pub words_per_minute: u32,
    pub cjk_chars_per_minute: u32,
}

impl Default for ReadingTimeOptions {
    fn default() -> Self {
        Self {
            enabled: false,
            words_per_minute: 200,
            cjk_chars_per_minute: 500,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ExcerptOptions {
    pub enabled: bool,
    pub separator: String,
    pub fallback_paragraphs: Option<u32>,
    pub fallback_characters: Option<u32>,
}

impl Default for ExcerptOptions {
    fn default() -> Self {
        Self {
            enabled: false,
            separator: "<!-- more -->".to_string(),
            fallback_paragraphs: Some(1),
            fallback_characters: None,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct AbbrOptions {
    pub enabled: bool,
}

#[derive(Debug, Clone, Default)]
pub struct CommentRemovalOptions {
    pub enabled: bool,
}

#[derive(Debug, Clone, Default)]
pub struct ImgLazyLoadingOptions {
    pub enabled: bool,
    pub skip_first: u32,
}

#[derive(Debug, Clone, Default)]
pub struct AccessibleEmojiOptions {
    pub enabled: bool,
}

#[derive(Debug, Clone, Default)]
pub struct AddClassesOptions {
    pub enabled: bool,
    pub rules: Vec<(String, String)>,
}

#[derive(Debug, Clone, Default)]
pub struct HtmlCleanupOptions {
    pub remove_empty_nodes: bool,
    pub minify_whitespace: bool,
}

#[derive(Debug, Clone, Default)]
pub struct MinifyOptions {
    pub enabled: bool,
}

#[derive(Default)]
pub struct CompileOptions {
    pub input_kind: InputKind,
    pub output_kind: OutputKind,
    pub gfm: GfmOptions,
    pub frontmatter: FrontmatterOptions,
    pub raw_html: RawHtmlPolicy,
    pub sanitize: SanitizeOptions,
    pub highlight: HighlightOptions,
    pub line_numbers: LineNumberOptions,
    pub slug: SlugOptions,
    pub toc: TocOptions,
    pub diagnostics: DiagnosticsOptions,
    pub extensions: Vec<Extension>,
    pub plugins: Vec<Box<dyn Plugin>>,
    pub external_links: ExternalLinksOptions,
    pub autolink_headings: AutolinkHeadingsOptions,
    pub sectionize: SectionizeOptions,
    pub breaks: BreaksOptions,
    pub smartypants: SmartypantsOptions,
    pub emoji: EmojiOptions,
    pub github_alert: GithubAlertOptions,
    pub math: MathOptions,
    pub directive: DirectiveOptions,
    pub wiki_link: WikiLinkOptions,
    pub definition_list: DefinitionListOptions,
    pub ruby_annotation: RubyAnnotationOptions,
    pub cjk: CjkOptions,
    pub code_import: CodeImportOptions,
    pub code_meta: CodeMetaOptions,
    pub figure: FigureOptions,
    pub custom_heading_id: CustomHeadingIdOptions,
    pub reading_time_opts: ReadingTimeOptions,
    pub excerpt_opts: ExcerptOptions,
    pub abbr: AbbrOptions,
    pub comment_removal: CommentRemovalOptions,
    pub img_lazy_loading: ImgLazyLoadingOptions,
    pub accessible_emoji: AccessibleEmojiOptions,
    pub add_classes: AddClassesOptions,
    pub html_cleanup: HtmlCleanupOptions,
    pub minify: MinifyOptions,
}

impl std::fmt::Debug for CompileOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CompileOptions")
            .field("input_kind", &self.input_kind)
            .field("output_kind", &self.output_kind)
            .field("gfm", &self.gfm)
            .field("frontmatter", &self.frontmatter)
            .field("raw_html", &self.raw_html)
            .field("sanitize", &self.sanitize)
            .field("highlight", &self.highlight)
            .field("line_numbers", &self.line_numbers)
            .field("slug", &self.slug)
            .field("toc", &self.toc)
            .field("diagnostics", &self.diagnostics)
            .field("extensions", &self.extensions)
            .field("plugins", &format_args!("[{} plugins]", self.plugins.len()))
            .field("external_links", &self.external_links)
            .field("autolink_headings", &self.autolink_headings)
            .field("sectionize", &self.sectionize)
            .field("breaks", &self.breaks)
            .field("smartypants", &self.smartypants)
            .field("emoji", &self.emoji)
            .field("github_alert", &self.github_alert)
            .field("math", &self.math)
            .field("directive", &self.directive)
            .field("wiki_link", &self.wiki_link)
            .field("definition_list", &self.definition_list)
            .field("ruby_annotation", &self.ruby_annotation)
            .field("cjk", &self.cjk)
            .field("code_import", &self.code_import)
            .field("code_meta", &self.code_meta)
            .field("figure", &self.figure)
            .field("custom_heading_id", &self.custom_heading_id)
            .field("reading_time_opts", &self.reading_time_opts)
            .field("excerpt_opts", &self.excerpt_opts)
            .field("abbr", &self.abbr)
            .field("comment_removal", &self.comment_removal)
            .field("img_lazy_loading", &self.img_lazy_loading)
            .field("accessible_emoji", &self.accessible_emoji)
            .field("add_classes", &self.add_classes)
            .field("html_cleanup", &self.html_cleanup)
            .field("minify", &self.minify)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compile_options_default() {
        let opts = CompileOptions::default();
        assert_eq!(opts.input_kind, InputKind::Markdown);
        assert_eq!(opts.output_kind, OutputKind::Html);
        assert_eq!(opts.raw_html, RawHtmlPolicy::Disallow);
        assert!(opts.gfm.tables);
        assert!(opts.gfm.task_list);
        assert!(opts.gfm.strikethrough);
        assert!(opts.gfm.footnotes);
        assert!(opts.gfm.autolink);
        assert!(opts.sanitize.enabled);
        assert!(opts.sanitize.schema.is_none());
        assert!(!opts.highlight.enabled);
        assert_eq!(opts.highlight.engine, HighlightEngine::None);
        assert_eq!(opts.slug.mode, SlugMode::GitHub);
        assert!(!opts.toc.enabled);
        assert_eq!(opts.toc.max_depth, 6);
        assert_eq!(opts.diagnostics.format, DiagnosticsFormat::Compact);
        assert!(opts.extensions.is_empty());
        assert!(opts.plugins.is_empty());
    }

    #[test]
    fn compile_options_debug_format() {
        let opts = CompileOptions::default();
        let debug_str = format!("{opts:?}");
        assert!(debug_str.contains("CompileOptions"));
        assert!(debug_str.contains("[0 plugins]"));
    }

    #[test]
    fn gfm_options_default_all_enabled() {
        let gfm = GfmOptions::default();
        assert!(gfm.tables);
        assert!(gfm.task_list);
        assert!(gfm.strikethrough);
        assert!(gfm.footnotes);
        assert!(gfm.autolink);
    }

    #[test]
    fn frontmatter_yaml_only() {
        let fm = FrontmatterOptions::yaml_only();
        assert!(fm.yaml);
        assert!(!fm.toml);
        assert!(!fm.json);
    }

    #[test]
    fn frontmatter_all() {
        let fm = FrontmatterOptions::all();
        assert!(fm.yaml);
        assert!(fm.toml);
        assert!(fm.json);
    }

    #[test]
    fn sanitize_options_default_enabled() {
        let san = SanitizeOptions::default();
        assert!(san.enabled);
        assert!(san.schema.is_none());
    }

    #[test]
    fn toc_options_default() {
        let toc = TocOptions::default();
        assert!(!toc.enabled);
        assert_eq!(toc.max_depth, 6);
    }
}
