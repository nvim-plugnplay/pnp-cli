use tokio::process::Command;

use std::process::Stdio;

pub fn append_to_data(append: &str) -> String {
    let data = dirs::data_local_dir().unwrap();
    let data = data.to_str().unwrap();
    #[cfg(target_family = "windows")]
    let nvim_data = format!("{data}/nvim-data");
    #[cfg(target_family = "unix")]
    let nvim_data = format!("{data}/nvim");

    nvim_data + append
}

pub async fn clone(url: String, dir_name: String) -> anyhow::Result<()> {
    let data_appendix = format!("/site/pack/pnp/opt/{dir_name}");
    let dir = append_to_data(&data_appendix);
    let mut cmd = Command::new("git");
    cmd.args(&["clone", &url, "--depth=1", &dir]).stdout(Stdio::piped());
    let mut child = cmd.spawn()?;
    tokio::spawn(async move {
        let _ = child.wait().await;
    });

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
    tokio::spawn(async move {
        let _ = child.wait().await;
    });

    Ok(())
}
