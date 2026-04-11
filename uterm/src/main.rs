use readchar::{History, ReadChar};
use ucalc_lib::{Functions, Variables};
use uterm_lib::winit::event::KeyEvent;
use uterm_lib::{Dimensions, LineBuffer, Term};
fn main() {
    Term::run(Program::default());
}
pub struct Program {
    readchar: ReadChar,
    vars: Variables,
    funs: Functions,
    buffer: String,
}
impl Default for Program {
    fn default() -> Self {
        Self {
            readchar: ReadChar::new(History::new(None).unwrap(), 16, 16),
            vars: Variables::default(),
            funs: Functions::default(),
            buffer: String::with_capacity(512),
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
        self.readchar.event(buffer, self.buffer).unwrap();
    }
}
