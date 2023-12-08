mod token;
mod node;
mod lexer;
mod parser;
mod generator;

use std::{
    env,
    fs,
    process,
};

use generator::Generator;
use lexer::Lexer;
use parser::Parser;

mod stdlib {
    include!("stdlib.rs");
}

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

    fs::write("flwrstdlib.h", stdlib::STDLIB_H).unwrap();
    fs::write("flwrstdlib.c", stdlib::STDLIB_C).unwrap();
    fs::write("varlist.h", stdlib::VARLIST_H).unwrap();
    fs::write("varlist.c", stdlib::VARLIST_C).unwrap();

    fs::write("main.c", code).expect("Error writing to file!");

    let status = process::Command::new("gcc")
        .arg("-g")
        .arg("flwrstdlib.c")
        .arg("main.c")
        .status();

    fs::remove_file("flwrstdlib.h").unwrap();
    fs::remove_file("flwrstdlib.c").unwrap();
    fs::remove_file("varlist.h").unwrap();
    fs::remove_file("varlist.c").unwrap();

    fs::remove_file("main.c").unwrap();

    if let Err(e) = status {
        panic!("Error compiling!\n{e}");
    }
}
