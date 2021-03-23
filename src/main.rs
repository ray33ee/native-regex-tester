use native_regex_lib::native_regex::NativeRegex;

mod tests;


include!(concat!(env!("OUT_DIR"), "/regexes.rs"));


fn main() {
    println!("This package is for testing only. Please run `cargo test`");

}
