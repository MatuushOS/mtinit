use std::io;

pub trait Mounting {
    fn mount(self);
    fn remount(self) -> io::Result<()>;
}
pub trait State {
    fn state(&mut self);
}
pub trait InitSystem: State {
    fn init(self, d: &str) -> io::Result<Self> where Self: Sized;
}
