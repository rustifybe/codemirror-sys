use wasm_bindgen::prelude::*;
use super::state::Extension;

#[wasm_bindgen(module = "@codemirror/autocomplete")]
extern "C" {
    #[wasm_bindgen(js_name = "closeBrackets")]
    pub fn close_brackets() -> Extension;

    #[wasm_bindgen(js_name = "autocompletion")]
    pub fn autocompletion() -> Extension;

    #[wasm_bindgen(thread_local_v2, js_name = "completionKeymap")]
    pub static COMPLETION_KEYMAP: js_sys::Array;

    #[wasm_bindgen(thread_local_v2, js_name = "closeBracketsKeymap")]
    pub static CLOSE_BRACKETS_KEYMAP: js_sys::Array;

}