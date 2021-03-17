
use native_regex_lib::native_regex_helper::NativeRegex;

include!(concat!(env!("OUT_DIR"), "/regexes.rs"));

fn main() {

    let haystack = "sdgDf gsDd333 sDdlf";

    let reg_test = NativeRegex::new(symbol_function);

    for all_captures in reg_test.captures_iter(haystack) {
        println!("Entire: {}", all_captures.get(0).unwrap().as_str());
        for caps in all_captures.iter() {
            match caps {
                Some(m) => println!("    Capture: {}", m.as_str()),
                None => println!("No capture")
            }
        }
    }

    let split: Vec<_> = reg_test.split(haystack).collect();

    println!("Split: {:?}", split);

}
