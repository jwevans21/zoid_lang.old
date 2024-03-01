use std::fs;

use anyhow::{anyhow, Result};
use zoid_parser::ZoidParser;

mod options;

use crate::options::cli;

fn main() -> Result<()> {
    let options = cli().try_get_matches()?;

    // println!("{:#?}", options);
    // println!();

    match options.subcommand() {
        None => Err(anyhow!("Not subcommand provided")),
        Some(("compile", options)) => {
            let path: &String = options
                .try_get_one("input")?
                .ok_or(anyhow::anyhow!("No input file"))?;

            let input = fs::read_to_string(&path)?;

            let arena = bumpalo::Bump::new();
            let mut parser = ZoidParser::new(&arena, &path, &input);

            match parser.parse() {
                Some(_) => {}
                None => {
                    std::process::exit(1);
                }
            }

            parser.pretty_print();

            println!("Done!");

            Ok(())
        }
        Some(_) => Err(anyhow!("Unhandled subcommand")),
    }
}
