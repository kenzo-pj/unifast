pub mod entities;
pub mod frontmatter;
pub mod gfm;
pub mod markdown;
pub mod mdx;

use std::collections::HashMap;

use crate::api::options::{FrontmatterOptions, GfmOptions};
use crate::ast::common::NodeIdGen;
use crate::ast::mdast::nodes::Document;
use crate::diagnostics::sink::DiagnosticSink;

pub type FrontmatterData = HashMap<String, serde_json::Value>;

pub struct ParseResult {
    pub document: Document,
    pub diagnostics: DiagnosticSink,
    pub frontmatter: FrontmatterData,
    pub id_gen: NodeIdGen,
}

#[must_use]
pub fn parse_markdown(
    input: &str,
    gfm: &GfmOptions,
    frontmatter_opts: &FrontmatterOptions,
) -> ParseResult {
    let mut id_gen = NodeIdGen::new();
    let mut diagnostics = DiagnosticSink::new();
    let mut frontmatter_data = FrontmatterData::new();

    let fm_offset = if frontmatter_opts.any_enabled() {
        if let Some(fm) = frontmatter::extract_frontmatter_filtered(input, frontmatter_opts) {
            frontmatter_data = fm.data;
            fm.end_offset
        } else {
            0
        }
    } else {
        0
    };

    let document =
        markdown::parser::parse_from_offset(input, fm_offset, &mut id_gen, &mut diagnostics, gfm);

    ParseResult {
        document,
        diagnostics,
        frontmatter: frontmatter_data,
        id_gen,
    }
}

#[must_use]
pub fn parse_mdx_input(
    input: &str,
    gfm: &GfmOptions,
    frontmatter_opts: &FrontmatterOptions,
) -> ParseResult {
    let result = mdx::parse_mdx(input, gfm, frontmatter_opts);
    ParseResult {
        document: result.document,
        diagnostics: result.diagnostics,
        frontmatter: result.frontmatter,
        id_gen: result.id_gen,
    }
}
