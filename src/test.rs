use std::{
    fs, 
    process,
    io::{Write, Seek, SeekFrom},
};

use crate::{
    translate,
    write_c_code,
    load_stdlib,
    compile,
    remove_c,
};

use pretty_assertions::assert_eq;

macro_rules! run_result {
    ( $( $arg1:expr $(,$arg:expr)* )? ) => {
        {
            fs::File::create("test_in.txt").unwrap().set_len(0).unwrap();
            fs::File::create("test_in.txt").unwrap().seek(SeekFrom::End(0)).unwrap();
            fs::OpenOptions::new()
                .write(true)
                .append(true)
                .open("test_in.txt").unwrap()
                .write_all(
                    concat!(""
                            $(
                                ,$arg1
                                $(
                                    ," "
                                    ,$arg
                                 )*
                             )?
                           ).as_bytes()
                    ).unwrap();
            let result = process::Command::new("./test")
                .stdin(fs::File::open("test_in.txt").unwrap())
                .output()
                .unwrap()
                .stdout;
            fs::remove_file("test_in.txt").unwrap();
            result
        }
    };
}

#[test]
#[should_panic(expected = "Error tokenizing!")]
fn tokenize_error() {
    let src = r#"
Hello, world!
"#.bytes().collect();

    compiles(src);
}

#[test]
#[should_panic(expected = "Error parsing!")]
fn parse_error() {
    let src = r#"
define define :> () :> ;>
"#.bytes().collect();

    compiles(src);
}

#[test]
fn minimal() {
    let src = r#"
define main :>
() :>
;>"#.bytes().collect();

    compiles(src);
    assert_eq!(run_result!(), "".as_bytes())
}

#[test]
fn io() {
    let src = r#"
define main :>
() :>
    readInt =>
    println
;>"#.bytes().collect();

    compiles(src);
    assert_eq!(
        run_result!(42),
        "42\n".as_bytes(),"
        The test is basically cat for input)(42)"
        )
}

#[test]
fn preserve() {
    let src = r#"
define main :>
() :>
    readInt =>
    println |>
    println
;>"#.bytes().collect();

    compiles(src);
    assert_eq!(
        run_result!(42),
        "42\n42\n".as_bytes(),
        "The test is double printing input(42)"
        )
}

#[test]
fn prepend() {
    let src = r#"
define main :>
() :>
    readInt =>
    readInt =>
    readInt =>
    add +>
    println
;>"#.bytes().collect();
    compiles(src);
    assert_eq!(
        run_result!(4, 2, 3),
        "6\n".as_bytes(),
        "The test is the sum of first two numbers in input(4 2 3)"
        )
}

#[test]
fn functions() {
    let src = r#"
define readAndDouble :>
Int :>
    readInt =>
    id |>
    add
;>
define main :>
() :>
    readAndDouble =>
    println
;>"#.bytes().collect();
    compiles(src);
    assert_eq!(
        run_result!(42),
        "84\n".as_bytes(),
        "The test is doubling input(42) in another function"
        )
}

#[test]
fn constant_arguments() {
    let src = r#"
define main :>
() :>
    readInt =>
    add 5 =>
    println
;>"#.bytes().collect();
    compiles(src);
    assert_eq!(
        run_result!(42),
        "47\n".as_bytes(),
        "The test adds input(42) to 5 defined in-place"
        )
}

#[test]
fn arguments() {
    let src = r#"
define add3 :>
Int -> Int -> Int -> Int :>
    add +> add
;>
define main :>
() :>
    readInt =>
    readInt =>
    readInt =>
    add3 =>
    println
;>"#.bytes().collect();
    compiles(src);
    assert_eq!(
        run_result!(4, 3, 2),
        "9\n".as_bytes(),
        "The test of function with several(3) arguments (sum) for input(4 3 2)"
        )
}

#[test]
fn string() {
    let src = r#"
define main :>
() :>
    readInt =>
    readString =>
    println
;>
"#.bytes().collect();
    compiles(src);
    assert_eq!(
        run_result!(5, "hello"),
        "hello\n".as_bytes(),
        "The test for string io for input(5 \"hello\")"
        )
}

#[test]
fn hello_world() {
    let src = r#"
define main :>
() :>
    println "Hello, world!"
;>"#.bytes().collect();
    compiles(src);
    assert_eq!(
        run_result!(),
        "Hello, world!\n".as_bytes(),
        "The \"Hello, world!\" program"
        )
}

#[test]
fn template() {
    let src = r#"
define identity :>
T -> T :>
    id
;>
define main :>
() :>
    readInt =>
    identity =>
    println
;>"#.bytes().collect();
    compiles(src);
    assert_eq!(
        run_result!(5),
        "5\n".as_bytes(),
        "Test of the template functions"
        )

}

#[test]
fn comparison() {
    let src = r#"
define main :>
() :>
    lt 1 1 =>
    println
    lt "Hello" "Hello" =>
    println
    lt 0 2 =>
    println
;>"#.bytes().collect();
    compiles(src);
    assert_eq!(
        run_result!(),
        "False\nFalse\nTrue\n".as_bytes(),
        "Test of the comparison function"
        )
}

#[test]
fn std_flwr() {
    let src = r#"
define f :>
() :>
    readInt =>
    eq 1 =>
    println
;>

define main :>
() :>
    f
    f
;>"#.bytes().collect();
    compiles(src);
    assert_eq!(
        run_result!(1, 2),
        "True\nFalse\n".as_bytes(),
        "Test of the std func in flwr"
        )
}

#[test]
fn condition() {
    let src = r#"
define f :>
() :>
    readInt =>
    eq 1 =>
    if :>
        println "One"
    :>
        println "Not one"
    ;>
;>

define main :>
() :>
    f
    f
;>"#.bytes().collect();
    compiles(src);
    assert_eq!(
        run_result!(1, 2),
        "One\nNot one\n".as_bytes(),
        "Test of the if-expression"
        )
}

#[test]
fn recursion() {
    let src = r#"
define nsquare :>
Int -> Int :>
    id |>
    eq 1 =>
    id =>
    if :>
        id 1
    :>
        id |> 
        add -2 =>
        id =>
        nsquare =>
        add
    ;>
;>
define f :>
Int -> Int :>
    id |>
    add =>
    add -1 =>
    nsquare
;>
define main :>
() :>
    readInt =>
    readInt =>
    f =>
    f =>
    println =>
    println
;>"#.bytes().collect();
    compiles(src);
    assert_eq!(
        run_result!(1, 3),
        "1\n9\n".as_bytes(),
        "Test of the recursion"
        )
}

fn compiles(src: Vec<u8>) {
    let mut src = src.clone();
    let code = translate(&mut src);
    write_c_code(code).expect("Error writing c code!");
    load_stdlib().expect("Error loading stdlib!");
    let status = compile!("test");
    //remove_c().expect("Error removing c files!");

    assert!(status.is_ok());
}
