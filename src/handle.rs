use std::fs::File;
use std::io::{Write, BufReader};
use std::collections::HashMap;
use serde_json::{Value, from_reader};
use regex::Regex;

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
pub fn search(_filter_by_author: bool) -> anyhow::Result<()> {
    // Custom HashMap type so we can iterate over our database
    type JsonMap = HashMap<String, Value>;

    // TODO: make the filter_by_author logic
    //
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
    for (plugin, metadata) in database_json.iter() {
        let description = match metadata["description"].as_str() {
            Some(desc) => desc,
            None => "No description available",
        };
        println!("plugin : {:?}\ndescription : {:?}", plugin, description);
    }

    Ok(())
}
