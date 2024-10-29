//! Module for configuration, fstab, etc... 
use crate::traits::{Mounting, State};
use log::{error, info};
use rustix::mount::MountFlags; // if this is highlighted, it's a bug in RustRover
use serde::{Deserialize, Serialize};
use std::{
    env::temp_dir,
    fs::read_dir,
    io,
    path::Path,
    process::Command,
};

macro_rules! script {
    ($script:expr) => {
        for script in &$script {
            info!("Starting script {}", script.name);
            let cmd = Command::new(script.path.clone())
                .args(&script.args)
                .output();
            res!(cmd);
        }
    };
}
#[macro_export]
macro_rules! res {
    ($v:expr) => {
        use log::{error, info};
        match $v {
            Ok(ok) => info!("{ok:#?}"),
            Err(e) => error!("{e:#?}"),
        }
    };
}

#[derive(Debug, Serialize, Deserialize)]
enum When {
    OnReboot,
    OnShutdown,
    OnHibernate,
    Immediately,
}
#[derive(Debug, Serialize, Deserialize)]
struct Script {
    name: String,
    path: String,
    args: Vec<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Cfg {
    imports: Option<Vec<String>>,
    name: String,
    when: When,
    script: Vec<Script>,
}
impl Cfg {
    pub fn new() -> Self {
        Self {
            imports: None,
            name: "".to_string(),
            when: When::OnReboot,
            script: vec![Script {
                name: "".to_string(),
                path: "".to_string(),
                args: vec![String::new()],
            }],
        }
    }
    pub fn load(&mut self, f: &str) -> Self {
        let file = std::fs::read_to_string(f).unwrap();
        let cfg: Self = serde_yaml::from_str(&file).unwrap();
        Self {
            imports: cfg.imports,
            name: cfg.name,
            when: cfg.when,
            script: cfg.script,
        }
    }
}
/// FSTab implementation
impl State for Cfg {
    fn reboot(&mut self) {
        match self.when {
            When::OnReboot => {
                script!(self.script);
                std::process::exit(0)
            }
            When::OnShutdown => {
                script!(self.script);
                std::process::exit(0);
            }
            When::OnHibernate => {
                std::thread::sleep(std::time::Duration::MAX);
                std::fs::write(Path::new(temp_dir().as_path()), "sleep").unwrap();
                script!(self.script);
            }
            When::Immediately => {
                script!(self.script);
            }
        }
    }
}
impl crate::traits::InitSystem for Cfg {
    fn init(self, d: &str) -> io::Result<()> {
        for ent in read_dir(d)? {
            let entry = ent?;
            let f = std::fs::read_to_string(entry.path())?;
            let cfg: Self = serde_yaml::from_str(&f).unwrap();
            info!("Starting service {}", cfg.name);
            for script in cfg.script {
                Command::new(&script.path).args(&script.args).spawn()?;
            }
        }
        Ok(())
    }
}
