// See also: https://github.com/rustwasm/console_error_panic_hook
pub mod console {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen] extern {
        #[wasm_bindgen(js_namespace = console)] pub fn error(msg: String);
    }
}
