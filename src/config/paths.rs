//! Config Paths
//!

use abscissa_core::path::{ExePath, RootPath, SecretsPath};
use std::path::PathBuf;

/// Standard set of "happy paths" used by Abscissa applications.
///
/// These are not yet finalized, but provide a standard application layout
/// (further expressed in the template) which future Abscissa
/// components/extensions should seek to adhere to.
#[derive(Clone, Debug)]
pub struct VersmPaths {
    /// Path to the application's executable.
    exe: PathBuf,

    /// Path to the application's root directory
    root: PathBuf,

    /// Path to the application's secrets
    secrets: Option<PathBuf>,
}

impl Default for VersmPaths {
    fn default() -> Self {
        todo!()
    }
}

impl ExePath for VersmPaths {
    fn exe(&self) -> &abscissa_core::path::AbsPath {
        todo!()
    }
}

impl RootPath for VersmPaths {
    fn root(&self) -> &abscissa_core::path::AbsPath {
        todo!()
    }
}

impl SecretsPath for VersmPaths {
    fn secrets(&self) -> &abscissa_core::path::AbsPath {
        todo!()
    }
}
