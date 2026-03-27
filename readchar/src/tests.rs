//use std::array;
//use crate::Return;
use crate::NoColor;
use crate::history::History;
use crate::readchar::ReadChar;
//use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
//use std::io::Write;
impl ReadChar {
    fn no_terminal() -> Self {
        Self {
            line: String::with_capacity(64),
            line_len: 0,
            row: 16,
            col: 16,
            insert: 0,
            cursor: 0,
            cursor_row: 0,
            cursor_row_max: 0,
            cursor_col: 0,
            new_lines: 0,
            history: History::default(),
            carrot: "> ",
            carrot_color: None,
        }
    }
}
fn get_str(s: impl AsRef<str>) -> String {
    let mut ret = String::new();
    let mut csi = false;
    for c in s.as_ref().chars() {
        if c == '\x1b' {
            csi = true;
        } else if csi && matches!(c, 'm' | 'G' | 'J') {
            csi = false;
        } else if !csi {
            ret.push(c);
        }
    }
    ret
}
/*#[derive(Debug)]
pub struct Grid<const N: usize>([[char;N];N]);
impl<const N: usize> Default for Grid<N> {
    fn default() -> Self {
        Self(array::from_fn(|_|array::from_fn(|_|'\0')))
    }
}
impl<const N: usize> Grid<N> {
    fn clear(&mut self) {
        *self = Self::default()
    }
}
#[derive(Debug)]
pub struct Terminal<const N: usize> {
    lines: Grid<N>,
    col: usize,
    row: usize,
    escape: bool,
    var_1: Option<usize>,
    var_2: Option<usize>,
}
impl<const N: usize> Default for Terminal<N> {
    fn default() -> Self {
        Self {
            lines: Grid::default(),
            col: 0,
            row: 0,
            escape: false,
            var_1: None,
            var_2: None,
        }
    }
}
impl<const N: usize> PartialEq<str> for Terminal<N> {
    fn eq(&self, str: &str) -> bool {
        let mut iter = str.chars();
        for row in &self.lines.0 {
            for c in row {
                if iter.next().is_none_or(|s| s != *c) {
                    return false;
                }
            }
            if iter.next().is_none_or(|s| s != '\n') {
                return false;
            }
        }
        iter.next().is_none()
    }
}
impl<const N: usize> Write for Terminal<N> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        if let Ok(buf) = str::from_utf8(buf) {
            for c in buf.chars() {
                println!("{self:?} {c:?}");
                match c {
                    '\x1b' => {
                        self.escape = true;
                        self.var_1 = None;
                        self.var_2 = None;
                    }
                    '[' if self.escape => {}
                    'H' if self.escape => {
                        let Some(v1) = self.var_1 else { unreachable!() };
                        let Some(v2) = self.var_2 else { unreachable!() };
                        self.escape = false;
                        self.col = v1;
                        self.row = v2;
                    }
                    'E' if self.escape => {
                        let Some(v1) = self.var_1 else { unreachable!() };
                        self.escape = false;
                        self.col = v1;
                    }
                    'G' if self.escape => {
                        let Some(v1) = self.var_1 else { unreachable!() };
                        self.escape = false;
                        self.row += v1;
                    }
                    'F' if self.escape => {
                        let Some(v1) = self.var_1 else { unreachable!() };
                        self.escape = false;
                        self.row -= v1;
                    }
                    'J' if self.escape => {
                        if let Some(v1) = self.var_1 {
                            match v1 {
                                1 => {
                                    todo!()
                                }
                                2 | 3 => {
                                    self.lines.clear();
                                    self.row = 0;
                                    self.col = 0;
                                }
                                _ => {
                                    unreachable!()
                                }
                            }
                        } else {
                            //TODO
                            /*if self.lines.len() >= self.row {
                                self.lines.iter_mut().skip(self.row + 1).drain(self.row + 1..);
                            }
                            self.lines[self.row].drain(self.col..);*/
                        }
                    }
                    'K' if self.escape => {
                        if let Some(v1) = self.var_1 {
                            assert_eq!(v1, 2);
                            //TODO self.lines[self.row].clear();
                            self.col = 0;
                        } else {
                            //TODO self.lines[self.row].drain(self.col..);
                        }
                    }
                    ';' if self.escape => {
                        self.var_2 = Some(0);
                    }
                    c if self.escape && c.is_ascii_digit() => {
                        if let Some(var) = self.var_2 {
                            self.var_2 = Some(var * 10 + c.to_digit(10).unwrap() as usize);
                        } else {
                            let var = self.var_1.unwrap_or(0);
                            self.var_1 = Some(var * 10 + c.to_digit(10).unwrap() as usize);
                        }
                    }
                    _ if self.escape => {
                        unreachable!()
                    }
                    c => {
                        if self.lines[self.row].len() == self.col {
                            self.lines[self.row].push(c);
                        } else {
                            self.lines[self.row][self.col] = c
                        }
                        if self.col + 1 == self.max_cols {
                            self.row += 1;
                            if self.row == self.lines.len() {
                                self.lines.push(Vec::new())
                            }
                            self.col = 0;
                        } else {
                            self.col += 1;
                        }
                    }
                }
                println!("{self:?}");
            }
            Ok(buf.len())
        } else {
            Ok(0)
        }
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
#[test]
fn test_print_term() {
    let mut readchar = ReadChar::no_terminal();
    let mut t = Terminal::default();
    let mut s = String::new();
    readchar.carrot(&mut t).unwrap();
    readchar
        .event(
            &mut t,
            &mut s,
            |_, _| {},
            |_, _, _| Ok(Return::Finish),
            None::<fn(&str) -> Vec<String>>,
            Event::Key(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::NONE)),
        )
        .unwrap();
    assert_eq!(t.row as u16, readchar.cursor_row);
    assert_eq!(t.row, 0);
    assert_eq!(t.col as u16, readchar.cursor_col);
    assert_eq!(t.col, 0);
    assert!(t == *"> c");
}*/
#[test]
fn test_print() {
    let mut readchar = ReadChar::no_terminal();
    let mut s = Vec::new();
    let mut o = String::new();
    readchar.carrot(&mut s).unwrap();
    assert_eq!(str::from_utf8(&s).unwrap(), "> ");
    let alpha = "abcdefghijklmnopqrstuvwxyz";
    readchar
        .put_str(&mut s, &mut o, |_, s| s.push_str("res"), NoColor, alpha)
        .unwrap();
    assert_eq!(
        get_str(str::from_utf8(&s).unwrap()).as_str(),
        format!("> {alpha}\nres")
    );
    readchar
        .put_str(&mut s, &mut o, |_, _| (), NoColor, alpha)
        .unwrap();
    assert_eq!(
        get_str(str::from_utf8(&s).unwrap()).as_str(),
        format!("> {alpha}\nres{alpha}{alpha}\nres")
    );
    assert_eq!(readchar.cursor, 2 * alpha.len() as u16);
    assert_eq!(readchar.cursor_col, 6);
    assert_eq!(readchar.cursor_row, 3);
    readchar.left(alpha.len() as u16).unwrap();
    assert_eq!(readchar.cursor, alpha.len() as u16);
    assert_eq!(readchar.cursor_col, 12);
    assert_eq!(readchar.cursor_row, 1);
    readchar
        .put_str(&mut s, &mut o, |_, _| (), NoColor, alpha)
        .unwrap();
    assert_eq!(
        get_str(str::from_utf8(&s).unwrap()).as_str(),
        format!("> {alpha}\nres{alpha}{alpha}\nres{alpha}{alpha}{alpha}\n\nres")
    );
    assert_eq!(readchar.cursor, 2 * alpha.len() as u16);
    assert_eq!(readchar.cursor_col, 6);
    assert_eq!(readchar.cursor_row, 3);
}
