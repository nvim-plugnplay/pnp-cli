use clap::{arg, command, Command};
use crate::handle;

fn build() -> Command<'static> {
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
                .arg(arg!(<request> "Part of GitHub's author/name")),
        ])
}

fn handle(matches: clap::ArgMatches) {
    match &matches.subcommand() {
        Some(("init", sub_matches)) => {
            handle::init(sub_matches.is_present("plugin"))
        },
        _ => ()
    }
}

pub fn run() {
    let command = build();
    let matches = command.get_matches();
    handle(matches);
}
