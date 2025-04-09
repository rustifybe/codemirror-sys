use wasm_bindgen::prelude::*;
use super::state::Extension;

#[wasm_bindgen(module = "@codemirror/commands")]
extern "C" {
    pub fn history() -> Extension;

    #[wasm_bindgen(thread_local_v2, js_name = "defaultKeymap")]
    pub static DEFAULT_KEYMAP: js_sys::Array;
    
    #[wasm_bindgen(thread_local_v2, js_name = "historyKeymap")]
    pub static HISTORY_KEYMAP: js_sys::Array;

    #[wasm_bindgen(thread_local_v2, js_name = "indentWithTab")]
    pub static IDENT_WITH_TAB: js_sys::Object;
}
