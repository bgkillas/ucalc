use uterm_lib::Term;
use uterm_lib::winit::event::KeyEvent;
use uterm_lib::winit::keyboard::{Key, NamedKey};
fn main() {
    Term::run(event);
}
fn event(event: KeyEvent, buffer: &mut String) {
    if event.state.is_pressed() {
        match event.logical_key {
            Key::Character(c) => {
                *buffer += c.as_str();
            }
            Key::Named(NamedKey::Enter) => buffer.push('\n'),
            Key::Named(NamedKey::Backspace) => buffer.push('\u{8}'),
            _ => {}
        }
    }
}
