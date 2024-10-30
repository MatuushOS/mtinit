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
impl Mounting for FsTab {
    fn mount(self) {
        for dev in &self.0 {
            let tab = rustix::mount::mount(
                &dev.source,
                &dev.target,
                &dev.fstype,
                MountFlags::RDONLY,
                &dev.data,
            );
            res!(tab);
        }
    }
    fn remount(self) -> io::Result<()> {
        Ok(for tab in &self.0 {
            rustix::mount::mount_remount(&tab.target, MountFlags::RDONLY, &tab.data)?;
        })
    }
}
impl FsTab {
    pub fn new(fstab: &str) -> Self {
        let fst = std::fs::read_to_string(fstab).unwrap();
        let cfg: Self = serde_yaml::from_str(&fst).unwrap();
        Self(cfg.0)
    }
    pub(crate) fn generate() -> Self {
        Self {
            0: vec![BetterFsTab {
                source: "".to_string(),
                target: "".to_string(),
                fstype: "".to_string(),
                data: "".to_string(),
            }],
        }
    }
}
