mod cfg;
mod traits;
mod fstab;

use crate::{
    cfg::{Cfg, res},
    traits::Mounting,
    fstab::FsTab
};
use clap::Parser;
use log::info;
use std::path::Path;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(long = "init", default_value = "/mtos/init")]
    /// location to initialize scripts
    location: Option<String>,
    #[clap(long = "rfstab")]
    /// Whether to reload the system fstab or not.
    /// Equivalent to `systemctl daemon-reload`.
    reload_fstab: Option<bool>,
    #[clap(long)]
    generate: Option<String>,
}
fn main() -> std::io::Result<()> {
    colog::init();
    if !Path::new("/mtos/fstab.yml").exists() {
        std::fs::File::create("/mtos/fstab.yml")?;
        std::fs::write(
            "/mtos/fstab.yml",
            serde_yaml::to_string(&FsTab::default()).unwrap(),
        )?;
    }
    let a = Cli::parse();
    if a.location.is_none() {
        Cfg::new().load("/mtos/init");
    }
    if let Some(location) = a.generate {
        info!("Generating init file at {}", location);
        let cfg = serde_yaml::to_string::<Cfg>(&Cfg::new());
        std::fs::write(location, res!(cfg))?
    } else if let Some(reload_fstab) = a.reload_fstab {
        match reload_fstab {
            true => {
                let ftab = FsTab::new("/mtos/fstab.yml");
                ftab.remount()?;
            }
            false => info!("Not reloading"),
        };
    }
    Ok(())
}
