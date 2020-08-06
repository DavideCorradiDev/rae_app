use crate::{EventHandler, EventLoop};

pub struct Application<Error, Event, EventHandlerType>
where
    Error: std::fmt::Display + std::error::Error + 'static,
    Event: 'static,
    EventHandlerType: EventHandler<Error, Event>,
{
    event_loop: EventLoop<Event>,
    event_handler: EventHandlerType,
    phantom: std::marker::PhantomData<Error>,
}

impl<Error, Event, EventHandlerType> Application<Error, Event, EventHandlerType>
where
    Error: std::fmt::Display + std::error::Error + 'static,
    Event: 'static,
    EventHandlerType: EventHandler<Error, Event>,
{
    pub fn new() -> Result<Self, Error> {
        let event_loop = EventLoop::<Event>::with_user_event();
        let event_handler = EventHandler::new(&event_loop)?;
        Ok(Self {
            event_loop,
            event_handler,
            phantom: std::marker::PhantomData,
        })
    }
}
