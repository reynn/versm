//! Versm Config
//!
//! See instructions in `commands.rs` to specify the path to your
//! application's configuration file and/or command-line options
//! for specifying it.

use serde::{Deserialize, Serialize};

/// Example configuration section.
///
/// Delete this and replace it with your actual configuration structs.
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct VersionsSection {
    /// Currently installed
    pub installed: Vec<String>,
    /// install in this run through
    pub to_be_installed: Vec<String>,
}
