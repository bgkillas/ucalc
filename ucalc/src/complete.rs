use crate::colors::{Colors, color_brackets};
use std::fmt::Display;
use ucalc_lib::FUNCTION_LIST;
#[repr(transparent)]
pub struct Complete<'a>(pub &'a Colors);
impl<'b> readchar::Complete<'b> for Complete<'b> {
    fn run<'a>(self, str: &'a str) -> Vec<(impl Display + 'a, usize)>
    where
        'b: 'a,
    {
        complete(str, self.0)
    }
}
fn complete<'a, 'b: 'a>(mut line: &'a str, colors: &'b Colors) -> Vec<(impl Display + 'a, usize)> {
    if line.ends_with(',') {
        let mut bracket = 0;
        for (i, c) in line.char_indices().rev() {
            if c == ')' {
                bracket += 1;
            } else if c == '(' {
                if bracket == 0 {
                    line = &line[..i];
                    break;
                } else {
                    bracket -= 1;
                }
            }
        }
    }
    if line.ends_with(['(', '{', '[', '|']) {
        line = &line[..line.len() - 1];
    }
    let word = if let Some(idx) = line.rfind(|c: char| !c.is_ascii_alphabetic() && c != '_') {
        if idx + 1 == line.len() {
            return Vec::new();
        }
        &line[idx + 1..]
    } else {
        line
    };
    let mut ret = Vec::with_capacity(FUNCTION_LIST.len());
    for w in FUNCTION_LIST {
        if w.starts_with(word) {
            ret.push((color_brackets(w, colors), w.len()))
        }
    }
    ret
}
