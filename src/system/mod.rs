extern crate winit;

pub mod window {
    pub use winit::{dpi::*, error::*, window::*};
}

pub mod monitor {
    pub use winit::monitor::*;
}

pub mod keyboard {
    pub use winit::event::{ElementState, ModifiersState, ScanCode, VirtualKeyCode as KeyCode};
}

pub mod mouse {
    pub use winit::event::{MouseButton as Button, MouseScrollDelta as ScrollDelta};
}

pub mod touch {
    pub use winit::event::Force;
    pub use winit::event::TouchPhase;
}

mod event_loop_any_thread;
pub mod event {
    pub use super::event_loop_any_thread::EventLoopAnyThread;
    pub use winit::event::DeviceEvent;
    pub use winit::event::Event;
    pub use winit::event::StartCause;
    pub use winit::event::WindowEvent;
    pub use winit::event_loop::*;
}
