//! Main entry point for Versm

#![deny(warnings, missing_docs, trivial_casts, unused_qualifications)]
#![forbid(unsafe_code)]

use versm::application::APP;

/// Boot Versm
fn main() {
    abscissa_core::boot(&APP);
}
