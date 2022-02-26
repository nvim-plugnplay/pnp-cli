use std::fs::File;
use std::io::prelude::*;
use crate::data::*;

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
            PluginValue::ShortHand(loc) => {
                loc
            },
            PluginValue::Verbose(verbose) => {
                verbose.plugin_location
            }
        };
        let parsed_location = Location::new(location).expect("Unknown format of plugin_location");
        println!("Installing from {}", parsed_location.get());
        parsed_location.install(name).await?;
        println!();
    }

    Ok(())
}


