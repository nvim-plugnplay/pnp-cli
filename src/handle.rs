use crate::data::*;
use std::fs::File;
use std::io::prelude::*;

const PLUGIN_CONTENT: &str = include_str!("../templates/plugin.json");
const CONFIG_CONTENT: &str = include_str!("../templates/cfg.jsonc");

/// `pnp init` logic
pub fn init(toggle: bool) -> anyhow::Result<()> {
    if toggle {
        let mut output = File::create("./plugin.json")?;
        write!(output, "{PLUGIN_CONTENT}")?;
    } else {
        let mut output = File::create("./cfg.jsonc")?;
        write!(output, "{CONFIG_CONTENT}")?;
    }
    Ok(())
}

/// `pnp install` logic
pub async fn install() -> anyhow::Result<()> {
    let parsed_contents = ConfigStructure::new()?;
    for (name, value) in parsed_contents.plugins {
        let location = match value {
            PluginValue::ShortHand(loc) => loc,
            PluginValue::Verbose(verbose) => verbose.plugin_location,
        };
        let parsed_location = Location::new(location).expect("Unknown format of plugin_location");
        parsed_location.install(name).await?;
        println!();
    }

    Ok(())
}

/// `pnp update` logic
pub async fn update(name: Option<&str>) -> anyhow::Result<()> {
    let parsed_contents = ConfigStructure::new()?;
    if let Some(dir_name) = name {
        let location = match &parsed_contents.plugins[dir_name] {
            PluginValue::ShortHand(loc) => loc,
            PluginValue::Verbose(verbose) => &verbose.plugin_location,
        };
        let parsed_location =
            Location::new(location.into()).expect("Unknown format of plugin_location");
        parsed_location.update(dir_name.to_string()).await?;
        println!()
    } else {
        for (name, value) in parsed_contents.plugins {
            let location = match value {
                PluginValue::ShortHand(loc) => loc,
                PluginValue::Verbose(verbose) => verbose.plugin_location,
            };
            let parsed_location =
                Location::new(location).expect("Unknown format of plugin_location");
            parsed_location.update(name).await?;
            println!();
        }
    }

    Ok(())
}
