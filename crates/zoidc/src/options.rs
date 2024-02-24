use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
pub struct Options {
    /// The initial source file to parse
    pub input: PathBuf,
    #[arg(short)]
    /// The file to write the output to
    pub output: Option<PathBuf>,
    #[arg(short = 'I')]
    /// The paths to search for C header files
    pub include_paths: Vec<PathBuf>,
    #[arg(short = 'i')]
    /// The paths of C header files to include
    pub include: Vec<PathBuf>,
    #[arg(short = 'L')]
    /// The paths to search for C libraries
    pub lib_paths: Vec<PathBuf>,
    #[arg(short = 'l')]
    /// The names of C libraries to link
    pub libs: Vec<String>,
}
