pub use crossterm;
use crossterm::cursor::{MoveTo, MoveToColumn, MoveToPreviousLine};
use crossterm::event::{DisableBracketedPaste, EnableBracketedPaste, Event, KeyCode, KeyEvent};
use crossterm::terminal::{BeginSynchronizedUpdate, Clear, ClearType, EndSynchronizedUpdate};
use crossterm::{ExecutableCommand, QueueableCommand, event, terminal};
use std::io::{StdoutLock, Write, stdout};
use std::process::exit;
pub struct Out<T> {
    line: String,
    row: u16,
    col: u16,
    cursor_row: u16,
    cursor_row_max: u16,
    cursor_col: u16,
    insert: usize,
    new_lines: u16,
    last_failed: bool,
    last_succeed: Option<T>,
    last: Option<T>,
    carrot: Box<str>,
    carrot_len: u16,
}
impl<T> Default for Out<T> {
    fn default() -> Self {
        terminal::enable_raw_mode().unwrap();
        #[cfg(debug_assertions)]
        let hook = std::panic::take_hook();
        #[cfg(debug_assertions)]
        std::panic::set_hook(Box::new(move |info| {
            stdout().execute(EndSynchronizedUpdate).unwrap();
            stdout().execute(DisableBracketedPaste).unwrap();
            _ = terminal::disable_raw_mode();
            println!();
            hook(info);
        }));
        let (col, row) = terminal::size().unwrap();
        Self {
            line: String::with_capacity(64),
            row,
            col,
            insert: 0,
            cursor_row: 0,
            cursor_row_max: 0,
            cursor_col: 0,
            new_lines: 1,
            last: None,
            last_failed: false,
            last_succeed: None,
            carrot: "> ".into(),
            carrot_len: 2,
        }
    }
}
impl<T> Drop for Out<T> {
    fn drop(&mut self) {
        terminal::disable_raw_mode().unwrap();
        stdout().execute(EndSynchronizedUpdate).unwrap();
        stdout().execute(DisableBracketedPaste).unwrap();
    }
}
fn str_len(s: impl AsRef<str>) -> u16 {
    let mut i = 0;
    let mut csi = false;
    for c in s.as_ref().chars() {
        if c == '\x1b' {
            csi = true;
        } else if csi && c == 'm' {
            csi = false;
        }
        if !csi {
            i += 1;
        }
    }
    i
}
impl<T> Out<T> {
    pub fn print_result(
        &mut self,
        string: &mut String,
        stdout: &mut StdoutLock,
        run: impl FnOnce(&str, &mut String) -> Option<Option<T>>,
    ) {
        println!();
        stdout.queue(MoveToColumn(0)).unwrap();
        stdout.queue(Clear(ClearType::FromCursorDown)).unwrap();
        let n = run(&self.line, string);
        let count = string
            .lines()
            .map(|l| str_len(l).div_ceil(self.col))
            .sum::<u16>()
            + 1;
        print!("{string}");
        self.new_lines = count;
        self.last_failed = n.is_none();
        if let Some(o) = n {
            self.last = o;
        }
        stdout
            .queue(MoveToPreviousLine(self.new_lines - 1))
            .unwrap();
        stdout.queue(MoveToColumn(self.col())).unwrap();
    }
    fn col(&self) -> u16 {
        self.cursor_col
            + if self.cursor_row == 0 {
                self.carrot_len
            } else {
                0
            }
    }
    fn left(&mut self, n: u16, stdout: &mut StdoutLock) -> bool {
        if self.col() < n {
            self.cursor_col = self.col
                - (n - self.col())
                - if self.cursor_row == 1 {
                    self.carrot_len
                } else {
                    0
                };
            self.cursor_row -= 1;
            stdout.queue(MoveToPreviousLine(1)).unwrap();
            true
        } else {
            self.cursor_col -= n;
            false
        }
    }
    fn right(&mut self, n: u16, stdout: &mut StdoutLock) -> bool {
        if self.col() + n >= self.col {
            self.cursor_col = self.col() + n - self.col;
            self.cursor_row += 1;
            println!();
            if self.cursor_row > self.cursor_row_max {
                self.cursor_row_max = self.cursor_row;
                stdout.queue(Clear(ClearType::CurrentLine)).unwrap();
            }
            true
        } else {
            self.cursor_col += n;
            false
        }
    }
    pub fn init(&mut self, stdout: &mut StdoutLock) {
        stdout.queue(EnableBracketedPaste).unwrap();
        print!("{}", self.carrot);
        stdout.flush().unwrap();
    }
    pub fn read(
        &mut self,
        stdout: &mut StdoutLock,
        string: &mut String,
        run: impl FnOnce(&str, &mut String) -> Option<Option<T>>,
        finish: impl FnOnce(&T),
    ) {
        let Ok(k) = event::read() else {
            return;
        };
        match k {
            Event::Paste(s) => {
                stdout.queue(BeginSynchronizedUpdate).unwrap();
                string.clear();
                self.line.insert_str(self.insert, &s);
                self.insert += s.len();
                print!("{s}");
                self.right(s.len() as u16, stdout);
                self.print_result(string, stdout, run);
                stdout.queue(EndSynchronizedUpdate).unwrap();
                stdout.flush().unwrap();
            }
            Event::Resize(col, row) => {
                (self.row, self.col) = (row, col);
                //TODO refresh
                stdout.queue(BeginSynchronizedUpdate).unwrap();
                stdout.queue(EndSynchronizedUpdate).unwrap();
                stdout.flush().unwrap();
            }
            Event::Key(KeyEvent {
                code: KeyCode::Enter,
                ..
            }) => {
                stdout.queue(BeginSynchronizedUpdate).unwrap();
                self.cursor_col = 0;
                self.cursor_row = 0;
                self.cursor_row_max = 0;
                self.insert = 0;
                if self.last_failed {
                    self.last = self.last_succeed.take();
                } else if let Some(n) = self.last.take() {
                    finish(&n);
                    self.last_succeed = Some(n);
                }
                for _ in 0..self.new_lines {
                    println!()
                }
                stdout.queue(MoveToColumn(0)).unwrap();
                match self.line.as_str() {
                    "exit" => {
                        stdout.queue(EndSynchronizedUpdate).unwrap();
                        stdout.queue(DisableBracketedPaste).unwrap();
                        terminal::disable_raw_mode().unwrap();
                        stdout.flush().unwrap();
                        exit(0);
                    }
                    "clear" => {
                        stdout.queue(Clear(ClearType::Purge)).unwrap();
                        stdout.queue(Clear(ClearType::All)).unwrap();
                        stdout.queue(MoveTo(0, 0)).unwrap();
                    }
                    _ => {}
                }
                self.init(stdout);
                stdout.queue(EndSynchronizedUpdate).unwrap();
                stdout.flush().unwrap();
                self.line.clear();
                self.new_lines = 1;
            }
            Event::Key(KeyEvent {
                code: KeyCode::Backspace,
                ..
            }) if self.cursor_col != 0 || self.cursor_row != 0 => {
                stdout.queue(BeginSynchronizedUpdate).unwrap();
                string.clear();
                self.insert -= self.line.pop().unwrap().len_utf8();
                self.left(1, stdout);
                stdout.queue(MoveToColumn(self.col())).unwrap();
                print!(" ");
                stdout.queue(MoveToColumn(self.col())).unwrap();
                self.print_result(string, stdout, run);
                stdout.queue(EndSynchronizedUpdate).unwrap();
                stdout.flush().unwrap();
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char(c),
                ..
            }) => {
                stdout.queue(BeginSynchronizedUpdate).unwrap();
                string.clear();
                self.line.insert(self.insert, c);
                self.insert += c.len_utf8();
                print!("{c}");
                self.right(1, stdout);
                self.print_result(string, stdout, run);
                stdout.queue(EndSynchronizedUpdate).unwrap();
                stdout.flush().unwrap();
            }
            _ => {}
        }
    }
}
