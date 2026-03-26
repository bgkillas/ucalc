mod history;
pub use history::History;
pub use readchar::{Complete, NoColor, NoComplete, ReadChar, Return, ToColor};
mod readchar;
#[cfg(test)]
mod test;
pub use crossterm;
