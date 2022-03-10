use crate::data;
use serde::{Deserialize, Serialize};
use std::fs;
use std::collections::BTreeMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Lock(BTreeMap<String, PlugItem>);
#[derive(Debug, Serialize, Deserialize)]
struct PlugItem {
    location: data::Location,
    branch: Option<String>,
    commit_hash: Option<String>,
    pinned_commit_hash: Option<String>,
    pin: bool,
    version: Option<String>,
    lazy_load: Option<data::LazyLoad>,
    configuration: Option<ConfigType>,
}

macro_rules! init {
    ( $( $v:ident, $t:ty ), * ) => {
        $(
            #[allow(unused_mut)]
            let mut $v: Option<$t> = None;
        )*
    }
}

// TODO: get commit hash from installed plugin
impl PlugItem {
    async fn new(name: String, value: data::PluginValue, previous_lockfile: &anyhow::Result<Lock>) -> anyhow::Result<Self> {
        init! {
            branch, String,
            commit_hash, String,
            pinned_commit_hash, String,
            version, String,
            lazy_load, data::LazyLoad,
            configuration, ConfigType
        }
        let mut pin = false;
        let location = match value {
            data::PluginValue::ShortHand(loc_string) => data::Location::new(loc_string).unwrap(),
            data::PluginValue::Verbose(verbose) => {
                branch = verbose.branch;
                pinned_commit_hash = verbose.commit;
                pin = verbose.pin.unwrap_or(false);
                version = verbose.version;
                lazy_load = verbose.load;
                configuration = {
                    if let Some(chunk) = verbose.config {
                        Some(ConfigType::Chunk(chunk))
                    } else {
                        verbose.config_file.map(ConfigType::Module)
                    }
                };

                data::Location::new(verbose.plugin_location).unwrap()
            }
        };
        if location.is_git() {
            commit_hash = location.commit_hash(name.clone()).await?;
            match branch {
                Some(_) => (),
                None => {
                    if let Ok(lock) = previous_lockfile {
                        match &lock.0.get(&name) {
                            Some(item) => {
                                if item.branch.is_some() {
                                    branch = item.branch.clone();
                                }
                            }
                            None => {
                                branch = location.branch(name.clone()).await?;
                            }
                        }
                    } else {
                        branch = location.branch(name.clone()).await?;
                    }
                }
            }
        }
        Ok(Self {
            location,
            branch,
            commit_hash,
            pinned_commit_hash,
            pin,
            version,
            lazy_load,
            configuration,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum ConfigType {
    Chunk(String),
    Module(String),
}

impl Lock {
    pub async fn new() -> anyhow::Result<Self> {
        let mut plugitems: BTreeMap<String, PlugItem> = BTreeMap::new();
        let cfg = data::ConfigStructure::new()?;
        let plugins = cfg.plugins;
        let previous_lockfile = Self::load();
        for (name, plugin_value) in plugins {
            plugitems.insert(name.clone(), PlugItem::new(name, plugin_value, &previous_lockfile).await?);
        }

        Ok(Self(plugitems))
    }

    pub fn generate(&self) -> anyhow::Result<()> {
        let lockfile = fs::File::create("./pnp.lock.json")?;
        serde_json::to_writer_pretty(lockfile, &self)?;
        Ok(())
    }

    pub fn load() -> anyhow::Result<Self> {
        let lockfile_raw = fs::File::open("./pnp.lock.json")?;
        Ok(serde_json::from_reader(lockfile_raw)?)
    }
}
