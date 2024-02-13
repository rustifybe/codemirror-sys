use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

use super::state::{Extension, Facet, EditorState};

#[wasm_bindgen(module = "@codemirror/view")]
extern "C" {
    #[derive(Clone)]
    pub type EditorView;

    #[wasm_bindgen(constructor)]
    pub fn new(config: &js_sys::Object) -> EditorView;

    #[wasm_bindgen(method, getter = state)]
    pub fn state(this: &EditorView) -> EditorState;

    #[wasm_bindgen(method, getter = dom)]
    pub fn dom(this: &EditorView) -> HtmlElement;

    #[wasm_bindgen(static_method_of = EditorView, getter = updateListener)]
    pub fn update_listener() -> Facet;

    #[wasm_bindgen(method)]
    pub fn dispatch(this: &EditorView, transaction: &js_sys::Object);

    pub type ViewUpdate;

    #[wasm_bindgen(method, getter = docChanged)]
    pub fn doc_changed(this: &ViewUpdate) -> bool;

    #[wasm_bindgen(method, getter = state)]
    pub fn state(this: &ViewUpdate) -> EditorState;

    #[wasm_bindgen(method, getter = view)]
    pub fn view(this: &ViewUpdate) -> EditorView;

    #[wasm_bindgen(js_name = "lineNumbers")]
    pub fn line_numbers() -> Extension;

    #[wasm_bindgen(js_name = "highlightActiveLineGutter")]
    pub fn highlight_active_line_gutter() -> Extension;

    #[wasm_bindgen(js_name = "highlightSpecialChars")]
    pub fn highlight_special_chars() -> Extension;

    #[wasm_bindgen(js_name = "drawSelection")]
    pub fn draw_selection() -> Extension;

    #[wasm_bindgen(js_name = "dropCursor")]
    pub fn drop_cursor() -> Extension;

    #[wasm_bindgen(js_name = "rectangularSelection")]
    pub fn rectangular_selection() -> Extension;

    #[wasm_bindgen(js_name = "crosshairCursor")]
    pub fn crosshair_cursor() -> Extension;

    #[wasm_bindgen(js_name = "highlightActiveLine")]
    pub fn highlight_active_line() -> Extension;

    #[wasm_bindgen(js_name = "keymap")]
    pub static KEYMAP: Facet;

}