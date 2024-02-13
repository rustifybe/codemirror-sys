use wasm_bindgen::prelude::*;
use super::state::Extension;

#[wasm_bindgen(module = "@codemirror/language")]
extern "C" {
    pub type HighlightStyle;

    #[wasm_bindgen(js_name = "foldGutter")]
    pub fn fold_gutter() -> Extension;

    #[wasm_bindgen(js_name = "indentOnInput")]
    pub fn indent_on_input() -> Extension;

    #[wasm_bindgen(js_name = "bracketMatching")]
    pub fn bracket_matching() -> Extension;

    #[wasm_bindgen(js_name = "syntaxHighlighting")]
    pub fn syntax_highlighting(
        style: &HighlightStyle,
        options: Option<js_sys::Object>
    ) -> Extension;

    #[wasm_bindgen(js_name = "defaultHighlightStyle")]
    pub static DEFAULT_HIGHLIGHT_STYLE: HighlightStyle;

    #[wasm_bindgen(js_name = "foldKeymap")]
    pub static FOLD_KEYMAP: js_sys::Array;
}