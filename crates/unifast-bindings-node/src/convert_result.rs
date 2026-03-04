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
pub struct JsCompileResult {
    pub output: String,
    pub frontmatter: String, // JSON string
    pub diagnostics: Vec<JsDiagnostic>,
    pub stats: JsCompileStats,
}

pub fn convert_result(result: CompileResult) -> JsCompileResult {
    let output = match &result.output {
        Output::Html(html) => html.clone(),
        Output::MdxJs { code, .. } => code.clone(),
        Output::Hast(root) => format!("{:#?}", root),
        Output::Mdast(doc) => format!("{:#?}", doc),
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
    }
}
