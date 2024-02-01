use std::{
    fs,
    env,
    process::{self, ExitStatus},
    io,
};

pub mod stdlib {
    include!("stdlib.rs");
}

mod token;
mod node;
mod lexer;
mod parser;
mod generator;

use token::Token;
use node::Node;
use lexer::Lexer;
use parser::Parser;
use generator::Generator;

pub fn read_src() -> Vec<u8> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("File not provided!");
    }
    fs::read(args[1].clone()).expect("Error reading file!")
}

pub fn load_stdlib() -> io::Result<()> {
    fs::write("flwrstdlib.h", stdlib::STDLIB_H)?;
    fs::write("flwrstdlib.c", stdlib::STDLIB_C)?;
    fs::write("varlist.h", stdlib::VARLIST_H)?;
    fs::write("varlist.c", stdlib::VARLIST_C)?;
    Ok(())
}

pub fn write_c_code(code: String) -> io::Result<()> {
    fs::write("main.c", code)?;
    Ok(())
}

pub fn remove_c() -> io::Result<()>{
    fs::remove_file("flwrstdlib.h")?;
    fs::remove_file("flwrstdlib.c")?;
    fs::remove_file("varlist.h")?;
    fs::remove_file("varlist.c")?;
    fs::remove_file("main.c")?;
    Ok(())
}

pub fn compile() -> io::Result<ExitStatus> {
    process::Command::new("gcc")
        .arg("-O3")
        .arg("flwrstdlib.c")
        .arg("main.c")
        .status()
}

pub fn translate(bytes: Vec<u8>) -> String {
    let tokens = tokenize(bytes);
    let nodes = parse(tokens).expect("Error parsing!");
    generate(nodes)
}

pub fn tokenize(bytes: Vec<u8>) -> Vec<Token> {
    let mut lexer = Lexer::new(bytes);
    lexer.tokenize()
}

pub fn parse(tokens: Vec<Token>) -> Option<Vec<Node>> {
    let mut parser = Parser::new(tokens);
    parser.parse()
}

pub fn generate(nodes: Vec<Node>) -> String {
    let mut generator = Generator::new(nodes);
    generator.generate()
}
