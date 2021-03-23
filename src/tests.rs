
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use native_regex_lib::native_regex::{NativeRegex, NativeRegexSet};

    include!(concat!(env!("OUT_DIR"), "/regexes.rs"));


    #[test]
    fn test_symbol_regex() {
        let haystack = "sdfsdfsd symbols:  Qoij345 and Ndds fgfdg";

        let reg_test = SymbolRegex::new();

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
        let ipv4_search2 = "168.0.1";

        let reg_test = Ipv4Regex::new();

        assert!(!reg_test.is_match(ipv4_search2));

        let first_match = reg_test.captures(ipv4_search).unwrap();

        assert_eq!(first_match.get(1).unwrap().as_str().parse::<u8>().unwrap(), 192);
        assert_eq!(first_match.get(2).unwrap().as_str().parse::<u8>().unwrap(), 168);
        assert_eq!(first_match.get(3).unwrap().as_str().parse::<u8>().unwrap(), 0);
        assert_eq!(first_match.get(4).unwrap().as_str().parse::<u8>().unwrap(), 1);
    }



    #[test]
    fn whitespace_regex() {

        let ipv4_search = "this will test the \n whitespace     spliiter    function";

        let reg_test = WhitespaceRegex::new();

        assert_eq!(reg_test.split(ipv4_search).collect::<Vec<_>>(), ["this", "will", "test", "the", "whitespace", "spliiter", "function"]);
    }

    #[test]
    fn float_regex() {

        let float_search = "dfoisdjf 123.324e44, 123, 123e59, 312.45 aoskp;osdk";

        let reg_test = FloatRegex::new();



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

        let reg_test = NonwordRegex::new();

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

        let reg_test = GreyRegex::new();

        assert_eq!(reg_test.replace(grey_search, |_, _| String::from("purple")).as_str(), "Text to show the replacing of purple and purple.");

    }

    #[test]
    fn ip_replace() {
        let ip_search = "Convert valid IP addresses 192.168.0.1 and 234.54.23.5 but not 192. or 300.45.2.4 into 32-bit numbers";

        let reg_test = Ipv4Regex::new();

        assert_eq!(reg_test.replace(ip_search, |_, cap| {
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

        let reg_test = WordboundaryRegex::new();

        //Make sure the regex finds all the matches
        {
            let mut capture_iter = reg_test.captures_iter(sample_string);
            assert_eq!(capture_iter.next().unwrap().get(0).unwrap().as_str(), "23");
            assert_eq!(capture_iter.next().unwrap().get(0).unwrap().as_str(), "23423");
            assert_eq!(capture_iter.next().unwrap().get(0).unwrap().as_str(), "34");
            assert_eq!(capture_iter.next().unwrap().get(0).unwrap().as_str(), "3");
        }

    }

    #[test]
    fn anchor_regex() {

        let sample_string = "hello hellohelloh hello hello";

        let start_test = StartRegex::new();
        let end_test = EndRegex::new();

        //Make sure the regex finds all the matches
        {
            let mut capture_iter = start_test.captures_iter(sample_string);
            let m = capture_iter.next().unwrap().get(0).unwrap();
            assert_eq!(m.as_str(), "hello");
            assert_eq!(m.start(), 0);
            assert_eq!(capture_iter.next(), None);
        }

        //Make sure the regex finds all the matches
        {
            let mut capture_iter = end_test.captures_iter(sample_string);
            let m = capture_iter.next().unwrap().get(0).unwrap();
            assert_eq!(m.as_str(), "hello");
            assert_eq!(m.start(), 24);
            assert_eq!(capture_iter.next(), None);
        }

    }

    #[test]
    fn name_test() {
        let sample = "asdos Na33 ps";

        let reg_test = NamesRegex::new();

        let cap = reg_test.captures(sample).unwrap();

        assert_eq!(cap.name("symbol").unwrap().as_str(), "Na");
        assert_eq!(cap.name("quantity").unwrap().as_str(), "33");

    }

    #[test]
    fn is_match_regset_test() {
        let ipv4 = Ipv4Regex::new();
        let symbol = SymbolRegex::new();
        let float = FloatRegex::new();

        let set = NativeRegexSet::new(vec![ipv4.engine(), symbol.engine(), float.engine()]);

        assert!(!set.is_match("this shouldn't match"));
        assert!(set.is_match("This should though"));
        assert!(set.is_match("This too 192.123.1.2"));
        assert!(set.is_match("and this 123.5e4"));
        assert!(!set.is_match("but this won't"));



    }

    #[test]
    fn matches_regset_test() {
        let ipv4 = Ipv4Regex::new();
        let symbol = SymbolRegex::new();
        let float = FloatRegex::new();

        let set = NativeRegexSet::new(vec![ipv4.into(), symbol.into(), float.into()]);

        assert_eq!(set.matches("testing!").iter().map(|x| *x).collect::<Vec<_>>(), Vec::<(usize, usize)>::new());
        assert_eq!(set.matches("testTing!").iter().map(|x| *x).collect::<Vec<_>>(), vec![(1, 4)]);
        assert_eq!(set.matches("192.168.0.0").iter().map(|x| *x).collect::<Vec<_>>(), vec![(0, 0), (2, 0)]);
        assert_eq!(set.matches("sdfsd 192.168.0.0 He45 ad").iter().map(|x| *x).collect::<Vec<_>>(), vec![(0, 6), (2, 6), (1, 18)]);


    }

}
