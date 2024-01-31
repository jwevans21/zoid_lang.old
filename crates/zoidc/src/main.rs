use std::fs;

use clap::Parser as ClapParser;

use zoid_lexer::Lexer;

use options::Options;
use zoid_lowering::ZoidLoweringContext;
use zoid_parser::Parser;
use zoid_codegen_llvm::ZoidCodeGenContext;

mod options;

fn main() {
    let opts = Options::parse();

    println!("Input file: {}", opts.input.display());
    let file_name = opts.input.file_name().expect("Cannot get file name");

    let source = fs::read_to_string(&opts.input).expect("Unable to read file");

    eprintln!("Source Code:");
    for (i, line) in source.lines().enumerate() {
        eprintln!("{:3} | {}", i + 1, line);
    }
    eprintln!();

    let mut lexer = Lexer::new(file_name.to_str().unwrap(), &source);
    eprintln!("Tokens:");
    while let Some(tok) = lexer.next_token() {
        eprintln!("\t{}", tok);
    }
    eprintln!();

    let mut parser = Parser::new(file_name.to_str().unwrap(), &source);
    eprintln!("AST:");
    let program = parser.parse();
    eprintln!("{:#?}", program);
    eprintln!();

    let mut lowering = ZoidLoweringContext::new(program);
    eprintln!("HLIR:");
    let hlir = lowering.lower();
    // eprintln!("{:#?}", lowering);
    eprintln!("{:#?}", hlir);

    let mut codegen = ZoidCodeGenContext::new(hlir);
    codegen.codegen();
    codegen.verify();

    eprintln!("LLVM IR:");
    codegen.dump();
    eprintln!();

    codegen.optimize();
    codegen.verify();
    eprintln!("Optimized LLVM IR:");
    codegen.dump();
    eprintln!();
}
