use uterm_lib::Term;
use uterm_lib::winit::event::KeyEvent;
fn main() {
    Term::run(event);
}
fn event(event: KeyEvent, buffer: &mut String) {
    if let Some(s) = event.text
        && event.state.is_pressed()
    {
        *buffer += s.as_str();
    }
}
