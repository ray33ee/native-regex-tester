// build.rs

use std::io::Write;
use std::env;
use std::fs::File;
use std::path::Path;

use native_regex_lib::rust_translate::translate;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("regexes.rs");
    let mut f = File::create(&dest_path).unwrap();

    let code = translate("([A-Z][a-z]*)([0-9]+)?", "symbol_function").unwrap();

    File::create("test.log").unwrap().write_all(code.as_bytes()).unwrap();

    f.write_all(code.as_bytes()).unwrap();

}