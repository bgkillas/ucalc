use crate::colors::{Colors, ToColor};
use crate::complete::Complete;
use crate::shared::{Options, process_line, to_alt};
use readchar::enumset::EnumSet;
use readchar::{Clear, ClearType, Event, History, KeyCode, KeyModifiers, MoveTo, ReadChar, Return};
use std::io::Write;
use ucalc_lib::{Functions, Number, Variables};
#[cfg(feature = "float_rand")]
use ucalc_lib::{Rand, rng};
use uterm_lib::winit::event::{KeyEvent, Modifiers};
use uterm_lib::winit::keyboard::{Key, NamedKey};
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
    fn key_event(&mut self, event: KeyEvent, modifiers: Modifiers, buffer: &mut LineBuffer) {
        if let Some(event) = into_event(event, modifiers) {
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
                    event,
                )
                .unwrap();
        }
    }
}
fn into_event(key_event: KeyEvent, modifiers: Modifiers) -> Option<Event> {
    if !key_event.state.is_pressed() {
        return None;
    }
    Some(Event::Key(readchar::KeyEvent {
        code: into_key_code(key_event)?,
        modifiers: into_modifiers(modifiers),
    }))
}
fn into_key_code(key_event: KeyEvent) -> Option<KeyCode> {
    Some(match key_event.logical_key {
        Key::Named(c) => match c {
            NamedKey::Enter => KeyCode::Enter,
            NamedKey::Tab => KeyCode::Tab,
            NamedKey::Home => KeyCode::Home,
            NamedKey::End => KeyCode::End,
            NamedKey::ArrowLeft => KeyCode::Left,
            NamedKey::ArrowRight => KeyCode::Right,
            NamedKey::ArrowUp => KeyCode::Up,
            NamedKey::ArrowDown => KeyCode::Down,
            NamedKey::Backspace => KeyCode::Backspace,
            NamedKey::Delete => KeyCode::Delete,
            _ => return None,
        },
        Key::Character(c)
            if let Some(c) = c.chars().next()
                && c.is_ascii() =>
        {
            KeyCode::Char(c)
        }
        _ => return None,
    })
}
fn into_modifiers(modifiers: Modifiers) -> EnumSet<KeyModifiers> {
    let mut set = EnumSet::new();
    let state = modifiers.state();
    if state.alt_key() {
        set.insert(KeyModifiers::Alt);
    }
    if state.shift_key() {
        set.insert(KeyModifiers::Shift);
    }
    if state.control_key() {
        set.insert(KeyModifiers::Control);
    }
    set
}
