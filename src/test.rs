#[cfg(test)]
mod test {
    use crate::{
        translate,
        write_c_code,
        load_stdlib,
        compile,
        remove_c,
    };

    #[test]
    fn test_minimal() {
        let src = r#"
define main :>
() :>
;>"#.bytes().collect();
        compiles(src);
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
    }

    #[test]
    fn test_functions() {
        let src = r#"
define readAndDouble :>
() :>
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
    }

    fn compiles(src: Vec<u8>) {
        let code = translate(src);
        write_c_code(code).expect("Error writing c code!");
        load_stdlib().expect("Error loading stdlib!");
        let status = compile();
        remove_c().expect("Error removing c files!");

        assert!(status.is_ok());
    }
}
