extern crate winit;

#[cfg(target_os = "windows")]
pub use winit::platform::windows::EventLoopExtWindows as EventLoopExt;

#[cfg(target_os = "linux")]
pub use winit::platform::unix::EventLoopExtUnix as EventLoopExt;

#[cfg(not(any(target_os = "linux", target_os = "windows")))]
pub use EventLoop as EventLoopExt;

pub trait EventLoopAnyThread<T: 'static> {
    fn new_any_thread() -> winit::event_loop::EventLoop<T>;
}

impl<T> EventLoopAnyThread<T> for winit::event_loop::EventLoop<T>
where
    T: 'static,
{
    #[cfg(any(target_os = "linux", target_os = "windows"))]
    fn new_any_thread() -> winit::event_loop::EventLoop<T> {
        <winit::event_loop::EventLoop<T> as EventLoopExt>::new_any_thread()
    }

    #[cfg(not(any(target_os = "linux", target_os = "windows")))]
    fn new_any_thread<T: 'static>() -> winit::event_loop::EventLoop<T> {
        winit::event_loop::EventLoop::new()
    }
}
