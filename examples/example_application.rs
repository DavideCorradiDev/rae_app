use rae_app::*;

use application::Application;
use event::{
    controller, keyboard, mouse, touch, DeviceId, EventHandler, EventLoop, EventLoopClosed,
    EventLoopProxy, EventLoopStartCause, ScrollDelta,
};
use window::{PhysicalPosition, PhysicalSize, Size, Window, WindowBuilder, WindowId};

#[derive(Debug)]
enum ApplicationError {
    WindowCreationError(window::OsError),
    CustomEventSendingError(EventLoopClosed<CustomEvent>),
}

impl std::fmt::Display for ApplicationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApplicationError::WindowCreationError(e) => {
                write!(f, "Failed to create window ({})", e)
            }
            ApplicationError::CustomEventSendingError(e) => {
                write!(f, "Failed to send a custom event ({})", e)
            }
        }
    }
}

impl std::error::Error for ApplicationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ApplicationError::WindowCreationError(e) => Some(e),
            ApplicationError::CustomEventSendingError(e) => Some(e),
        }
    }
}

impl From<window::OsError> for ApplicationError {
    fn from(e: window::OsError) -> Self {
        ApplicationError::WindowCreationError(e)
    }
}

impl From<EventLoopClosed<CustomEvent>> for ApplicationError {
    fn from(e: EventLoopClosed<CustomEvent>) -> Self {
        ApplicationError::CustomEventSendingError(e)
    }
}

#[derive(Debug, Clone, Copy)]
enum CustomEvent {
    SomeTimePassed,
    LongTimePassed,
}

