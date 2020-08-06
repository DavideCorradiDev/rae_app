use crate::{ControlFlow, Event, EventHandler, EventLoop};

pub struct Application {
    fixed_update_period: std::time::Duration,
    last_fixed_update_time: std::time::Instant,
    last_variable_update_time: std::time::Instant,
}

impl Application {
    pub fn new(fixed_update_frequency_hz: u64) -> Self {
        assert!(
            fixed_update_frequency_hz > 0,
            "The update frequency must be higher than 0"
        );
        let current_time = std::time::Instant::now();
        Self {
            fixed_update_period: std::time::Duration::from_secs_f64(
                1. / fixed_update_frequency_hz as f64,
            ),
            last_fixed_update_time: current_time,
            last_variable_update_time: current_time,
        }
    }

    pub fn run<Error, CustomEvent, EventHandlerType>(mut self)
    where
        Error: std::fmt::Display + std::error::Error + 'static,
        CustomEvent: 'static,
        EventHandlerType: EventHandler<Error, CustomEvent> + 'static,
    {
        let event_loop = EventLoop::<CustomEvent>::with_user_event();
        let mut event_handler = match EventHandlerType::new(&event_loop) {
            Ok(v) => v,
            Err(e) => {
                Self::report_error(e);
                return;
            }
        };

        let current_time = std::time::Instant::now();
        self.last_fixed_update_time = current_time;
        self.last_variable_update_time = current_time;

        event_loop.run(move |event, _, control_flow| {
            if let Err(e) = self.run_frame(&mut event_handler, event) {
                Self::report_error(e);
                *control_flow = ControlFlow::Exit;
            };
            if event_handler.is_close_requested() {
                *control_flow = ControlFlow::Exit;
            }
        });
    }

    fn run_frame<Error, CustomEvent, EventHandlerType>(
        &mut self,
        event_handler: &mut EventHandlerType,
        event: Event<CustomEvent>,
    ) -> Result<(), Error>
    where
        Error: std::fmt::Display + std::error::Error + 'static,
        CustomEvent: 'static,
        EventHandlerType: EventHandler<Error, CustomEvent> + 'static,
    {
        Self::handle_event(event, event_handler)?;

        let current_time = std::time::Instant::now();

        while current_time - self.last_fixed_update_time >= self.fixed_update_period {
            event_handler.on_fixed_update(self.fixed_update_period)?;
            self.last_fixed_update_time += self.fixed_update_period;
        }

        event_handler.on_variable_update(current_time - self.last_variable_update_time)?;
        self.last_variable_update_time = current_time;

        Ok(())
    }

    fn handle_event<Error, CustomEvent, EventHandlerType>(
        event: Event<CustomEvent>,
        event_handler: &mut EventHandlerType,
    ) -> Result<(), Error>
    where
        Error: std::fmt::Display + std::error::Error + 'static,
        CustomEvent: 'static,
        EventHandlerType: EventHandler<Error, CustomEvent> + 'static,
    {
        Ok(())
    }

    fn report_error<Error>(error: Error)
    where
        Error: std::fmt::Display + std::error::Error + 'static,
    {
        println!("The application shut down due to an error ({})", error);
    }
}
