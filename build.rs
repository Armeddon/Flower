use std::{fs, io::{Seek, SeekFrom, Write}};

fn main() {
    let stdlib_h = String::from_utf8(fs::read("flwrstdlib/flwrstdlib.h").unwrap()).unwrap();
    let stdlib_c = String::from_utf8(fs::read("flwrstdlib/flwrstdlib.c").unwrap()).unwrap();
    let varlist_h = String::from_utf8(fs::read("flwrstdlib/varlist.h").unwrap()).unwrap();
    let varlist_c = String::from_utf8(fs::read("flwrstdlib/varlist.c").unwrap()).unwrap();
    let string_h = String::from_utf8(fs::read("flwrstdlib/string.h").unwrap()).unwrap();
    let string_c = String::from_utf8(fs::read("flwrstdlib/string.c").unwrap()).unwrap();

    fs::File::create("src/stdlib.rs").unwrap().set_len(0).unwrap();
    fs::File::create("src/stdlib.rs").unwrap().seek(SeekFrom::End(0)).unwrap();

    std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open("src/stdlib.rs").unwrap()
        .write_all(format!("pub const STDLIB_H: &[u8] = r#\"{stdlib_h}\"#.as_bytes();\n").as_bytes())
        .unwrap();
    std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open("src/stdlib.rs").unwrap()
        .write_all(format!("pub const STDLIB_C: &[u8] = r#\"{stdlib_c}\"#.as_bytes();\n").as_bytes())
        .unwrap();
    std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open("src/stdlib.rs").unwrap()
        .write_all(format!("pub const VARLIST_H: &[u8] = r#\"{varlist_h}\"#.as_bytes();\n").as_bytes())
        .unwrap();
    std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open("src/stdlib.rs").unwrap()
        .write_all(format!("pub const VARLIST_C: &[u8] = r#\"{varlist_c}\"#.as_bytes();\n").as_bytes())
        .unwrap();
    std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open("src/stdlib.rs").unwrap()
        .write_all(format!("pub const STRING_H: &[u8] = r#\"{string_h}\"#.as_bytes();\n").as_bytes())
        .unwrap();
    std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open("src/stdlib.rs").unwrap()
        .write_all(format!("pub const STRING_C: &[u8] = r#\"{string_c}\"#.as_bytes();\n").as_bytes())
        .unwrap();
}
