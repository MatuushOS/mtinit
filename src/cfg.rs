use crate::traits::{Parsing, State};
use log::{info, trace};
use serde::{Deserialize, Serialize};
use std::env::temp_dir;
use std::fs::read_dir;
use std::path::Path;
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
}

impl State for Cfg {
    fn reboot(&mut self) {
        match self.when {
            When::OnReboot => {
                std::process::exit(0);
            }
            When::OnShutdown => {
                std::process::exit(0);
            }
            When::OnHibernate => {
                std::thread::sleep(std::time::Duration::MAX);
                std::fs::write(Path::new(temp_dir().as_path()), "sleep").unwrap();
                todo!()
            }
            When::Immediately => {
                for cmd in &self.script {
                    info!("Spawning {:?}", cmd);
                    let mut c = std::process::Command::new(&cmd.path)
                        .args(&cmd.args)
                        .spawn()
                        .unwrap();
                    while c.wait().unwrap().success() {
                        std::fs::write(
                            "/tmp/init.log",
                            trace!("{:#?}", Ok(c.stdout.take())),
                        )
                        .unwrap()
                    }
                }
            }
        }
    }
}

impl Parsing for Cfg {
    fn parse(self, f: &str) {
        todo!()
    }
}

impl crate::traits::InitSystem for Cfg {
    fn init(self, d: &str) -> std::io::Result<()> {
        for ent in read_dir(d)? {
            let entry = ent?;
            let f = std::fs::read_to_string(entry.path())?;
            let cfg: Self = serde_yaml::from_str(&f).unwrap();
        }
        Ok(())
    }
}
impl Into<MountFlags> for FsTab {
    fn into(self) -> MountFlags {
        for i in self.0.iter() {
            for flags in &i.flags {
                flags.parse();
            }
        }
    }
}
impl Parsing for FsTab {
    fn parse(self, f: &str) {
        let file = std::fs::read_to_string(f).unwrap();
        let cfg: FsTab = serde_yaml::from_str(&file).unwrap();
        for dev in cfg.0.into_iter() {
            rustix::mount::mount();
        }
    }
}
