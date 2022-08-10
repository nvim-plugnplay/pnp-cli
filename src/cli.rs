use clap::{arg, command, Command};

pub struct Application {
    matches: clap::ArgMatches
}

impl Application {
    pub fn new() -> Self {
        Self {
            matches: command!()
                .propagate_version(true)
                .arg_required_else_help(true)
                .get_matches()
        }
    }
    pub fn run(&self) {
        // TODO
    }
}