struct ApplicationImpl {
    _window: Window,
    event_loop_proxy: EventLoopProxy<CustomEvent>,
    close_requested: bool,
    processed_fixed_frames: u64,
    processed_variable_frames: u64,
    processed_cursor_moved_events: u64,
    processed_device_cursor_moved_events: u64,
    processed_device_axis_moved_events: u64,
    processed_new_events_events: u64,
    processed_main_events_cleared_events: u64,
    processed_redraw_events_cleared_events: u64,
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
            event_loop_proxy: event_loop.create_proxy(),
            close_requested: false,
            processed_fixed_frames: 0,
            processed_variable_frames: 0,
            processed_cursor_moved_events: 0,
            processed_device_cursor_moved_events: 0,
            processed_device_axis_moved_events: 0,
            processed_new_events_events: 0,
            processed_main_events_cleared_events: 0,
            processed_redraw_events_cleared_events: 0,
        })
    }

    fn is_close_requested(&self) -> bool {
        self.close_requested
    }

    fn on_fixed_update(&mut self, dt: std::time::Duration) -> Result<(), Self::Error> {
        if self.processed_fixed_frames % 30 == 0 {
            println!("Processed 'fixed update' event, dt = {:?}", dt);
        }

        if self.processed_fixed_frames % 30 == 0 {
            self.event_loop_proxy
                .send_event(CustomEvent::SomeTimePassed)?;
        }

        if self.processed_fixed_frames % 90 == 0 {
            self.event_loop_proxy
                .send_event(CustomEvent::LongTimePassed)?;
        }

        self.processed_fixed_frames = self.processed_fixed_frames + 1;
        Ok(())
    }

    fn on_variable_update(&mut self, dt: std::time::Duration) -> Result<(), Self::Error> {
        if self.processed_variable_frames % 30 == 0 {
            println!("Processed 'variable update' event, dt = {:?}", dt);
        }
        self.processed_variable_frames = self.processed_variable_frames + 1;
        Ok(())
    }

    fn on_close_requested(&mut self, wid: WindowId) -> Result<(), Self::Error> {
        self.close_requested = true;
        println!("Processed 'close requested' event, window {:?}.", wid);
        Ok(())
    }

    fn on_destroyed(&mut self, wid: WindowId) -> Result<(), Self::Error> {
        self.close_requested = true;
        println!("Processed 'destroyed' event, window {:?}.", wid);
        Ok(())
    }

    fn on_focus_gained(&mut self, wid: WindowId) -> Result<(), Self::Error> {
        println!("Processed 'focus gained' event, window {:?}", wid);
        Ok(())
    }

    fn on_focus_lost(&mut self, wid: WindowId) -> Result<(), Self::Error> {
        println!("Processed 'focus lost' event, window {:?}", wid);
        Ok(())
    }

    fn on_received_character(&mut self, wid: WindowId, c: char) -> Result<(), Self::Error> {
        println!(
            "Processed 'received character' event, window {:?}, character {:?}",
            wid, c
        );
        Ok(())
    }

    fn on_resized(&mut self, wid: WindowId, size: PhysicalSize<u32>) -> Result<(), Self::Error> {
        println!(
            "Processed 'resized' event, window {:?}, size {:?}",
            wid, size
        );
        Ok(())
    }

    fn on_scale_factor_changed<'a>(
        &self,
        wid: WindowId,
        scale_factor: f64,
        new_inner_size: &'a mut PhysicalSize<u32>,
    ) -> Result<(), Self::Error> {
        println!(
            "Processed 'scale factor changed' event, window {:?}, scale_factor {:?}, new size {:?}",
            wid, scale_factor, *new_inner_size
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

    fn on_hovered_file_dropped(
        &mut self,
        wid: WindowId,
        path: std::path::PathBuf,
    ) -> Result<(), Self::Error> {
        println!(
            "Processed 'hovered file dropped' event, window {:?}, path {:?}",
            wid, path
        );
        Ok(())
    }

    fn on_hovered_file_entered(
        &mut self,
        wid: WindowId,
        path: std::path::PathBuf,
    ) -> Result<(), Self::Error> {
        println!(
            "Processed 'hovered file entered' event, window {:?}, path {:?}",
            wid, path
        );
        Ok(())
    }

    fn on_hovered_file_left(&mut self, wid: WindowId) -> Result<(), Self::Error> {
        println!("Processed 'hovered file left' event, window {:?}", wid);
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

    fn on_cursor_moved(
        &mut self,
        wid: WindowId,
        device_id: DeviceId,
        position: PhysicalPosition<f64>,
    ) -> Result<(), Self::Error> {
        if self.processed_cursor_moved_events % 20 == 0 {
            println!(
                "Processed 'cursor moved' event, window: {:?}, device: {:?}, position: {:?}",
                wid, device_id, position
            );
        }
        self.processed_cursor_moved_events = self.processed_cursor_moved_events + 1;
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

    fn on_axis_moved(
        &mut self,
        wid: WindowId,
        device_id: DeviceId,
        axis: controller::AxisId,
        value: f64,
    ) -> Result<(), Self::Error> {
        println!(
            "Processed 'axis motion' event, window {:?}, device {:?}, axis {:?}, value {:?}",
            wid, device_id, axis, value
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

    fn on_device_added(&mut self, device_id: DeviceId) -> Result<(), Self::Error> {
        println!("Processed 'device added' event, device {:?}", device_id);
        Ok(())
    }

    fn on_device_removed(&mut self, device_id: DeviceId) -> Result<(), Self::Error> {
        println!("Processed 'device removed' event, device {:?}", device_id);
        Ok(())
    }

    fn on_device_cursor_moved(
        &mut self,
        device_id: DeviceId,
        position_delta: PhysicalPosition<f64>,
    ) -> Result<(), Self::Error> {
        if self.processed_device_cursor_moved_events % 20 == 0 {
            println!(
                "Processed 'device cursor moved' event, device {:?}, position delta {:?}",
                device_id, position_delta
            );
        }
        self.processed_device_cursor_moved_events = self.processed_device_cursor_moved_events + 1;
        Ok(())
    }

    fn on_device_scroll(
        &mut self,
        device_id: DeviceId,
        scroll_delta: ScrollDelta,
    ) -> Result<(), Self::Error> {
        println!(
            "Processed 'device cursor moved' event, device {:?}, scroll delta {:?}",
            device_id, scroll_delta
        );
        Ok(())
    }

    fn on_device_axis_moved(
        &mut self,
        device_id: DeviceId,
        axis: controller::AxisId,
        value: f64,
    ) -> Result<(), Self::Error> {
        if self.processed_device_axis_moved_events % 20 == 0 {
            println!(
                "Processed 'device axis motion' event, device {:?}, axis {:?}, value {:?}",
                device_id, axis, value
            );
        }
        self.processed_device_axis_moved_events = self.processed_device_axis_moved_events + 1;
        Ok(())
    }

    fn on_device_button_pressed(
        &mut self,
        device_id: DeviceId,
        button: controller::ButtonId,
    ) -> Result<(), Self::Error> {
        println!(
            "Processed 'device button pressed' event, device {:?}, button {:?}",
            device_id, button
        );
        Ok(())
    }

    fn on_device_button_released(
        &mut self,
        device_id: DeviceId,
        button: controller::ButtonId,
    ) -> Result<(), Self::Error> {
        println!(
            "Processed 'device button released' event, device {:?}, button {:?}",
            device_id, button
        );
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
                "Processed 'device key pressed' event,
                device: {:?}, scan code: {:?}, key code: {:?}, repeat {:?}",
                device_id, scan_code, key_code, is_repeat
            );
        }
        Ok(())
    }

    fn on_device_key_released(
        &mut self,
        device_id: DeviceId,
        scan_code: keyboard::ScanCode,
        key_code: Option<keyboard::KeyCode>,
    ) -> Result<(), Self::Error> {
        println!(
            "Processed 'device key released' event, device: {:?}, scan code: {:?}, key code: {:?}",
            device_id, scan_code, key_code,
        );
        Ok(())
    }

    fn on_device_text(&mut self, device_id: DeviceId, codepoint: char) -> Result<(), Self::Error> {
        println!(
            "Processed 'device text' event, device: {:?}, codepoint: {:?}",
            device_id, codepoint
        );
        Ok(())
    }

    fn on_custom_event(&mut self, event: Self::CustomEvent) -> Result<(), Self::Error> {
        println!("Processed 'custom' event, {:?}", event);
        Ok(())
    }

    fn on_new_events(&mut self, start_cause: EventLoopStartCause) -> Result<(), Self::Error> {
        if self.processed_new_events_events % 100000 == 0 {
            println!(
                "Processed 'new events' event, start cause {:?}",
                start_cause
            );
        }
        self.processed_new_events_events = self.processed_new_events_events + 1;
        Ok(())
    }

    fn on_main_events_cleared(&mut self) -> Result<(), Self::Error> {
        if self.processed_main_events_cleared_events % 100000 == 0 {
            println!("Processed 'main events cleared' event");
        }
        self.processed_main_events_cleared_events = self.processed_main_events_cleared_events + 1;
        Ok(())
    }

    fn on_redraw_requested(&mut self, wid: WindowId) -> Result<(), Self::Error> {
        println!("Processed 'redraw requested' event, window id {:?}", wid);
        Ok(())
    }

    fn on_redraw_events_cleared(&mut self) -> Result<(), Self::Error> {
        if self.processed_redraw_events_cleared_events % 100000 == 0 {
            println!("Processed 'redraw events cleared' event");
        }
        self.processed_redraw_events_cleared_events =
            self.processed_redraw_events_cleared_events + 1;
        Ok(())
    }

    fn on_suspended(&mut self) -> Result<(), Self::Error> {
        println!("Processed 'suspended' event");
        Ok(())
    }

    fn on_resumed(&mut self) -> Result<(), Self::Error> {
        println!("Processed 'resumed' event");
        Ok(())
    }

    fn on_event_loop_destroyed(&mut self) -> Result<(), Self::Error> {
        println!("Processed 'event loop destroyed' event");
        Ok(())
    }
}

fn main() {
    const FIXED_FRAMERATE: u64 = 30;
    const VARIABLE_FRAMERATE_CAP: u64 = 60;
    Application::<ApplicationImpl, _, _>::new(FIXED_FRAMERATE, Some(VARIABLE_FRAMERATE_CAP)).run();
}
