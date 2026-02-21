use crossterm::cursor::{MoveTo, MoveToColumn, MoveToPreviousLine};
use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::terminal::{BeginSynchronizedUpdate, Clear, ClearType, EndSynchronizedUpdate};
use crossterm::{ExecutableCommand, event, terminal};
use std::io::{StdoutLock, Write, stdout};
use std::process::exit;
pub struct Out<T> {
    line: String,
    row: u16,
    col: u16,
    cursor_row: u16,
    cursor_row_max: u16,
    cursor_col: u16,
    new_lines: u16,
    last_failed: bool,
    last_succeed: Option<T>,
    last: Option<T>,
    carrot: &'static str,
}
impl<T> Default for Out<T> {
    fn default() -> Self {
        terminal::enable_raw_mode().unwrap();
        #[cfg(debug_assertions)]
        let hook = std::panic::take_hook();
        #[cfg(debug_assertions)]
        std::panic::set_hook(Box::new(move |info| {
            stdout().execute(EndSynchronizedUpdate).unwrap();
            _ = terminal::disable_raw_mode();
            println!();
            hook(info);
        }));
        let (col, row) = terminal::size().unwrap();
        Self {
            line: String::with_capacity(64),
            row,
            col,
            cursor_row: 0,
            cursor_row_max: 0,
            cursor_col: 0,
            new_lines: 1,
            last: None,
            last_failed: false,
            last_succeed: None,
            carrot: "> ",
        }
    }
}
impl<T> Drop for Out<T> {
    fn drop(&mut self) {
        terminal::disable_raw_mode().unwrap();
    }
}
impl<T> Out<T> {
    pub fn print_result(
        &mut self,
        string: &mut String,
        stdout: &mut StdoutLock,
        run: impl FnOnce(&str, &mut String) -> Option<Option<T>>,
    ) {
        println!();
        stdout.execute(MoveToColumn(0)).unwrap();
        stdout.execute(Clear(ClearType::FromCursorDown)).unwrap();
        let n = run(&self.line, string);
        let count = string
            .lines()
            .map(|l| (l.len() as u16).div_ceil(self.col))
            .sum::<u16>()
            + 1;
        print!("{string}");
        string.clear();
        self.new_lines = count;
        self.last_failed = n.is_none();
        if let Some(o) = n {
            self.last = o;
        }
        stdout
            .execute(MoveToPreviousLine(self.new_lines - 1))
            .unwrap();
        stdout.execute(MoveToColumn(self.col())).unwrap();
    }
    fn col(&self) -> u16 {
        self.cursor_col
            + if self.cursor_row == 0 {
                self.carrot.len() as u16
            } else {
                0
            }
    }
    fn left(&mut self, n: u16, stdout: &mut StdoutLock) -> bool {
        if self.col() < n {
            self.cursor_col = self.col
                - (n - self.col())
                - if self.cursor_row == 1 {
                    self.carrot.len() as u16
                } else {
                    0
                };
            self.cursor_row -= 1;
            stdout.execute(MoveToPreviousLine(1)).unwrap();
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
                stdout.execute(Clear(ClearType::CurrentLine)).unwrap();
            }
            true
        } else {
            self.cursor_col += n;
            false
        }
    }
    pub fn init(&mut self, stdout: &mut StdoutLock) {
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
        stdout.execute(BeginSynchronizedUpdate).unwrap();
        match k {
            Event::Resize(col, row) => {
                (self.row, self.col) = (row, col);
            }
            Event::Key(KeyEvent {
                code: KeyCode::Enter,
                ..
            }) => {
                self.cursor_col = 0;
                self.cursor_row = 0;
                self.cursor_row_max = 0;
                if self.last_failed {
                    self.last = self.last_succeed.take();
                } else if let Some(n) = self.last.take() {
                    finish(&n);
                    self.last_succeed = Some(n);
                }
                for _ in 0..self.new_lines {
                    println!()
                }
                stdout.execute(MoveToColumn(0)).unwrap();
                match self.line.as_str() {
                    "exit" => {
                        stdout.execute(EndSynchronizedUpdate).unwrap();
                        stdout.flush().unwrap();
                        terminal::disable_raw_mode().unwrap();
                        exit(0);
                    }
                    "clear" => {
                        stdout.execute(Clear(ClearType::All)).unwrap();
                        stdout.execute(MoveTo(0, 0)).unwrap();
                    }
                    _ => {}
                }
                self.init(stdout);
                stdout.flush().unwrap();
                self.line.clear();
                self.new_lines = 1;
            }
            Event::Key(KeyEvent {
                code: KeyCode::Backspace,
                ..
            }) if self.cursor_col != 0 || self.cursor_row != 0 => {
                self.line.pop();
                self.left(1, stdout);
                stdout.execute(MoveToColumn(self.col())).unwrap();
                print!(" ");
                stdout.execute(MoveToColumn(self.col())).unwrap();
                self.print_result(string, stdout, run);
                stdout.flush().unwrap();
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char(c),
                ..
            }) => {
                self.line.push(c);
                print!("{c}");
                self.right(1, stdout);
                self.print_result(string, stdout, run);
                stdout.flush().unwrap();
            }
            _ => {}
        }
        stdout.execute(EndSynchronizedUpdate).unwrap();
    }
}
