mod cli;
mod handle;
mod manager;

#[tokio::main]
async fn main() {
    println!("{}", manager::is_outdated().await);
    //let cmd = cli::build();
    //let matches = cmd.get_matches();
    //cli::handle(matches);
}
