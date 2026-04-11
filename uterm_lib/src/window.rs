use bdf2::Font;
use softbuffer::{Buffer, Context, Surface};
use std::io::Write;
use std::mem;
use std::num::NonZeroU32;
use std::ops::{Deref, DerefMut};
use winit::application::ApplicationHandler;
use winit::event::{KeyEvent, StartCause, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop, OwnedDisplayHandle};
use winit::window::{Window, WindowId};
pub struct Term<T: Program> {
    context: Context<OwnedDisplayHandle>,
    state: WindowState,
    lines: Lines,
    cursor: Cursor,
    buffer: LineBuffer,
    screen: Dimensions,
    screen_cells: Dimensions,
    font_size: Dimensions,
    font: Font,
    foreground: Color,
    background: Color,
    program: T,
}
#[repr(transparent)]
pub struct LineBuffer(String);
impl Write for LineBuffer {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.extend(str::from_utf8(buf).unwrap().chars());
        Ok(buf.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
pub trait Program {
    fn resize(&mut self, cells: Dimensions);
    fn init(&mut self, buffer: &mut LineBuffer);
    fn event(&mut self, event: KeyEvent, buffer: &mut LineBuffer);
}
#[repr(transparent)]
#[derive(Default, Clone, Copy)]
pub struct Color(u32);
#[derive(Default, Clone, Copy)]
pub struct Dimensions {
    pub x: u32,
    pub y: u32,
}
#[derive(Default, Clone, Copy)]
pub struct Cursor {
    x: u32,
    y: u32,
}
pub struct Lines {
    vec: Vec<Line>,
}
impl Lines {
    pub fn write(&mut self, c: char, cursor: Cursor) {
        for _ in self.vec.len()..=cursor.y as usize {
            self.vec.push(Line::default())
        }
        self.vec[cursor.y as usize].write(c, cursor.x)
    }
    pub fn clear_line(&mut self, cursor: Cursor) {
        self.vec[cursor.y as usize].clear();
    }
    pub fn clear(&mut self) {
        for line in &mut self.vec {
            line.clear()
        }
    }
    pub fn clear_down(&mut self, cursor: Cursor) {
        self.vec[cursor.y as usize].clear_down(cursor.x);
        if self.vec.len() as u32 >= cursor.y {
            for line in &mut self.vec[cursor.y as usize + 1..] {
                line.clear()
            }
        }
    }
    pub fn get_lines(&self) -> impl Iterator<Item = &Line> {
        self.vec.iter()
    }
}
impl Default for Lines {
    fn default() -> Self {
        Self {
            vec: Vec::with_capacity(1 << 12),
        }
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
    pub fn clear(&mut self) {
        self.vec.clear();
    }
    pub fn clear_down(&mut self, cursor: u32) {
        self.vec.drain(cursor as usize + 1..);
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
    pub fn right(&mut self, screen_cells: Dimensions, n: u32) {
        //TODO
        for _ in 0..n {
            self.x += 1;
            if self.x == screen_cells.x {
                self.x = 0;
                self.y += 1;
            }
        }
    }
    pub fn left(&mut self, screen_cells: Dimensions, n: u32) {
        for _ in 0..n {
            if self.x == 0 {
                self.y -= 1;
                self.x = screen_cells.x - 1;
            } else {
                self.x -= 1;
            }
        }
    }
    pub fn down(&mut self, n: u32) {
        self.y += n;
    }
    pub fn up(&mut self, n: u32) {
        self.y -= n;
    }
    pub fn move_to(&mut self, x: u32, y: u32) {
        self.x = x;
        self.y = y;
    }
    pub fn move_col(&mut self, x: u32) {
        self.x = x;
    }
}
impl<T: Program> Term<T> {
    pub fn run(program: T) {
        let event_loop = EventLoop::new().unwrap();
        let context = Context::new(event_loop.owned_display_handle()).unwrap();
        let font = bdf2::read(&include_bytes!("../terminus.bdf")[..]).unwrap();
        let char = font.glyphs().get(&'a').unwrap();
        let mut app = Self {
            context,
            state: WindowState::Initial,
            cursor: Cursor::default(),
            lines: Lines::default(),
            buffer: LineBuffer(String::with_capacity(1 << 10)),
            screen: Dimensions::default(),
            screen_cells: Dimensions::default(),
            font_size: Dimensions {
                x: char.width(),
                y: char.height(),
            },
            font,
            foreground: Color(0xffffff),
            background: Color(0x000000),
            program,
        };
        event_loop.run_app(&mut app).unwrap();
    }
    fn clear_buffer(&mut self) {
        let mut chars = self.buffer.char_indices();
        while let Some((i, c)) = chars.next() {
            match c {
                '\u{8}' => {
                    self.cursor.left(self.screen_cells, 1);
                    self.lines.write(' ', self.cursor);
                }
                '\n' => self.cursor.down(1),
                '\x1b' => {
                    if !matches!(chars.next(), Some((_, '['))) {
                        unreachable!()
                    }
                    fn get_numbers<const N: usize>(
                        str: &str,
                        chars: &mut impl Iterator<Item = (usize, char)>,
                    ) -> [Option<u32>; N] {
                        let mut is_done = false;
                        std::array::from_fn(|_| {
                            if is_done {
                                None
                            } else {
                                let (start, c) = chars.next().unwrap();
                                if !c.is_ascii_digit() {
                                    is_done = true;
                                    None
                                } else {
                                    let end = loop {
                                        let (i, c) = chars.next().unwrap();
                                        if !c.is_ascii_digit() {
                                            break i;
                                        }
                                    };
                                    Some(str[start..end].parse().unwrap())
                                }
                            }
                        })
                    }
                    match self.buffer[i..]
                        .chars()
                        .find(|c| c.is_ascii_alphabetic())
                        .unwrap()
                    {
                        'B' => {
                            let [Some(n)] = get_numbers(&self.buffer, &mut chars) else {
                                unreachable!()
                            };
                            self.cursor.down(n)
                        }
                        'C' => {
                            let [Some(n)] = get_numbers(&self.buffer, &mut chars) else {
                                unreachable!()
                            };
                            self.cursor.right(self.screen_cells, n)
                        }
                        'F' => {
                            let [Some(n)] = get_numbers(&self.buffer, &mut chars) else {
                                unreachable!()
                            };
                            self.cursor.up(n);
                            self.cursor.move_col(0);
                        }
                        'E' => {
                            let [Some(n)] = get_numbers(&self.buffer, &mut chars) else {
                                unreachable!()
                            };
                            self.cursor.up(n);
                            self.cursor.move_col(0);
                        }
                        'H' => {
                            let [Some(x), Some(y)] = get_numbers(&self.buffer, &mut chars) else {
                                unreachable!()
                            };
                            self.cursor.move_to(x, y)
                        }
                        'K' => {
                            let [Some(2)] = get_numbers(&self.buffer, &mut chars) else {
                                unreachable!()
                            };
                            self.lines.clear_line(self.cursor);
                        }
                        'J' => {
                            let [n] = get_numbers(&self.buffer, &mut chars);
                            match n {
                                Some(2) => self.lines.clear(),
                                Some(3) => self.lines.clear(),
                                None => self.lines.clear_down(self.cursor),
                                _ => unreachable!(),
                            }
                        }
                        'm' => {
                            let [Some(n), _, r, g, b] = get_numbers(&self.buffer, &mut chars)
                            else {
                                unreachable!()
                            };
                            if n == 38 {
                                let (Some(r), Some(g), Some(b)) = (r, g, b) else {
                                    unreachable!()
                                };
                                _ = r;
                                _ = g;
                                _ = b;
                                todo!()
                            } else {
                                _ = n;
                            }
                        }
                        _ => unreachable!(),
                    }
                }
                c => {
                    self.lines.write(c, self.cursor);
                    self.cursor.right(self.screen_cells, 1);
                }
            }
        }
        self.buffer.clear();
    }
}
impl<T: Program> ApplicationHandler for Term<T> {
    fn new_events(&mut self, event_loop: &ActiveEventLoop, cause: StartCause) {
        if let StartCause::Init = cause {
            let window_attrs = Window::default_attributes();
            let window = Box::new(event_loop.create_window(window_attrs).unwrap());
            let window = Box::leak(window);
            self.state = WindowState::Suspended { window };
            self.program.init(&mut self.buffer);
            if !self.buffer.is_empty() {
                self.clear_buffer();
                window.request_redraw();
            }
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
            self.screen_cells.x = self.screen.x / self.font_size.x;
            self.screen_cells.y = self.screen.y / self.font_size.y;
            self.program.resize(self.screen_cells);
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
                    self.screen_cells.x = self.screen.x / self.font_size.x;
                    self.screen_cells.y = self.screen.y / self.font_size.y;
                    self.program.resize(self.screen_cells);
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
                self.program.event(event, &mut self.buffer);
                if !self.buffer.is_empty() {
                    surface.window().request_redraw();
                    self.clear_buffer();
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
                    buffer[index as usize] = self.background.0;
                }
                let mut cursor = Cursor::default();
                for line in self.lines.get_lines() {
                    for c in line.vec.iter().copied() {
                        write_char(
                            &mut buffer,
                            c,
                            cursor,
                            self.screen,
                            &self.font,
                            self.foreground,
                            self.background,
                        );
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
    foreground: Color,
    background: Color,
) {
    if let Some(glyph) = font.glyphs().get(&c) {
        for y in 0..glyph.height() {
            for x in 0..glyph.width() {
                buffer[(x
                    + screen.x * y
                    + cursor.x * glyph.width()
                    + screen.x * cursor.y * glyph.height()) as usize] = if glyph.get(x, y) {
                    foreground.0
                } else {
                    background.0
                };
            }
        }
    }
}
impl Deref for LineBuffer {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for LineBuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
