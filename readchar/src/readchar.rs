use crate::History;
use crossterm::cursor::{MoveToColumn, MoveToNextLine, MoveToPreviousLine};
use crossterm::event::{
    DisableBracketedPaste, EnableBracketedPaste, Event, KeyCode, KeyEvent, KeyModifiers,
};
use crossterm::style::{Color, ResetColor, SetForegroundColor};
use crossterm::terminal::{Clear, ClearType};
use crossterm::{ExecutableCommand, QueueableCommand, event, terminal};
use std::io;
use std::io::{Write, stdout};
/// the ReadChar struct, see `read` for usage
///
/// on drop disables bracketed paste and terminal raw mode
#[derive(Debug)]
pub struct ReadChar {
    pub(crate) line: String,
    pub(crate) line_len: u16,
    pub(crate) row: u16,
    pub(crate) col: u16,
    pub(crate) cursor: u16,
    pub(crate) cursor_row: u16,
    pub(crate) cursor_row_max: u16,
    pub(crate) cursor_col: u16,
    pub(crate) insert: u16,
    pub(crate) new_lines: u16,
    pub(crate) history: History,
    pub carrot: &'static str,
    pub carrot_color: Option<Color>,
}
pub enum Return {
    Finish,
    Cancel,
    None,
}
impl Default for ReadChar {
    fn default() -> Self {
        Self::new(History::new(None).unwrap())
    }
}
impl Drop for ReadChar {
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
        } else if !csi {
            i += 1;
        }
    }
    i
}
impl ReadChar {
    /// creates a new `ReadChar` struct with `History` and
    /// enables terminal raw mode and panic hook
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
            carrot: "> ",
            carrot_color: Some(Color::DarkBlue),
        }
    }
    pub(crate) fn out_lines(&self, string: &str) -> u16 {
        string
            .lines()
            .map(|l| str_len(l).div_ceil(self.col))
            .sum::<u16>()
    }
    pub(crate) fn print_result(
        &mut self,
        string: &mut String,
        stdout: &mut impl Write,
        run: impl FnOnce(&str, &mut String),
    ) -> io::Result<()> {
        run(&self.line, string);
        self.new_lines = self.out_lines(string);
        stdout.queue(Clear(ClearType::FromCursorDown))?;
        for l in string.lines() {
            stdout.queue(MoveToColumn(0))?;
            write!(stdout, "\n{l}")?;
        }
        if self.new_lines + self.cursor_row_max != self.cursor_row {
            stdout.queue(MoveToPreviousLine(
                self.new_lines + self.cursor_row_max - self.cursor_row,
            ))?;
        }
        stdout.queue(MoveToColumn(self.col()))?;
        Ok(())
    }
    pub(crate) fn col(&self) -> u16 {
        self.cursor_col
            + if self.cursor_row == 0 {
                self.carrot.len() as u16
            } else {
                0
            }
    }
    pub(crate) fn up(&mut self, n: u16, stdout: &mut impl Write) -> io::Result<bool> {
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
    pub(crate) fn down(&mut self, n: u16, stdout: &mut impl Write) -> io::Result<bool> {
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
            self.cursor_col = if self.cursor_row == 0 {
                self.cursor
            } else {
                self.cursor + self.carrot.len() as u16
            } % self.col;
        }
        Ok(false)
    }
    pub(crate) fn left(&mut self, n: u16) -> io::Result<u16> {
        self.cursor -= n;
        if self.col() < n {
            let rows = n.div_ceil(self.col);
            self.cursor_col = rows * self.col + self.col()
                - n
                - if self.cursor_row == rows {
                    self.carrot.len() as u16
                } else {
                    0
                };
            self.cursor_row -= rows;
            Ok(rows)
        } else {
            self.cursor_col -= n;
            Ok(0)
        }
    }
    pub(crate) fn right(&mut self, n: u16) -> io::Result<u16> {
        self.cursor += n;
        if self.col() + n >= self.col {
            let rows = (self.col() + n) / self.col;
            self.cursor_col = self.col() + n - self.col * rows;
            self.cursor_row += rows;
            Ok(rows)
        } else {
            self.cursor_col += n;
            Ok(0)
        }
    }
    /// prints the prompt and enables brackted paste
    pub fn init(&mut self, stdout: &mut impl Write) -> io::Result<()> {
        stdout.queue(EnableBracketedPaste)?;
        self.carrot(stdout)?;
        stdout.flush()?;
        Ok(())
    }
    pub(crate) fn carrot(&mut self, stdout: &mut impl Write) -> io::Result<()> {
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
    pub(crate) fn move_history(
        &mut self,
        stdout: &mut impl Write,
        string: &mut String,
        run: impl FnOnce(&str, &mut String),
        up: bool,
    ) -> io::Result<()> {
        if self.cursor_row != 0 {
            stdout.queue(MoveToPreviousLine(self.cursor_row))?;
        }
        stdout.queue(MoveToColumn(self.carrot.len() as u16))?;
        let s = self.history.mv(&self.line, up);
        self.line.clear();
        self.line.push_str(s);
        self.cursor = self.line.chars().count() as u16;
        self.insert = self.line.len() as u16;
        self.line_len = self.insert;
        self.cursor_row = (self.cursor + self.carrot.len() as u16) / self.col;
        self.cursor_col = if self.cursor_row == 0 {
            self.cursor
        } else {
            self.cursor + self.carrot.len() as u16
        } % self.col;
        self.cursor_row_max = (self.line_len + self.carrot.len() as u16) / self.col;
        stdout.queue(Clear(ClearType::FromCursorDown))?;
        write!(stdout, "{}", self.line)?;
        self.print_result(string, stdout, run)?;
        stdout.flush()?;
        Ok(())
    }
    pub(crate) fn exit(&mut self, stdout: &mut impl Write) -> io::Result<()> {
        if self.cursor_row_max != self.cursor_row {
            stdout.execute(MoveToNextLine(self.cursor_row_max - self.cursor_row))?;
        }
        for _ in 0..=self.new_lines {
            writeln!(stdout)?;
        }
        stdout.queue(MoveToColumn(0))?;
        stdout.flush()?;
        Ok(())
    }
    pub fn close(&self, stdout: &mut impl Write) -> io::Result<()> {
        stdout.queue(DisableBracketedPaste)?;
        terminal::disable_raw_mode()?;
        stdout.flush()?;
        Ok(())
    }
    pub(crate) fn new_line<T>(
        &mut self,
        stdout: &mut T,
        finish: impl FnOnce(&ReadChar, &mut T, &str),
    ) -> io::Result<()>
    where
        T: Write,
    {
        if !self.line.is_empty() {
            self.history.push(&self.line)?;
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
        finish(self, stdout, &self.line);
        self.carrot(stdout)?;
        stdout.flush()?;
        self.new_lines = 0;
        self.line.clear();
        Ok(())
    }
    pub(crate) fn put_str(
        &mut self,
        stdout: &mut impl Write,
        string: &mut String,
        run: impl FnOnce(&str, &mut String),
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
        let rows = self.right(count)?;
        if rows != 0 {
            self.cursor_row_max += rows;
            stdout.queue(Clear(ClearType::FromCursorDown))?;
            write!(stdout, "{}", self.line)?;
            let rem = (self.line_len + self.carrot.len() as u16) % self.col;
            if rem == 0 {
                writeln!(stdout)?;
            }
        } else {
            write!(stdout, "{}", self.line)?;
        }
        self.print_result(string, stdout, run)?;
        stdout.flush()?;
        Ok(())
    }
    pub(crate) fn resize(&mut self, col: u16, row: u16, string: &str) {
        self.cursor_row = (self.cursor + self.carrot.len() as u16) / col;
        self.cursor_col = if self.cursor_row == 0 {
            self.cursor
        } else {
            self.cursor + self.carrot.len() as u16
        } % col;
        self.cursor_row_max = (self.line_len + self.carrot.len() as u16) / col;
        (self.row, self.col) = (row, col);
        self.new_lines = self.out_lines(string);
    }
    pub(crate) fn go_left_word(&mut self, stdout: &mut impl Write) -> io::Result<()> {
        let mut n = 0;
        while self.insert != 0
            && let Some(c) = self.line
                [self.line.floor_char_boundary(self.insert as usize - 1)..self.insert as usize]
                .chars()
                .next()
            && (c.is_alphanumeric() || n == 0)
        {
            self.insert -= c.len_utf8() as u16;
            n += 1;
        }
        let rows = self.left(n)?;
        if rows != 0 {
            stdout.queue(MoveToPreviousLine(rows))?;
        }
        stdout.queue(MoveToColumn(self.col()))?;
        stdout.flush()?;
        Ok(())
    }
    pub(crate) fn go_right_word(&mut self, stdout: &mut impl Write) -> io::Result<()> {
        let mut n = 0;
        while self.insert != self.line_len
            && let Some(c) = self.line
                [self.insert as usize..self.line.ceil_char_boundary(self.insert as usize + 1)]
                .chars()
                .next()
            && (c.is_alphanumeric() || n == 0)
        {
            self.insert += c.len_utf8() as u16;
            n += 1;
        }
        let rows = self.right(n)?;
        if rows != 0 {
            stdout.queue(MoveToNextLine(rows))?;
        }
        stdout.queue(MoveToColumn(self.col()))?;
        stdout.flush()?;
        Ok(())
    }
    pub(crate) fn home(&mut self, stdout: &mut impl Write) -> io::Result<()> {
        self.insert = 0;
        self.cursor = 0;
        if self.cursor_row != 0 {
            stdout.queue(MoveToPreviousLine(self.cursor_row))?;
        }
        self.cursor_row = 0;
        self.cursor_col = 0;
        stdout.queue(MoveToColumn(self.col()))?;
        stdout.flush()?;
        Ok(())
    }
    pub(crate) fn end(&mut self, stdout: &mut impl Write) -> io::Result<()> {
        self.insert = self.line.len() as u16;
        self.cursor = self.line_len;
        if self.cursor_row != self.cursor_row_max {
            stdout.queue(MoveToNextLine(self.cursor_row_max - self.cursor_row))?;
        }
        self.cursor_row = self.cursor_row_max;
        self.cursor_col = if self.cursor_row == 0 {
            self.cursor
        } else {
            self.cursor + self.carrot.len() as u16
        } % self.col;
        stdout.queue(MoveToColumn(self.col()))?;
        stdout.flush()?;
        Ok(())
    }
    pub(crate) fn go_left(&mut self, stdout: &mut impl Write) -> io::Result<()> {
        self.insert -= self.line
            [self.line.floor_char_boundary(self.insert as usize - 1)..self.insert as usize]
            .chars()
            .next()
            .unwrap()
            .len_utf8() as u16;
        if self.left(1)? != 0 {
            stdout.queue(MoveToPreviousLine(1))?;
        }
        stdout.queue(MoveToColumn(self.col()))?;
        stdout.flush()?;
        Ok(())
    }
    pub(crate) fn go_right(&mut self, stdout: &mut impl Write) -> io::Result<()> {
        self.insert += self.line
            [self.insert as usize..self.line.ceil_char_boundary(self.insert as usize + 1)]
            .chars()
            .next()
            .unwrap()
            .len_utf8() as u16;
        if self.right(1)? != 0 {
            stdout.queue(MoveToNextLine(1))?;
        }
        stdout.queue(MoveToColumn(self.col()))?;
        stdout.flush()?;
        Ok(())
    }
    pub(crate) fn go_up(&mut self, stdout: &mut impl Write) -> io::Result<()> {
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
    pub(crate) fn go_down(&mut self, stdout: &mut impl Write) -> io::Result<()> {
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
    pub(crate) fn backspace(
        &mut self,
        stdout: &mut impl Write,
        string: &mut String,
        run: impl FnOnce(&str, &mut String),
        n: u16,
    ) -> io::Result<()> {
        for _ in 0..n {
            self.insert -= self
                .line
                .remove(self.line.floor_char_boundary(self.insert as usize - 1))
                .len_utf8() as u16;
        }
        self.line_len -= n;
        if self.cursor_row != 0 {
            stdout.queue(MoveToPreviousLine(self.cursor_row))?;
        }
        stdout.queue(MoveToColumn(self.carrot.len() as u16))?;
        self.left(n)?;
        self.cursor_row_max = (self.line_len + self.carrot.len() as u16) / self.col;
        stdout.queue(Clear(ClearType::FromCursorDown))?;
        write!(stdout, "{}", self.line)?;
        if (self.line_len + self.carrot.len() as u16).is_multiple_of(self.col) {
            writeln!(stdout)?;
        }
        self.print_result(string, stdout, run)?;
        stdout.flush()?;
        Ok(())
    }
    pub(crate) fn delete(
        &mut self,
        stdout: &mut impl Write,
        string: &mut String,
        run: impl FnOnce(&str, &mut String),
        n: u16,
    ) -> io::Result<()> {
        for _ in 0..n {
            self.line.remove(self.insert as usize);
        }
        self.line_len -= n;
        if self.cursor_row != 0 {
            stdout.queue(MoveToPreviousLine(self.cursor_row))?;
        }
        stdout.queue(MoveToColumn(self.carrot.len() as u16))?;
        self.cursor_row_max = (self.line_len + self.carrot.len() as u16) / self.col;
        stdout.queue(Clear(ClearType::FromCursorDown))?;
        write!(stdout, "{}", self.line)?;
        if (self.line_len + self.carrot.len() as u16).is_multiple_of(self.col) {
            writeln!(stdout)?;
        }
        self.print_result(string, stdout, run)?;
        stdout.flush()?;
        Ok(())
    }
    pub(crate) fn put_char(
        &mut self,
        stdout: &mut impl Write,
        string: &mut String,
        run: impl FnOnce(&str, &mut String),
        c: char,
    ) -> io::Result<()> {
        self.line.insert(self.insert as usize, c);
        self.insert += c.len_utf8() as u16;
        self.line_len += 1;
        if self.cursor_row != 0 {
            stdout.queue(MoveToPreviousLine(self.cursor_row))?;
        }
        stdout.queue(MoveToColumn(self.carrot.len() as u16))?;
        self.right(1)?;
        if (self.line_len + self.carrot.len() as u16).is_multiple_of(self.col) {
            self.cursor_row_max += 1;
            stdout.queue(Clear(ClearType::FromCursorDown))?;
            writeln!(stdout, "{}", self.line)?;
        } else {
            write!(stdout, "{}", self.line)?;
        }
        self.print_result(string, stdout, run)?;
        stdout.flush()?;
        Ok(())
    }
    pub(crate) fn event<T>(
        &mut self,
        stdout: &mut T,
        string: &mut String,
        run: impl FnOnce(&str, &mut String),
        finish: impl FnOnce(&ReadChar, &mut T, &str),
        event: Event,
    ) -> io::Result<Return>
    where
        T: Write,
    {
        match event {
            Event::Paste(s) => self.put_str(stdout, string, run, &s)?,
            Event::Resize(col, row) => self.resize(col, row, string),
            Event::Key(KeyEvent {
                code: KeyCode::Enter,
                modifiers: KeyModifiers::NONE,
                ..
            }) => {
                self.new_line(stdout, finish)?;
                return Ok(Return::Finish);
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
                ..
            }) => {
                self.exit(stdout)?;
                return Ok(Return::Cancel);
            }
            Event::Key(KeyEvent {
                code: KeyCode::Home,
                modifiers: KeyModifiers::NONE,
                ..
            }) if self.cursor != 0 => self.home(stdout)?,
            Event::Key(KeyEvent {
                code: KeyCode::End,
                modifiers: KeyModifiers::NONE,
                ..
            }) if self.cursor != self.line_len => self.end(stdout)?,
            Event::Key(KeyEvent {
                code: KeyCode::Left,
                modifiers: KeyModifiers::NONE,
                ..
            }) if self.cursor != 0 => self.go_left(stdout)?,
            Event::Key(KeyEvent {
                code: KeyCode::Right,
                modifiers: KeyModifiers::NONE,
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
                code: KeyCode::Left,
                modifiers: KeyModifiers::CONTROL,
                ..
            }) if self.cursor != 0 => self.go_left_word(stdout)?,
            Event::Key(KeyEvent {
                code: KeyCode::Right,
                modifiers: KeyModifiers::CONTROL,
                ..
            }) if self.cursor != self.line_len => self.go_right_word(stdout)?,
            Event::Key(KeyEvent {
                code: KeyCode::Backspace,
                modifiers: KeyModifiers::NONE,
                ..
            }) if self.cursor != 0 => self.backspace(stdout, string, run, 1)?,
            Event::Key(KeyEvent {
                code: KeyCode::Delete,
                modifiers: KeyModifiers::NONE,
                ..
            }) if self.cursor != self.line_len => self.delete(stdout, string, run, 1)?,
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
        Ok(Return::None)
    }
    /// reads the next user input and reacts accordingly
    /// # Arguments
    /// - `stdout` the output which you are writing to
    /// - `string` your string buffer used in run
    /// - `run` runs when the input line has changed contents, then prints
    ///   contents of the string buffer passed into it, expected to have no trailing newlines
    /// # Returns
    /// returns if the line has been completed or not by the enter key
    pub fn read<T>(
        &mut self,
        stdout: &mut T,
        string: &mut String,
        run: impl FnOnce(&str, &mut String),
        finish: impl FnOnce(&ReadChar, &mut T, &str),
    ) -> io::Result<Return>
    where
        T: Write,
    {
        self.event(stdout, string, run, finish, event::read()?)
    }
}
