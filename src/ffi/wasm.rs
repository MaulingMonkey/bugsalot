// See also: https://github.com/rustwasm/console_error_panic_hook

#[cfg(feature = "stdweb")]
pub mod console {
    pub fn error(msg: String) {
        stdweb0::console!(error, msg);
    }
}

#[cfg(all(feature = "wasm-bindgen", not(feature = "stdweb")))]
pub mod console {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen] extern {
        #[wasm_bindgen(js_namespace = console)] pub fn error(msg: String);
    }
}

#[cfg(not(any(feature = "stdweb", feature="wasm-bindgen")))]
pub mod console {
    pub fn error(_msg: String) {}
}
