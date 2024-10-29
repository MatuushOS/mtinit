use std::ffi::OsStr;
use crate::res;
use crate::traits::Mounting;
use log::{error, info};
use rustix::mount::MountFlags;
use serde::{Deserialize, Serialize};
use std::io;
#[derive(Serialize, Deserialize, Default)]
pub struct FsTab(Vec<BetterFsTab>);
#[derive(Serialize, Deserialize)]
struct BetterFsTab {
    source: String,
    target: String,
    fstype: String,
    data: String,
}

impl Into<MountFlags> for BetterFsTab {
    fn into(self) -> MountFlags {
        MountFlags::all()
    }
}
impl Mounting for FsTab {
    fn mount(self, f: &str) {
        let file = std::fs::read_to_string(f).unwrap();
        let cfg: FsTab = serde_yaml::from_str(&file).unwrap();
        for &dev in &cfg.0 {
            if dev.target == "/mtos" || 
                dev.target == "/dev" || 
                dev.target == "/home" || 
                dev.target == "/tmp" {
                let dev = rustix::mount::mount(
                    dev.source.into(),
                    dev.target.into(),
                    dev.fstype.into(),
                    MountFlags::empty(),
                    dev.data.into(),
                );
                res!(dev);
            }
            else {
            let dev = rustix::mount::mount(
                dev.source.into(),
                dev.target.into(),
                dev.fstype.into(),
                MountFlags::RDONLY,
                dev.data.into(),
            );
            res!(dev);
        }
    }
    }
    fn remount(self) -> io::Result<()> {
        Ok(for tab in &self.0 {
                rustix::mount::mount_remount(OsStr::new(tab.target.as_str()), MountFlags::RDONLY, tab.data.clone())?;
        })
    }
}
impl FsTab {
    pub fn new(fstab: &str) -> Self {
        let fst = std::fs::read_to_string(fstab).unwrap();
        let cfg: Self = serde_yaml::from_str(&fst).unwrap();
        Self(cfg.0)
    }
}
