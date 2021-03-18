
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use native_regex_lib::native_regex::NativeRegex;
    use native_regex_lib::parse::Token::Alternation;

    include!(concat!(env!("OUT_DIR"), "/regexes.rs"));

    #[test]
    fn test_symbol_regex() {
        let haystack = "sdfsdfsd symbols:  Qoij345 and Ndds fgfdg";

        let reg_test = NativeRegex::new(symbol_function);

        //Make sure the regex finds all the matches
        {
            let mut capture_iter = reg_test.captures_iter(haystack);
            assert_eq!(capture_iter.next().unwrap().get(0).unwrap().as_str(), "Qoij345");
            assert_eq!(capture_iter.next().unwrap().get(0).unwrap().as_str(), "Ndds");
        }

        //Make sure the regex correctly identifies the captures in the first regex
        {
            let first_match = reg_test.captures(haystack).unwrap();
            assert_eq!(first_match.get(0).unwrap().as_str(), "Qoij345");
            assert_eq!(first_match.get(1).unwrap().as_str(), "Qoij");
            assert_eq!(first_match.get(2).unwrap().as_str(), "345");
        }
    }

    #[test]
    fn test_ipv4_regex() {

        let ipv4_search = "aoksfdsf 192.168.0.1 [aspspd";

        let reg_test = NativeRegex::new(ipv4_function);

        let first_match = reg_test.captures(ipv4_search).unwrap();

        assert_eq!(first_match.get(1).unwrap().as_str().parse::<u8>().unwrap(), 192);
        assert_eq!(first_match.get(2).unwrap().as_str().parse::<u8>().unwrap(), 168);
        assert_eq!(first_match.get(3).unwrap().as_str().parse::<u8>().unwrap(), 0);
        assert_eq!(first_match.get(4).unwrap().as_str().parse::<u8>().unwrap(), 1);
    }



    #[test]
    fn whitespace_regex() {

        let ipv4_search = "this will test the \n whitespace     spliiter    function";

        let reg_test = NativeRegex::new(whitespace_function);

        assert_eq!(reg_test.split(ipv4_search).collect::<Vec<_>>(), ["this", "will", "test", "the", "whitespace", "spliiter", "function"]);
    }

    #[test]
    fn float_regex() {

        let float_search = "dfoisdjf 123.324e44, 123, 123e59, 312.45 aoskp;osdk";

        let reg_test = NativeRegex::new(float_function);



        //Make sure the regex finds all the matches
        {
            let mut capture_iter = reg_test.captures_iter(float_search);
            assert_eq!(capture_iter.next().unwrap().get(0).unwrap().as_str(), "123.324e44");
            assert_eq!(capture_iter.next().unwrap().get(0).unwrap().as_str(), "123");
            assert_eq!(capture_iter.next().unwrap().get(0).unwrap().as_str(), "123e59");
            assert_eq!(capture_iter.next().unwrap().get(0).unwrap().as_str(), "312.45");
        }
    }

    #[test]
    fn nonword_regex() {
        let nonword_search = "This ^*&%*&^ sentence also has non-words which will   be identified";

        let reg_test = NativeRegex::new(nonword_function);

        //Make sure the regex finds all the matches
        {
            let mut capture_iter = reg_test.captures_iter(nonword_search);
            assert_eq!(capture_iter.next().unwrap().get(0).unwrap().as_str(), " ^*&%*&^ ");
            assert_eq!(capture_iter.next().unwrap().get(0).unwrap().as_str(), " ");
            assert_eq!(capture_iter.next().unwrap().get(0).unwrap().as_str(), " ");
            assert_eq!(capture_iter.next().unwrap().get(0).unwrap().as_str(), " ");
            assert_eq!(capture_iter.next().unwrap().get(0).unwrap().as_str(), "-");
            assert_eq!(capture_iter.next().unwrap().get(0).unwrap().as_str(), " ");
            assert_eq!(capture_iter.next().unwrap().get(0).unwrap().as_str(), " ");
            assert_eq!(capture_iter.next().unwrap().get(0).unwrap().as_str(), "   ");
            assert_eq!(capture_iter.next().unwrap().get(0).unwrap().as_str(), " ");
        }

    }

    #[test]
    fn grey_regex() {
        let grey_search = "Text to show the replacing of grey and gray.";

        let reg_test = NativeRegex::new(grey_function);

        assert_eq!(reg_test.replace(grey_search, |_, _| String::from("purple")).as_str(), "Text to show the replacing of purple and purple.");

    }

    #[test]
    fn ip_replace() {
        let ip_search = "Convert valid IP addresses 192.168.0.1 and 234.54.23.5 but not 192. or 300.45.2.4 into 32-bit numbers";

        let reg_test = NativeRegex::new(ipv4_function);

        assert_eq!(reg_test.replace(ip_search, |i, cap| {
            let mut num = 0u32;
            let mut scale = 256*256*256u32;
            for index in 1..5 {
                match cap.get(index).unwrap().as_str().parse::<u8>() {
                    Ok(byte) => { num += byte as u32 * scale; scale /= 256; }
                    Err(_) => return format!("{}", cap.get(0).unwrap().as_str())
                }
            }
            format!("{}", num)
        }), "Convert valid IP addresses 3232235521 and 3929413381 but not 192. or 300.45.2.4 into 32-bit numbers")

    }

    #[test]
    fn wordboundary_regex() {

        let sample_string = "23 we234s dk2 23423 34 3";

        let reg_test = NativeRegex::new(wordboundary_function);

        //Make sure the regex finds all the matches
        {
            let mut capture_iter = reg_test.captures_iter(sample_string);
            assert_eq!(capture_iter.next().unwrap().get(0).unwrap().as_str(), "23");
            assert_eq!(capture_iter.next().unwrap().get(0).unwrap().as_str(), "23423");
            assert_eq!(capture_iter.next().unwrap().get(0).unwrap().as_str(), "34");
            assert_eq!(capture_iter.next().unwrap().get(0).unwrap().as_str(), "3");
        }

    }

}
