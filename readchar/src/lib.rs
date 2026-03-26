mod history;
pub use history::History;
pub use readchar::{NoColor, ReadChar, Return, ToColor};
mod readchar;
#[cfg(test)]
mod test;
pub use crossterm;
