extern crate winit;

use std::collections::BTreeMap;

use crate::{
    event::{
        keyboard::ScanCode, ControlFlow, DeviceEvent, DeviceId, ElementState, Event, EventHandler,
        EventLoop, WindowEvent,
    },
    window::{PhysicalPosition, WindowId},
};

#[derive(Clone)]
pub struct Application<EventHandlerType, Error, CustomEvent>
where
    EventHandlerType: EventHandler<Error, CustomEvent> + 'static,
    Error: std::fmt::Display + std::error::Error + 'static,
    CustomEvent: 'static,
{
    keyboard_state: KeyboardState,
    fixed_update_period: std::time::Duration,
    variable_update_min_period: std::time::Duration,
    last_fixed_update_time: std::time::Instant,
    last_variable_update_time: std::time::Instant,
    p0: std::marker::PhantomData<Error>,
    p1: std::marker::PhantomData<CustomEvent>,
    p2: std::marker::PhantomData<EventHandlerType>,
}

impl<EventHandlerType, Error, CustomEvent> Application<EventHandlerType, Error, CustomEvent>
where
    EventHandlerType: EventHandler<Error, CustomEvent> + 'static,
    Error: std::fmt::Display + std::error::Error + 'static,
    CustomEvent: 'static,
{
    pub fn new(
        fixed_update_frequency_hz: u64,
        variable_update_max_frequency_hz: Option<u64>,
    ) -> Self {
        assert!(
            fixed_update_frequency_hz > 0,
            "The fixed update frequency must be higher than 0"
        );
        if let Some(v) = variable_update_max_frequency_hz {
            assert!(v > 0, "The variable update frequency must be higher than 0");
        }

        let fixed_update_period =
            std::time::Duration::from_secs_f64(1. / fixed_update_frequency_hz as f64);
        let variable_update_min_period = match variable_update_max_frequency_hz {
            Some(v) => std::time::Duration::from_secs_f64(1. / v as f64),
            None => std::time::Duration::from_secs_f64(0.),
        };
        let current_time = std::time::Instant::now();

        Self {
            keyboard_state: KeyboardState::new(),
            fixed_update_period,
            variable_update_min_period,
            last_fixed_update_time: current_time,
            last_variable_update_time: current_time,
            p0: std::marker::PhantomData,
            p1: std::marker::PhantomData,
            p2: std::marker::PhantomData,
        }
    }

    pub fn run(mut self) {
        let event_loop = EventLoop::<EventHandlerType::CustomEvent>::with_user_event();
        let mut event_handler = EventHandlerType::new(&event_loop)
            .expect("Failed to initialize the application event handler");

        let current_time = std::time::Instant::now();
        self.last_fixed_update_time = current_time;
        self.last_variable_update_time = current_time;

        event_loop.run(move |event, _, control_flow| {
            match self.handle_event(&mut event_handler, event) {
                Ok(flow) => match flow {
                    ControlFlow::Continue => *control_flow = winit::event_loop::ControlFlow::Poll,
                    ControlFlow::Exit => *control_flow = winit::event_loop::ControlFlow::Exit,
                },
                Err(e) => {
                    event_handler.on_error(e);
                    *control_flow = winit::event_loop::ControlFlow::Exit;
                }
            }
        });
    }

    fn handle_event(
        &mut self,
        eh: &mut EventHandlerType,
        event: Event<EventHandlerType::CustomEvent>,
    ) -> Result<ControlFlow, EventHandlerType::Error> {
        match event {
            Event::NewEvents(start_cause) => eh.on_new_events(start_cause),

            Event::UserEvent(event) => eh.on_custom_event(event),

            Event::Suspended => eh.on_suspended(),

            Event::Resumed => eh.on_resumed(),

            Event::MainEventsCleared => self.update(eh),

            Event::RedrawRequested(window_id) => eh.on_redraw_requested(window_id),

            Event::RedrawEventsCleared => eh.on_redraw_events_cleared(),

            Event::LoopDestroyed => eh.on_event_loop_destroyed(),

            Event::WindowEvent { window_id, event } => match event {
                WindowEvent::CloseRequested => eh.on_close_requested(window_id),

                WindowEvent::Destroyed => eh.on_destroyed(window_id),

                WindowEvent::Focused(focused) => {
                    if focused {
                        eh.on_focus_gained(window_id)
                    } else {
                        eh.on_focus_lost(window_id)
                    }
                }

                WindowEvent::Resized(size) => eh.on_resized(window_id, size),

                WindowEvent::ScaleFactorChanged {
                    scale_factor,
                    new_inner_size,
                } => eh.on_scale_factor_changed(window_id, scale_factor, new_inner_size),

                WindowEvent::Moved(pos) => eh.on_moved(window_id, pos),

                WindowEvent::ReceivedCharacter(c) => eh.on_received_character(window_id, c),

                WindowEvent::DroppedFile(path) => eh.on_hovered_file_dropped(window_id, path),

                WindowEvent::HoveredFile(path) => eh.on_hovered_file_entered(window_id, path),

                WindowEvent::HoveredFileCancelled => eh.on_hovered_file_left(window_id),

                WindowEvent::KeyboardInput {
                    device_id,
                    input,
                    is_synthetic,
                } => {
                    let last_key_state =
                        self.keyboard_state
                            .key_state(Some(window_id), device_id, input.scancode);
                    let is_repeat = *last_key_state == input.state;
                    *last_key_state = input.state;
                    match input.state {
                        ElementState::Pressed => eh.on_key_pressed(
                            window_id,
                            device_id,
                            input.scancode,
                            input.virtual_keycode,
                            is_synthetic,
                            is_repeat,
                        ),
                        ElementState::Released => eh.on_key_released(
                            window_id,
                            device_id,
                            input.scancode,
                            input.virtual_keycode,
                            is_synthetic,
                        ),
                    }
                }

                WindowEvent::ModifiersChanged(mods) => eh.on_modifiers_changed(window_id, mods),

                WindowEvent::CursorMoved {
                    device_id,
                    position,
                    ..
                } => eh.on_cursor_moved(window_id, device_id, position),

                WindowEvent::CursorEntered { device_id } => {
                    eh.on_cursor_entered(window_id, device_id)
                }

                WindowEvent::CursorLeft { device_id } => eh.on_cursor_left(window_id, device_id),

                WindowEvent::MouseInput {
                    device_id,
                    state,
                    button,
                    ..
                } => match state {
                    ElementState::Pressed => {
                        eh.on_mouse_button_pressed(window_id, device_id, button)
                    }
                    ElementState::Released => {
                        eh.on_mouse_button_released(window_id, device_id, button)
                    }
                },

                WindowEvent::MouseWheel {
                    device_id,
                    delta,
                    phase,
                    ..
                } => eh.on_scroll(window_id, device_id, delta, phase),

                WindowEvent::Touch(touch) => eh.on_touch(
                    window_id,
                    touch.device_id,
                    touch.phase,
                    touch.location,
                    touch.force,
                    touch.id,
                ),

                WindowEvent::AxisMotion {
                    device_id,
                    axis,
                    value,
                } => eh.on_axis_moved(window_id, device_id, axis, value),

                // Not universally supported.
                WindowEvent::TouchpadPressure { .. } => Ok(ControlFlow::Continue),

                // Not universally supported.
                WindowEvent::ThemeChanged(_) => Ok(ControlFlow::Continue),
            },

            Event::DeviceEvent { device_id, event } => match event {
                DeviceEvent::Added => eh.on_device_added(device_id),

                DeviceEvent::Removed => eh.on_device_removed(device_id),

                DeviceEvent::MouseMotion { delta } => {
                    eh.on_device_cursor_moved(device_id, PhysicalPosition::new(delta.0, delta.1))
                }

                DeviceEvent::MouseWheel { delta } => eh.on_device_scroll(device_id, delta),

                DeviceEvent::Motion { axis, value } => {
                    eh.on_device_axis_moved(device_id, axis, value)
                }

                DeviceEvent::Button { button, state } => match state {
                    ElementState::Pressed => eh.on_device_button_pressed(device_id, button),
                    ElementState::Released => eh.on_device_button_released(device_id, button),
                },

                DeviceEvent::Key(input) => {
                    let last_key_state =
                        self.keyboard_state
                            .key_state(None, device_id, input.scancode);
                    let is_repeat = *last_key_state == input.state;
                    *last_key_state = input.state;
                    match input.state {
                        ElementState::Pressed => eh.on_device_key_pressed(
                            device_id,
                            input.scancode,
                            input.virtual_keycode,
                            is_repeat,
                        ),
                        ElementState::Released => eh.on_device_key_released(
                            device_id,
                            input.scancode,
                            input.virtual_keycode,
                        ),
                    }
                }

                DeviceEvent::Text { codepoint } => eh.on_device_text(device_id, codepoint),
            },
        }
    }

    fn update(
        &mut self,
        eh: &mut EventHandlerType,
    ) -> Result<ControlFlow, EventHandlerType::Error> {
        let current_time = std::time::Instant::now();

        while current_time - self.last_fixed_update_time >= self.fixed_update_period {
            match eh.on_fixed_update(self.fixed_update_period) {
                Ok(v) => match v {
                    ControlFlow::Exit => return Ok(ControlFlow::Exit),
                    _ => (),
                },
                Err(e) => return Err(e),
            };
            self.last_fixed_update_time += self.fixed_update_period;
        }

        let time_since_last_variable_update = current_time - self.last_variable_update_time;
        if time_since_last_variable_update > self.variable_update_min_period {
            match eh.on_variable_update(time_since_last_variable_update) {
                Ok(v) => match v {
                    ControlFlow::Exit => return Ok(ControlFlow::Exit),
                    _ => (),
                },
                Err(e) => return Err(e),
            }
            self.last_variable_update_time = current_time;
        }

        eh.on_main_events_cleared()
    }
}

