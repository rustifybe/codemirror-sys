use wasm_bindgen::prelude::*;
use super::state::Extension;

#[wasm_bindgen(module = "@codemirror/lang-python")]
extern "C" {
    pub fn python() -> Extension;
}