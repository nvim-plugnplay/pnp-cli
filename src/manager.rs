use reqwest::Client;
use serde_json::Value;
use filetime::FileTime;
async fn fetch_remote_updatetime() -> i64 {
    let client = Client::builder()
        .user_agent("plugnplay.nvim/0.1.0")
        .build().unwrap();
    let resp = client.get("https://api.github.com/repos/nvim-plugnplay/database/commits?path=database.json&page=1&per_page=1")
        .send().await.unwrap()
        .text().await.unwrap();
    let parsed: Value = serde_json::from_str(&resp).unwrap();
    let raw_remote = parsed[0]["commit"]["committer"]["date"].to_string();
    let time_remote = chrono::NaiveDateTime::parse_from_str(&raw_remote, "\"%+\"").unwrap();
    time_remote.timestamp()
}

async fn fetch_local_updatetime() -> i64 {
    let path = format!("{}/{}", dirs::data_dir().unwrap().to_str().unwrap(), "pnp/database.json");
    let prev_metadata = std::fs::metadata(&path);
    let metadata = match prev_metadata {
        Ok(data) => data,
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                let dir = format!("{}/{}", dirs::data_dir().unwrap().to_str().unwrap(), "pnp");
                load_database(&path, &dir).await;
                std::fs::metadata(&path).unwrap()
            } else {
                unimplemented!()
            }
        }
    };
    FileTime::from_last_modification_time(&metadata)
        .unix_seconds()
}

pub async fn load_database(path: &str, dir: &str) {
    let client = Client::new();
    let resp = client.get("https://raw.githubusercontent.com/nvim-plugnplay/database/main/database.json")
        .send().await.unwrap()
        .bytes().await.unwrap();
    std::fs::create_dir_all(dir).unwrap();
    let mut file = std::fs::File::create(path).unwrap();
    let mut content = std::io::Cursor::new(resp);
    std::io::copy(&mut content, &mut file).unwrap();
}

pub async fn is_outdated() -> bool {
    fetch_remote_updatetime().await > fetch_local_updatetime().await
}