#[derive(Clone)]
struct KeyboardState {
    state: BTreeMap<(Option<WindowId>, DeviceId), [ElementState; 128]>,
}

impl KeyboardState {
    pub fn new() -> Self {
        Self {
            state: BTreeMap::new(),
        }
    }

    pub fn key_state(
        &mut self,
        window_id: Option<WindowId>,
        device_id: DeviceId,
        scan_code: ScanCode,
    ) -> &mut ElementState {
        let key = (window_id, device_id);
        if !self.state.contains_key(&key) {
            self.state.insert(key, [ElementState::Released; 128]);
        }
        // Guaranteed to succeed due to the previous lines.
        let keyboard_state = self.state.get_mut(&key).unwrap();
        // Assuming at most a certain number of scancodes. It should be enough.
        // Asserting just for safety.
        let key_idx = scan_code as usize;
        assert!(
            key_idx < keyboard_state.len(),
            "Invalid scan code {}",
            key_idx
        );
        &mut keyboard_state[key_idx]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Clone, Copy)]
    enum MyError {}

    impl std::fmt::Display for MyError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "MyError")
        }
    }

    impl std::error::Error for MyError {
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
            None
        }
    }

    #[derive(Debug)]
    struct MyEventHandler {}

    impl EventHandler<MyError, ()> for MyEventHandler {
        type Error = MyError;
        type CustomEvent = ();

        fn new(_: &EventLoop<()>) -> Result<Self, Self::Error> {
            Ok(Self {})
        }

        fn on_fixed_update(&mut self, _: std::time::Duration) -> Result<ControlFlow, Self::Error> {
            Ok(ControlFlow::Exit)
        }
    }

    #[test]
    fn application_creation() {
        let _app = Application::<MyEventHandler, _, _>::new(10, Some(10));
    }

    #[test]
    fn run() {
        Application::<MyEventHandler, _, _>::new(10, Some(10)).run();
    }
}
