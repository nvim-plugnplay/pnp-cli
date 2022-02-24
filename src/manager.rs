use filetime::FileTime;
use reqwest::Client;
use serde_json::Value;

async fn fetch_remote_updatetime() -> anyhow::Result<i64> {
    let client = Client::builder()
        .user_agent("plugnplay.nvim/0.1.0")
        .build()?;
    let resp = client.get("https://api.github.com/repos/nvim-plugnplay/database/commits?path=database.json&page=1&per_page=1")
        .send().await?
        .text().await?;
    let parsed: Value = serde_json::from_str(&resp)?;
    let raw_remote = parsed[0]["commit"]["committer"]["date"].to_string();
    let time_remote = chrono::NaiveDateTime::parse_from_str(&raw_remote, "\"%+\"")?;
    Ok(time_remote.timestamp())
}

async fn fetch_local_updatetime() -> anyhow::Result<i64> {
    let path = format!(
        "{}/{}",
        dirs::data_dir().unwrap().to_str().unwrap(),
        "pnp/database.json"
    );
    let prev_metadata = std::fs::metadata(&path);
    let metadata = match prev_metadata {
        Ok(data) => data,
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                load_database().await?;
                std::fs::metadata(&path)?
            } else {
                unimplemented!()
            }
        }
    };
    Ok(FileTime::from_last_modification_time(&metadata).unix_seconds())
}

pub async fn load_database() -> anyhow::Result<()> {
    println!("Updating database...");
    let dir = format!("{}/{}", dirs::data_dir().unwrap().to_str().unwrap(), "pnp");
    let path = format!(
        "{}/{}",
        dirs::data_dir().unwrap().to_str().unwrap(),
        "pnp/database.json"
    );
    let client = Client::new();
    let resp = client
        .get("https://raw.githubusercontent.com/nvim-plugnplay/database/main/database.json")
        .send()
        .await?
        .bytes()
        .await?;
    std::fs::create_dir_all(dir)?;
    let mut file = std::fs::File::create(path)?;
    let mut content = std::io::Cursor::new(resp);
    std::io::copy(&mut content, &mut file)?;
    println!("Database is up to date!");

    Ok(())
}

pub async fn is_outdated() -> anyhow::Result<bool> {
    Ok(fetch_remote_updatetime().await? > fetch_local_updatetime().await?)
}
