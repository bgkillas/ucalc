use bdf2::Font;
use softbuffer::{Buffer, Context, Surface};
use std::mem;
use std::num::NonZeroU32;
use std::ops::{Deref, DerefMut};
use winit::application::ApplicationHandler;
use winit::event::{KeyEvent, StartCause, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop, OwnedDisplayHandle};
use winit::window::{Window, WindowId};
pub struct Term<K: Fn(KeyEvent, &mut String)> {
    context: Context<OwnedDisplayHandle>,
    state: WindowState,
    lines: Lines,
    cursor: Cursor,
    buffer: String,
    screen: Dimensions,
    keyboard_input: K,
    font: Font,
}
#[derive(Default, Clone, Copy)]
pub struct Dimensions {
    x: u32,
    y: u32,
}
#[derive(Default, Clone, Copy)]
pub struct Cursor {
    x: u32,
    y: u32,
}
pub struct Lines(Vec<Line>);
impl Lines {
    pub fn write(&mut self, c: char, cursor: Cursor) {
        for _ in self.len()..=cursor.y as usize {
            self.push(Line::default())
        }
        self[cursor.y as usize].write(c, cursor.x)
    }
    pub fn get_lines(&self) -> impl Iterator<Item = &Line> {
        self.0.iter()
    }
}
impl Default for Lines {
    fn default() -> Self {
        Self(Vec::with_capacity(1 << 12))
    }
}
pub struct Line {
    vec: Vec<char>,
}
impl Line {
    pub fn write(&mut self, c: char, cursor: u32) {
        for _ in self.vec.len()..cursor as usize {
            self.vec.push(' ');
        }
        if let Some(cur) = self.vec.get_mut(cursor as usize) {
            *cur = c;
        } else {
            self.vec.push(c);
        }
    }
}
impl Default for Line {
    fn default() -> Self {
        Self {
            vec: Vec::with_capacity(1 << 10),
        }
    }
}
enum WindowState {
    Initial,
    Suspended {
        window: &'static Window,
    },
    Running {
        surface: Surface<OwnedDisplayHandle, &'static Window>,
    },
}
impl Cursor {
    pub fn right(&mut self) {
        self.x += 1;
    }
}
impl<K: Fn(KeyEvent, &mut String)> Term<K> {
    pub fn run(keyboard_input: K) {
        let event_loop = EventLoop::new().unwrap();
        let context = Context::new(event_loop.owned_display_handle()).unwrap();
        let mut app = Self {
            context,
            state: WindowState::Initial,
            cursor: Cursor::default(),
            lines: Lines::default(),
            buffer: String::with_capacity(1 << 10),
            screen: Dimensions::default(),
            keyboard_input,
            font: bdf2::read(&include_bytes!("../terminus.bdf")[..]).unwrap(),
        };
        event_loop.run_app(&mut app).unwrap();
    }
}
impl<K: Fn(KeyEvent, &mut String)> ApplicationHandler for Term<K> {
    fn new_events(&mut self, event_loop: &ActiveEventLoop, cause: StartCause) {
        if let StartCause::Init = cause {
            let window_attrs = Window::default_attributes();
            let window = Box::new(event_loop.create_window(window_attrs).unwrap());
            let window = Box::leak(window);
            self.state = WindowState::Suspended { window };
        }
    }
    fn resumed(&mut self, _: &ActiveEventLoop) {
        let WindowState::Suspended { window } = mem::replace(&mut self.state, WindowState::Initial)
        else {
            unreachable!();
        };
        let size = window.inner_size();
        let mut surface = Surface::new(&self.context, window).unwrap();
        if let (Some(width), Some(height)) =
            (NonZeroU32::new(size.width), NonZeroU32::new(size.height))
        {
            self.screen.x = width.get();
            self.screen.y = height.get();
            surface.resize(width, height).unwrap();
        }
        self.state = WindowState::Running { surface };
    }
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let WindowState::Running { surface } = &mut self.state else {
            unreachable!();
        };
        if surface.window().id() != window_id {
            return;
        }
        match event {
            WindowEvent::ActivationTokenDone { .. } => {}
            WindowEvent::Resized(size) => {
                if let (Some(width), Some(height)) =
                    (NonZeroU32::new(size.width), NonZeroU32::new(size.height))
                {
                    self.screen.x = width.get();
                    self.screen.y = height.get();
                    surface.resize(width, height).unwrap();
                }
            }
            WindowEvent::Moved(_) => {}
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Destroyed => {}
            WindowEvent::DroppedFile(_) => {}
            WindowEvent::HoveredFile(_) => {}
            WindowEvent::HoveredFileCancelled => {}
            WindowEvent::Focused(_) => {}
            WindowEvent::KeyboardInput { event, .. } => {
                (self.keyboard_input)(event, &mut self.buffer);
                if !self.buffer.is_empty() {
                    for c in self.buffer.chars() {
                        self.lines.write(c, self.cursor);
                        self.cursor.right();
                    }
                    surface.window().request_redraw();
                    self.buffer.clear();
                }
            }
            WindowEvent::ModifiersChanged(_) => {}
            WindowEvent::Ime(_) => {}
            WindowEvent::CursorMoved { .. } => {}
            WindowEvent::CursorEntered { .. } => {}
            WindowEvent::CursorLeft { .. } => {}
            WindowEvent::MouseWheel { .. } => {}
            WindowEvent::MouseInput { .. } => {}
            WindowEvent::PinchGesture { .. } => {}
            WindowEvent::PanGesture { .. } => {}
            WindowEvent::DoubleTapGesture { .. } => {}
            WindowEvent::RotationGesture { .. } => {}
            WindowEvent::TouchpadPressure { .. } => {}
            WindowEvent::AxisMotion { .. } => {}
            WindowEvent::Touch(_) => {}
            WindowEvent::ScaleFactorChanged { .. } => {}
            WindowEvent::ThemeChanged(_) => {}
            WindowEvent::Occluded(_) => {}
            WindowEvent::RedrawRequested => {
                let mut buffer = surface.buffer_mut().unwrap();
                for index in 0..(buffer.width().get() * buffer.height().get()) {
                    buffer[index as usize] = 0;
                }
                let mut cursor = Cursor::default();
                for line in self.lines.get_lines() {
                    for c in line.vec.iter().copied() {
                        write_char(&mut buffer, c, cursor, self.screen, &self.font);
                        cursor.x += 1;
                    }
                    cursor.x = 0;
                    cursor.y += 1;
                }
                buffer.present().unwrap();
            }
        }
    }
    fn suspended(&mut self, _: &ActiveEventLoop) {
        let WindowState::Running { surface } = mem::replace(&mut self.state, WindowState::Initial)
        else {
            unreachable!();
        };
        let window = surface.window();
        self.state = WindowState::Suspended { window };
    }
}
pub fn write_char(
    buffer: &mut Buffer<OwnedDisplayHandle, &'static Window>,
    c: char,
    cursor: Cursor,
    screen: Dimensions,
    font: &Font,
) {
    if let Some(glyph) = font.glyphs().get(&c) {
        for y in 0..glyph.height() {
            for x in 0..glyph.width() {
                buffer[(x
                    + screen.x * y
                    + cursor.x * glyph.width()
                    + screen.x * cursor.y * glyph.height()) as usize] =
                    if glyph.get(x, y) { 0xffffff } else { 0x000000 };
            }
        }
    }
}
impl Deref for Lines {
    type Target = Vec<Line>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Lines {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
