use rae_app::*;

use application::Application;
use event::{keyboard, DeviceId, EventHandler, EventLoop};
use window::{PhysicalSize, Size, Window, WindowBuilder, WindowId};

#[derive(Debug)]
enum ApplicationError {
    WindowCreationError(window::OsError),
}

impl std::fmt::Display for ApplicationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApplicationError::WindowCreationError(e) => {
                write!(f, "Failed to create window ({})", e)
            }
        }
    }
}

impl std::error::Error for ApplicationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ApplicationError::WindowCreationError(e) => Some(e),
        }
    }
}

impl From<window::OsError> for ApplicationError {
    fn from(e: window::OsError) -> Self {
        ApplicationError::WindowCreationError(e)
    }
}

struct CustomEvent {}

struct ApplicationImpl {
    _window: Window,
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
            .build(event_loop)?;
        Ok(Self {
            _window: window,
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

    fn on_focus_gained(&mut self, _: WindowId) -> Result<(), Self::Error> {
        println!("Processed 'focus gained' event");
        Ok(())
    }

    fn on_focus_lost(&mut self, _: WindowId) -> Result<(), Self::Error> {
        println!("Processed 'focus lost' event");
        Ok(())
    }

    fn on_key_pressed(
        &mut self,
        _wid: WindowId,
        device_id: DeviceId,
        scan_code: keyboard::ScanCode,
        key_code: Option<keyboard::KeyCode>,
        is_repeat: bool,
    ) -> Result<(), Self::Error> {
        if !is_repeat {
            println!(
                "Processed 'key pressed' event, device: {:?}, scan code: {:?}, key code: {:?}",
                device_id, scan_code, key_code
            );
        }
        Ok(())
    }

    fn on_key_released(
        &mut self,
        _wid: WindowId,
        device_id: DeviceId,
        scan_code: keyboard::ScanCode,
        key_code: Option<keyboard::KeyCode>,
        is_repeat: bool,
    ) -> Result<(), Self::Error> {
        if !is_repeat {
            println!(
                "Processed 'key released' event, device: {:?}, scan code: {:?}, key code: {:?}",
                device_id, scan_code, key_code
            );
        }
        Ok(())
    }
}

fn main() {
    const FIXED_FRAMERATE: u64 = 30;
    const VARIABLE_FRAMERATE_CAP: u64 = 60;
    Application::<ApplicationImpl, _, _>::new(FIXED_FRAMERATE, Some(VARIABLE_FRAMERATE_CAP)).run();
}
