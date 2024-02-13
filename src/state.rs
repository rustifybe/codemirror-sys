use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "@codemirror/state")]
extern "C" {
    #[derive(Clone)]
    pub type Extension;
    
    #[derive(Clone)]
    pub type Facet;

    #[wasm_bindgen(method)]
    pub fn of(this: &Facet, input: &JsValue) -> Extension;

    #[derive(Clone)]
    pub type Compartment;

    #[wasm_bindgen(constructor)]
    pub fn new() -> Compartment;

    #[wasm_bindgen(method)]
    pub fn of(this: &Compartment, extension: &Extension) -> Extension;

    #[derive(Clone)]
    pub type EditorState;

    #[wasm_bindgen(static_method_of = EditorState)]
    pub fn create(config: &js_sys::Object) -> EditorState;

    #[wasm_bindgen(method, getter = doc)]
    pub fn doc(this: &EditorState) -> Text;

    #[derive(Clone)]
    pub type Text;

    #[wasm_bindgen(method, js_name = toString)]
    pub fn to_string(this: &Text) -> String;

    #[wasm_bindgen(method, getter = length)]
    pub fn length(this: &Text) -> usize;



}