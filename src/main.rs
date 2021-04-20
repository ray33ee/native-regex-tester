

mod tests;

use std::time::{Instant};

use native_regex_lib::native_regex::NativeRegex;
use regex::Regex;

use native_regex_lib::native_regex::character::{AdvancerIterator};

pub struct ExampleRegex1 {
    named_groups: std::collections::HashMap<& 'static str, usize>
}

impl ExampleRegex1 {
    pub fn new() -> Self {
        let named_groups = std::collections::HashMap::new();



        ExampleRegex1 {
            named_groups
        }
    }
}

impl Into<native_regex_lib::native_regex::Engine> for ExampleRegex1 {

    fn into(self) -> native_regex_lib::native_regex::Engine {
        self.engine()
    }

}

impl native_regex_lib::native_regex::NativeRegex for ExampleRegex1 {

    // Function to match regex '(repeat)+'
    #[allow(unused_parens, unused_comparisons)]
    #[inline(always)]
    fn step(mut chars: native_regex_lib::native_regex::character::Advancer, captures: & mut native_regex_lib::vectormap::VectorMap<(usize, usize)>) -> Option<()> {

        //Advance to first character & bounds check
        let mut character = chars.advance();

        if character.current().is_some() {  } else  { return None; }

        {

            let capture_0_start = character.index();

            {

                let mut match_count = 0;

                let mut last = chars.clone();

                while character.current().is_some() {

                    {

                        let capture_1_start = character.index();

                        if character.current().is_some() {  } else  { return None; }

                        if (character.current().unwrap() as u32) == 114 {  } else  { break; }

                        character = chars.advance();

                        if character.current().is_some() {  } else  { return None; }

                        if (character.current().unwrap() as u32) == 101 {  } else  { break; }

                        character = chars.advance();


                        captures.insert(1, (capture_1_start, character.index()));

                        last = chars.clone();

                    }

                    match_count += 1;

                }

                chars = last;

                if match_count < 1 { return None; }

            }

            captures.insert(0, (capture_0_start, character.index()));

        }



        Some(())
    }

    fn capture_names(&self) -> &std::collections::HashMap<& 'static str, usize> {
        &self.named_groups
    }

    fn capture_count(&self) -> usize { 2 }

}

fn my_search(haystack: & str, needle: & str) -> bool {
    let count = haystack.len() - needle.len();

    for i in 0..count {
        if &haystack[i..i+needle.len()] == needle {
            return true;
        }
    }

    false
}


