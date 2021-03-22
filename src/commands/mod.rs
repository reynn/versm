//! Versm Subcommands
//!
//! This is where you specify the subcommands of your application.
//!
//! The default application comes with two subcommands:
//!
//! - `start`: launches the application
//! - `version`: print application version
//!
//! See the `impl Configurable` below for how to specify the path to the
//! application's configuration file.

pub mod config;
pub mod install;
pub mod list;
pub mod remove;
pub mod version;

use self::{
    config::ConfigCmd, install::InstallCmd, list::ListCmd, remove::RemoveCmd, version::VersionCmd,
};
use crate::config::VersmConfig;
use abscissa_core::{
    config::Override, Command, Configurable, FrameworkError, Help, Options, Runnable,
};
use std::path::PathBuf;

/// Versm Configuration Filename
pub const CONFIG_FILE: &str = "versm.toml";

/// Versm Subcommands
#[derive(Command, Debug, Options, Runnable)]
pub enum VersmCmd {
    /// The `config` subcommand
    #[options(help = "Manage the app config")]
    Config(ConfigCmd),
    /// The `install` subcommand
    #[options(help = "Install a managed version")]
    Install(InstallCmd),
    /// The `list` subcommand
    #[options(help = "List versions and managers")]
    List(ListCmd),
    /// The `remove` subcommand
    #[options(help = "Remove a managed version")]
    Remove(RemoveCmd),
    /// The `help` subcommand
    #[options(help = "get usage information")]
    Help(Help<Self>),
    /// The `version` subcommand
    #[options(help = "display version information")]
    Version(VersionCmd),
}

/// This trait allows you to define how application configuration is loaded.
impl Configurable<VersmConfig> for VersmCmd {
    /// Location of the configuration file
    fn config_path(&self) -> Option<PathBuf> {
        // Check if the config file exists, and if it does not, ignore it.
        // If you'd like for a missing configuration file to be a hard error
        // instead, always return `Some(CONFIG_FILE)` here.
        let filename = PathBuf::from(CONFIG_FILE);

        if filename.exists() {
            Some(filename)
        } else {
            None
        }
    }

    /// Apply changes to the config after it's been loaded, e.g. overriding
    /// values in a config file using command-line options.
    ///
    /// This can be safely deleted if you don't want to override config
    /// settings from command-line options.
    fn process_config(&self, config: VersmConfig) -> Result<VersmConfig, FrameworkError> {
        match self {
            VersmCmd::Install(cmd) => cmd.override_config(config),
            VersmCmd::Config(cmd) => cmd.override_config(config),
            VersmCmd::Remove(cmd) => cmd.override_config(config),
            VersmCmd::List(cmd) => cmd.override_config(config),
            _ => Ok(config),
        }
    }
}
