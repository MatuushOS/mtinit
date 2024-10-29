use std::io;
use serde::{Deserialize, Serialize};
use crate::res;
use crate::traits::Mounting;
use rustix::mount::MountFlags;
#[derive(Serialize, Deserialize, Default)]
pub struct FsTab(Vec<BetterFsTab>);
#[derive(Serialize, Deserialize)]
struct BetterFsTab {
	source: String,
	target: String,
	fstype: String,
	flags: Vec<String>,
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
			let dev = rustix::mount::mount(
				dev.source.into(),
				dev.target.into(),
				dev.fstype.into(),
				dev.flags.into(),
				dev.data.into(),
			);
			res!(dev);
		}
	}
	fn remount(self) -> io::Result<()> {
		Ok(for &tab in &self.0 {
			rustix::mount::mount_remount(tab.target.into(), tab.flags.into(), tab.data.into())?
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
