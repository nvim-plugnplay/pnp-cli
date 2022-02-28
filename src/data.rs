use anyhow::Context;
use serde::Deserialize;
use std::collections::HashMap;

use crate::fs;
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
        println!("Installing from {}", self.get());
        match self {
            Self::GitHub(repo) => {
                let dir = crate::git::append_to_data(&format!("/site/pack/pnp/opt/{name}"));
                let exists = fs::Exists::new(&dir);
                if exists.path && exists.git {
                    println!("{name} is already installed!");
                } else {
                    let url = "https://github.com/".to_string() + repo;
                    crate::git::clone(url, name).await?;
                }
            }
            Self::Remote(link) => {
                let dir = crate::git::append_to_data(&format!("/site/pack/pnp/opt/{name}"));
                let exists = fs::Exists::new(&dir);
                if exists.path && exists.git {
                    println!("{name} is already installed!");
                } else {
                    crate::git::clone(link.into(), name).await?;
                }
            }
            _ => (),
        }
        Ok(())
    }

    pub async fn update(&self, name: String) -> anyhow::Result<()> {
        match self {
            Self::GitHub(repo) => {
                let dir = crate::git::append_to_data(&format!("/site/pack/pnp/opt/{name}"));
                let exists = fs::Exists::new(&dir);
                if !exists.path {
                    let url = "https://github.com/".to_string() + repo;
                    crate::git::clone(url, name).await?;
                } else if !exists.git {
                    unimplemented!(".git does not exist");
                } else {
                    crate::git::update(name).await?;
                }
            }
            Self::Remote(url) => {
                let dir = crate::git::append_to_data(&format!("/site/pack/pnp/opt/{name}"));
                let exists = fs::Exists::new(&dir);
                if !exists.path {
                    crate::git::clone(url.into(), name).await?;
                } else if !exists.git {
                    unimplemented!(".git does not exist");
                } else {
                    crate::git::update(name).await?;
                }
            }
            _ => (),
        }

        Ok(())
    }
}
