mod history;
pub use history::History;
pub use readchar::{
    Clear, ClearType, Color, Complete, Event, KeyCode, KeyEvent, KeyModifiers, MoveTo, NoColor,
    NoComplete, ReadChar, Return, ToColor,
};
mod readchar;
#[cfg(test)]
mod tests;
#[cfg(feature = "crossterm")]
pub use crossterm;
pub use enumset;
