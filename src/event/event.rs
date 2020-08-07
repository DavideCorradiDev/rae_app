extern crate winit;

pub use winit::{
    event::{
        DeviceEvent, DeviceId, ElementState, Event, StartCause as EventLoopStartCause, WindowEvent,
    },
    event_loop::*,
};
