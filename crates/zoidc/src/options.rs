use clap::{Arg, Command, ValueHint};

// #[derive(Debug, Parser)]
// #[clap(
//     version = env!("CARGO_PKG_VERSION"),
//     about = "The Zoid compiler",
//     author = env!("CARGO_PKG_AUTHORS"),
// )]
// pub struct Options {
//     /// The initial source file to parse
//     pub input: PathBuf,
//     #[arg(short)]
//     /// The file to write the output to
//     pub output: Option<PathBuf>,
//     // #[arg(short = 'I')]
//     // /// The paths to search for C header files
//     // pub include_paths: Vec<PathBuf>,
//     // #[arg(short = 'i')]
//     // /// The paths of C header files to include
//     // pub include: Vec<PathBuf>,
//     // #[arg(short = 'L')]
//     // /// The paths to search for C libraries
//     // pub lib_paths: Vec<PathBuf>,
//     // #[arg(short = 'l')]
//     // /// The names of C libraries to link
//     // pub libs: Vec<String>,
//     // #[arg(short = 'D')]
//     // /// Define a macro
//     // pub defines: Vec<String>,
//     // #[arg(short = 'U')]
//     // /// Undefine a macro
//     // pub undefines: Vec<String>,
//     #[arg(short = 'O', default_value = "0", value_parser = ["0", "1", "2", "3"])]
//     /// Optimize the output
//     pub optimize: u8,
//     #[arg(short = 'g')]
//     /// Generate debug information
//     pub debug: bool,
//     #[arg(short = 'S')]
//     /// Generate assembly code
//     pub assembly: bool,
//     #[arg(short = 'c')]
//     /// Generate object code
//     pub object: bool,
//     #[arg(long = "emit-llvm")]
//     /// Emit LLVM IR or bitcode (depends on the -S flag)
//     pub emit_llvm: bool,
// }

pub fn cli() -> Command {
    Command::new("Zoid Language Compiler")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("The Zoid compiler")
        // .arg(
        //     Arg::new("input")
        //         .help("The initial source file to parse")
        //         .value_hint(ValueHint::FilePath)
        //         .required(true)
        //         .num_args(1),
        // )
        // .arg(
        //     Arg::new("output")
        //         .short('o')
        //         .help("The file to write the output to")
        //         .value_hint(ValueHint::FilePath)
        //         .num_args(1),
        // )
        // .arg(
        //     Arg::new("debug")
        //         .short('g')
        //         .help("Generate debug information")
        //         .action(ArgAction::SetTrue),
        // )
        // .arg(
        //     Arg::new("assembly")
        //         .short('S')
        //         .help("Generate assembly code")
        //         .action(ArgAction::SetTrue),
        // )
        // .arg(
        //     Arg::new("object")
        //         .short('c')
        //         .help("Generate object code")
        //         .action(ArgAction::SetTrue),
        // )
        // .arg(
        //     Arg::new("emit-llvm")
        //         .long("emit-llvm")
        //         .help("Emit LLVM IR or bitcode (depends on the -S flag)")
        //         .action(ArgAction::SetTrue),
        // )
        // .arg(
        //     Arg::new("optimize")
        //         .short('O')
        //         .default_value("0")
        //         .value_parser(["0", "1", "2", "3"])
        //         .help("Optimize the output"),
        // )
        .subcommand(
            Command::new("new").about("Create a new Zoid project").arg(
                Arg::new("name")
                    .help("The name of the project")
                    .value_hint(ValueHint::DirPath)
                    .required(true)
                    .num_args(1),
            ),
        )
        .subcommand(
            Command::new("init").about("Create a new Zoid project").arg(
                Arg::new("name")
                    .help("The name of the project")
                    .value_hint(ValueHint::DirPath)
                    .required(false)
                    .num_args(1),
            ),
        )
        .subcommand(Command::new("compile").arg(Arg::new("input")))
}
