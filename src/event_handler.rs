use crate::{keyboard, DeviceId, EventLoop, WindowId};

pub trait EventHandler<Error, CustomEvent>
where
    Self: std::marker::Sized,
    Error: std::fmt::Display + std::error::Error + 'static,
    CustomEvent: 'static,
{
    fn new(event_loop: &EventLoop<CustomEvent>) -> Result<Self, Error>;

    fn is_close_requested(&self) -> bool {
        false
    }

    fn on_close_requested(&mut self, _wid: WindowId) -> Result<(), Error> {
        Ok(())
    }

    fn on_focus_gained(&mut self, _wid: WindowId) -> Result<(), Error> {
        Ok(())
    }

    fn on_focus_lost(&mut self, _wid: WindowId) -> Result<(), Error> {
        Ok(())
    }

    fn on_key_pressed(
        &mut self,
        _wid: WindowId,
        _device_id: DeviceId,
        _scan_code: keyboard::ScanCode,
        _key_code: Option<keyboard::KeyCode>,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn on_key_released(
        &mut self,
        _wid: WindowId,
        _device_id: DeviceId,
        _scan_code: keyboard::ScanCode,
        _key_code: Option<keyboard::KeyCode>,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn on_fixed_update(&mut self, _dt: std::time::Duration) -> Result<(), Error> {
        Ok(())
    }

    fn on_variable_update(&mut self, _dt: std::time::Duration) -> Result<(), Error> {
        Ok(())
    }
}
