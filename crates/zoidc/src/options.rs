use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Clone, Parser)]
#[clap(version, author, about)]
/// The Zoid language compiler
pub struct Options {
    /// Input file for the Zoid language compiler
    pub input: PathBuf
}
