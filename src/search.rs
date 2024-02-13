use wasm_bindgen::prelude::*;
use super::state::Extension;

#[wasm_bindgen(module = "@codemirror/search")]
extern "C" {
    #[wasm_bindgen(js_name = "highlightSelectionMatches")]
    pub fn highlight_selection_matches() -> Extension;

    #[wasm_bindgen(js_name = "searchKeymap")]
    pub static SEARCH_KEYMAP: js_sys::Array;
}