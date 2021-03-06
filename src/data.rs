use anyhow::{Context, Error};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::fs;
use crate::git;
use crate::symlink;
use std::fmt;
use std::fs::File;
use std::io::{self, prelude::*};

#[derive(Deserialize, Debug)]
pub struct ConfigStructure {
    pub plugnplay: BTreeMap<String, String>,
    pub plugins: BTreeMap<String, PluginValue>,
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
    pub config: Option<String>,
    pub config_file: Option<String>,
    pub load: Option<LazyLoad>,
    pub version: Option<String>,
    pub branch: Option<String>,
    pub commit: Option<String>,
    pub pin: Option<bool>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum LazyLoadValue {
    Simple(String),
    Long(Vec<String>),
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LazyLoad {
    pub with: Option<LazyLoadValue>,
    pub command: Option<LazyLoadValue>,
    pub keybind: Option<LazyLoadValue>,
    pub event: Option<LazyLoadValue>,
    pub filetype: Option<LazyLoadValue>,
    pub module: Option<LazyLoadValue>,
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
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
    fn url(&self) -> anyhow::Result<String> {
        match self {
            Self::GitHub(repo) => Ok(format!("https://github.com/{repo}")),
            Self::Remote(link) => Ok(link.to_string()),
            _ => anyhow::private::Err(Error::msg("Unknown link format")),
        }
    }
    pub async fn commit_hash(&self, name: String) -> anyhow::Result<Option<String>> {
        Ok(match self {
            Self::GitHub(_) | Self::Remote(_) => Some(git::commit_hash(name).await?),
            _ => None,
        })
    }
    pub async fn branch(&self, name: String) -> anyhow::Result<Option<String>> {
        Ok(match self {
            Self::GitHub(_) | Self::Remote(_) => Some(git::branch(name).await?),
            _ => None,
        })
    }
    pub fn is_git(&self) -> bool {
        match self {
            Self::GitHub(_) | Self::Remote(_) => true,
            _ => false,
        }
    }
    fn sym_path(&self, name: String) -> Option<String> {
        match self {
            Self::Local(_) => Some(git::append_to_data(&format!("/site/pack/pnp/opt/{name}"))),
            _ => None,
        }
    }
    // TODO: key is the same, value is different
    pub async fn install(&self, name: String) -> anyhow::Result<()> {
        let dir = crate::git::append_to_data(&format!("/site/pack/pnp/opt/{name}"));
        let exists = fs::Exists::new(&dir);
        if exists.path {
            Ok(())
        } else if let Self::Local(path) = self {
            let target = self.sym_path(name).unwrap();
            symlink::SymLink::new(path.into(), target).create().await?;
            Ok(())
        } else {
            let url = self.url()?;
            if exists.git {
                Ok(())
            } else {
                crate::git::clone(url, name).await?;
                Ok(())
            }
        }
    }

    pub async fn update(&self, name: String) -> anyhow::Result<()> {
        if let Self::Local(_) = self {
            return Ok(());
        }
        let url = self.url()?;
        let dir = crate::git::append_to_data(&format!("/site/pack/pnp/opt/{name}"));
        let exists = fs::Exists::new(&dir);
        if !exists.path {
            crate::git::clone(url, name).await?;
        } else if !exists.git {
            unimplemented!(".git does not exist");
        } else {
            crate::git::update(name).await?;
        }

        Ok(())
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::GitHub(data) | Self::Remote(data) | Self::Local(data) => {
                write!(f, "{data}")
            }
        }
    }
}
