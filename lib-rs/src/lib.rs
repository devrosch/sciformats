#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

pub mod andi;
pub mod api;
pub mod common;
pub mod spc;
pub(crate) mod utils;
pub mod bind;

#[cfg(target_family = "wasm")]
#[wasm_bindgen(start)]
pub fn start() {
    use web_sys::console;

    const NAME: &str = env!("CARGO_PKG_NAME");
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    console::log_1(&format!("Rust: {} {} loaded", NAME, VERSION).into());
}
