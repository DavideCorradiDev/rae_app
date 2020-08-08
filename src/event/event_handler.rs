use super::{keyboard, DeviceId, EventLoop};
use crate::window::{PhysicalPosition, PhysicalSize, WindowId};

pub trait EventHandler<ErrorType, CustomEventType>
where
    Self: std::marker::Sized,
    ErrorType: std::fmt::Display + std::error::Error + 'static,
    CustomEventType: 'static,
{
    type Error: std::fmt::Display + std::error::Error + 'static;
    type CustomEvent: 'static;

    fn new(event_loop: &EventLoop<Self::CustomEvent>) -> Result<Self, Self::Error>;

    fn is_close_requested(&self) -> bool {
        false
    }

    fn on_window_close_requested(&mut self, _wid: WindowId) -> Result<(), Self::Error> {
        Ok(())
    }

    fn on_window_destroyed(&mut self, _wid: WindowId) -> Result<(), Self::Error> {
        Ok(())
    }

    fn on_window_focus_gained(&mut self, _wid: WindowId) -> Result<(), Self::Error> {
        Ok(())
    }

    fn on_window_focus_lost(&mut self, _wid: WindowId) -> Result<(), Self::Error> {
        Ok(())
    }

    fn on_window_resized(
        &mut self,
        _wid: WindowId,
        _size: PhysicalSize<u32>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn on_window_moved(
        &mut self,
        _wid: WindowId,
        _position: PhysicalPosition<i32>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn on_window_received_character(
        &mut self,
        _wid: WindowId,
        _c: char,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn on_window_key_pressed(
        &mut self,
        _wid: WindowId,
        _device_id: DeviceId,
        _scan_code: keyboard::ScanCode,
        _key_code: Option<keyboard::KeyCode>,
        _is_synthetic: bool,
        _is_repeat: bool,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn on_device_key_pressed(
        &mut self,
        _device_id: DeviceId,
        _scan_code: keyboard::ScanCode,
        _key_code: Option<keyboard::KeyCode>,
        _is_repeat: bool,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn on_window_key_released(
        &mut self,
        _wid: WindowId,
        _device_id: DeviceId,
        _scan_code: keyboard::ScanCode,
        _key_code: Option<keyboard::KeyCode>,
        _is_synthetic: bool,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn on_device_key_released(
        &mut self,
        _device_id: DeviceId,
        _scan_code: keyboard::ScanCode,
        _key_code: Option<keyboard::KeyCode>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn on_window_modifiers_changed(
        &mut self,
        _wid: WindowId,
        _modifiers_state: keyboard::ModifiersState,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn on_fixed_update(&mut self, _dt: std::time::Duration) -> Result<(), Self::Error> {
        Ok(())
    }

    fn on_variable_update(&mut self, _dt: std::time::Duration) -> Result<(), Self::Error> {
        Ok(())
    }
}
