use crate::handle;
use crate::database;
use clap::{arg, command, Command};

/// Generate clap cli command
pub fn build() -> Command<'static> {
    command!()
        .propagate_version(true)
        .arg_required_else_help(true)
        .arg(arg!(--freeze "Do not update database.json"))
        .subcommands(vec![
            Command::new("init")
                .about("Intialize config files")
                .arg(arg!(--plugin "Initialize plugin file")),
            Command::new("add")
                .about("Add plugin to plugin list")
                .arg(arg!(<repository> "GitHub author/name")),
            Command::new("rm")
                .about("Remove plugin from plugin list")
                .arg(arg!(<repository> "GitHub author/name")),
            Command::new("install")
                .about("Install plugins")
                .arg(arg!(--preview "Temp installation")),
            Command::new("update")
                .about("Update plugins")
                .arg(arg!([name] "Plugin name")),
            Command::new("search")
                .about("Search through plugin database")
                .arg(arg!(--author <name> "Filter by plugin author").required(false).takes_value(true))
                .arg(arg!(<request> "Part of GitHub's author/name").multiple_values(true)),
        ])
}

/// Handle clap cli matches
pub async fn handle(matches: clap::ArgMatches) -> anyhow::Result<()> {
    if !&matches.is_present("freeze") {
        let outdated = database::is_outdated().await?;
        if outdated {
            database::load_database().await?;
        }
    }
    match &matches.subcommand() {
        Some(("init", sub_matches)) => handle::init(sub_matches.is_present("plugin"))?,
        Some(("search", sub_matches)) => {
            let mut author = String::new();
            let should_filter_by_author = sub_matches.is_present("author");
            let params: Vec<&str> = sub_matches.values_of("request").unwrap().collect();
            if should_filter_by_author {
                author = sub_matches.values_of("author").unwrap().collect();
            }
            handle::search(should_filter_by_author, &author, params)?
        },
        _ => (),
    }
    Ok(())
}
