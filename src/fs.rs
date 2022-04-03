use std::env;
use std::path::Path;

pub struct Exists {
    pub path: bool,
    pub git: bool,
}

impl Exists {
    pub fn new(path: &str) -> Self {
        Self {
            path: Path::new(path).exists(),
            git: Path::new(&format!("{path}/.git")).exists(),
        }
    }
}

pub struct Manager;

#[cfg(target_family = "windows")]
impl Manager {
    pub fn data() -> anyhow::Result<String> {
        let local = env::var("localappdata")?;
        Ok(format!("{local}/nvim-data"))
    }
    pub fn config() -> anyhow::Result<String> {
        let xdg = env::var("xdg_config_home");
        if let Ok(dir) = xdg {
            Ok(format!("{dir}/nvim"))
        } else {
            let local = env::var("localappdata")?;
            Ok(format!("{local}/nvim"))
        }
    }
    pub fn cache() -> anyhow::Result<String> {
        let roam = env::var("appdata")?;
        Ok(format!("{roam}/pnp"))
    }
}

#[cfg(target_family = "unix")]
impl Manager {
    pub fn data() -> anyhow::Result<String> {
        let share = shellexpand::tilde("~/.local/share");
        Ok(format!("{share}/nvim"))
    }
    pub fn config() -> anyhow::Result<String> {
        let xdg = env::var("xdg_config_home");
        if let Ok(dir) = xdg {
            Ok(format!("{dir}/nvim"))
        } else {
            let config = shellexpand::tilde("~/.config");
            Ok(format!("{config}/nvim"))
        }
    }
    pub fn cache() -> anyhow::Result<String> {
        let cache = shellexpand::tilde("~/.cache");
        Ok(format!("{cache}/pnp"))
    }
}
