use clap::{arg, command, Command};

/// Structure that holds Clap's CLI state
pub struct Application {
    matches: clap::ArgMatches,
}

impl Application {
    /// Parse CLI arguments using Clap and provide matches
    pub fn new() -> Self {
        Self {
            matches: command!()
                .propagate_version(true)
                .arg_required_else_help(true)
                .get_matches(),
        }
    }
    /// Handle Clap's matches
    pub fn handle(&self) {
        // TODO
    }
}
