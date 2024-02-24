use std::fs;

use clap::Parser;
use zoid_parser::ZoidParser;

mod options;

use options::Options;

fn main() {
    let options = Options::parse();
    println!("{:#?}", options);

    let input = fs::read_to_string(&options.input).unwrap();

    let file_name = options.input.to_str().unwrap();
    let arena = bumpalo::Bump::new();
    let mut parser = ZoidParser::new(&arena, file_name, &input);

    match parser.parse() {
        Ok(_) => {}
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    }

    parser.pretty_print();

    println!("Done!");
}
