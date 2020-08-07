use crate::event::{ControlFlow, ElementState, Event, EventHandler, EventLoop, WindowEvent};

pub struct Application<EventHandlerType, Error, CustomEvent>
where
    EventHandlerType: EventHandler<Error, CustomEvent> + 'static,
    Error: std::fmt::Display + std::error::Error + 'static,
    CustomEvent: 'static,
{
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
        let event_loop = EventLoop::<CustomEvent>::with_user_event();
        let mut event_handler = EventHandlerType::new(&event_loop)
            .expect("Failed to initialize the application event handler");

        let current_time = std::time::Instant::now();
        self.last_fixed_update_time = current_time;
        self.last_variable_update_time = current_time;

        event_loop.run(move |event, _, control_flow| {
            self.run_frame(&mut event_handler, event)
                .expect("The application shut down due to an error");
            if event_handler.is_close_requested() {
                *control_flow = ControlFlow::Exit;
            }
        });
    }

    fn run_frame(
        &mut self,
        event_handler: &mut EventHandlerType,
        event: Event<CustomEvent>,
    ) -> Result<(), Error> {
        Self::handle_event(event, event_handler)?;

        let current_time = std::time::Instant::now();

        while current_time - self.last_fixed_update_time >= self.fixed_update_period {
            event_handler.on_fixed_update(self.fixed_update_period)?;
            self.last_fixed_update_time += self.fixed_update_period;
        }

        let time_since_last_variable_update = current_time - self.last_variable_update_time;
        if time_since_last_variable_update > self.variable_update_min_period {
            event_handler.on_variable_update(time_since_last_variable_update)?;
            self.last_variable_update_time = current_time;
        }

        Ok(())
    }

    fn handle_event(
        event: Event<CustomEvent>,
        event_handler: &mut EventHandlerType,
    ) -> Result<(), Error> {
        match event {
            Event::WindowEvent { window_id, event } => match event {
                WindowEvent::CloseRequested => {
                    event_handler.on_close_requested(window_id)?;
                }

                WindowEvent::Focused(focused) => {
                    if focused {
                        event_handler.on_focus_gained(window_id)?;
                    } else {
                        event_handler.on_focus_lost(window_id)?;
                    }
                }

                WindowEvent::KeyboardInput {
                    device_id, input, ..
                } => match input.state {
                    ElementState::Pressed => event_handler.on_key_pressed(
                        window_id,
                        device_id,
                        input.scancode,
                        input.virtual_keycode,
                    )?,
                    ElementState::Released => event_handler.on_key_released(
                        window_id,
                        device_id,
                        input.scancode,
                        input.virtual_keycode,
                    )?,
                },

                _ => (),
            },

            _ => (),
        }
        Ok(())
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

    struct MyEventHandler {}

    impl EventHandler<MyError, ()> for MyEventHandler {
        fn new(_: &EventLoop<()>) -> Result<Self, MyError> {
            Ok(Self {})
        }
    }

    #[test]
    fn application_creation() {
        let _app = Application::<MyEventHandler, MyError, ()>::new(10, Some(10));
    }
}
