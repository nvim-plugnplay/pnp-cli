use std::fs::File;
use std::io::{Write, BufReader};
use std::collections::HashMap;
use serde_json::{Value, from_reader};
use regex::RegexSet;
use colored::*;

use crate::database;

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

/// `pnp search` logic
pub fn search(filter_by_author: bool, author_name: &str, params: Vec<&str>) -> anyhow::Result<()> {
    // Custom HashMap type so we can iterate over our database
    type JsonMap = HashMap<String, Value>;

    // Get the database path and open database if exists
    let pnp_database = match database::get_database_path() {
        Ok(database) => File::open(database)?,
        Err(e) => {
            panic!("{e:?}");
        }
    };
    // Read database contents and convert it to a string
    let buf_reader = BufReader::new(pnp_database);
    // Deserialize JSON database from reader
    let database_json: JsonMap = from_reader(buf_reader)?;

    // Iterate over all plugins and filter based on search arguments
    let search_params = RegexSet::new(&params)?;
    for (plugin, metadata) in database_json.iter() {
        let author = metadata["owner"]["login"].as_str().unwrap();
        let description = metadata["description"].as_str().unwrap_or("No description available");
        let matches: Vec<_> = search_params.matches(description).into_iter().collect();
        if matches.len() == params.len() {
            let author_and_sep = author.to_owned() + "/";
            if filter_by_author {
                if author == author_name {
                    println!("{}{}\n    {}\n", author_and_sep.purple().bold(), plugin.bold(), description)
                }
            } else {
                println!("{}{}\n    {}\n", author_and_sep.purple().bold(), plugin.bold(), description)
            }
        }
    }

    Ok(())
}
