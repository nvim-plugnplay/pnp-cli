use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::Command,
};
use std::process::Stdio;

pub struct SymLink {
    path: String,
    target: String,
    cmd: Option<Command>,
}

impl SymLink {
    pub fn new(path: String, target: String) -> Self {
        Self { path, target, cmd: None }
    }

    #[cfg(target_family = "windows")]
    fn cmd(&mut self) {
        let mut cmd = Command::new("cmd");
        cmd.args(&["/c", "mklink", "/D", &self.path, &self.target]);
        self.cmd = Some(cmd);
    }

    #[cfg(target_family = "unix")]
    fn cmd(&mut self) {
        let mut cmd = Command::new("sh");
        cmd.args(&["-c", &format!("'ln -sf {} {}'", &self.path, &self.target)]);
        self.cmd = Some(cmd)
    }

    pub async fn create(&mut self) -> anyhow::Result<()> {
        let cmd = self.cmd.as_mut().unwrap();
        cmd.stderr(Stdio::piped());
        let mut child = cmd.spawn()?;
        let stderr = child.stderr.take().unwrap();
        let mut reader = BufReader::new(stderr).lines();
        tokio::spawn(async move {
            let _ = child.wait().await;
        });

        while let Some(line) = reader.next_line().await? {
            println!("{line}");
        }

        Ok(())
    }

    pub fn delete(&self) -> anyhow::Result<()> {
        std::fs::remove_file(&self.path)?;
        Ok(())
    }
}
