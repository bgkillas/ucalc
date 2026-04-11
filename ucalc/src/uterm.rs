use crate::colors::{Colors, ToColor};
use crate::complete::Complete;
use crate::shared::{Options, process_line, to_alt};
use readchar::{Clear, ClearType, Event, History, MoveTo, ReadChar, Return};
use std::io::Write;
use ucalc_lib::{Functions, Number, Variables};
#[cfg(feature = "float_rand")]
use ucalc_lib::{Rand, rng};
use uterm_lib::winit::event::KeyEvent;
use uterm_lib::{Dimensions, LineBuffer, Term};
pub fn uterm() {
    Term::run(Program::default());
}
pub struct Program {
    readchar: ReadChar,
    vars: Variables,
    funs: Functions,
    buffer: String,
    last: Option<Number>,
    options: Options,
    colors: Colors,
    #[cfg(feature = "float_rand")]
    rand: Rand,
}
impl Default for Program {
    fn default() -> Self {
        Self {
            readchar: ReadChar::new(History::new(None).unwrap(), 16, 16),
            vars: Variables::default(),
            funs: Functions::default(),
            buffer: String::with_capacity(512),
            last: None,
            options: Options::default(),
            colors: Colors::default(),
            #[cfg(feature = "float_rand")]
            rand: rng(),
        }
    }
}
impl uterm_lib::Program for Program {
    fn resize(&mut self, cells: Dimensions) {
        self.readchar
            .resize(cells.x as u16, cells.y as u16, &self.buffer)
    }
    fn init(&mut self, buffer: &mut LineBuffer) {
        self.readchar.init(buffer).unwrap();
    }
    fn event(&mut self, event: KeyEvent, buffer: &mut LineBuffer) {
        self.readchar
            .event(
                buffer,
                &mut self.buffer,
                |line, string| {
                    self.last = process_line(
                        line,
                        &mut self.vars,
                        &mut self.funs,
                        self.options,
                        string,
                        &self.colors,
                        #[cfg(feature = "float_rand")]
                        &mut self.rand,
                    )
                    .unwrap()
                },
                ToColor(&self.colors),
                |_, stdout, line| {
                    Ok(match line {
                        "exit" => Return::Cancel,
                        "clear" => {
                            write!(stdout, "{}", Clear(ClearType::Purge))?;
                            write!(stdout, "{}", Clear(ClearType::All))?;
                            write!(stdout, "{}", MoveTo(0, 0))?;
                            Return::Finish
                        }
                        _ => Return::Finish,
                    })
                },
                to_alt,
                Complete(&self.colors),
                into_event(event),
            )
            .unwrap();
    }
}
fn into_event(event: KeyEvent) -> Event {
    todo!()
}
