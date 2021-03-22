//! `Config` subcommand - example of how to write a subcommand

mod check;
mod save;

/// App-local prelude includes `app_reader()`/`app_writer()`/`app_config()`
/// accessors along with logging macros. Customize as you see fit.
// use crate::prelude::*;
use self::{check::ConfigCheckCmd, save::ConfigSaveCmd};
use crate::config::VersmConfig;
use abscissa_core::{config, Command, FrameworkError, Help, Options, Runnable};

/// `Config` subcommand
///
/// The `Options` proc macro generates an option parser based on the struct
/// definition, and is defined in the `gumdrop` crate. See their documentation
/// for a more comprehensive example:
///
/// <https://docs.rs/gumdrop/>
#[derive(Command, Debug, Options, Runnable)]
pub enum ConfigCmd {
    /// The `save` subcommand
    #[options(help = "Save a default config file to the provide path")]
    Save(ConfigSaveCmd),
    /// The `check` subcommand
    #[options(help = "Check a config file for validity")]
    Check(ConfigCheckCmd),
    /// The `help` subcommand
    #[options(help = "get usage information")]
    Help(Help<Self>),
}

impl config::Override<VersmConfig> for ConfigCmd {
    // Process the given command line options, overriding settings from
    // a configuration file using explicit flags taken from command-line
    // arguments.
    fn override_config(&self, config: VersmConfig) -> Result<VersmConfig, FrameworkError> {
        Ok(config)
    }
}
