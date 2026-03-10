mod convert_options;
mod convert_result;
mod parse_hast;

use convert_options::{JsCompileOptions, convert_options};
use convert_result::convert_result;
use napi_derive::napi;

#[napi]
pub fn compile(
    input: String,
    options: Option<JsCompileOptions>,
) -> napi::Result<convert_result::JsCompileResult> {
    let opts = convert_options(options);
    let result = unifast_core::api::compile::compile(&input, &opts);
    Ok(convert_result(result))
}

#[napi]
pub fn stringify_hast(json: String) -> napi::Result<String> {
    let root = parse_hast::hroot_from_json(&json)
        .map_err(|e| napi::Error::from_reason(format!("Failed to parse HAST JSON: {e}")))?;
    Ok(unifast_core::emit::html::stringify::stringify(&root))
}
