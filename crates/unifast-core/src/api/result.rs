use crate::ast::hast::nodes::HRoot;
use crate::ast::mdast::nodes::Document;
use crate::diagnostics::diagnostic::Diagnostic;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Output {
    Html(String),
    Hast(HRoot),
    Mdast(Document),
    MdxJs { code: String, map: Option<String> },
}

pub type FrontmatterData = HashMap<String, serde_json::Value>;

#[derive(Debug, Clone, Default)]
pub struct CompileStats {
    pub parse_ms: f64,
    pub transform_ms: f64,
    pub emit_ms: f64,
}

pub struct CompileResult {
    pub output: Output,
    pub frontmatter: FrontmatterData,
    pub diagnostics: Vec<Diagnostic>,
    pub stats: CompileStats,
}
