pub trait EventHandler<Error, Event>
where
    Error: std::fmt::Display + std::error::Error + 'static,
    Event: 'static,
{
}
