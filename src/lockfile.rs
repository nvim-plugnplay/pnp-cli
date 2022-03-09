use crate::data;
use serde::Serialize;
use std::fs;

#[derive(Serialize)]
pub struct Lock(Vec<PlugItem>);
#[derive(Debug, Serialize)]
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
    fn new(name: String, value: data::PluginValue) -> Self {
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
        Self {
            name,
            location,
            branch,
            commit_hash,
            pinned_commit_hash,
            pin,
            version,
            lazy_load,
            configuration,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
enum ConfigType {
    Chunk(String),
    Module(String),
}

impl Lock {
    pub fn new() -> anyhow::Result<Self> {
        let mut plugitems: Vec<PlugItem> = Vec::new();
        let cfg = data::ConfigStructure::new()?;
        let plugins = cfg.plugins;
        for (name, plugin_value) in plugins {
            plugitems.push(PlugItem::new(name, plugin_value));
        }

        Ok(Self(plugitems))
    }

    pub fn generate(&self) -> anyhow::Result<()> {
        let lockfile = fs::File::create("./pnp.lock.json")?;
        serde_json::to_writer_pretty(lockfile, &self)?;
        Ok(())
    }
}
