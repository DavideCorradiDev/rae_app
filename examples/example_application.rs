use rae_app::*;

use application::Application;
use event::{keyboard, mouse, touch, DeviceId, EventHandler, EventLoop, ScrollDelta};
use window::{PhysicalPosition, PhysicalSize, Size, Window, WindowBuilder, WindowId};

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

    fn on_close_requested(&mut self, wid: WindowId) -> Result<(), Self::Error> {
        self.close_requested = true;
        println!(
            "Processed 'window close requested' event, window {:?}.",
            wid
        );
        Ok(())
    }

    fn on_destroyed(&mut self, wid: WindowId) -> Result<(), Self::Error> {
        self.close_requested = true;
        println!("Processed 'window destroyed' event, window {:?}.", wid);
        Ok(())
    }

    fn on_focus_gained(&mut self, wid: WindowId) -> Result<(), Self::Error> {
        println!("Processed 'window focus gained' event, window {:?}", wid);
        Ok(())
    }

    fn on_focus_lost(&mut self, wid: WindowId) -> Result<(), Self::Error> {
        println!("Processed 'window focus lost' event, window {:?}", wid);
        Ok(())
    }

    fn on_received_character(&mut self, wid: WindowId, c: char) -> Result<(), Self::Error> {
        println!(
            "Processed 'window received character' event, window {:?}, character {:?}",
            wid, c
        );
        Ok(())
    }

    fn on_resized(&mut self, wid: WindowId, size: PhysicalSize<u32>) -> Result<(), Self::Error> {
        println!(
            "Processed 'window resized' event, window {:?}, size {:?}",
            wid, size
        );
        Ok(())
    }

    fn on_moved(
        &mut self,
        wid: WindowId,
        position: PhysicalPosition<i32>,
    ) -> Result<(), Self::Error> {
        println!(
            "Processed 'window moved' event, window {:?}, position {:?}",
            wid, position
        );
        Ok(())
    }

    fn on_key_pressed(
        &mut self,
        wid: WindowId,
        device_id: DeviceId,
        scan_code: keyboard::ScanCode,
        key_code: Option<keyboard::KeyCode>,
        is_synthetic: bool,
        is_repeat: bool,
    ) -> Result<(), Self::Error> {
        if !is_repeat {
            println!(
                "Processed 'window key pressed' event, \
                window {:?}, device: {:?}, scan code: {:?}, key code: {:?}, repeat {:?}, synthetic {:?}",
                wid, device_id, scan_code, key_code, is_repeat, is_synthetic
            );
        }
        Ok(())
    }

    fn on_device_key_pressed(
        &mut self,
        device_id: DeviceId,
        scan_code: keyboard::ScanCode,
        key_code: Option<keyboard::KeyCode>,
        is_repeat: bool,
    ) -> Result<(), Self::Error> {
        if !is_repeat {
            println!(
                "Processed 'device key pressed' event, \
                device: {:?}, scan code: {:?}, key code: {:?}, repeat {:?}",
                device_id, scan_code, key_code, is_repeat
            );
        }
        Ok(())
    }

    fn on_key_released(
        &mut self,
        wid: WindowId,
        device_id: DeviceId,
        scan_code: keyboard::ScanCode,
        key_code: Option<keyboard::KeyCode>,
        is_synthetic: bool,
    ) -> Result<(), Self::Error> {
        println!(
            "Processed 'window key released' event, \
            window: {:?}, device: {:?}, scan code: {:?}, key code: {:?}, synthetic {:?}",
            wid, device_id, scan_code, key_code, is_synthetic
        );
        Ok(())
    }

    fn on_device_key_released(
        &mut self,
        device_id: DeviceId,
        scan_code: keyboard::ScanCode,
        key_code: Option<keyboard::KeyCode>,
    ) -> Result<(), Self::Error> {
        println!(
            "Processed 'device key released' event, \
            device: {:?}, scan code: {:?}, key code: {:?}",
            device_id, scan_code, key_code,
        );
        Ok(())
    }

    fn on_cursor_moved(
        &mut self,
        wid: WindowId,
        device_id: DeviceId,
        position: PhysicalPosition<f64>,
    ) -> Result<(), Self::Error> {
        println!(
            "Processed 'cursor moved' event, window: {:?}, device: {:?}, position: {:?}",
            wid, device_id, position
        );
        Ok(())
    }

    fn on_cursor_entered(&mut self, wid: WindowId, device_id: DeviceId) -> Result<(), Self::Error> {
        println!(
            "Processed 'cursor entered' event, window: {:?}, device: {:?}",
            wid, device_id
        );
        Ok(())
    }

    fn on_cursor_left(&mut self, wid: WindowId, device_id: DeviceId) -> Result<(), Self::Error> {
        println!(
            "Processed 'cursor left' event, window: {:?}, device: {:?}",
            wid, device_id
        );
        Ok(())
    }

    fn on_modifiers_changed(
        &mut self,
        wid: WindowId,
        modifiers_state: keyboard::ModifiersState,
    ) -> Result<(), Self::Error> {
        println!(
            "Processed 'modifiers changed' event, window {:?}, modifiers {:?}",
            wid, modifiers_state
        );
        Ok(())
    }

    fn on_mouse_button_pressed(
        &mut self,
        wid: WindowId,
        device_id: DeviceId,
        button: mouse::Button,
    ) -> Result<(), Self::Error> {
        println!(
            "Processed 'mouse button pressed' event, window {:?}, device {:?}, button {:?}",
            wid, device_id, button
        );
        Ok(())
    }

    fn on_mouse_button_released(
        &mut self,
        wid: WindowId,
        device_id: DeviceId,
        button: mouse::Button,
    ) -> Result<(), Self::Error> {
        println!(
            "Processed 'mouse button released' event, window {:?}, device {:?}, button {:?}",
            wid, device_id, button
        );
        Ok(())
    }

    fn on_scroll(
        &mut self,
        wid: WindowId,
        device_id: DeviceId,
        delta: ScrollDelta,
        phase: touch::TouchPhase,
    ) -> Result<(), Self::Error> {
        println!(
            "Processed 'scroll' event, window {:?}, device {:?}, delta {:?}, phase {:?}",
            wid, device_id, delta, phase
        );
        Ok(())
    }

    fn on_touch(
        &mut self,
        wid: WindowId,
        device_id: DeviceId,
        phase: touch::TouchPhase,
        location: PhysicalPosition<f64>,
        force: Option<touch::Force>,
        id: u64,
    ) -> Result<(), Self::Error> {
        println!(
            "Processed 'on touch' event, \
            window {:?}, device {:?}, phase {:?}, location {:?}, force {:?}, id {:?}",
            wid, device_id, phase, location, force, id
        );
        Ok(())
    }
}

fn main() {
    const FIXED_FRAMERATE: u64 = 30;
    const VARIABLE_FRAMERATE_CAP: u64 = 60;
    Application::<ApplicationImpl, _, _>::new(FIXED_FRAMERATE, Some(VARIABLE_FRAMERATE_CAP)).run();
}
