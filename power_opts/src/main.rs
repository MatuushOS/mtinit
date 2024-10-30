use init_system::res;
use log::{info, error};
use rustix::path::Arg;
use rustix::system::{reboot, RebootCommand};
fn main() {
    colog::init();
    let mut arge = std::env::args();
    match arge.nth(1).as_deref() {
        None => {
            info!("The reboot system");
            info!("reboot\tReboot the system");
            info!("shutdown\tShutdown the system");
        }
        Some(action) => match action.as_str() {
            Ok("reboot") => {
                info!("Rebooting");
                res!(reboot(RebootCommand::Restart));
            }
            Ok("poweroff") => {
                info!("Powering off");
                res!(reboot(RebootCommand::PowerOff));
            }
            _ => (),
        },
    }
}
