use rae_app::*;

use application::Application;
use event::{EventHandler, EventLoop};
use window::{PhysicalSize, Size, Window, WindowBuilder, WindowId};

#[derive(Debug)]
enum ApplicationError {}

impl std::fmt::Display for ApplicationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error")
    }
}

impl std::error::Error for ApplicationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

struct CustomEvent {}

struct ApplicationImpl {
    window: Window,
    close_requested: bool,
    processed_fixed_frames: u64,
    processed_variable_frames: u64,
}

impl EventHandler<ApplicationError, CustomEvent> for ApplicationImpl {
    type Error = ApplicationError;
    type CustomEvent = CustomEvent;

    fn new(event_loop: &EventLoop<Self::CustomEvent>) -> Result<Self, Self::Error> {
        let window = WindowBuilder::new()
            .with_title("Example application")
            .with_inner_size(Size::Physical(PhysicalSize {
                width: 800,
                height: 600,
            }))
            .build(event_loop)
            .unwrap();
        Ok(Self {
            window,
            close_requested: false,
            processed_fixed_frames: 0,
            processed_variable_frames: 0,
        })
    }

    fn is_close_requested(&self) -> bool {
        self.close_requested
    }

    fn on_close_requested(&mut self, _: WindowId) -> Result<(), Self::Error> {
        self.close_requested = true;
        println!("Processed 'close requested' event.");
        Ok(())
    }

    fn on_fixed_update(&mut self, dt: std::time::Duration) -> Result<(), Self::Error> {
        if self.processed_fixed_frames % 30 == 0 {
            println!(
                "Fixed update called, frame {}, dt = {:?}",
                self.processed_fixed_frames, dt
            );
        }
        self.processed_fixed_frames = self.processed_fixed_frames + 1;
        Ok(())
    }

    fn on_variable_update(&mut self, dt: std::time::Duration) -> Result<(), Self::Error> {
        if self.processed_variable_frames % 30 == 0 {
            println!(
                "Variable update called, frame {}, dt = {:?}",
                self.processed_variable_frames, dt
            );
        }
        self.processed_variable_frames = self.processed_variable_frames + 1;
        Ok(())
    }
}

fn main() {
    const FIXED_FRAMERATE: u64 = 30;
    const VARIABLE_FRAMERATE_CAP: u64 = 60;
    Application::<ApplicationImpl, _, _>::new(FIXED_FRAMERATE, Some(VARIABLE_FRAMERATE_CAP)).run();
}
