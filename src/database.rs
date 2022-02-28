use filetime::FileTime;
use reqwest::Client;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashMap;
use serde_json::{from_reader, Value};

const DATABASE_COMMITS_LINK: &str = "https://api.github.com/repos/nvim-plugnplay/database/commits?path=database.json&page=1&per_page=1";
const DATABASE_RAW_LINK: &str =
    "https://raw.githubusercontent.com/nvim-plugnplay/database/main/database.json";

// Custom HashMap type so we can iterate over our database
pub type JsonMap = HashMap<String, Value>;

/// Get last modification time of database.json from remote source
async fn fetch_remote_updatetime() -> anyhow::Result<i64> {
    let client = Client::builder()
        .user_agent("plugnplay.nvim/0.1.0")
        .build()?;
    let resp = client
        .get(DATABASE_COMMITS_LINK)
        .send()
        .await?
        .text()
        .await?;
    let parsed: Value = serde_json::from_str(&resp)?;
    let raw_remote = parsed[0]["commit"]["committer"]["date"].to_string();
    let time_remote = chrono::NaiveDateTime::parse_from_str(&raw_remote, "\"%+\"")?;
    Ok(time_remote.timestamp())
}

/// Get last modification time of database.json from local source
async fn fetch_local_updatetime() -> anyhow::Result<i64> {
    let path = format!(
        "{}/pnp/database.json",
        dirs::data_dir().unwrap().to_str().unwrap(),
    );
    let prev_metadata = std::fs::metadata(&path);
    let metadata = match prev_metadata {
        Ok(data) => data,
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                load_database().await?;
                std::fs::metadata(&path)? // I guess if we have error on this stage we can just `?` it
            } else {
                panic!("{e:?}");
            }
        }
    };
    Ok(FileTime::from_last_modification_time(&metadata).unix_seconds())
}

/// Download database.json
pub async fn load_database() -> anyhow::Result<()> {
    println!("Updating database...");
    let dir = format!("{}/pnp", dirs::data_dir().unwrap().to_str().unwrap());
    let path = format!(
        "{}/pnp/database.json",
        dirs::data_dir().unwrap().to_str().unwrap(),
    );
    let client = Client::new();
    let resp = client.get(DATABASE_RAW_LINK).send().await?.bytes().await?;
    std::fs::create_dir_all(dir)?;
    let mut file = std::fs::File::create(path)?;
    let mut content = std::io::Cursor::new(resp);
    std::io::copy(&mut content, &mut file)?;
    println!("Database is up to date!");

    Ok(())
}

/// Check if database.json is outdated based on last modification time
pub async fn is_outdated() -> anyhow::Result<bool> {
    Ok(fetch_remote_updatetime().await? > fetch_local_updatetime().await?)
}

/// Retrieve local database path
pub fn get_database_path() -> anyhow::Result<String> {
    let path = format!(
        "{}/{}",
        dirs::data_dir().unwrap().to_str().unwrap(),
        "pnp/database.json"
    );
    Ok(path)
}

/// Get the local database contents
pub fn read_database() -> anyhow::Result<JsonMap> {
    // Get the database path and open database if exists
    let pnp_database = match get_database_path() {
        Ok(database) => File::open(database)?,
        Err(e) => {
            panic!("{e:?}");
        }
    };
    // Read database contents and convert it to a string
    let buf_reader = BufReader::new(pnp_database);
    // Deserialize JSON database from reader
    let database_json: JsonMap = from_reader(buf_reader)?;
    Ok(database_json)
}
