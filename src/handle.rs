use crate::data::*;
use colored::*;
use regex::RegexSet;
use std::fs::File;
use std::io::Write;

use crate::database::{self, JsonMap};

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

/// `pnp search` logic
pub fn search(filter_by_author: bool, author_name: &str, params: Vec<&str>) -> anyhow::Result<()> {
    let database_json: JsonMap = database::read_database()?;

    // Iterate over all plugins and filter based on search arguments
    let search_params = RegexSet::new(&params)?;
    for (plugin, metadata) in database_json.iter() {
        let author = metadata["owner"]["login"].as_str().unwrap();
        let author_and_sep = author.to_owned() + "/";
        let description = metadata["description"]
            .as_str()
            .unwrap_or("No description available");

        let desc_matches = search_params.matches(description);
        let name_matches = search_params.matches(plugin);
        if ((params[0] == plugin) || (name_matches.into_iter().count() == params.len()) || (desc_matches.into_iter().count() == params.len())) && ((author == author_name) || !filter_by_author) {
            println!(
                "{}{}\n\t{}\n",
                author_and_sep.purple().bold(),
                plugin.bold(),
                description
            )
        }
    }

    Ok(())
}

// `pnp info` logic
pub fn info(plugin_name: &str) -> anyhow::Result<()> {
    let database_json: JsonMap = database::read_database()?;

    let plugin = database_json.get_key_value(plugin_name);
    for (_, metadata) in plugin.iter() {
        // Metadata we want to extract:
        // - description
        // - clone_url
        // - maintainer (repository owner)
        // - updated_at (YYYY-MM-DD HH:MM:SS, we will replace some chars)
        // - stargazers
        // - forks_count
        // - topics
        // - license (missing field, remote database isn't updated with this field)
        // - size (missing field, not implemented yet)
        let repo = metadata["clone_url"].as_str().unwrap().replace(".git", "");
        let maintainer = metadata["owner"]["login"].as_str().unwrap();
        let description = metadata["description"]
            .as_str()
            .unwrap_or("No description available");
        let stars_count = metadata["stargazers_count"].as_u64().unwrap();
        let forks_count = metadata["forks_count"].as_u64().unwrap();
        let updated_date = metadata["updated_at"]
            .as_str()
            .unwrap()
            .replace("T", " ")
            .replace("Z", "");
        let topics_arr = metadata["topics"].as_array().unwrap();

        // Get topics from JSON topics array
        let mut topics: Vec<&str> = Vec::new();
        for topic in topics_arr {
            topics.push(topic.as_str().unwrap());
        }

        println!("{} {}", "Maintainer\t:".bold(), maintainer);
        println!("{} {}", "Repository\t:".bold(), repo);
        println!("{} {}", "Description\t:".bold(), description);
        println!("{} {}", "Topics\t\t:".bold(), topics.join(", "));
        println!("{} {}", "Stars count\t:".bold(), stars_count);
        println!("{} {}", "Forks count\t:".bold(), forks_count);
        println!("{} {}", "Latest update\t:".bold(), updated_date);
    }

    Ok(())
}
