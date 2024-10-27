mod cfg;
mod traits;

use clap::Parser;
use log::info;
use std::path::Path;
use crate::cfg::Cfg;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(long = "init", default_value = "/mtos/init")]
    /// location to initialize scripts
    location: Option<String>,
    #[clap(long)]
    /// Whether to reload the system or not
    reload: Option<bool>,
    #[clap(long)]
    generate: Option<String>,
}
fn main() -> std::io::Result<()> {
    colog::init();
    if !Path::new("/mtos/fstab.yml").exists() {
        std::fs::File::create("/mtos/fstab.yml")?;
        std::fs::write("/mtos/fstab.yml", serde_yaml::to_string(&cfg::FsTab::default()).unwrap())?;
    }
    let a = Cli::parse();
    if let Some(location) = a.generate {
        info!("Generating init file at {}", location);
        Ok(std::fs::write(
            location,
            serde_yaml::to_string::<Cfg>(&Cfg::new()).unwrap(),
        )?)
    } else if let Some(reload) = a.reload {
        Ok(if reload {
            let f = std::fs::read_to_string("/mtos/fstab.yml")?;
        } else {
        })
    }
}
