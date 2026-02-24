pub use crossterm;
use crossterm::cursor::{MoveTo, MoveToColumn, MoveToNextLine, MoveToPreviousLine};
use crossterm::event::{
    DisableBracketedPaste, EnableBracketedPaste, Event, KeyCode, KeyEvent, KeyModifiers,
};
use crossterm::style::{Color, ResetColor, SetForegroundColor};
use crossterm::terminal::{Clear, ClearType};
use crossterm::{ExecutableCommand, QueueableCommand, event, terminal};
use std::io;
use std::io::{Write, stdout};
use std::process::exit;
pub struct ReadChar<T> {
    line: String,
    line_len: usize,
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
    carrot_color: Option<Color>,
}
impl<T> Default for ReadChar<T> {
    fn default() -> Self {
        terminal::enable_raw_mode().unwrap();
        #[cfg(debug_assertions)]
        let hook = std::panic::take_hook();
        #[cfg(debug_assertions)]
        std::panic::set_hook(Box::new(move |info| {
            stdout().execute(DisableBracketedPaste).unwrap();
            _ = terminal::disable_raw_mode();
            println!();
            hook(info);
        }));
        let (col, row) = terminal::size().unwrap();
        Self {
            line: String::with_capacity(64),
            line_len: 0,
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
            carrot_color: Some(Color::DarkBlue),
        }
    }
}
impl<T> Drop for ReadChar<T> {
    fn drop(&mut self) {
        terminal::disable_raw_mode().unwrap();
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
impl<T> ReadChar<T> {
    pub fn out_lines(&self, string: &str) -> u16 {
        string
            .lines()
            .map(|l| str_len(l).div_ceil(self.col))
            .sum::<u16>()
    }
    pub fn print_result(
        &mut self,
        string: &mut String,
        stdout: &mut impl Write,
        run: impl FnOnce(&str, &mut String) -> Option<Option<T>>,
    ) -> Result<(), io::Error> {
        writeln!(stdout)?;
        stdout.queue(MoveToColumn(0))?;
        stdout.queue(Clear(ClearType::FromCursorDown))?;
        let n = run(&self.line, string);
        self.new_lines = self.out_lines(string);
        write!(stdout, "{string}")?;
        self.last_failed = n.is_none();
        if let Some(o) = n {
            self.last = o;
        }
        stdout.queue(MoveToPreviousLine(
            self.new_lines + self.cursor_row_max - self.cursor_row,
        ))?;
        stdout.queue(MoveToColumn(self.col()))?;
        Ok(())
    }
    fn col(&self) -> u16 {
        self.cursor_col
            + if self.cursor_row == 0 {
                self.carrot.len() as u16
            } else {
                0
            }
    }
    fn left(&mut self, n: u16, stdout: &mut impl Write) -> Result<bool, io::Error> {
        if self.col() < n {
            self.cursor_col = self.col
                - (n - self.col())
                - if self.cursor_row == 1 {
                    self.carrot.len() as u16
                } else {
                    0
                };
            stdout.queue(MoveToPreviousLine(1))?;
            self.cursor_row -= 1;
            Ok(true)
        } else {
            self.cursor_col -= n;
            Ok(false)
        }
    }
    fn right(&mut self, n: u16, _stdout: &mut impl Write) -> Result<bool, io::Error> {
        if self.col() + n >= self.col {
            self.cursor_col = self.col() + n - self.col;
            self.cursor_row += 1;
            Ok(true)
        } else {
            self.cursor_col += n;
            Ok(false)
        }
    }
    pub fn init(&mut self, stdout: &mut impl Write) -> Result<(), io::Error> {
        stdout.queue(EnableBracketedPaste)?;
        self.carrot(stdout)?;
        stdout.flush()?;
        Ok(())
    }
    pub fn carrot(&mut self, stdout: &mut impl Write) -> Result<(), io::Error> {
        if let Some(color) = self.carrot_color {
            write!(
                stdout,
                "{}{}{}",
                SetForegroundColor(color),
                self.carrot,
                ResetColor
            )
        } else {
            write!(stdout, "{}", self.carrot)
        }
    }
    pub fn read(
        &mut self,
        stdout: &mut impl Write,
        string: &mut String,
        run: impl FnOnce(&str, &mut String) -> Option<Option<T>>,
        finish: impl FnOnce(&T),
    ) -> Result<(), io::Error> {
        let Ok(k) = event::read() else {
            return Ok(());
        };
        match k {
            Event::Paste(s) => {
                self.line.insert_str(self.insert, &s);
                self.insert += s.len();
                write!(stdout, "{s}{}", &self.line[self.insert..])?;
                if self.right(s.len() as u16, stdout)? {
                    stdout.queue(MoveToNextLine(1))?;
                }
                self.print_result(string, stdout, run)?;
                stdout.flush()?;
            }
            Event::Resize(col, row) => {
                if self.col > col {
                    let mut wrap = self.cursor_row * self.col.div_ceil(col);
                    if self.cursor_row == self.cursor_row_max {
                        wrap +=
                            ((self.line_len + self.carrot.len()) as u16 % self.col).div_ceil(col);
                    }
                    if wrap != 0 {
                        stdout.queue(MoveToPreviousLine(wrap - 1))?;
                    }
                    stdout.queue(MoveToColumn(self.carrot.len() as u16))?;
                    stdout.queue(Clear(ClearType::FromCursorDown))?;
                    writeln!(stdout, "{}", self.line)?;
                    stdout.queue(MoveToColumn(0))?;
                    write!(stdout, "{string}")?;
                    stdout.flush()?;
                } else if self.col < col {
                    if self.cursor_row != 0 {
                        stdout.queue(MoveToPreviousLine(self.cursor_row))?;
                    }
                    stdout.queue(MoveToColumn(self.carrot.len() as u16))?;
                    stdout.queue(Clear(ClearType::FromCursorDown))?;
                    writeln!(stdout, "{}", self.line)?;
                    stdout.queue(MoveToColumn(0))?;
                    write!(stdout, "{string}")?;
                    stdout.flush()?;
                } else if self.cursor_row_max + self.new_lines > self.row
                    && self.cursor_row_max + self.new_lines <= row
                {
                    //stdout.flush()?;
                }
                (self.row, self.col) = (row, col);
            }
            Event::Key(KeyEvent {
                code: KeyCode::Enter,
                ..
            }) => {
                self.cursor_col = 0;
                self.cursor_row = 0;
                self.cursor_row_max = 0;
                self.insert = 0;
                self.line_len = 0;
                if self.last_failed {
                    self.last = self.last_succeed.take();
                } else if let Some(n) = self.last.take() {
                    finish(&n);
                    self.last_succeed = Some(n);
                }
                for _ in 0..=self.new_lines {
                    writeln!(stdout)?;
                }
                stdout.queue(MoveToColumn(0))?;
                match self.line.as_str() {
                    "exit" => {
                        stdout.queue(DisableBracketedPaste)?;
                        terminal::disable_raw_mode()?;
                        stdout.flush()?;
                        exit(0);
                    }
                    "clear" => {
                        stdout.queue(Clear(ClearType::Purge))?;
                        stdout.queue(Clear(ClearType::All))?;
                        stdout.queue(MoveTo(0, 0))?;
                    }
                    _ => {}
                }
                self.carrot(stdout)?;
                stdout.flush()?;
                self.line.clear();
                self.new_lines = 0;
            }
            Event::Key(KeyEvent {
                code: KeyCode::Left,
                ..
            }) if self.insert != 0 => {
                self.insert -= self.line
                    [self.line.floor_char_boundary(self.insert - 1)..self.insert]
                    .chars()
                    .next()
                    .unwrap()
                    .len_utf8();
                self.left(1, stdout)?;
                stdout.queue(MoveToColumn(self.col()))?;
                stdout.flush()?;
            }
            Event::Key(KeyEvent {
                code: KeyCode::Right,
                ..
            }) if self.insert != self.line_len => {
                self.insert += self.line
                    [self.insert..self.line.ceil_char_boundary(self.insert + 1)]
                    .chars()
                    .next()
                    .unwrap()
                    .len_utf8();
                if self.right(1, stdout)? {
                    stdout.queue(MoveToNextLine(1))?;
                }
                stdout.queue(MoveToColumn(self.col()))?;
                stdout.flush()?;
            }
            Event::Key(KeyEvent {
                code: KeyCode::Backspace,
                ..
            }) if self.insert != 0 => {
                self.insert -= self
                    .line
                    .remove(self.line.floor_char_boundary(self.insert - 1))
                    .len_utf8();
                self.line_len -= 1;
                self.left(1, stdout)?;
                stdout.queue(MoveToColumn(self.col()))?;
                write!(stdout, "{} ", &self.line[self.insert..])?;
                stdout.queue(MoveToColumn(self.col()))?;
                self.print_result(string, stdout, run)?;
                stdout.flush()?;
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char(c),
                modifiers: KeyModifiers::NONE,
                ..
            })
            | Event::Key(KeyEvent {
                code: KeyCode::Char(c),
                modifiers: KeyModifiers::SHIFT,
                ..
            }) => {
                self.line.insert(self.insert, c);
                self.insert += c.len_utf8();
                self.line_len += 1;
                if self.cursor_row != 0 {
                    stdout.queue(MoveToPreviousLine(self.cursor_row))?;
                }
                stdout.queue(MoveToColumn(self.carrot.len() as u16))?;
                self.right(1, stdout)?;
                if (self.line_len + self.carrot.len()).is_multiple_of(self.col as usize) {
                    self.cursor_row_max += 1;
                    stdout.queue(Clear(ClearType::FromCursorDown))?;
                    writeln!(stdout, "{}", self.line)?;
                } else {
                    write!(stdout, "{}", self.line)?;
                }
                self.print_result(string, stdout, run)?;
                stdout.flush()?;
            }
            _ => {}
        }
        Ok(())
    }
}
