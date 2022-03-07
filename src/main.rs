mod cli;
mod data;
mod database;
mod fs;
mod git;
mod handle;
mod symlink;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cmd = cli::build();
    let matches = cmd.get_matches();
    cli::handle(matches).await?;
    Ok(())
}
