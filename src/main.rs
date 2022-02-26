mod cli;
mod handle;
mod database;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cmd = cli::build();
    let matches = cmd.get_matches();
    cli::handle(matches).await?;
    Ok(())
}
