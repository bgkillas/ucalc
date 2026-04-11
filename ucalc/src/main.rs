#[cfg(feature = "cli")]
use ucalc::cli::cli;
#[cfg(feature = "uterm")]
use ucalc::uterm::uterm;
#[cfg(feature = "cli")]
fn main() {
    cli();
}
#[cfg(feature = "uterm")]
fn main() {
    uterm();
}
