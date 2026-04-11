#[cfg(feature = "cli")]
use crate::cli::cli;
#[cfg(feature = "uterm")]
use crate::uterm::uterm;
#[cfg(feature = "cli")]
mod cli;
mod colors;
mod complete;
#[cfg(feature = "uterm")]
mod uterm;
#[cfg(feature = "mimalloc")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;
#[cfg(feature = "cli")]
fn main() {
    cli();
}
#[cfg(feature = "uterm")]
fn main() {
    uterm();
}
