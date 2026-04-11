#[cfg(feature = "cli")]
pub mod cli;
mod colors;
mod complete;
mod shared;
#[cfg(feature = "uterm")]
pub mod uterm;
#[cfg(feature = "mimalloc")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;
#[cfg(feature = "wasm")]
#[cfg_attr(feature = "wasm", wasm_bindgen(start))]
fn main() {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    #[cfg(debug_assertions)]
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    crate::uterm::uterm();
}
