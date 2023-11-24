mod token;
mod node;
mod lexer;
mod parser;
mod generator;

use std::{
    env,
    fs, process::Command,
};

use generator::Generator;
use lexer::Lexer;
use parser::Parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("File not provided!");
    }
    let bytes = fs::read(args[1].clone()).expect("Error reading file!");
    
    let mut lexer = Lexer::new(bytes);
    let tokens = lexer.tokenize();

    let mut parser = Parser::new(tokens);
    let nodes = parser.parse().expect("Error parsing!");

    let mut generator = Generator::new(nodes);
    let code = generator.generate();

    fs::write("main.c", code).expect("Error writing to file!");
    Command::new("gcc").arg("main.c").status().expect("Failed to compile C code!");
}
