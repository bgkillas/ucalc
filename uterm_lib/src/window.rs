use bdf2::Font;
use softbuffer::{Buffer, Context, Surface};
use std::io::Write;
use std::mem;
use std::num::NonZeroU32;
use std::ops::{Deref, DerefMut};
use winit::application::ApplicationHandler;
use winit::event::{KeyEvent, Modifiers, StartCause, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop, OwnedDisplayHandle};
#[cfg(feature = "wasm")]
use winit::platform::web::WindowAttributesExtWebSys;
use winit::window::{Window, WindowId};
#[derive(Debug)]
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
    modifiers: Modifiers,
    first_sized: bool,
    program: T,
}
#[repr(transparent)]
#[derive(Debug)]
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
    fn key_event(&mut self, key_event: KeyEvent, modifiers: Modifiers, buffer: &mut LineBuffer);
}
#[repr(transparent)]
#[derive(Default, Debug, Clone, Copy)]
pub struct Color(u32);
impl From<u32> for Color {
    fn from(value: u32) -> Self {
        Self(match value {
            30 => 0x000000,
            31 => 0xaa0000,
            32 => 0x00aa00,
            33 => 0xaaaa00,
            34 => 0x3333ff,
            35 => 0xaa00aa,
            36 => 0x00aaaa,
            37 => 0xaaaaaa,
            90 => 0x555555,
            91 => 0xff5555,
            92 => 0x55ff55,
            93 => 0xffff55,
            94 => 0x5555ff,
            95 => 0xff55ff,
            96 => 0x55ffff,
            97 => 0xffffff,
            _ => unreachable!(),
        })
    }
}
#[derive(Default, Debug, Clone, Copy)]
pub struct Dimensions {
    pub x: u32,
    pub y: u32,
}
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Cursor {
    x: u32,
    y: u32,
}
#[derive(Debug)]
pub struct Lines {
    vec: Vec<Line>,
}
impl Lines {
    pub fn write(&mut self, c: char, cursor: Cursor, foreground: Color, background: Color) {
        for _ in self.vec.len()..=cursor.y as usize {
            self.vec.push(Line::default())
        }
        self.vec[cursor.y as usize].write(c, cursor.x, foreground, background)
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
        if (self.vec.len() as u32) > cursor.y {
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
#[derive(Debug)]
pub struct Line {
    vec: Vec<(char, Color, Color)>,
}
impl Line {
    pub fn write(&mut self, c: char, cursor: u32, foreground: Color, background: Color) {
        for _ in self.vec.len()..cursor as usize {
            self.vec.push((' ', foreground, background));
        }
        if let Some(cur) = self.vec.get_mut(cursor as usize) {
            *cur = (c, foreground, background);
        } else {
            self.vec.push((c, foreground, background));
        }
    }
    pub fn clear(&mut self) {
        self.vec.clear();
    }
    pub fn clear_down(&mut self, cursor: u32) {
        if (self.vec.len() as u32) > cursor {
            self.vec.drain(cursor as usize..);
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
#[derive(Default, Debug)]
enum WindowState {
    #[default]
    Initial,
    Suspended(&'static Window),
    NeedsSize(Surface<OwnedDisplayHandle, &'static Window>),
    Running(Surface<OwnedDisplayHandle, &'static Window>),
}
impl Cursor {
    pub fn right(&mut self, screen_cells: Dimensions, n: u32) {
        self.x += n;
        (self.y, self.x) = (self.y + self.x / screen_cells.x, self.x % screen_cells.x);
    }
    pub fn left(&mut self, screen_cells: Dimensions, n: u32) {
        let n = self.x as i32 - n as i32;
        (self.y, self.x) = (
            (self.y as i32 + n / screen_cells.x as i32) as u32,
            n.rem_euclid(screen_cells.x as i32) as u32,
        );
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
            state: WindowState::default(),
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
            modifiers: Modifiers::default(),
            foreground: Color(0xffffff),
            background: Color(0x000000),
            first_sized: false,
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
                    self.lines
                        .write(' ', self.cursor, self.foreground, self.background);
                }
                '\n' => self.cursor.down(1),
                '\x1b' => {
                    if !matches!(chars.next(), Some((_, '['))) {
                        unreachable!()
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
                        'G' => {
                            let [Some(n)] = get_numbers(&self.buffer, &mut chars) else {
                                unreachable!()
                            };
                            self.cursor.move_col(n - 1);
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
                            if n == 39 {
                                self.background = Color(0x000000);
                                self.foreground = Color(0xffffff);
                            } else if n == 38 {
                                let (Some(r), Some(g), Some(b)) = (r, g, b) else {
                                    unreachable!()
                                };
                                self.foreground = Color((r << 16) + (g << 8) + b);
                            } else {
                                self.foreground = Color::from(n);
                            }
                        }
                        _ => unreachable!(),
                    }
                }
                c => {
                    self.lines
                        .write(c, self.cursor, self.foreground, self.background);
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
            #[cfg(feature = "wasm")]
            let document = web_sys::window().unwrap().document().unwrap();
            #[cfg(feature = "wasm")]
            let canvas = wasm_bindgen::JsValue::from(document.get_element_by_id("canvas").unwrap());
            #[cfg(feature = "wasm")]
            let window_attrs = window_attrs.with_canvas(Some(canvas.into()));
            let window = Box::new(event_loop.create_window(window_attrs).unwrap());
            let window = Box::leak(window);
            self.state = WindowState::Suspended(window);
        }
    }
    fn resumed(&mut self, _: &ActiveEventLoop) {
        let WindowState::Suspended(window) = mem::replace(&mut self.state, WindowState::Initial)
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
            surface.resize(width, height).unwrap();
            if !self.first_sized {
                self.program.init(&mut self.buffer);
                if !self.buffer.is_empty() {
                    self.clear_buffer();
                    window.request_redraw();
                }
                self.first_sized = true;
            } else {
                self.program.resize(self.screen_cells);
            }
        } else {
            self.state = WindowState::NeedsSize(surface);
            return;
        }
        self.state = WindowState::Running(surface);
    }
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let WindowState::Running(surface) = &mut self.state else {
            if let WindowEvent::Resized(size) = event
                && let (Some(width), Some(height)) =
                    (NonZeroU32::new(size.width), NonZeroU32::new(size.height))
                && let WindowState::NeedsSize(mut surface) = mem::take(&mut self.state)
            {
                self.screen.x = width.get();
                self.screen.y = height.get();
                self.screen_cells.x = self.screen.x / self.font_size.x;
                self.screen_cells.y = self.screen.y / self.font_size.y;
                surface.resize(width, height).unwrap();
                if !self.first_sized {
                    self.program.init(&mut self.buffer);
                    if !self.buffer.is_empty() {
                        self.clear_buffer();
                        surface.window().request_redraw();
                    }
                    self.first_sized = true;
                } else {
                    self.program.resize(self.screen_cells);
                }
                self.state = WindowState::Running(surface);
                return;
            } else {
                return;
            }
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
                } else {
                    let WindowState::Running(surface) = mem::take(&mut self.state) else {
                        unreachable!()
                    };
                    self.state = WindowState::NeedsSize(surface)
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
                self.program
                    .key_event(event, self.modifiers, &mut self.buffer);
                if !self.buffer.is_empty() {
                    surface.window().request_redraw();
                    self.clear_buffer();
                }
            }
            WindowEvent::ModifiersChanged(modifiers) => self.modifiers = modifiers,
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
                let mut exists = false;
                for line in self.lines.get_lines() {
                    let chars = line.vec.iter().copied();
                    for (c, fg, bg) in chars {
                        write_char(&mut buffer, c, cursor, self.screen, &self.font, fg, bg);
                        if cursor == self.cursor {
                            exists = true;
                            write_cursor(&mut buffer, cursor, self.screen, self.font_size, fg);
                        }
                        cursor.x += 1;
                    }
                    cursor.x = 0;
                    cursor.y += 1;
                }
                if !exists {
                    write_cursor(
                        &mut buffer,
                        self.cursor,
                        self.screen,
                        self.font_size,
                        self.foreground,
                    );
                }
                buffer.present().unwrap();
            }
        }
    }
    fn suspended(&mut self, _: &ActiveEventLoop) {
        let WindowState::Running(surface) = mem::replace(&mut self.state, WindowState::Initial)
        else {
            unreachable!();
        };
        let window = surface.window();
        self.state = WindowState::Suspended(window);
    }
}
pub fn write_cursor(
    buffer: &mut Buffer<OwnedDisplayHandle, &'static Window>,
    cursor: Cursor,
    screen: Dimensions,
    font_size: Dimensions,
    foreground: Color,
) {
    let y = font_size.y - 1;
    for x in 0..font_size.x {
        buffer[(x + screen.x * y + cursor.x * font_size.x + screen.x * cursor.y * font_size.y)
            as usize] = foreground.0
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
                    if c == ';' {
                        break i;
                    } else if !c.is_ascii_digit() {
                        is_done = true;
                        break i;
                    }
                };
                Some(str[start..end].parse().unwrap())
            }
        }
    })
}
