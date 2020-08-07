use super::{keyboard, DeviceId, EventLoop};
use crate::window::WindowId;

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

    fn on_close_requested(&mut self, _wid: WindowId) -> Result<(), Self::Error> {
        Ok(())
    }

    fn on_focus_gained(&mut self, _wid: WindowId) -> Result<(), Self::Error> {
        Ok(())
    }

    fn on_focus_lost(&mut self, _wid: WindowId) -> Result<(), Self::Error> {
        Ok(())
    }

    fn on_key_pressed(
        &mut self,
        _wid: WindowId,
        _device_id: DeviceId,
        _scan_code: keyboard::ScanCode,
        _key_code: Option<keyboard::KeyCode>,
        _is_repeat: bool,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn on_key_released(
        &mut self,
        _wid: WindowId,
        _device_id: DeviceId,
        _scan_code: keyboard::ScanCode,
        _key_code: Option<keyboard::KeyCode>,
        _is_repeat: bool,
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
