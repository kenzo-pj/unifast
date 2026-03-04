mod convert_options;
mod convert_result;

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
