//! Versm Config
//!
//! See instructions in `commands.rs` to specify the path to your
//! application's configuration file and/or command-line options
//! for specifying it.

mod managers;
mod versions;

use self::{managers::ManagersSection, versions::VersionsSection};
use serde::{Deserialize, Serialize};

/// Versm Configuration
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct VersmConfig {
    ///
    pub managers: ManagersSection,
    ///
    pub versions: VersionsSection,
}
