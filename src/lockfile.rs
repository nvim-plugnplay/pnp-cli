use crate::data;
use serde::{ Serialize, Deserialize };
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct Lock(Vec<PlugItem>);
#[derive(Debug, Serialize, Deserialize)]
struct PlugItem {
    name: String,
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
    async fn new(name: String, value: data::PluginValue) -> anyhow::Result<Self> {
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
            let previous_lockfile = Lock::load();
            match branch {
                Some(_) => (),
                None => {
                    let mut was_generated = false;
                    if let Ok(lock) = previous_lockfile {
                        for item in lock.0 {
                            if item.name == name && item.branch.is_some() {
                                was_generated = true;
                                branch = item.branch;
                                break
                            }
                        }

                    }
                    if !was_generated {
                        branch = location.branch(name.clone()).await?;
                    }
                }

            }
        }
        Ok(Self {
            name,
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
        let mut plugitems: Vec<PlugItem> = Vec::new();
        let cfg = data::ConfigStructure::new()?;
        let plugins = cfg.plugins;
        for (name, plugin_value) in plugins {
            plugitems.push(PlugItem::new(name, plugin_value).await?);
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

