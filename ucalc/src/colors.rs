use std::fmt;
use std::fmt::{Display, Formatter};
#[derive(Clone, Copy)]
pub enum Color {
    Black(bool),
    Red(bool),
    Green(bool),
    Yellow(bool),
    Blue(bool),
    Magenta(bool),
    Cyan(bool),
    White(bool),
}
impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "\x1b[{}m", usize::from(*self))
    }
}
pub struct Colors {
    pub bracket_colors: Vec<Color>,
    pub default_color: Color,
}
impl Default for Colors {
    fn default() -> Self {
        Self {
            bracket_colors: vec![
                Color::Red(true),
                Color::Green(true),
                Color::Yellow(true),
                Color::Blue(true),
                Color::Magenta(true),
                Color::Cyan(true),
            ],
            default_color: Color::White(true),
        }
    }
}
impl TryFrom<usize> for Color {
    type Error = ();
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Ok(match value {
            30 => Color::Black(false),
            31 => Color::Red(false),
            32 => Color::Green(false),
            33 => Color::Yellow(false),
            34 => Color::Blue(false),
            35 => Color::Magenta(false),
            36 => Color::Cyan(false),
            37 => Color::White(false),
            90 => Color::Black(true),
            91 => Color::Red(true),
            92 => Color::Green(true),
            93 => Color::Yellow(true),
            94 => Color::Blue(true),
            95 => Color::Magenta(true),
            96 => Color::Cyan(true),
            97 => Color::White(true),
            _ => return Err(()),
        })
    }
}
impl From<Color> for usize {
    fn from(value: Color) -> Self {
        match value {
            Color::Black(false) => 30,
            Color::Red(false) => 31,
            Color::Green(false) => 32,
            Color::Yellow(false) => 33,
            Color::Blue(false) => 34,
            Color::Magenta(false) => 35,
            Color::Cyan(false) => 36,
            Color::White(false) => 37,
            Color::Black(true) => 90,
            Color::Red(true) => 91,
            Color::Green(true) => 92,
            Color::Yellow(true) => 93,
            Color::Blue(true) => 94,
            Color::Magenta(true) => 95,
            Color::Cyan(true) => 96,
            Color::White(true) => 97,
        }
    }
}
pub fn color_brackets(line: &str, colors: &Colors) -> impl Display {
    fmt::from_fn(|f| {
        let mut bracket = 0;
        for c in line.chars() {
            match c {
                '(' => {
                    write!(
                        f,
                        "{}",
                        colors.bracket_colors[bracket % colors.bracket_colors.len()]
                    )?;
                    write!(f, "{c}")?;
                    write!(f, "{}", colors.default_color)?;
                    bracket += 1;
                }
                ')' => {
                    bracket -= 1;
                    write!(
                        f,
                        "{}",
                        colors.bracket_colors[bracket % colors.bracket_colors.len()]
                    )?;
                    write!(f, "{c}")?;
                    write!(f, "{}", colors.default_color)?;
                }
                c => {
                    write!(f, "{c}")?;
                }
            }
        }
        Ok(())
    })
}
