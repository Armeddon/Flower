use std::{
    fs,
    env,
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

use lexer::tokenize;
use parser::parse;
use generator::generate;

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
    fs::write("string.h", stdlib::STRING_H)?;
    fs::write("string.c", stdlib::STRING_C)?;
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
    fs::remove_file("string.h")?;
    fs::remove_file("string.c")?;
    fs::remove_file("main.c")?;
    Ok(())
}

#[macro_export]
macro_rules! compile {
    ( $($name:expr)? ) => {
         std::process::Command::new("gcc")
             .arg("-g")
             .arg("flwrstdlib.c")
             .arg("main.c")
             $(
             .arg("-o")
             .arg($name)
             )*
             .status()
    };
}

pub fn translate(bytes: Vec<u8>) -> String {
    let tokens = tokenize(bytes).expect("Error tokenizing!");
    let nodes = parse(tokens).expect("Error parsing!");
    generate(nodes)
}
