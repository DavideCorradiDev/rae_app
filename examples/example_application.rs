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

struct ApplicationImpl {}

impl EventHandler<ApplicationError, CustomEvent> for ApplicationImpl {
    type Error = ApplicationError;
    type CustomEvent = CustomEvent;

    fn new(_event_loop: &EventLoop<Self::CustomEvent>) -> Result<Self, Self::Error> {
        Ok(Self {})
    }

    fn is_close_requested(&self) -> bool {
        println!("Closing application");
        true
    }
}

fn main() {
    const FIXED_FRAMERATE: u64 = 30;
    const VARIABLE_FRAMERATE_CAP: u64 = 60;
    Application::<ApplicationImpl, _, _>::new(FIXED_FRAMERATE, Some(VARIABLE_FRAMERATE_CAP)).run();
}
