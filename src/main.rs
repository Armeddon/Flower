use flower::{
    read_src,
    translate,
    write_c_code,
    load_stdlib,
    compile,
    remove_c,
};

#[cfg(test)]
mod test;

fn main() {
    let mut bytes = read_src();
    let code = translate(&mut bytes);
    write_c_code(code).expect("Error writing c code!");
    load_stdlib().expect("Error loading stdlib!");
    let status = compile!();
    //remove_c().expect("Error removing c code!");
    if let Err(e) = status {
        panic!("Error compiling! {e}");
    }
}

