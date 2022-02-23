use clap::{arg, command, Command};

fn main() {
    let _ = command!()
        .propagate_version(true)
        .arg_required_else_help(true)
        .arg(arg!(--freeze "Do not update database.json"))
        .subcommands(vec![
            Command::new("init")
                .about("Intialize plugnplay files")
                .arg_required_else_help(true)
                .args(&[
                    arg!(--config "Initialize config file").exclusive(true),
                    arg!(--plugin "Initialize plugin file").exclusive(true),
                ]),
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
        .get_matches();
}
