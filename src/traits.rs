use std::io;

pub trait Mounting {
    fn mount(self, f: &str);
    fn remount(self) -> io::Result<()>;
}
pub trait State {
    fn reboot(&mut self);
}
pub trait InitSystem: State {
    fn init(self, d: &str) -> io::Result<()>;
}
