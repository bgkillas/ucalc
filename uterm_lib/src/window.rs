use softbuffer::{Context, Surface};
use std::mem;
use std::num::NonZeroU32;
use winit::application::ApplicationHandler;
use winit::event::{StartCause, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop, OwnedDisplayHandle};
use winit::window::{Window, WindowId};
pub struct Term {
    context: Context<OwnedDisplayHandle>,
    state: WindowState,
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
impl Term {
    pub fn run() {
        let event_loop = EventLoop::new().unwrap();
        let context = Context::new(event_loop.owned_display_handle()).unwrap();
        let mut app = Self {
            context,
            state: WindowState::Initial,
        };
        event_loop.run_app(&mut app).unwrap();
    }
}
impl ApplicationHandler for Term {
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
            WindowEvent::KeyboardInput { .. } => {}
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
                    let (red, green, blue) = match (index / 16) % 3 {
                        0 => (255, 0, 0),
                        1 => (0, 255, 0),
                        2 => (0, 0, 255),
                        _ => unreachable!(),
                    };
                    buffer[index as usize] = blue | (green << 8) | (red << 16);
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
