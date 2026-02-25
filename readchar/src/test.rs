use crate::ReadChar;
use crate::history::History;
impl ReadChar<()> {
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
            last: None,
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
#[test]
fn test_print() {
    let mut readchar = ReadChar::no_terminal();
    let mut s = Vec::new();
    let mut o = String::new();
    readchar.carrot(&mut s).unwrap();
    assert_eq!(str::from_utf8(&s).unwrap(), "> ");
    let alpha = "abcdefghijklmnopqrstuvwxyz";
    readchar
        .put_str(
            &mut s,
            &mut o,
            |_, s| {
                s.push_str("res");
                None
            },
            alpha,
        )
        .unwrap();
    assert_eq!(
        get_str(str::from_utf8(&s).unwrap()).as_str(),
        format!("> {alpha}\nres")
    );
    readchar
        .put_str(&mut s, &mut o, |_, _| None, alpha)
        .unwrap();
    assert_eq!(
        get_str(str::from_utf8(&s).unwrap()).as_str(),
        format!("> {alpha}\nres{alpha}{alpha}\nres")
    );
    assert_eq!(readchar.cursor, 2 * alpha.len() as u16);
    assert_eq!(readchar.cursor_col, 6);
    assert_eq!(readchar.cursor_row, 3);
    readchar.left(alpha.len() as u16, &mut s).unwrap();
    assert_eq!(readchar.cursor, alpha.len() as u16);
    assert_eq!(readchar.cursor_col, 12);
    assert_eq!(readchar.cursor_row, 1);
    readchar
        .put_str(&mut s, &mut o, |_, _| None, alpha)
        .unwrap();
    assert_eq!(
        get_str(str::from_utf8(&s).unwrap()).as_str(),
        format!("> {alpha}\nres{alpha}{alpha}\nres{alpha}{alpha}{alpha} \nres")
    );
    assert_eq!(readchar.cursor, 2 * alpha.len() as u16);
    assert_eq!(readchar.cursor_col, 6);
    assert_eq!(readchar.cursor_row, 3);
}
