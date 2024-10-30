#![warn(clippy::all, clippy::pedantic, clippy::perf)]
mod cfg;
mod fstab;
mod traits;

use crate::{cfg::Cfg, fstab::FsTab, traits::Mounting};
use clap::Parser;
use log::{error, info};
use std::path::Path;
use crate::traits::{InitSystem, State};

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
            serde_yaml::to_string(&FsTab::generate()).unwrap(),
        )?;
    } else {
        FsTab::new("/mtos/fstab.yml").mount();
        let mut cfg = Cfg::new();
        cfg.clone().init("/mtos/init")?;
        cfg.state();
    }
    let a = Cli::parse();
    if a.location.is_none() {
        Cfg::new().init("/mtos/init")?;
    }
    if let Some(location) = a.generate {
        info!("Generating init file at {}", location);
        let cfg = serde_yaml::to_string::<Cfg>(&Cfg::new());
        res!(std::fs::write(location, cfg.unwrap()))
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
