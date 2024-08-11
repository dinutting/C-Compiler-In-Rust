mod lexer;
use regex::{Match, Regex};
use lexer::*;
use json::*;
//mod regexLoader;

fn main() {
    //say_hello();
    println!("Hello, world!");
    lexer::lexer();

    // let instantiated = regexLoader::load_tokens();

    // println!("{}",instantiated["tokens"][0]["tokenType"]);
    //println!("{}", WHITESPACE.regex);

    // let re = Regex::new(RETURNKEYWORD.regex).unwrap();

    // let Some(caps) = re.captures("test return thing")
    //     else { println!("no match!"); return;};

    //let rematch: Match = RETURNKEYWORD.rematch("my return is nigh");

    //let Some(rematch) = INTKEYWORD.rematch("int main()")
    //    else { println!("No match"); return; };

    //rematch;
    //println!("{}", rematch.as_str());
}
