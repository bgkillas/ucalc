use std::fmt;
use std::fmt::{Display, Formatter, Write};
use ucalc_lib::Operators;
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
#[repr(transparent)]
pub struct ToColor<'a>(pub &'a Colors);
impl<'b> readchar::ToColor<'b> for ToColor<'b> {
    fn run<'a>(self, str: &'a str) -> impl Display + 'a
    where
        'b: 'a,
    {
        color_brackets(str, self.0)
    }
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
fn write_bracket(f: &mut impl Write, colors: &Colors, bracket: usize, c: char) -> fmt::Result {
    write!(
        f,
        "{}",
        colors.bracket_colors[bracket % colors.bracket_colors.len()]
    )?;
    write!(f, "{c}")?;
    write!(f, "{}", colors.default_color)?;
    Ok(())
}
pub fn color_brackets<'a, 'b: 'a>(line: &'a str, colors: &'b Colors) -> impl Display + 'a {
    fmt::from_fn(|f| {
        let mut bracket = 0;
        let mut abs = 0;
        let mut last_abs = false;
        let mut req_input = false;
        let mut chars = line.char_indices();
        while let Some((i, c)) = chars.next() {
            match c {
                '|' => {
                    if abs == 0 || last_abs || req_input {
                        write_bracket(f, colors, bracket, c)?;
                        bracket += 1;
                        abs += 1;
                        last_abs = true;
                        req_input = false;
                    } else {
                        bracket -= 1;
                        write_bracket(f, colors, bracket, c)?;
                        abs -= 1;
                        last_abs = false;
                    }
                }
                '(' | '[' | '{' => {
                    write_bracket(f, colors, bracket, c)?;
                    bracket += 1;
                    last_abs = false;
                    req_input = false;
                }
                ')' | ']' | '}' => {
                    bracket -= 1;
                    write_bracket(f, colors, bracket, c)?;
                    last_abs = false;
                }
                c => {
                    write!(f, "{c}")?;
                    let mut l = c.len_utf8();
                    if let Some(next) = line[i + l..].chars().next()
                        && Operators::try_from(&line[i..i + l + next.len_utf8()]).is_ok()
                    {
                        chars.next();
                        write!(f, "{next}")?;
                        l += next.len_utf8();
                    }
                    let s = &line[i..i + l];
                    if let Ok(operator) = Operators::try_from(s)
                        && (operator.inputs().get() == 2 || operator.unary_left())
                    {
                        req_input = true;
                    } else {
                        req_input = false;
                    }
                    last_abs = false;
                }
            }
        }
        Ok(())
    })
}
