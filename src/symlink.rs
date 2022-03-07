use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::Command,
};
use std::process::Stdio;

// TODO: check if symlink can be created (path exists)
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
        let path = shellexpand::tilde(&self.path).to_string().replace("/", "\\");
        let target = self.target.replace("/", "\\");
        cmd.args(&["/c", "mklink", "/D", &target, &path]);
        self.cmd = Some(cmd);
    }

    #[cfg(target_family = "unix")]
    fn cmd(&mut self) {
        let mut cmd = Command::new("sh");
        cmd.args(&["-c", &format!("'ln -sf {} {}'", &self.path, &self.target)]);
        self.cmd = Some(cmd)
    }

    pub async fn create(&mut self) -> anyhow::Result<()> {
        self.cmd();
        let cmd = self.cmd.as_mut().unwrap();
        cmd.stdout(Stdio::piped());
        let mut child = cmd.spawn()?;
        let stderr = child.stdout.take().unwrap();
        let mut reader = BufReader::new(stderr).lines();
        let succeed = child.wait().await?.success();
        if !succeed {
            while let Some(line) = reader.next_line().await? {
                println!("Symlink err: {line}");
            }
        }

        Ok(())
    }

    pub fn delete(&self) -> anyhow::Result<()> {
        std::fs::remove_file(&self.path)?;
        Ok(())
    }
}
