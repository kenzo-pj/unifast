use unifast_core::api::compile;
use unifast_core::api::options::{
    CompileOptions, HighlightEngine, HighlightOptions, InputKind, OutputKind, RawHtmlPolicy,
};
use unifast_core::api::result::Output;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[must_use]
pub fn compile_to_html(input: &str) -> String {
    let opts = CompileOptions::default();
    let result = compile::compile(input, &opts);
    match result.output {
        Output::Html(html) => html,
        _ => String::new(),
    }
}

#[wasm_bindgen]
#[must_use]
pub fn compile_with_options(input: &str, options_json: &str) -> String {
    let opts = parse_options_from_json(options_json);
    let result = compile::compile(input, &opts);

    let output_str = match &result.output {
        Output::Html(html) => html.clone(),
        Output::MdxJs { code, .. } => code.clone(),
        Output::Hast(root) => format!("{root:?}"),
        Output::Mdast(doc) => format!("{doc:?}"),
    };

    let frontmatter_json = serde_json::to_string(&result.frontmatter).unwrap_or_default();
    let diagnostics_json: Vec<serde_json::Value> = result
        .diagnostics
        .iter()
        .map(|d| {
            serde_json::json!({
                "level": match d.level {
                    unifast_core::diagnostics::diagnostic::DiagLevel::Error => "error",
                    unifast_core::diagnostics::diagnostic::DiagLevel::Warning => "warn",
                },
                "message": d.message,
                "start": d.span.start,
                "end": d.span.end,
            })
        })
        .collect();

    serde_json::json!({
        "output": output_str,
        "frontmatter": serde_json::from_str::<serde_json::Value>(&frontmatter_json).unwrap_or_default(),
        "diagnostics": diagnostics_json,
        "stats": {
            "parseMs": result.stats.parse_ms,
            "transformMs": result.stats.transform_ms,
            "emitMs": result.stats.emit_ms,
        }
    })
    .to_string()
}

fn parse_options_from_json(json: &str) -> CompileOptions {
    #[derive(serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct JsOpts {
        input_kind: Option<String>,
        output_kind: Option<String>,
        raw_html: Option<String>,
        highlight: Option<HighlightOpts>,
    }

    #[derive(serde::Deserialize)]
    struct HighlightOpts {
        enabled: Option<bool>,
    }

    let parsed: JsOpts = serde_json::from_str(json).unwrap_or(JsOpts {
        input_kind: None,
        output_kind: None,
        raw_html: None,
        highlight: None,
    });

    CompileOptions {
        input_kind: match parsed.input_kind.as_deref() {
            Some("mdx") => InputKind::Mdx,
            _ => InputKind::Markdown,
        },
        output_kind: match parsed.output_kind.as_deref() {
            Some("hast") => OutputKind::Hast,
            Some("mdast") => OutputKind::Mdast,
            Some("mdxJs") => OutputKind::MdxJs,
            _ => OutputKind::Html,
        },
        raw_html: match parsed.raw_html.as_deref() {
            Some("allowDangerous") => RawHtmlPolicy::AllowDangerous,
            Some("parseAndSanitize") => RawHtmlPolicy::ParseAndSanitize,
            _ => RawHtmlPolicy::Disallow,
        },
        highlight: if parsed
            .highlight
            .as_ref()
            .and_then(|h| h.enabled)
            .unwrap_or(false)
        {
            HighlightOptions {
                enabled: true,
                engine: HighlightEngine::Syntect,
            }
        } else {
            HighlightOptions::default()
        },
        ..Default::default()
    }
}
