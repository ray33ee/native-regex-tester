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

    let regexes = [
        ("([A-Z][a-z]*)([0-9]+)?", "symbol_function"), //Match chemical symbols, capturing the symbol and the quantity
        ("([0-9]{1,3})\\.([0-9]{1,3})\\.([0-9]{1,3})\\.([0-9]{1,3})", "ipv4_function"), //Match IPV4 addresses, capturing the 4 byte values
        ("\\s+", "whitespace_function"),
        ("[-+]?[0-9]+(?:\\.[0-9]+)?(?:[eE][-+]?[0-9]+)?", "float_function"),
        ("[\\W]+", "nonword_function"),
        ("gr[ea]y", "grey_function"),
        ("\\b[0-9]+\\b", "wordboundary_function")
    ];

    for (regex, function) in regexes.iter() {
        f.write_all(translate(*regex, *function).unwrap().as_bytes()).unwrap();
    }


}