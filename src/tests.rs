
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use native_regex_lib::native_regex::native_regex_set::{NativeRegexSet};
    use native_regex_lib::native_regex::{NativeRegex};
    use native_regex_lib::native_regex::captures::Captures;
    use native_regex_lib::native_regex::replacer::NoExpand;
    use std::collections::HashMap;
    use std::time::Instant;

    include!(concat!(env!("OUT_DIR"), "/regexes.rs"));


    #[test]
    fn test_symbol_regex() {
        let haystack = "sdfsdfsd symbols:  Qoij345 and Ndds fgfdg";

        let reg_test = SymbolRegex::new();

        //Make sure the regex finds all the matches
        {
            let mut capture_iter = reg_test.captures_iter(haystack);
            assert_eq!(capture_iter.next().unwrap().first().as_str(), "Qoij345");
            assert_eq!(capture_iter.next().unwrap().first().as_str(), "Ndds");
        }

        //Make sure the regex correctly identifies the captures in the first regex
        {
            let first_match = reg_test.captures(haystack).unwrap();
            assert_eq!(first_match.first().as_str(), "Qoij345");
            assert_eq!(first_match.get(1).unwrap().as_str(), "Qoij");
            assert_eq!(first_match.get(2).unwrap().as_str(), "345");
        }
    }

    #[test]
    fn optional_regex() {
        let haystack = "hep";

        let reg_test = OptionalRegex::new();

        //Make sure the regex correctly identifies the captures in the first regex
        {
            let first_match = reg_test.captures(haystack).unwrap();
            assert_eq!(first_match.first().as_str(), "hep");
            assert_eq!(first_match.get(1).unwrap().as_str(), "h");
            assert_eq!(first_match.get(2).unwrap().as_str(), "e");
            assert_eq!(first_match.get(3), None);
            assert_eq!(first_match.get(4).unwrap().as_str(), "p");
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
            assert_eq!(capture_iter.next().unwrap().first().as_str(), "123.324e44");
            assert_eq!(capture_iter.next().unwrap().first().as_str(), "123");
            assert_eq!(capture_iter.next().unwrap().first().as_str(), "123e59");
            assert_eq!(capture_iter.next().unwrap().first().as_str(), "312.45");
        }
    }


    #[test]
    fn nonword_regex() {
        let nonword_search = "This ^*&%*&^ sentence also has non-words which will   be identified";

        let reg_test = NonwordRegex::new();

        //Make sure the regex finds all the matches
        {
            let mut capture_iter = reg_test.captures_iter(nonword_search);
            assert_eq!(capture_iter.next().unwrap().first().as_str(), " ^*&%*&^ ");
            assert_eq!(capture_iter.next().unwrap().first().as_str(), " ");
            assert_eq!(capture_iter.next().unwrap().first().as_str(), " ");
            assert_eq!(capture_iter.next().unwrap().first().as_str(), " ");
            assert_eq!(capture_iter.next().unwrap().first().as_str(), "-");
            assert_eq!(capture_iter.next().unwrap().first().as_str(), " ");
            assert_eq!(capture_iter.next().unwrap().first().as_str(), " ");
            assert_eq!(capture_iter.next().unwrap().first().as_str(), "   ");
            assert_eq!(capture_iter.next().unwrap().first().as_str(), " ");
        }

    }

    #[test]
    fn grey_regex() {
        let grey_search = "Text to show the replacing of grey and gray.";

        let reg_test = GreyRegex::new();

        assert_eq!(reg_test.replace(grey_search, |_: &Captures, _| {"purple"}), "Text to show the replacing of purple and purple.");

    }

    #[test]
    fn ip_replace() {
        let ip_search = "Convert valid IP addresses 192.168.0.1 and 234.54.23.5 but not 192. or 300.45.2.4 into 32-bit numbers";

        let reg_test = Ipv4Regex::new();

        assert_eq!(reg_test.replace(ip_search, |cap: &Captures, _| {
            let mut num = 0u32;
            let mut scale = 256*256*256u32;
            for index in 1..5 {
                match cap.get(index).unwrap().as_str().parse::<u8>() {
                    Ok(byte) => { num += byte as u32 * scale; scale /= 256; }
                    Err(_) => return format!("{}", cap.first().as_str())
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
            let m = capture_iter.next().unwrap().first();
            assert_eq!(m.as_str(), "hello");
            assert_eq!(m.start(), 0);
            assert!(capture_iter.next().is_none());
        }

        //Make sure the regex finds all the matches
        {
            let mut capture_iter = end_test.captures_iter(sample_string);
            let m = capture_iter.next().unwrap().first();
            assert_eq!(m.as_str(), "hello");
            assert_eq!(m.start(), 24);
            assert!(capture_iter.next().is_none());
        }

    }


    #[test]
    fn anchor_line_regex() {

        let sample_string = "startolinesdokfpsdok \nstartofline endofline\n oaisjdoijendoline";

        let start_test = StartLineRegex::new();
        let end_test = EndLineRegex::new();

        //Make sure the regex finds all the matches
        {
            let mut capture_iter = start_test.captures_iter(sample_string);
            let m = capture_iter.next().unwrap().first();
            assert_eq!(m.as_str(), "startoline");
            let m = capture_iter.next().unwrap().first();
            assert_eq!(m.as_str(), "startofline");
            assert!(capture_iter.next().is_none());
        }

        //Make sure the regex finds all the matches
        {
            let mut capture_iter = end_test.captures_iter(sample_string);
            let m = capture_iter.next().unwrap().first();
            assert_eq!(m.as_str(), "endofline");
            let m = capture_iter.next().unwrap().first();
            assert_eq!(m.as_str(), "endoline");
            assert!(capture_iter.next().is_none());
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

        let mut map = HashMap::new();

        assert_eq!(set.matches("testing!").iter().map(|(regex, capture)| (*regex, capture.first().start())).collect::<Vec<_>>(), Vec::<(usize, usize)>::new());
        assert_eq!(set.matches("testTing!").iter().map(|(regex, capture)| (*regex, capture.first().start())).collect::<Vec<_>>(), vec![(1, 4)]);

        map.insert(0, 0);
        map.insert(2, 0);

        assert_eq!(set.matches("192.168.0.0").iter().map(|(regex, capture)| (*regex, capture.first().start())).collect::<HashMap<_, _>>(), map);

        map.clear();
        map.insert(0, 6);
        map.insert(1, 18);
        map.insert(2, 6);

        assert_eq!(set.matches("sdfsd 192.168.0.0 He45 ad").iter().map(|(regex, capture)| (*regex, capture.first().start())).collect::<HashMap<_, _>>(), map);

        let set_matches = set.matches("sdfsd 192.168.0.0 He45 ad");
        let set_matches = set_matches.iter().map(|(regex, capture)| (*regex, capture.first().as_str())).collect::<HashMap<_, _>>();

        let mut map = HashMap::new();

        map.insert(0, "192.168.0.0");
        map.insert(1, "He45");
        map.insert(2, "192.168");

        assert_eq!(set_matches, map);
    }

    #[test]
    fn empty_regex_test() {
        let reg_test = EmptyRegex::new();

        assert_eq!(reg_test.find("asitgdoisd").unwrap().as_str(), "tg");

    }

    #[test]
    fn replacer_regex_test() {
        let reg_test = SymbolRegex::new();

        let haystack = "things like He2 and Na13 also H and F";

        assert_eq!(reg_test.replace(haystack, "(S: ${1}, Q: ${2})"), format!("things like (S: He, Q: 2) and (S: Na, Q: 13) also (S: H, Q: ) and (S: F, Q: )"));
        assert_eq!(reg_test.replace(haystack, NoExpand::new("CHEMICAL")), format!("things like CHEMICAL and CHEMICAL also CHEMICAL and CHEMICAL"));
        assert_eq!(reg_test.replace(haystack, String::from("(${1}, ${2})")), format!("things like (He, 2) and (Na, 13) also (H, ) and (F, )"));
    }

    #[test]
    fn repetition_regex_test() {
        let reg = RepeatRegex::new();

        let haystack = "repeatrepeatrep";

        assert!(reg.is_match(haystack))

    }

    #[test]
    fn nth_replacer_test() {
        let haystack = "tellous telloustellousas asdstellous tellous    tellous";

        let reg = TellousRegex::new();

        assert_eq!(reg.replace(haystack, "$#"), "0 12as asds3 4    5");
    }

    #[test]
    fn rough_benchmark() {


        let haystack = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec aliquet felis vel luctus accumsan. Curabitur et mauris facilisis, vulputate nisi a, commodo dui. Pellentesque egestas fringilla justo quis venenatis. Praesent iaculis lobortis sem id mattis. Aenean eu diam aliquet, condimentum nunc sollicitudin, pulvinar dolor. Nunc non suscipit nisl. Nulla ut dictum lacus, volutpat auctor purus. Quisque blandit nulla sed dictum varius. Donec finibus, libero in placerat iaculis, ante ante pulvinar magna, ac malesuada dolor nisi a nisl. Nam quis tristique augue. Donec quis lorem efficitur, euismod diam non, consequat turpis. Donec massa leo, fermentum sit amet nisl fermentum, faucibus consequat est.Cras euismod ligula nulla, eget finibus dui fringilla id. Ut nec vestibulum urna, vitae auctor elit. Mauris leo tortor, commodo eget sagittis et, pharetra at mi. Proin lobortis in quam ut dictum. Aliquam suscipit cursus tellus, vitae auctor risus pulvinar id. Maecenas mollis metus quis placerat elementum. Pellentesque nibh metus, iaculis commodo sapien ac, lacinia accumsan eros. Fusce condimentum nisl eu dictum euismod. Etiam non blandit erat. Etiam porta augue sit amet dignissim pharetra. Duis in feugiat eros, eget elementum ipsum. In dictum lacus a odio aliquet efficitur. Nulla imperdiet metus nunc, vitae egestas neque fringilla in. Ut dolor enim, dignissim ac odio eu, blandit ultricies neque.Mauris euismod, elit in aliquam efficitur, sem dui porta mauris, sed rhoncus eros sapien eget dolor. In vel neque in mi lacinia facilisis. Donec vel tellus ac enim malesuada maximus. Cras bibendum ex nec libero venenatis vestibulum. Nam consequat nulla quam, eu congue augue luctus ut. Vestibulum bibendum pretium ex quis venenatis. Nam vitae dignissim magna. Aliquam sit amet elit ornare, lobortis urna a, fringilla sem. Aliquam in felis congue, lacinia massa eu, imperdiet lorem. Vivamus interdum rhoncus nulla, a fringilla mi ornare vel. Maecenas et tellus pellentesque, finibus mi quis, pulvinar lectus. Suspendisse potenti.Maecenas pretium justo a augue laoreet, at bibendum velit mattis. Aliquam convallis feugiat ligula, eget scelerisque ligula luctus non. Suspendisse maximus massa id est auctor, a bibendum lacus volutpat. Nulla facilisi. Nunc quis hendrerit purus, eget rutrum ex. Suspendisse finibus sed est scelerisque dapibus. Vivamus ornare hendrerit arcu, nec pharetra tellus scelerisque quis.Maecenas urna justo, imperdiet quis leo nec, luctus fermentum felis. Proin dictum vulputate egestas. Nunc sed elit eu lacus rhoncus mollis sed vel felis. Sed vitae libero mattis quam lobortis pellentesque. Vestibulum non erat nec metus lacinia porta et vitae velit. Sed nec lacus cursus, sodales diam non, tincidunt quam. Sed sollicitudin nibh lacus, ut sagittis risus pharetra quis. Praesent scelerisque elit placerat enim laoreet ultrices. Sed faucibus mollis consequat. Proin sodales a neque eu sodales. Integer vestibulum justo nec ex auctor, non mollis metus interdum. Donec varius euismod nibh ac efficitur. Mauris et molestie enim, at volutpat arcu. Nulla dolor dolor, dictum et tempus sed, imperdiet sed odio. Nulla in lorem tristique risus commodo tristique. Aenean dapibus lorem at purus efficitur fringilla.Ut vestibulum lorem quis augue interdum, nec fermentum urna facilisis. Aenean aliquet placerat vulputate. Mauris nisi ligula, tincidunt ac libero fermentum, mattis consectetur metus. Nam scelerisque facilisis suscipit. Pellentesque pretium, ipsum sit amet mattis tincidunt, lectus ante venenatis enim, a dapibus tellus ex at diam. In mattis, nunc id laoreet malesuada, metus diam blandit massa, hendrerit mollis erat justo sit amet nibh. Aliquam fermentum cursus dui vel posuere. In pulvinar massa ut dictum fringilla.In pulvinar vehicula tortor, eget rutrum sapien accumsan eget. Aenean scelerisque erat eu vulputate lobortis. Interdum et malesuada fames ac ante ipsum primis in faucibus. Donec eleifend sem elementum tempus congue. Cras porttitor suscipit quam, ac dignissim felis vestibulum suscipit. Quisque commodo ac tellus et iaculis. Proin pulvinar mollis nulla eget rutrum. Praesent gravida lorem egestas, suscipit enim eu, bibendum sem. Cras a tortor dictum, accumsan massa in, interdum augue.Nullam iaculis erat in elit rutrum, sit amet pretium nunc finibus. Vestibulum imperdiet dolor purus, in tempus nunc finibus at. Ut cursus sagittis facilisis. Fusce eu sem in magna ultrices egestas. Donec vestibulum risus ut ipsum congue pellentesque. Sed bibendum lectus ac neque pretium, eu blandit sem ultrices. Vestibulum sit amet cursus eros. Vestibulum a luctus urna, a eleifend tortor. Vestibulum massa quam, pellentesque et nunc volutpat, dapibus vehicula quam. Suspendisse vitae sapien turpis. Quisque consequat nisl sed odio varius, mattis feugiat odio auctor. Donec condimentum lorem tellus, eu vestibulum dolor sagittis nec. Quisque commodo, lorem quis sodales maximus, elit dui dapibus tellus, id auctor elit velit ac dolor. Mauris at ligula nec turpis interdum sagittis in vel turpis.Proin et condimentum urna. Morbi pellentesque placerat laoreet. Nunc vel risus et risus fermentum pulvinar. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Fusce ac ante tincidunt, consectetur felis sed, semper eros. Fusce vel auctor diam. Sed mollis bibendum urna, ut venenatis augue dapibus et. Vivamus turpis justo, aliquam non mattis eget, porta sed erat. Duis vehicula lectus sit amet neque fermentum fringilla.Cras vulputate leo massa, in tincidunt ex pulvinar in. Donec feugiat, massa quis egestas sagittis, est leo condimentum purus, at convallis risus nisi sit amet purus. Praesent at velit nisi. Fusce euismod neque urna, tempor commodo libero semper a. Praesent sagittis purus sit amet diam vehicula, et fringilla ex porta. Suspendisse potenti. Donec felis neque, mollis et sodales sit amet, sodales a ipsum. Maecenas vehicula, arcu id mollis porta, ipsum magna varius libero, ullamcorper posuere nisl quam quis ipsum. Integer id congue lacus. Cras eu ipsum lobortis, accumsan augue in, elementum mauris. Duis ligula elit, mollis in purus sed, sollicitudin ultrices arcu.Integer lacinia, dolor nec sollicitudin fringilla, purus urna sodales urna, sit amet sollicitudin magna elit quis odio. In tristique lacus sed ante finibus, nec euismod ligula facilisis. Nunc porta laoreet libero, sit amet bibendum elit convallis id. Proin sapien massa, lacinia in risus id, porttitor rutrum lacus. Nulla facilisi. Praesent in felis a velit accumsan sollicitudin. Donec ac consectetur diam. Integer sit amet urna feugiat, vehicula lorem non, bibendum erat. Quisque consequat risus non dui elementum tristique. Maecenas non sapien vel tellus vehicula luctus. Ut non fringilla ipsum, vitae malesuada sapien. Cras ac lacus enim. Nulla accumsan justo tristique venenatis pharetra.In vitae sem ut purus placerat bibendum. Ut enim mi, eleifend a malesuada non, tristique ac odio. Phasellus gravida ultrices orci eu vestibulum. Sed lobortis cursus lorem, eu mollis quam. Curabitur sodales dictum dolor ut interdum. Quisque eget convallis libero. Pellentesque at convallis libero, nec volutpat libero. Suspendisse sed est interdum, aliquam risus et, dapibus nunc.Sed id orci enim. Pellentesque ultrices scelerisque semper. Morbi eu ligula mi. Proin et ullamcorper quam. Donec convallis volutpat est in tempus. Etiam suscipit mollis turpis, a pretium nibh feugiat in. Suspendisse gravida ac turpis sit amet ultrices. Sed quis turpis condimentum risus porta semper et vitae justo. Praesent vel diam porttitor, sagittis eros et, ornare eros. Morbi vitae justo felis. Nulla aliquet neque a imperdiet pharetra. Maecenas interdum malesuada lorem, eu hendrerit purus finibus id. Fusce cursus ipsum ipsum, vitae maximus velit cursus id. Aliquam ante nunc, condimentum vel faucibus ac, faucibus et risus. Sed rutrum convallis venenatis. Nunc semper augue ac augue dictum, et hendrerit tellus pretium.Etiam sodales eleifend velit et efficitur. Mauris id egestas nunc. Nulla at erat eget nisl facilisis vehicula. Integer ut mauris ut elit feugiat feugiat. In eget ex cursus, iaculis nunc quis, blandit risus. Praesent ac arcu dolor. Mauris et cursus magna. Morbi fermentum ac erat ac dictum. Quisque dapibus lacus eu maximus iaculis. Fusce luctus neque a diam finibus, et sollicitudin nibh faucibus. Donec sit amet fermentum risus. Nulla facilisi. Sed vulputate varius lorem eget lobortis.Vestibulum lectus libero, rhoncus ut nulla feugiat, dapibus luctus tellous. Quisque tincidunt eros metus, et convallis urna elementum sit amet. Pellentesque laoreet lobortis est ac imperdiet. Nam venenatis purus ut dui laoreet, et dignissim tortor egestas. Fusce vitae tristique nulla. Curabitur eleifend enim mauris, quis faucibus ligula scelerisque eget. Sed lacinia, ante at commodo tempor, tortor neque convallis arcu, non pharetra lorem ante et erat. Nam nec imperdiet leo. Nullam porttitor mauris at ullamcorper ullamcorper. Duis nibh quam, vestibulum ut ante ut, eleifend eleifend enim. Quisque sodales tincidunt augue eget varius.z";

        let reg = TellousRegex::new();

        let now = Instant::now();
        reg.is_match(haystack);
        let duration = now.elapsed();

        if duration.as_micros() > 40 {
            panic!(format!("Rouch benchmark failed. Duration: {:?}", duration));
        }

    }

}
