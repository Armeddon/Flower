#[cfg(test)]
mod test {
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
    fn test_minimal() {
        let src = r#"
define main :>
() :>
;>"#.bytes().collect();

        compiles(src);
        assert_eq!(run_result!(), "".as_bytes())
    }

    #[test]
    fn test_io() {
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
    fn test_pipe() {
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
    fn test_functions() {
        let src = r#"
define readAndDouble :>
Int :>
    readInt =>
    identity |>
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
    fn test_constants() {
        let src = r#"
define five :> Int :> 5 ;>
define main :>
() :>
    readInt =>
    five =>
    add =>
    println
;>"#.bytes().collect();
        compiles(src);
        assert_eq!(
            run_result!(42),
            "47\n".as_bytes(),
            "The test adds input(42) to the constant file (=5)"
        )
    }

    #[test]
    fn test_constant_arguments() {
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

    fn compiles(src: Vec<u8>) {
        let code = translate(src);
        write_c_code(code).expect("Error writing c code!");
        load_stdlib().expect("Error loading stdlib!");
        let status = compile!("test");
        remove_c().expect("Error removing c files!");

        assert!(status.is_ok());
    }
}
