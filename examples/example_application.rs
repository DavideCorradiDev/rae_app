use rae_app::*;

use application::Application;
use event::{EventHandler, EventLoop};

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
    processed_fixed_frames: u64,
    processed_variable_frames: u64,
}

impl EventHandler<ApplicationError, CustomEvent> for ApplicationImpl {
    type Error = ApplicationError;
    type CustomEvent = CustomEvent;

    fn new(_event_loop: &EventLoop<Self::CustomEvent>) -> Result<Self, Self::Error> {
        Ok(Self {
            processed_fixed_frames: 0,
            processed_variable_frames: 0,
        })
    }

    fn is_close_requested(&self) -> bool {
        self.processed_fixed_frames > 150
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
