use crate::history::History;
use crate::readchar::ReadChar;
use std::io::Write;
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
pub struct Terminal {
    lines: Vec<Vec<char>>,
    col: usize,
    row: usize,
    max_cols: usize,
}
impl Default for Terminal {
    fn default() -> Self {
        Self {
            lines: vec![vec![]],
            col: 0,
            row: 0,
            max_cols: 16,
        }
    }
}
impl PartialEq<str> for Terminal {
    fn eq(&self, str: &str) -> bool {
        let mut iter = str.chars();
        for row in &self.lines {
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
impl Write for Terminal {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        if let Ok(buf) = str::from_utf8(buf) {
            let mut escape = false;
            let mut var_1 = None;
            let mut var_2 = None;
            for c in buf.chars() {
                match c {
                    '\x1b' => {
                        escape = true;
                        var_1 = None;
                        var_2 = None;
                    }
                    '[' if escape => {}
                    'H' if escape => {
                        let Some(v1) = var_1 else { unreachable!() };
                        let Some(v2) = var_2 else { unreachable!() };
                        escape = false;
                        self.col = v1;
                        self.row = v2;
                    }
                    'E' if escape => {
                        let Some(v1) = var_1 else { unreachable!() };
                        escape = false;
                        self.col = v1;
                    }
                    'G' if escape => {
                        let Some(v1) = var_1 else { unreachable!() };
                        escape = false;
                        self.row += v1;
                    }
                    'F' if escape => {
                        let Some(v1) = var_1 else { unreachable!() };
                        escape = false;
                        self.row -= v1;
                    }
                    ';' if escape => {
                        var_2 = Some(0);
                    }
                    c if escape && c.is_ascii_digit() => {
                        if let Some(var) = var_2 {
                            var_2 = Some(var * 10 + c.to_digit(10).unwrap() as usize);
                        } else {
                            let var = var_1.unwrap_or(0);
                            var_1 = Some(var * 10 + c.to_digit(10).unwrap() as usize);
                        }
                    }
                    _ if escape => {
                        unreachable!()
                    }
                    c => {
                        if self.lines[self.row].len() == self.col {
                            self.lines[self.row].push(c);
                        } else {
                            self.lines[self.row][self.col] = c
                        }
                    }
                }
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
fn test_print() {
    let mut readchar = ReadChar::no_terminal();
    let mut s = Vec::new();
    let mut o = String::new();
    readchar.carrot(&mut s).unwrap();
    assert_eq!(str::from_utf8(&s).unwrap(), "> ");
    let alpha = "abcdefghijklmnopqrstuvwxyz";
    readchar
        .put_str(&mut s, &mut o, |_, s| s.push_str("res"), alpha)
        .unwrap();
    assert_eq!(
        get_str(str::from_utf8(&s).unwrap()).as_str(),
        format!("> {alpha}\nres")
    );
    readchar.put_str(&mut s, &mut o, |_, _| (), alpha).unwrap();
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
    readchar.put_str(&mut s, &mut o, |_, _| (), alpha).unwrap();
    assert_eq!(
        get_str(str::from_utf8(&s).unwrap()).as_str(),
        format!("> {alpha}\nres{alpha}{alpha}\nres{alpha}{alpha}{alpha}\n\nres")
    );
    assert_eq!(readchar.cursor, 2 * alpha.len() as u16);
    assert_eq!(readchar.cursor_col, 6);
    assert_eq!(readchar.cursor_row, 3);
}
