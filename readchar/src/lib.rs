mod history;
pub use history::History;
pub use readchar::{
    Color, Complete, Event, KeyCode, KeyEvent, KeyModifiers, NoColor, NoComplete, ReadChar, Return,
    ToColor,
};
mod readchar;
#[cfg(test)]
mod tests;
#[cfg(feature = "crossterm")]
pub use crossterm;
