pub mod frontmatter;
pub mod gfm;
pub mod markdown;
pub mod mdx;

use std::collections::HashMap;

use crate::ast::common::NodeIdGen;
use crate::ast::mdast::nodes::Document;
use crate::diagnostics::sink::DiagnosticSink;

pub type FrontmatterData = HashMap<String, serde_json::Value>;

pub struct ParseResult {
    pub document: Document,
    pub diagnostics: DiagnosticSink,
    pub frontmatter: FrontmatterData,
}

#[must_use]
pub fn parse_markdown(input: &str) -> ParseResult {
    let mut id_gen = NodeIdGen::new();
    let mut diagnostics = DiagnosticSink::new();
    let mut frontmatter_data = FrontmatterData::new();

    let fm_offset = if let Some(fm) = frontmatter::extract_frontmatter(input) {
        frontmatter_data = fm.data;
        fm.end_offset
    } else {
        0
    };

    let document =
        markdown::parser::parse_from_offset(input, fm_offset, &mut id_gen, &mut diagnostics);

    ParseResult {
        document,
        diagnostics,
        frontmatter: frontmatter_data,
    }
}

#[must_use]
pub fn parse_mdx_input(input: &str) -> ParseResult {
    let result = mdx::parse_mdx(input);
    ParseResult {
        document: result.document,
        diagnostics: result.diagnostics,
        frontmatter: result.frontmatter,
    }
}
