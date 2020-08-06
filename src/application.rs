use crate::{ControlFlow, Event, EventHandler, EventLoop};

pub struct Application {}

impl Application {
    pub fn run<Error, CustomEvent, EventHandlerType>(
        fixed_update_frequency_hz: u64,
    ) -> Result<(), Error>
    where
        Error: std::fmt::Display + std::error::Error + 'static,
        CustomEvent: 'static,
        EventHandlerType: EventHandler<Error, CustomEvent> + 'static,
    {
        let event_loop = EventLoop::<CustomEvent>::with_user_event();
        let mut event_handler = EventHandlerType::new(&event_loop)?;

        let fixed_update_period =
            std::time::Duration::from_secs_f64(1. / fixed_update_frequency_hz as f64);

        let mut last_fixed_update_time = std::time::Instant::now();
        let mut last_variable_update_time = last_fixed_update_time;

        event_loop.run(move |event, _, control_flow| {
            if let Err(e) = Self::handle_event(event, &mut event_handler) {
                Self::report_error(e);
                *control_flow = ControlFlow::Exit;
            }
            let current_time = std::time::Instant::now();

            while current_time - last_fixed_update_time >= fixed_update_period {
                if let Err(e) = event_handler.on_fixed_update(fixed_update_period) {
                    Self::report_error(e);
                    *control_flow = ControlFlow::Exit;
                    return;
                }
                last_fixed_update_time += fixed_update_period;
            }

            if let Err(e) =
                event_handler.on_variable_update(current_time - last_variable_update_time)
            {
                Self::report_error(e);
                *control_flow = ControlFlow::Exit;
                return;
            }
            last_variable_update_time = current_time;

            if event_handler.is_close_requested() {
                *control_flow = ControlFlow::Exit;
                return;
            }
        });
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
