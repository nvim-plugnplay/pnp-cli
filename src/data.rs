use anyhow::Context;
use serde::Deserialize;
use std::collections::HashMap;

use std::process::Stdio;
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::Command,
};

use std::fs::File;
use std::io::{self, prelude::*};

#[derive(Deserialize, Debug)]
pub struct ConfigStructure {
    pub plugnplay: HashMap<String, String>,
    pub plugins: HashMap<String, PluginValue>,
}
#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum PluginValue {
    ShortHand(String),
    Verbose(PluginVerbose),
}
#[derive(Deserialize, Debug)]
pub struct PluginVerbose {
    pub plugin_location: String,
}

impl ConfigStructure {
    pub fn new() -> anyhow::Result<Self> {
        let config_file = File::open("./cfg.jsonc")?;
        let mut reader = io::BufReader::new(config_file);
        let mut buffer = String::new();
        reader.read_to_string(&mut buffer)?;
        let stripped = json_comments::StripComments::new(buffer.as_bytes());
        let parsed = serde_json::from_reader(stripped);
        parsed.context("Failed to parse cfg.jsonc")
    }
}

async fn clone_repository(url: String, dir_name: String) -> anyhow::Result<()> {
    #[cfg(target_family = "windows")]
    let dir = format!(
        "{}/nvim-data/site/pack/pnp/{}",
        dirs::data_local_dir().unwrap().to_str().unwrap(),
        dir_name
    );
    #[cfg(target_family = "unix")]
    let dir = format!(
        "{}/nvim/site/pack/pnp/{}",
        dirs::data_local_dir().unwrap().to_str().unwrap(),
        dir_name
    );
    let mut cmd = Command::new("git");
    cmd.args(&["clone", &url, "--depth=1", &dir])
        .stdout(Stdio::piped());
    println!(
        "Command: {}",
        format!("git clone {} --depth=1 {}", &url, &dir)
    );
    let mut child = cmd.spawn()?;
    let stdout = child.stdout.take().unwrap();
    let mut reader = BufReader::new(stdout).lines();
    tokio::spawn(async move {
        let _ = child.wait().await;
    });

    while let Some(line) = reader.next_line().await? {
        println!("{line}");
    }
    Ok(())
}

pub enum Location {
    GitHub(String),
    Remote(String),
    Local(String),
}

impl Location {
    pub fn new(location: String) -> Option<Self> {
        match &location[0..3] {
            "gh:" => Some(Self::GitHub(location[3..].to_string())),
            "git" => Some(Self::Remote(location[4..].to_string())),
            "loc" => Some(Self::Local(location[4..].to_string())),
            &_ => None,
        }
    }
    pub fn get(&self) -> &str {
        match self {
            Self::GitHub(val) | Self::Local(val) | Self::Remote(val) => val,
        }
    }
    pub async fn install(&self, name: String) -> anyhow::Result<()> {
        match self {
            Self::GitHub(repo) => {
                let url = "https://github.com/".to_string() + &repo;
                clone_repository(url, name).await?;
            }
            _ => (),
        }
        Ok(())
    }
}
