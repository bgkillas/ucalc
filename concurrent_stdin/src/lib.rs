use crossterm::cursor::{MoveLeft, MoveTo, MoveToColumn, MoveToPreviousLine};
use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::terminal::{Clear, ClearType};
use crossterm::{ExecutableCommand, event, terminal};
use std::io::{StdoutLock, Write};
use std::process::exit;
pub struct Out<T: Clone> {
    line: String,
    cursor: u16,
    new_lines: u16,
    last_failed: bool,
    last_succeed: Option<T>,
    last: Option<T>,
}
impl<T: Clone> Default for Out<T> {
    fn default() -> Self {
        terminal::enable_raw_mode().unwrap();
        #[cfg(debug_assertions)]
        let hook = std::panic::take_hook();
        #[cfg(debug_assertions)]
        std::panic::set_hook(Box::new(move |info| {
            _ = terminal::disable_raw_mode();
            println!();
            hook(info);
        }));
        Self {
            line: String::with_capacity(64),
            cursor: 0,
            new_lines: 1,
            last: None,
            last_failed: false,
            last_succeed: None,
        }
    }
}
impl<T: Clone> Drop for Out<T> {
    fn drop(&mut self) {
        terminal::disable_raw_mode().unwrap();
    }
}
impl<T: Clone> Out<T> {
    fn print_result(
        &mut self,
        stdout: &mut StdoutLock,
        run: impl FnOnce(&str) -> (Option<Option<T>>, u16),
    ) {
        stdout.execute(MoveToColumn(0)).unwrap();
        stdout.execute(Clear(ClearType::FromCursorDown)).unwrap();
        let n;
        (n, self.new_lines) = run(&self.line);
        self.last_failed = n.is_none();
        if let Some(o) = n {
            self.last = o;
        }
        if self.new_lines > 1 {
            stdout
                .execute(MoveToPreviousLine(self.new_lines - 1))
                .unwrap();
        }
        stdout.execute(MoveToColumn(self.cursor)).unwrap();
    }
    pub fn init(
        &mut self,
        stdout: &mut StdoutLock,
        run: impl FnOnce(&str) -> (Option<Option<T>>, u16),
    ) {
        self.print_result(stdout, run);
    }
    pub fn read(
        &mut self,
        stdout: &mut StdoutLock,
        run: impl FnOnce(&str) -> (Option<Option<T>>, u16),
        finish: impl FnOnce(T),
    ) {
        let Ok(k) = event::read() else {
            return;
        };
        match k {
            Event::Key(KeyEvent {
                code: KeyCode::Enter,
                ..
            }) => {
                self.cursor = 0;
                if self.last_failed {
                    self.last = self.last_succeed.take();
                } else if let Some(n) = self.last.take() {
                    self.last_succeed = Some(n.clone());
                    finish(n);
                }
                for _ in 0..self.new_lines {
                    println!()
                }
                stdout.execute(MoveToColumn(0)).unwrap();
                match self.line.as_str() {
                    "exit" => {
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
                stdout.flush().unwrap();
                self.line.clear();
                self.print_result(stdout, run);
            }
            Event::Key(KeyEvent {
                code: KeyCode::Backspace,
                ..
            }) if self.cursor != 0 => {
                self.line.pop();
                self.cursor -= 1;
                stdout.execute(MoveLeft(1)).unwrap();
                print!(" ");
                stdout.execute(MoveLeft(1)).unwrap();
                println!();
                self.print_result(stdout, run);
                stdout.flush().unwrap();
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char(c),
                ..
            }) => {
                self.line.push(c);
                self.cursor += 1;
                println!("{c}");
                self.print_result(stdout, run);
                stdout.flush().unwrap();
            }
            _ => {}
        }
    }
}
