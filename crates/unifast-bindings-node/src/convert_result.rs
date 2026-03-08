use napi_derive::napi;
use unifast_core::api::result::{CompileResult, Output};
use unifast_core::diagnostics::diagnostic::DiagLevel;

#[napi(object)]
pub struct JsDiagnostic {
    pub level: String,
    pub message: String,
    pub start: Option<u32>,
    pub end: Option<u32>,
}

#[napi(object)]
pub struct JsCompileStats {
    pub parse_ms: f64,
    pub transform_ms: f64,
    pub emit_ms: f64,
}

#[napi(object)]
pub struct JsTocEntry {
    pub depth: u32,
    pub text: String,
    pub slug: String,
}

#[napi(object)]
pub struct JsCompileResult {
    pub output: String,
    pub frontmatter: String,
    pub diagnostics: Vec<JsDiagnostic>,
    pub stats: JsCompileStats,
    pub toc: Vec<JsTocEntry>,
}

pub fn convert_result(result: CompileResult) -> JsCompileResult {
    let output = match &result.output {
        Output::Html(html) => html.clone(),
        Output::MdxJs { code, .. } => code.clone(),
        Output::Hast(root) => {
            serde_json::to_string(root).unwrap_or_else(|e| format!("{{\"error\":\"{e}\"}}"))
        }
        Output::Mdast(doc) => {
            serde_json::to_string(doc).unwrap_or_else(|e| format!("{{\"error\":\"{e}\"}}"))
        }
    };

    JsCompileResult {
        output,
        frontmatter: serde_json::to_string(&result.frontmatter)
            .unwrap_or_else(|_| "{}".to_string()),
        diagnostics: result
            .diagnostics
            .iter()
            .map(|d| JsDiagnostic {
                level: match d.level {
                    DiagLevel::Error => "error".to_string(),
                    DiagLevel::Warning => "warn".to_string(),
                },
                message: d.message.clone(),
                start: Some(d.span.start),
                end: Some(d.span.end),
            })
            .collect(),
        stats: JsCompileStats {
            parse_ms: result.stats.parse_ms,
            transform_ms: result.stats.transform_ms,
            emit_ms: result.stats.emit_ms,
        },
        toc: result
            .toc
            .iter()
            .map(|e| JsTocEntry {
                depth: u32::from(e.depth),
                text: e.text.clone(),
                slug: e.slug.clone(),
            })
            .collect(),
    }
}
