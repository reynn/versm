//! `Config` subcommand - example of how to write a subcommand

/// App-local prelude includes `app_reader()`/`app_writer()`/`app_config()`
/// accessors along with logging macros. Customize as you see fit.
use crate::prelude::*;

use crate::config::VersmConfig;
use abscissa_core::{config, Command, FrameworkError, Options, Runnable};

/// `Config` subcommand
///
/// The `Options` proc macro generates an option parser based on the struct
/// definition, and is defined in the `gumdrop` crate. See their documentation
/// for a more comprehensive example:
///
/// <https://docs.rs/gumdrop/>
#[derive(Command, Debug, Options)]
pub struct ConfigCmd {}

impl Runnable for ConfigCmd {
    /// Config
    fn run(&self) {
        let _config = APP.config();
    }
}

impl config::Override<VersmConfig> for ConfigCmd {
    // Process the given command line options, overriding settings from
    // a configuration file using explicit flags taken from command-line
    // arguments.
    fn override_config(&self, config: VersmConfig) -> Result<VersmConfig, FrameworkError> {
        Ok(config)
    }
}