fn main() {
    println!("This package is for testing only. Please run `cargo test`");

    let haystack = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec aliquet felis vel luctus accumsan. Curabitur et mauris facilisis, vulputate nisi a, commodo dui. Pellentesque egestas fringilla justo quis venenatis. Praesent iaculis lobortis sem id mattis. Aenean eu diam aliquet, condimentum nunc sollicitudin, pulvinar dolor. Nunc non suscipit nisl. Nulla ut dictum lacus, volutpat auctor purus. Quisque blandit nulla sed dictum varius. Donec finibus, libero in placerat iaculis, ante ante pulvinar magna, ac malesuada dolor nisi a nisl. Nam quis tristique augue. Donec quis lorem efficitur, euismod diam non, consequat turpis. Donec massa leo, fermentum sit amet nisl fermentum, faucibus consequat est.Cras euismod ligula nulla, eget finibus dui fringilla id. Ut nec vestibulum urna, vitae auctor elit. Mauris leo tortor, commodo eget sagittis et, pharetra at mi. Proin lobortis in quam ut dictum. Aliquam suscipit cursus tellus, vitae auctor risus pulvinar id. Maecenas mollis metus quis placerat elementum. Pellentesque nibh metus, iaculis commodo sapien ac, lacinia accumsan eros. Fusce condimentum nisl eu dictum euismod. Etiam non blandit erat. Etiam porta augue sit amet dignissim pharetra. Duis in feugiat eros, eget elementum ipsum. In dictum lacus a odio aliquet efficitur. Nulla imperdiet metus nunc, vitae egestas neque fringilla in. Ut dolor enim, dignissim ac odio eu, blandit ultricies neque.Mauris euismod, elit in aliquam efficitur, sem dui porta mauris, sed rhoncus eros sapien eget dolor. In vel neque in mi lacinia facilisis. Donec vel tellus ac enim malesuada maximus. Cras bibendum ex nec libero venenatis vestibulum. Nam consequat nulla quam, eu congue augue luctus ut. Vestibulum bibendum pretium ex quis venenatis. Nam vitae dignissim magna. Aliquam sit amet elit ornare, lobortis urna a, fringilla sem. Aliquam in felis congue, lacinia massa eu, imperdiet lorem. Vivamus interdum rhoncus nulla, a fringilla mi ornare vel. Maecenas et tellus pellentesque, finibus mi quis, pulvinar lectus. Suspendisse potenti.Maecenas pretium justo a augue laoreet, at bibendum velit mattis. Aliquam convallis feugiat ligula, eget scelerisque ligula luctus non. Suspendisse maximus massa id est auctor, a bibendum lacus volutpat. Nulla facilisi. Nunc quis hendrerit purus, eget rutrum ex. Suspendisse finibus sed est scelerisque dapibus. Vivamus ornare hendrerit arcu, nec pharetra tellus scelerisque quis.Maecenas urna justo, imperdiet quis leo nec, luctus fermentum felis. Proin dictum vulputate egestas. Nunc sed elit eu lacus rhoncus mollis sed vel felis. Sed vitae libero mattis quam lobortis pellentesque. Vestibulum non erat nec metus lacinia porta et vitae velit. Sed nec lacus cursus, sodales diam non, tincidunt quam. Sed sollicitudin nibh lacus, ut sagittis risus pharetra quis. Praesent scelerisque elit placerat enim laoreet ultrices. Sed faucibus mollis consequat. Proin sodales a neque eu sodales. Integer vestibulum justo nec ex auctor, non mollis metus interdum. Donec varius euismod nibh ac efficitur. Mauris et molestie enim, at volutpat arcu. Nulla dolor dolor, dictum et tempus sed, imperdiet sed odio. Nulla in lorem tristique risus commodo tristique. Aenean dapibus lorem at purus efficitur fringilla.Ut vestibulum lorem quis augue interdum, nec fermentum urna facilisis. Aenean aliquet placerat vulputate. Mauris nisi ligula, tincidunt ac libero fermentum, mattis consectetur metus. Nam scelerisque facilisis suscipit. Pellentesque pretium, ipsum sit amet mattis tincidunt, lectus ante venenatis enim, a dapibus tellus ex at diam. In mattis, nunc id laoreet malesuada, metus diam blandit massa, hendrerit mollis erat justo sit amet nibh. Aliquam fermentum cursus dui vel posuere. In pulvinar massa ut dictum fringilla.In pulvinar vehicula tortor, eget rutrum sapien accumsan eget. Aenean scelerisque erat eu vulputate lobortis. Interdum et malesuada fames ac ante ipsum primis in faucibus. Donec eleifend sem elementum tempus congue. Cras porttitor suscipit quam, ac dignissim felis vestibulum suscipit. Quisque commodo ac tellus et iaculis. Proin pulvinar mollis nulla eget rutrum. Praesent gravida lorem egestas, suscipit enim eu, bibendum sem. Cras a tortor dictum, accumsan massa in, interdum augue.Nullam iaculis erat in elit rutrum, sit amet pretium nunc finibus. Vestibulum imperdiet dolor purus, in tempus nunc finibus at. Ut cursus sagittis facilisis. Fusce eu sem in magna ultrices egestas. Donec vestibulum risus ut ipsum congue pellentesque. Sed bibendum lectus ac neque pretium, eu blandit sem ultrices. Vestibulum sit amet cursus eros. Vestibulum a luctus urna, a eleifend tortor. Vestibulum massa quam, pellentesque et nunc volutpat, dapibus vehicula quam. Suspendisse vitae sapien turpis. Quisque consequat nisl sed odio varius, mattis feugiat odio auctor. Donec condimentum lorem tellus, eu vestibulum dolor sagittis nec. Quisque commodo, lorem quis sodales maximus, elit dui dapibus tellus, id auctor elit velit ac dolor. Mauris at ligula nec turpis interdum sagittis in vel turpis.Proin et condimentum urna. Morbi pellentesque placerat laoreet. Nunc vel risus et risus fermentum pulvinar. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Fusce ac ante tincidunt, consectetur felis sed, semper eros. Fusce vel auctor diam. Sed mollis bibendum urna, ut venenatis augue dapibus et. Vivamus turpis justo, aliquam non mattis eget, porta sed erat. Duis vehicula lectus sit amet neque fermentum fringilla.Cras vulputate leo massa, in tincidunt ex pulvinar in. Donec feugiat, massa quis egestas sagittis, est leo condimentum purus, at convallis risus nisi sit amet purus. Praesent at velit nisi. Fusce euismod neque urna, tempor commodo libero semper a. Praesent sagittis purus sit amet diam vehicula, et fringilla ex porta. Suspendisse potenti. Donec felis neque, mollis et sodales sit amet, sodales a ipsum. Maecenas vehicula, arcu id mollis porta, ipsum magna varius libero, ullamcorper posuere nisl quam quis ipsum. Integer id congue lacus. Cras eu ipsum lobortis, accumsan augue in, elementum mauris. Duis ligula elit, mollis in purus sed, sollicitudin ultrices arcu.Integer lacinia, dolor nec sollicitudin fringilla, purus urna sodales urna, sit amet sollicitudin magna elit quis odio. In tristique lacus sed ante finibus, nec euismod ligula facilisis. Nunc porta laoreet libero, sit amet bibendum elit convallis id. Proin sapien massa, lacinia in risus id, porttitor rutrum lacus. Nulla facilisi. Praesent in felis a velit accumsan sollicitudin. Donec ac consectetur diam. Integer sit amet urna feugiat, vehicula lorem non, bibendum erat. Quisque consequat risus non dui elementum tristique. Maecenas non sapien vel tellus vehicula luctus. Ut non fringilla ipsum, vitae malesuada sapien. Cras ac lacus enim. Nulla accumsan justo tristique venenatis pharetra.In vitae sem ut purus placerat bibendum. Ut enim mi, eleifend a malesuada non, tristique ac odio. Phasellus gravida ultrices orci eu vestibulum. Sed lobortis cursus lorem, eu mollis quam. Curabitur sodales dictum dolor ut interdum. Quisque eget convallis libero. Pellentesque at convallis libero, nec volutpat libero. Suspendisse sed est interdum, aliquam risus et, dapibus nunc.Sed id orci enim. Pellentesque ultrices scelerisque semper. Morbi eu ligula mi. Proin et ullamcorper quam. Donec convallis volutpat est in tempus. Etiam suscipit mollis turpis, a pretium nibh feugiat in. Suspendisse gravida ac turpis sit amet ultrices. Sed quis turpis condimentum risus porta semper et vitae justo. Praesent vel diam porttitor, sagittis eros et, ornare eros. Morbi vitae justo felis. Nulla aliquet neque a imperdiet pharetra. Maecenas interdum malesuada lorem, eu hendrerit purus finibus id. Fusce cursus ipsum ipsum, vitae maximus velit cursus id. Aliquam ante nunc, condimentum vel faucibus ac, faucibus et risus. Sed rutrum convallis venenatis. Nunc semper augue ac augue dictum, et hendrerit tellus pretium.Etiam sodales eleifend velit et efficitur. Mauris id egestas nunc. Nulla at erat eget nisl facilisis vehicula. Integer ut mauris ut elit feugiat feugiat. In eget ex cursus, iaculis nunc quis, blandit risus. Praesent ac arcu dolor. Mauris et cursus magna. Morbi fermentum ac erat ac dictum. Quisque dapibus lacus eu maximus iaculis. Fusce luctus neque a diam finibus, et sollicitudin nibh faucibus. Donec sit amet fermentum risus. Nulla facilisi. Sed vulputate varius lorem eget lobortis.Vestibulum lectus libero, rhoncus ut nulla feugiat, dapibus luctus tellous. Quisque tincidunt eros metus, et convallis urna elementum sit amet. Pellentesque laoreet lobortis est ac imperdiet. Nam venenatis purus ut dui laoreet, et dignissim tortor egestas. Fusce vitae tristique nulla. Curabitur eleifend enim mauris, quis faucibus ligula scelerisque eget. Sed lacinia, ante at commodo tempor, tortor neque convallis arcu, non pharetra lorem ante et erat. Nam nec imperdiet leo. Nullam porttitor mauris at ullamcorper ullamcorper. Duis nibh quam, vestibulum ut ante ut, eleifend eleifend enim. Quisque sodales tincidunt augue eget varius.z";

    let now = Instant::now();
    haystack.contains("tellous");
    let duration = now.elapsed();

    println!("Contains duration: {:?}", duration);

    let now = Instant::now();
    my_search(haystack, "999999999");
    let duration = now.elapsed();

    println!("My search duration: {:?}", duration);

    let reg = Regex::new("d").unwrap();

    let now = Instant::now();
    reg.is_match(haystack);
    let duration = now.elapsed();

    println!("Regex duration: {:?}", duration);

    let reg = ExampleRegex1::new();

    let now = Instant::now();
    let thing = reg.captures("rererer");
    let duration = now.elapsed();

    println!("Native duration: {:?} {:?}", duration, thing);



    let iterator = AdvancerIterator::new(&haystack[..], 0);

    let now = Instant::now();
    for mut iteriter in iterator {
        iteriter.advance();

    }
    let duration = now.elapsed();

    println!("Iteriter duration: {:?}", duration);

}
