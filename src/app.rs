use clap::ArgMatches;

use crate::cmd::{new::build_new, root::build_root, tasks::build_new_task};

pub struct App {
    pub matches: ArgMatches<'static>,
}

impl App {
    pub fn new() -> Result<Self, String> {
        Ok(App {
            matches: Self::matches()?,
        })
    }

    fn matches() -> Result<ArgMatches<'static>, String> {
        Ok(build_root()
            .subcommand(build_new().subcommand(build_new_task()))
            .get_matches())
    }
}
