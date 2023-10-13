//! Build script for RuSnap

mod config;
pub use config::*;

mod generate;
pub use generate::*;

pub(crate) mod snap;

pub(crate) mod package;
