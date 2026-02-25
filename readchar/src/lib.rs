pub mod history;
use crate::history::History;
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
    line_len: u16,
    row: u16,
    col: u16,
    cursor: u16,
    cursor_row: u16,
    cursor_row_max: u16,
    cursor_col: u16,
    insert: u16,
    new_lines: u16,
    history: History,
    last: Option<T>,
    carrot: &'static str,
    carrot_color: Option<Color>,
}
impl<T> Default for ReadChar<T> {
    fn default() -> Self {
        Self::new(History::new(None).unwrap())
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
    pub fn new(history: History) -> Self {
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
            cursor: 0,
            cursor_row: 0,
            cursor_row_max: 0,
            cursor_col: 0,
            new_lines: 0,
            history,
            last: None,
            carrot: "> ",
            carrot_color: Some(Color::DarkBlue),
        }
    }
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
        run: impl FnOnce(&str, &mut String) -> Option<T>,
    ) -> io::Result<()> {
        self.last = run(&self.line, string);
        self.new_lines = self.out_lines(string);
        writeln!(stdout)?;
        stdout.queue(MoveToColumn(0))?;
        stdout.queue(Clear(ClearType::FromCursorDown))?;
        write!(stdout, "{string}")?;
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
    fn up(&mut self, n: u16, stdout: &mut impl Write) -> io::Result<bool> {
        if self.cursor >= n * self.col {
            self.cursor -= n * self.col;
            self.cursor_row -= 1;
            if self.cursor_row == 0 {
                self.cursor_col -= self.carrot.len() as u16;
            }
            stdout.queue(MoveToPreviousLine(1))?;
        } else {
            if self.cursor_row != 0 {
                self.cursor_row = 0;
                stdout.queue(MoveToPreviousLine(1))?;
                self.cursor_col = self.carrot.len() as u16;
            }
            self.cursor = 0;
            self.cursor_col = 0;
        }
        Ok(false)
    }
    fn down(&mut self, n: u16, stdout: &mut impl Write) -> io::Result<bool> {
        if self.line_len > self.cursor + n * self.col {
            self.cursor = self.line_len.min(self.cursor + n * self.col);
            if self.cursor_row == 0 {
                self.cursor_col += self.carrot.len() as u16;
            }
            self.cursor_row += 1;
            stdout.queue(MoveToNextLine(1))?;
        } else {
            if self.cursor_row_max > self.cursor_row {
                self.cursor_row = self.cursor_row_max;
                stdout.queue(MoveToNextLine(1))?;
            }
            self.cursor = self.line_len;
            self.cursor_col = (self.line_len + self.carrot.len() as u16) % self.col;
        }
        Ok(false)
    }
    fn left(&mut self, n: u16, _stdout: &mut impl Write) -> io::Result<bool> {
        self.cursor -= n;
        if self.col() < n {
            self.cursor_col = self.col
                - (n - self.col())
                - if self.cursor_row == 1 {
                    self.carrot.len() as u16
                } else {
                    0
                };
            self.cursor_row -= 1;
            Ok(true)
        } else {
            self.cursor_col -= n;
            Ok(false)
        }
    }
    fn right(&mut self, n: u16, _stdout: &mut impl Write) -> io::Result<bool> {
        self.cursor += n;
        if self.col() + n >= self.col {
            self.cursor_col = self.col() + n - self.col;
            self.cursor_row += 1;
            Ok(true)
        } else {
            self.cursor_col += n;
            Ok(false)
        }
    }
    pub fn init(&mut self, stdout: &mut impl Write) -> io::Result<()> {
        stdout.queue(EnableBracketedPaste)?;
        self.carrot(stdout)?;
        stdout.flush()?;
        Ok(())
    }
    pub fn carrot(&mut self, stdout: &mut impl Write) -> io::Result<()> {
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
    pub fn move_history(
        &mut self,
        stdout: &mut impl Write,
        string: &mut String,
        run: impl FnOnce(&str, &mut String) -> Option<T>,
        up: bool,
    ) -> io::Result<()> {
        let s = self.history.mv(&self.line, up);
        self.line.clear();
        self.line.push_str(s);
        self.cursor = self.line.chars().count() as u16;
        self.insert = self.line.len() as u16;
        self.line_len = self.insert;
        self.cursor_row = self.cursor / self.col;
        self.cursor_col = self.cursor % self.col;
        self.cursor_row_max = (self.line_len + self.carrot.len() as u16) / self.col;
        if self.cursor_row != 0 {
            stdout.queue(MoveToPreviousLine(self.cursor_row))?;
        }
        stdout.queue(MoveToColumn(self.carrot.len() as u16))?;
        stdout.queue(Clear(ClearType::FromCursorDown))?;
        write!(stdout, "{}", self.line)?;
        self.print_result(string, stdout, run)?;
        stdout.flush()?;
        Ok(())
    }
    pub fn new_line(&mut self, stdout: &mut impl Write, finish: impl FnOnce(&T)) -> io::Result<()> {
        if !self.line.is_empty() {
            self.history.push(&self.line)?;
        }
        if let Some(n) = self.last.take() {
            finish(&n);
        }
        if self.cursor_row_max != self.cursor_row {
            stdout.execute(MoveToNextLine(self.cursor_row_max - self.cursor_row))?;
        }
        for _ in 0..=self.new_lines {
            writeln!(stdout)?;
        }
        stdout.queue(MoveToColumn(0))?;
        self.cursor = 0;
        self.cursor_col = 0;
        self.cursor_row = 0;
        self.cursor_row_max = 0;
        self.insert = 0;
        self.line_len = 0;
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
        Ok(())
    }
    pub fn put_str(
        &mut self,
        stdout: &mut impl Write,
        string: &mut String,
        run: impl FnOnce(&str, &mut String) -> Option<T>,
        s: &str,
    ) -> io::Result<()> {
        self.line.insert_str(self.insert as usize, s);
        self.insert += s.len() as u16;
        let count = s.chars().count() as u16;
        self.line_len += count;
        if self.cursor_row != 0 {
            stdout.queue(MoveToPreviousLine(self.cursor_row))?;
        }
        stdout.queue(MoveToColumn(self.carrot.len() as u16))?;
        self.right(count, stdout)?;
        let rem = (self.line_len + self.carrot.len() as u16) % self.col;
        if rem <= s.len() as u16 {
            self.cursor_row_max += 1;
            stdout.queue(Clear(ClearType::FromCursorDown))?;
            write!(stdout, "{}", self.line)?;
            if rem == 0 {
                write!(stdout, " ")?;
            }
        } else {
            write!(stdout, "{}", self.line)?;
        }
        self.print_result(string, stdout, run)?;
        stdout.flush()
    }
    pub fn resize(&mut self, col: u16, row: u16) {
        self.cursor_row = self.cursor / col;
        self.cursor_col = self.cursor % col;
        self.cursor_row_max = (self.line_len + self.carrot.len() as u16) / col;
        (self.row, self.col) = (row, col);
    }
    pub fn go_left(&mut self, stdout: &mut impl Write) -> io::Result<()> {
        self.insert -= self.line
            [self.line.floor_char_boundary(self.insert as usize - 1)..self.insert as usize]
            .chars()
            .next()
            .unwrap()
            .len_utf8() as u16;
        if self.left(1, stdout)? {
            stdout.queue(MoveToPreviousLine(1))?;
        }
        stdout.queue(MoveToColumn(self.col()))?;
        stdout.flush()?;
        Ok(())
    }
    pub fn go_right(&mut self, stdout: &mut impl Write) -> io::Result<()> {
        self.insert += self.line
            [self.insert as usize..self.line.ceil_char_boundary(self.insert as usize + 1)]
            .chars()
            .next()
            .unwrap()
            .len_utf8() as u16;
        if self.right(1, stdout)? {
            stdout.queue(MoveToNextLine(1))?;
        }
        stdout.queue(MoveToColumn(self.col()))?;
        stdout.flush()?;
        Ok(())
    }
    pub fn go_up(&mut self, stdout: &mut impl Write) -> io::Result<()> {
        self.up(1, stdout)?;
        self.insert = self
            .line
            .char_indices()
            .nth(self.cursor as usize)
            .map(|(i, _)| i)
            .unwrap() as u16;
        stdout.queue(MoveToColumn(self.col()))?;
        stdout.flush()?;
        Ok(())
    }
    pub fn go_down(&mut self, stdout: &mut impl Write) -> io::Result<()> {
        self.down(1, stdout)?;
        self.insert = self
            .line
            .char_indices()
            .nth(self.cursor as usize)
            .map(|(i, _)| i as u16)
            .unwrap_or(self.line_len);
        stdout.queue(MoveToColumn(self.col()))?;
        stdout.flush()?;
        Ok(())
    }
    pub fn backspace(
        &mut self,
        stdout: &mut impl Write,
        string: &mut String,
        run: impl FnOnce(&str, &mut String) -> Option<T>,
    ) -> io::Result<()> {
        self.insert -= self
            .line
            .remove(self.line.floor_char_boundary(self.insert as usize - 1))
            .len_utf8() as u16;
        self.line_len -= 1;
        if self.cursor_row != 0 {
            stdout.queue(MoveToPreviousLine(self.cursor_row))?;
        }
        stdout.queue(MoveToColumn(self.carrot.len() as u16))?;
        self.left(1, stdout)?;
        if (self.line_len + self.carrot.len() as u16 + 1).is_multiple_of(self.col) {
            self.cursor_row_max -= 1;
            stdout.queue(Clear(ClearType::FromCursorDown))?;
            write!(stdout, "{}", self.line)?;
        } else {
            write!(stdout, "{} ", self.line)?;
        }
        self.print_result(string, stdout, run)?;
        stdout.flush()?;
        Ok(())
    }
    pub fn delete(
        &mut self,
        stdout: &mut impl Write,
        string: &mut String,
        run: impl FnOnce(&str, &mut String) -> Option<T>,
    ) -> io::Result<()> {
        self.line.remove(self.insert as usize).len_utf8() as u16;
        self.line_len -= 1;
        if self.cursor_row != 0 {
            stdout.queue(MoveToPreviousLine(self.cursor_row))?;
        }
        stdout.queue(MoveToColumn(self.carrot.len() as u16))?;
        if (self.line_len + self.carrot.len() as u16 + 1).is_multiple_of(self.col) {
            self.cursor_row_max -= 1;
            stdout.queue(Clear(ClearType::FromCursorDown))?;
            write!(stdout, "{}", self.line)?;
        } else {
            write!(stdout, "{} ", self.line)?;
        }
        self.print_result(string, stdout, run)?;
        stdout.flush()?;
        Ok(())
    }
    pub fn put_char(
        &mut self,
        stdout: &mut impl Write,
        string: &mut String,
        run: impl FnOnce(&str, &mut String) -> Option<T>,
        c: char,
    ) -> io::Result<()> {
        self.line.insert(self.insert as usize, c);
        self.insert += c.len_utf8() as u16;
        self.line_len += 1;
        if self.cursor_row != 0 {
            stdout.queue(MoveToPreviousLine(self.cursor_row))?;
        }
        stdout.queue(MoveToColumn(self.carrot.len() as u16))?;
        self.right(1, stdout)?;
        if (self.line_len + self.carrot.len() as u16).is_multiple_of(self.col) {
            self.cursor_row_max += 1;
            stdout.queue(Clear(ClearType::FromCursorDown))?;
            write!(stdout, "{} ", self.line)?;
        } else {
            write!(stdout, "{}", self.line)?;
        }
        self.print_result(string, stdout, run)?;
        stdout.flush()?;
        Ok(())
    }
    pub fn event(
        &mut self,
        stdout: &mut impl Write,
        string: &mut String,
        run: impl FnOnce(&str, &mut String) -> Option<T>,
        finish: impl FnOnce(&T),
        event: Event,
    ) -> io::Result<()> {
        match event {
            Event::Paste(s) => self.put_str(stdout, string, run, &s)?,
            Event::Resize(col, row) => self.resize(col, row),
            Event::Key(KeyEvent {
                code: KeyCode::Enter,
                ..
            }) => self.new_line(stdout, finish)?,
            Event::Key(KeyEvent {
                code: KeyCode::Left,
                ..
            }) if self.cursor != 0 => self.go_left(stdout)?,
            Event::Key(KeyEvent {
                code: KeyCode::Right,
                ..
            }) if self.cursor != self.line_len => self.go_right(stdout)?,
            Event::Key(KeyEvent {
                code: KeyCode::Up,
                modifiers: KeyModifiers::NONE,
                ..
            }) if !self.history.at_start() => self.move_history(stdout, string, run, true)?,
            Event::Key(KeyEvent {
                code: KeyCode::Down,
                modifiers: KeyModifiers::NONE,
                ..
            }) if !self.history.at_end() => self.move_history(stdout, string, run, false)?,
            Event::Key(KeyEvent {
                code: KeyCode::Up,
                modifiers: KeyModifiers::CONTROL,
                ..
            }) if self.cursor != 0 => self.go_up(stdout)?,
            Event::Key(KeyEvent {
                code: KeyCode::Down,
                modifiers: KeyModifiers::CONTROL,
                ..
            }) if self.cursor != self.line_len => self.go_down(stdout)?,
            Event::Key(KeyEvent {
                code: KeyCode::Backspace,
                ..
            }) if self.cursor != 0 => self.backspace(stdout, string, run)?,
            Event::Key(KeyEvent {
                code: KeyCode::Delete,
                ..
            }) if self.cursor != self.line_len => self.delete(stdout, string, run)?,
            Event::Key(KeyEvent {
                code: KeyCode::Char(c),
                modifiers: KeyModifiers::NONE,
                ..
            })
            | Event::Key(KeyEvent {
                code: KeyCode::Char(c),
                modifiers: KeyModifiers::SHIFT,
                ..
            }) => self.put_char(stdout, string, run, c)?,
            _ => {}
        }
        Ok(())
    }
    pub fn read(
        &mut self,
        stdout: &mut impl Write,
        string: &mut String,
        run: impl FnOnce(&str, &mut String) -> Option<T>,
        finish: impl FnOnce(&T),
    ) -> io::Result<()> {
        self.event(stdout, string, run, finish, event::read()?)
    }
}
