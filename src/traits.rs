use std::io;
pub trait Parsing {
    fn parse(self, f: &str);
}
pub trait State {
    fn reboot(&mut self);
}
pub trait InitSystem: State + Parsing {
    fn init(self, d: &str) -> io::Result<()>;
}
