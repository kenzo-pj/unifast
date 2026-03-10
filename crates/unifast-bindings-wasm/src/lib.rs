use unifast_core::api::compile;
use unifast_core::api::options::CompileOptions;
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
pub fn compile_with_options(input: &str, options_json: &str) -> Result<String, JsValue> {
    let opts = CompileOptions::from_json(options_json)
        .map_err(|e| JsValue::from_str(&format!("invalid options JSON: {e}")))?;
    let result = compile::compile(input, &opts);
    Ok(result.to_json_string())
}
