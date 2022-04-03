use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::Command,
};

use std::process::Stdio;

use crate::fs;

pub fn append_to_data(append: &str) -> String {
    let nvim_data = fs::Manager::data().unwrap();

    nvim_data + append
}

pub async fn clone(url: String, dir_name: String) -> anyhow::Result<()> {
    let data_appendix = format!("/site/pack/pnp/opt/{dir_name}");
    let dir = append_to_data(&data_appendix);
    let mut cmd = Command::new("git");
    cmd.args(&["clone", &url, "--depth=1", &dir])
        .stdout(Stdio::piped());
    let mut child = cmd.spawn()?;
    let _ = child.wait().await;

    Ok(())
}

pub async fn update(dir_name: String) -> anyhow::Result<()> {
    let data_appendix = format!("/site/pack/pnp/opt/{dir_name}");
    let dir = append_to_data(&data_appendix);
    let mut cmd = Command::new("git");
    cmd.args(&["pull", "origin", "main"])
        .current_dir(dir)
        .stdout(Stdio::piped());
    let mut child = cmd.spawn()?;
    let _ = child.wait().await;

    Ok(())
}

pub async fn commit_hash(dir_name: String) -> anyhow::Result<String> {
    let data_appendix = format!("/site/pack/pnp/opt/{dir_name}");
    let dir = append_to_data(&data_appendix);
    let mut cmd = Command::new("git");
    cmd.args(&["rev-parse", "HEAD"])
        .current_dir(dir)
        .stdout(Stdio::piped());
    let mut child = cmd.spawn()?;
    let stdout = child.stdout.take().unwrap();
    let mut reader = BufReader::new(stdout).lines();

    if let Some(line) = reader.next_line().await? {
        Ok(line)
    } else {
        Err(anyhow::format_err!("Could not read commit hash"))
    }
}

pub async fn branch(dir_name: String) -> anyhow::Result<String> {
    let data_appendix = format!("/site/pack/pnp/opt/{dir_name}");
    let dir = append_to_data(&data_appendix);
    let mut cmd = Command::new("git");
    cmd.args(&["rev-parse", "--abbrev-ref", "HEAD"])
        .current_dir(dir)
        .stdout(Stdio::piped());
    let mut child = cmd.spawn()?;
    let stdout = child.stdout.take().unwrap();
    let mut reader = BufReader::new(stdout).lines();

    if let Some(line) = reader.next_line().await? {
        Ok(line)
    } else {
        Err(anyhow::format_err!("Could not read current branch"))
    }
}
