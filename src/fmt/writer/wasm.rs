//! logging functions from wasm-bindgen.
//!
//! Here use the one-param logging functions, all messages should be transformed
//! to string before passing to the functions. Note that we only need this
//! module for `wasm32-unknown-unknown` target
#![cfg(not(feature = "termcolor"))]
#![cfg(all(target_arch = "wasm32", target_vendor = "unknown"))]

// use log::Level;
use wasm_bindgen::prelude::*;

use crate::fmt::glob::Target;

pub(in crate::fmt::writer) fn print(msg: &str, t: Target) {
    // work around for unused variable
    let _ = t;

    log(&msg);
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace=console)]
    fn error(text: &str);

    #[wasm_bindgen(js_namespace=console)]
    fn info(text: &str);

    #[wasm_bindgen(js_namespace=console)]
    fn warn(text: &str);

    #[wasm_bindgen(js_namespace=console)]
    fn debug(text: &str);

    #[wasm_bindgen(js_namespace=console)]
    fn log(text: &str);
}
