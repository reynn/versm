//! Versm Config
//!
//! See instructions in `commands.rs` to specify the path to your
//! application's configuration file and/or command-line options
//! for specifying it.

use serde::{Deserialize, Serialize};

/// Example configuration section.
///
/// Delete this and replace it with your actual configuration structs.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ManagersSection {
    /// Example configuration value
    pub available: Vec<String>,
}

impl Default for ManagersSection {
    fn default() -> Self {
        Self {
            available: vec!["world".to_owned()],
        }
    }
}
