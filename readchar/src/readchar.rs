use crate::History;
#[cfg(feature = "crossterm")]
use crossterm::event::{DisableBracketedPaste, EnableBracketedPaste};
#[cfg(feature = "crossterm")]
use crossterm::{ExecutableCommand, QueueableCommand, event, terminal};
use enumset::{EnumSet, EnumSetType, enum_set};
use std::fmt::{Display, Formatter};
use std::io;
use std::io::Write;
#[cfg(feature = "crossterm")]
use std::io::stdout;
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
#[derive(Debug, PartialEq)]
pub enum Return {
    Finish,
    Cancel,
    None,
}
pub trait ToColor<'b> {
    fn run<'a>(self, str: &'a str) -> impl Display + 'a
    where
        'b: 'a;
}
pub struct NoColor;
impl ToColor<'static> for NoColor {
    fn run<'a>(self, str: &'a str) -> impl Display + 'a
    where
        'static: 'a,
    {
        str
    }
}
pub trait Complete<'b> {
    fn run<'a>(self, str: &'a str) -> Vec<(impl Display + 'a, usize)>
    where
        'b: 'a;
}
pub struct NoComplete;
impl Complete<'static> for NoComplete {
    #[allow(refining_impl_trait)]
    fn run<'a>(self, _: &'a str) -> Vec<(String, usize)>
    where
        'static: 'a,
    {
        Vec::new()
    }
}
#[cfg(feature = "crossterm")]
impl Default for ReadChar {
    fn default() -> Self {
        Self::new(History::new(None).unwrap())
    }
}
#[cfg(feature = "crossterm")]
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
    #[cfg(feature = "crossterm")]
    pub fn new(history: History) -> Self {
        terminal::enable_raw_mode().unwrap();
        #[cfg(debug_assertions)]
        let hook = std::panic::take_hook();
        #[cfg(debug_assertions)]
        std::panic::set_hook(Box::new(move |info| {
            _ = stdout().execute(DisableBracketedPaste);
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
    #[cfg(not(feature = "crossterm"))]
    pub fn new(history: History, col: u16, row: u16) -> Self {
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
        write!(stdout, "{}", Clear(ClearType::FromCursorDown))?;
        for l in string.lines() {
            write!(stdout, "{}", MoveToColumn(0))?;
            write!(stdout, "\n{l}")?;
        }
        if self.new_lines + self.cursor_row_max != self.cursor_row {
            write!(
                stdout,
                "{}",
                MoveToPreviousLine(self.new_lines + self.cursor_row_max - self.cursor_row,)
            )?;
        }
        write!(stdout, "{}", MoveToColumn(self.col()))?;
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
            write!(stdout, "{}", MoveToPreviousLine(1))?;
        } else {
            if self.cursor_row != 0 {
                self.cursor_row = 0;
                write!(stdout, "{}", MoveToPreviousLine(1))?;
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
            write!(stdout, "{}", MoveToNextLine(1))?;
        } else {
            if self.cursor_row_max > self.cursor_row {
                self.cursor_row = self.cursor_row_max;
                write!(stdout, "{}", MoveToNextLine(1))?;
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
        #[cfg(feature = "crossterm")]
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
    pub(crate) fn print_line<'a>(
        &self,
        stdout: &mut impl Write,
        color: impl ToColor<'a>,
    ) -> io::Result<()> {
        write!(stdout, "{}", color.run(&self.line))
    }
    pub(crate) fn move_history<'a>(
        &mut self,
        stdout: &mut impl Write,
        string: &mut String,
        run: impl FnOnce(&str, &mut String),
        color: impl ToColor<'a>,
        up: bool,
    ) -> io::Result<()> {
        if self.cursor_row != 0 {
            write!(stdout, "{}", MoveToPreviousLine(self.cursor_row))?;
        }
        write!(stdout, "{}", MoveToColumn(self.carrot.len() as u16))?;
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
        write!(stdout, "{}", Clear(ClearType::FromCursorDown))?;
        self.print_line(stdout, color)?;
        stdout.flush()?;
        self.print_result(string, stdout, run)?;
        stdout.flush()?;
        Ok(())
    }
    pub(crate) fn exit(&mut self, stdout: &mut impl Write, string: &str) -> io::Result<()> {
        if self.cursor_row_max != self.cursor_row {
            write!(
                stdout,
                "{}",
                MoveToNextLine(self.cursor_row_max - self.cursor_row)
            )?;
        }
        for _ in 0..self.new_lines {
            writeln!(stdout)?;
        }
        if string.is_empty() {
            write!(stdout, "{}", MoveToColumn(0))?;
            write!(stdout, "{}", Clear(ClearType::CurrentLine))?;
        } else {
            writeln!(stdout)?;
        }
        write!(stdout, "{}", MoveToColumn(0))?;
        stdout.flush()?;
        Ok(())
    }
    #[cfg(feature = "crossterm")]
    pub fn close(&self, stdout: &mut impl Write) -> io::Result<()> {
        stdout.execute(DisableBracketedPaste)?;
        terminal::disable_raw_mode()?;
        Ok(())
    }
    pub fn clear(&self, stdout: &mut impl Write) -> io::Result<()> {
        write!(stdout, "{}", Clear(ClearType::Purge))?;
        write!(stdout, "{}", Clear(ClearType::All))?;
        write!(stdout, "{}", MoveTo(0, 0))?;
        Ok(())
    }
    pub(crate) fn new_line<T: Write>(
        &mut self,
        stdout: &mut T,
        finish: impl FnOnce(&ReadChar, &mut T, &str) -> io::Result<Return>,
    ) -> io::Result<Return> {
        if !self.line.is_empty() {
            self.history.push(&self.line)?;
        }
        if self.cursor_row_max != self.cursor_row {
            write!(
                stdout,
                "{}",
                MoveToNextLine(self.cursor_row_max - self.cursor_row)
            )?;
        }
        for _ in 0..=self.new_lines {
            writeln!(stdout)?;
        }
        write!(stdout, "{}", MoveToColumn(0))?;
        self.cursor = 0;
        self.cursor_col = 0;
        self.cursor_row = 0;
        self.cursor_row_max = 0;
        self.insert = 0;
        self.line_len = 0;
        let ret = finish(self, stdout, &self.line)?;
        if ret != Return::Cancel {
            self.carrot(stdout)?;
        }
        stdout.flush()?;
        self.new_lines = 0;
        self.line.clear();
        Ok(ret)
    }
    pub(crate) fn put_str<'a>(
        &mut self,
        stdout: &mut impl Write,
        string: &mut String,
        run: impl FnOnce(&str, &mut String),
        color: impl ToColor<'a>,
        s: &str,
    ) -> io::Result<()> {
        self.line.insert_str(self.insert as usize, s);
        self.insert += s.len() as u16;
        let count = s.chars().count() as u16;
        self.line_len += count;
        if self.cursor_row != 0 {
            write!(stdout, "{}", MoveToPreviousLine(self.cursor_row))?;
        }
        write!(stdout, "{}", MoveToColumn(self.carrot.len() as u16))?;
        let rows = self.right(count)?;
        if rows != 0 {
            self.cursor_row_max += rows;
            write!(stdout, "{}", Clear(ClearType::FromCursorDown))?;
            self.print_line(stdout, color)?;
            let rem = (self.line_len + self.carrot.len() as u16) % self.col;
            if rem == 0 {
                writeln!(stdout)?;
            }
        } else {
            self.print_line(stdout, color)?;
        }
        stdout.flush()?;
        self.print_result(string, stdout, run)?;
        stdout.flush()?;
        Ok(())
    }
    pub fn resize(&mut self, col: u16, row: u16, string: &str) {
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
            write!(stdout, "{}", MoveToPreviousLine(rows))?;
        }
        write!(stdout, "{}", MoveToColumn(self.col()))?;
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
            write!(stdout, "{}", MoveToNextLine(rows))?;
        }
        write!(stdout, "{}", MoveToColumn(self.col()))?;
        stdout.flush()?;
        Ok(())
    }
    pub(crate) fn home(&mut self, stdout: &mut impl Write) -> io::Result<()> {
        self.insert = 0;
        self.cursor = 0;
        if self.cursor_row != 0 {
            write!(stdout, "{}", MoveToPreviousLine(self.cursor_row))?;
        }
        self.cursor_row = 0;
        self.cursor_col = 0;
        write!(stdout, "{}", MoveToColumn(self.col()))?;
        stdout.flush()?;
        Ok(())
    }
    pub(crate) fn end(&mut self, stdout: &mut impl Write) -> io::Result<()> {
        self.insert = self.line.len() as u16;
        self.cursor = self.line_len;
        if self.cursor_row != self.cursor_row_max {
            write!(
                stdout,
                "{}",
                MoveToNextLine(self.cursor_row_max - self.cursor_row)
            )?;
        }
        self.cursor_row = self.cursor_row_max;
        self.cursor_col = if self.cursor_row == 0 {
            self.cursor
        } else {
            self.cursor + self.carrot.len() as u16
        } % self.col;
        write!(stdout, "{}", MoveToColumn(self.col()))?;
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
            write!(stdout, "{}", MoveToPreviousLine(1))?;
        }
        write!(stdout, "{}", MoveToColumn(self.col()))?;
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
            write!(stdout, "{}", MoveToNextLine(1))?;
        }
        write!(stdout, "{}", MoveToColumn(self.col()))?;
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
        write!(stdout, "{}", MoveToColumn(self.col()))?;
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
        write!(stdout, "{}", MoveToColumn(self.col()))?;
        stdout.flush()?;
        Ok(())
    }
    pub(crate) fn backspace<'a>(
        &mut self,
        stdout: &mut impl Write,
        string: &mut String,
        run: impl FnOnce(&str, &mut String),
        color: impl ToColor<'a>,
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
            write!(stdout, "{}", MoveToPreviousLine(self.cursor_row))?;
        }
        write!(stdout, "{}", MoveToColumn(self.carrot.len() as u16))?;
        self.left(n)?;
        self.cursor_row_max = (self.line_len + self.carrot.len() as u16) / self.col;
        write!(stdout, "{}", Clear(ClearType::FromCursorDown))?;
        self.print_line(stdout, color)?;
        if (self.line_len + self.carrot.len() as u16).is_multiple_of(self.col) {
            writeln!(stdout)?;
        }
        stdout.flush()?;
        self.print_result(string, stdout, run)?;
        stdout.flush()?;
        Ok(())
    }
    pub(crate) fn delete<'a>(
        &mut self,
        stdout: &mut impl Write,
        string: &mut String,
        run: impl FnOnce(&str, &mut String),
        color: impl ToColor<'a>,
        n: u16,
    ) -> io::Result<()> {
        for _ in 0..n {
            self.line.remove(self.insert as usize);
        }
        self.line_len -= n;
        if self.cursor_row != 0 {
            write!(stdout, "{}", MoveToPreviousLine(self.cursor_row))?;
        }
        write!(stdout, "{}", MoveToColumn(self.carrot.len() as u16))?;
        self.cursor_row_max = (self.line_len + self.carrot.len() as u16) / self.col;
        write!(stdout, "{}", Clear(ClearType::FromCursorDown))?;
        self.print_line(stdout, color)?;
        if (self.line_len + self.carrot.len() as u16).is_multiple_of(self.col) {
            writeln!(stdout)?;
        }
        stdout.flush()?;
        self.print_result(string, stdout, run)?;
        stdout.flush()?;
        Ok(())
    }
    pub(crate) fn put_char<'a>(
        &mut self,
        stdout: &mut impl Write,
        string: &mut String,
        run: impl FnOnce(&str, &mut String),
        color: impl ToColor<'a>,
        c: char,
    ) -> io::Result<()> {
        self.line.insert(self.insert as usize, c);
        self.insert += c.len_utf8() as u16;
        self.line_len += 1;
        if self.cursor_row != 0 {
            write!(stdout, "{}", MoveToPreviousLine(self.cursor_row))?;
        }
        write!(stdout, "{}", MoveToColumn(self.carrot.len() as u16))?;
        self.right(1)?;
        if (self.line_len + self.carrot.len() as u16).is_multiple_of(self.col) {
            self.cursor_row_max += 1;
            write!(stdout, "{}", Clear(ClearType::FromCursorDown))?;
            writeln!(stdout, "{}", self.line)?;
        } else {
            self.print_line(stdout, color)?;
        }
        stdout.flush()?;
        self.print_result(string, stdout, run)?;
        stdout.flush()?;
        Ok(())
    }
    pub(crate) fn complete<'a>(
        &mut self,
        stdout: &mut impl Write,
        complete: impl Complete<'a>,
    ) -> io::Result<()> {
        let list = complete.run(&self.line[..self.cursor as usize]);
        if !list.is_empty() {
            if self.cursor_row_max != self.cursor_row {
                write!(
                    stdout,
                    "{}",
                    MoveToNextLine(self.cursor_row_max - self.cursor_row)
                )?;
            }
            writeln!(stdout)?;
            write!(stdout, "{}", MoveToColumn(0))?;
            write!(stdout, "{}", Clear(ClearType::FromCursorDown))?;
            let longest = list.iter().map(|s| s.1).max().unwrap() + 4;
            self.new_lines = 1;
            let words = self.col / longest as u16;
            for (i, s) in list.iter().enumerate() {
                write!(stdout, "{}", s.0)?;
                for _ in s.1..longest {
                    write!(stdout, " ")?;
                }
                if i + 1 != list.len() && (i + 1) % words as usize == 0 {
                    self.new_lines += 1;
                    writeln!(stdout)?;
                    write!(stdout, "{}", MoveToColumn(0))?;
                }
            }
            write!(
                stdout,
                "{}",
                MoveToPreviousLine(self.cursor_row_max - self.cursor_row + self.new_lines,)
            )?;
            write!(stdout, "{}", MoveToColumn(self.col()))?;
            stdout.flush()?;
        }
        Ok(())
    }
    #[allow(clippy::too_many_arguments)]
    pub fn event<'a, T: Write>(
        &mut self,
        stdout: &mut T,
        string: &mut String,
        run: impl FnOnce(&str, &mut String),
        color: impl ToColor<'a>,
        finish: impl FnOnce(&ReadChar, &mut T, &str) -> io::Result<Return>,
        to_alt: impl FnOnce(char) -> Option<char>,
        complete: impl Complete<'a>,
        event: Event,
    ) -> io::Result<Return> {
        match event {
            Event::Paste(s) => self.put_str(stdout, string, run, color, &s)?,
            Event::Resize(col, row) => self.resize(col, row, string),
            Event::Key(KeyEvent {
                code: KeyCode::Enter,
                modifiers,
                ..
            }) if modifiers.is_empty() => return self.new_line(stdout, finish),
            Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers,
                ..
            }) if modifiers == enum_set! {KeyModifiers::Control} => {
                self.exit(stdout, string)?;
                return Ok(Return::Cancel);
            }
            Event::Key(KeyEvent {
                code: KeyCode::Tab,
                modifiers,
                ..
            }) if modifiers.is_empty() => self.complete(stdout, complete)?,
            Event::Key(KeyEvent {
                code: KeyCode::Home,
                modifiers,
                ..
            }) if modifiers.is_empty() && self.cursor != 0 => self.home(stdout)?,
            Event::Key(KeyEvent {
                code: KeyCode::End,
                modifiers,
                ..
            }) if modifiers.is_empty() && self.cursor != self.line_len => self.end(stdout)?,
            Event::Key(KeyEvent {
                code: KeyCode::Left,
                modifiers,
                ..
            }) if modifiers.is_empty() && self.cursor != 0 => self.go_left(stdout)?,
            Event::Key(KeyEvent {
                code: KeyCode::Right,
                modifiers,
                ..
            }) if modifiers.is_empty() && self.cursor != self.line_len => self.go_right(stdout)?,
            Event::Key(KeyEvent {
                code: KeyCode::Up,
                modifiers,
                ..
            }) if modifiers.is_empty() && !self.history.at_start() => {
                self.move_history(stdout, string, run, color, true)?
            }
            Event::Key(KeyEvent {
                code: KeyCode::Down,
                modifiers,
                ..
            }) if modifiers.is_empty() && !self.history.at_end() => {
                self.move_history(stdout, string, run, color, false)?
            }
            Event::Key(KeyEvent {
                code: KeyCode::Up,
                modifiers,
                ..
            }) if modifiers == enum_set![KeyModifiers::Control] && self.cursor != 0 => {
                self.go_up(stdout)?
            }
            Event::Key(KeyEvent {
                code: KeyCode::Down,
                modifiers,
                ..
            }) if modifiers == enum_set![KeyModifiers::Control] && self.cursor != self.line_len => {
                self.go_down(stdout)?
            }
            Event::Key(KeyEvent {
                code: KeyCode::Left,
                modifiers,
                ..
            }) if modifiers == enum_set![KeyModifiers::Control] && self.cursor != 0 => {
                self.go_left_word(stdout)?
            }
            Event::Key(KeyEvent {
                code: KeyCode::Right,
                modifiers,
                ..
            }) if modifiers == enum_set![KeyModifiers::Control] && self.cursor != self.line_len => {
                self.go_right_word(stdout)?
            }
            Event::Key(KeyEvent {
                code: KeyCode::Backspace,
                modifiers,
                ..
            }) if modifiers.is_empty() && self.cursor != 0 => {
                self.backspace(stdout, string, run, color, 1)?
            }
            Event::Key(KeyEvent {
                code: KeyCode::Delete,
                modifiers,
                ..
            }) if modifiers.is_empty() && self.cursor != self.line_len => {
                self.delete(stdout, string, run, color, 1)?
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char('r'),
                modifiers,
                ..
            }) if modifiers == enum_set![KeyModifiers::Control] => {
                write!(stdout, "{}", MoveRight(u16::MAX))?;
                if self.cursor_row_max != self.cursor_row {
                    write!(
                        stdout,
                        "{}",
                        MoveDown(self.cursor_row_max - self.cursor_row)
                    )?;
                }
                self.print_result(string, stdout, run)?;
                stdout.flush()?;
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char(c),
                modifiers,
                ..
            }) if modifiers
                .is_disjoint(EnumSet::all() - KeyModifiers::Shift - KeyModifiers::Alt) =>
            {
                if modifiers.contains(KeyModifiers::Alt) {
                    if let Some(c) = to_alt(c) {
                        self.put_char(stdout, string, run, color, c)?
                    }
                } else {
                    self.put_char(stdout, string, run, color, c)?
                }
            }
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
    #[allow(clippy::too_many_arguments)]
    #[cfg(feature = "crossterm")]
    pub fn read<'a, T: Write>(
        &mut self,
        stdout: &mut T,
        string: &mut String,
        run: impl FnOnce(&str, &mut String),
        color: impl ToColor<'a>,
        finish: impl FnOnce(&ReadChar, &mut T, &str) -> io::Result<Return>,
        to_alt: impl FnOnce(char) -> Option<char>,
        complete: impl Complete<'a>,
    ) -> io::Result<Return> {
        if let Ok(event) = event::read()?.try_into() {
            self.event(stdout, string, run, color, finish, to_alt, complete, event)
        } else {
            Ok(Return::None)
        }
    }
}
pub enum Event {
    Key(KeyEvent),
    Paste(String),
    Resize(u16, u16),
}
pub struct KeyEvent {
    code: KeyCode,
    modifiers: EnumSet<KeyModifiers>,
}
pub enum KeyCode {
    Char(char),
    Enter,
    Tab,
    Home,
    End,
    Left,
    Right,
    Up,
    Down,
    Backspace,
    Delete,
}
#[derive(EnumSetType)]
pub enum KeyModifiers {
    Control,
    Shift,
    Alt,
}
#[cfg(feature = "crossterm")]
impl TryFrom<event::Event> for Event {
    type Error = ();
    fn try_from(value: event::Event) -> Result<Self, Self::Error> {
        Ok(match value {
            event::Event::Key(key) => Event::Key(key.try_into()?),
            event::Event::Paste(s) => Event::Paste(s),
            event::Event::Resize(x, y) => Event::Resize(x, y),
            _ => return Err(()),
        })
    }
}
#[cfg(feature = "crossterm")]
impl TryFrom<event::KeyEvent> for KeyEvent {
    type Error = ();
    fn try_from(value: event::KeyEvent) -> Result<Self, Self::Error> {
        let mut modifiers = EnumSet::new();
        for modifier in value.modifiers.iter() {
            modifiers.insert(match modifier {
                event::KeyModifiers::ALT => KeyModifiers::Alt,
                event::KeyModifiers::SHIFT => KeyModifiers::Shift,
                event::KeyModifiers::CONTROL => KeyModifiers::Control,
                _ => return Err(()),
            });
        }
        Ok(Self {
            code: value.code.try_into()?,
            modifiers,
        })
    }
}
#[cfg(feature = "crossterm")]
impl TryFrom<event::KeyCode> for KeyCode {
    type Error = ();
    fn try_from(value: event::KeyCode) -> Result<Self, Self::Error> {
        Ok(match value {
            event::KeyCode::Char(c) => Self::Char(c),
            event::KeyCode::Enter => Self::Enter,
            event::KeyCode::Tab => Self::Tab,
            event::KeyCode::Home => Self::Home,
            event::KeyCode::End => Self::End,
            event::KeyCode::Left => Self::Left,
            event::KeyCode::Right => Self::Right,
            event::KeyCode::Up => Self::Up,
            event::KeyCode::Down => Self::Down,
            event::KeyCode::Backspace => Self::Backspace,
            event::KeyCode::Delete => Self::Delete,
            _ => return Err(()),
        })
    }
}
#[derive(Debug, Clone, Copy)]
pub enum Color {
    Black,
    DarkGrey,
    Red,
    DarkRed,
    Green,
    DarkGreen,
    Yellow,
    DarkYellow,
    Blue,
    DarkBlue,
    Magenta,
    DarkMagenta,
    Cyan,
    DarkCyan,
    White,
    Grey,
    Rgb { r: u8, g: u8, b: u8 },
}
pub struct ResetColor;
pub struct SetForegroundColor(Color);
pub struct MoveDown(u16);
pub struct MoveRight(u16);
pub struct MoveTo(u16, u16);
pub struct MoveToColumn(u16);
pub struct MoveToNextLine(u16);
pub struct MoveToPreviousLine(u16);
pub struct Clear(ClearType);
pub enum ClearType {
    All,
    Purge,
    FromCursorDown,
    CurrentLine,
}
impl Display for SetForegroundColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\x1b[{}m",
            match self.0 {
                Color::Black => "30",
                Color::DarkGrey => "90",
                Color::Red => "31",
                Color::DarkRed => "91",
                Color::Green => "32",
                Color::DarkGreen => "92",
                Color::Yellow => "33",
                Color::DarkYellow => "93",
                Color::Blue => "34",
                Color::DarkBlue => "94",
                Color::Magenta => "35",
                Color::DarkMagenta => "95",
                Color::Cyan => "36",
                Color::DarkCyan => "96",
                Color::White => "37",
                Color::Grey => "97",
                Color::Rgb { r, g, b } => return write!(f, "\x1b[38;2;{r};{g};{b}m"),
            }
        )
    }
}
impl Display for MoveDown {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "\x1b[{}B", self.0)
    }
}
impl Display for ResetColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "\x1b[39;49m")
    }
}
impl Display for MoveRight {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "\x1b[{}C", self.0)
    }
}
impl Display for MoveTo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "\x1b[{};{}H", self.0 + 1, self.1 + 1)
    }
}
impl Display for MoveToColumn {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "\x1b[{}G", self.0 + 1)
    }
}
impl Display for MoveToNextLine {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "\x1b[{}E", self.0)
    }
}
impl Display for MoveToPreviousLine {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "\x1b[{}F", self.0)
    }
}
impl Display for Clear {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            ClearType::All => write!(f, "\x1b[2J"),
            ClearType::Purge => write!(f, "\x1b[3J"),
            ClearType::FromCursorDown => write!(f, "\x1b[J"),
            ClearType::CurrentLine => write!(f, "\x1b[2K"),
        }
    }
}
