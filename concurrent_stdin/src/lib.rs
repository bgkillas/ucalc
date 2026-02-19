use crossterm::cursor::{MoveLeft, MoveToColumn, MoveToPreviousLine};
use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::terminal::{Clear, ClearType};
use crossterm::{ExecutableCommand, event, terminal};
use std::io::{StdoutLock, Write};
use std::process::exit;
pub struct Out<T: Clone> {
    line: String,
    cursor: u16,
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
    pub fn read(
        &mut self,
        stdout: &mut StdoutLock,
        run: impl FnOnce(&str) -> Option<Option<T>>,
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
                    println!("\n");
                } else if let Some(n) = self.last.take() {
                    self.last_succeed = Some(n.clone());
                    finish(n);
                    println!("\n");
                } else {
                    println!();
                }
                stdout.execute(MoveToColumn(0)).unwrap();
                stdout.flush().unwrap();
                if self.line == "exit" {
                    terminal::disable_raw_mode().unwrap();
                    exit(0);
                }
                self.line.clear();
            }
            Event::Key(KeyEvent {
                code: KeyCode::Backspace,
                ..
            }) if self.cursor != 0 => {
                self.line.pop();
                self.cursor -= 1;
                stdout.execute(MoveLeft(1)).unwrap();
                stdout.execute(Clear(ClearType::FromCursorDown)).unwrap();
                println!();
                stdout.execute(MoveToColumn(0)).unwrap();
                stdout.execute(Clear(ClearType::CurrentLine)).unwrap();
                let n = run(&self.line);
                self.last_failed = n.is_none();
                if let Some(o) = n {
                    self.last = o;
                }
                stdout.execute(MoveToPreviousLine(1)).unwrap();
                stdout.execute(MoveToColumn(self.cursor)).unwrap();
                stdout.flush().unwrap();
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char(c),
                ..
            }) => {
                self.line.push(c);
                self.cursor += 1;
                println!("{c}");
                stdout.execute(MoveToColumn(0)).unwrap();
                stdout.execute(Clear(ClearType::CurrentLine)).unwrap();
                let n = run(&self.line);
                self.last_failed = n.is_none();
                if let Some(o) = n {
                    self.last = o;
                }
                stdout.execute(MoveToPreviousLine(1)).unwrap();
                stdout.execute(MoveToColumn(self.cursor)).unwrap();
                stdout.flush().unwrap();
            }
            _ => {}
        }
    }
}
