//! Versm Subcommands

// region: Modules
pub mod config;
pub mod install;
pub mod list;
pub mod remove;
pub mod version;
// endregion
// region: Imports
use crate::{
    commands::{
        config::ConfigCmd, install::InstallCmd, list::ListCmd, remove::RemoveCmd,
        version::VersionCmd,
    },
    config::VersmConfig,
};
use abscissa_core::{
    config::Override, Command, Configurable, FrameworkError, Help, Options, Runnable,
};
use directories_next::{BaseDirs, ProjectDirs};
use std::path::PathBuf;
// endregion

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
        let project_dirs = ProjectDirs::from("dev", "reynn", "versm");
        // Check if the config file exists, and if it does not, ignore it.
        // If you'd like for a missing configuration file to be a hard error
        // instead, always return `Some(CONFIG_FILE)` here.
        let filename = if let Some(proj_dirs) = project_dirs {
            proj_dirs.config_dir().join(CONFIG_FILE)
        } else {
            let base_dirs = BaseDirs::new()?;
            base_dirs.config_dir().join(CONFIG_FILE)
        };

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
            VersmCmd::Config(cmd) => cmd.override_config(config),
            VersmCmd::Install(cmd) => cmd.override_config(config),
            VersmCmd::List(cmd) => cmd.override_config(config),
            VersmCmd::Remove(cmd) => cmd.override_config(config),
            _ => Ok(config),
        }
    }
}
