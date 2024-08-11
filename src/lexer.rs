// Add new tokens in the generate_tmakers function

use regex::Regex;

#[derive(Copy, Clone)]
pub enum TokenTypes {
    Whitespace,
    Constant,
    IntKeyword,
    VoidKeyword,
    ReturnKeyword,
    Identifier,
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    Semicolon,
    Empty
}
// I might not need this token and instead use the enum value thing
pub struct Token {
    pub token_type: TokenTypes,
    pub value: String,
}

struct TMaker<'a> {
    pub token_type: TokenTypes,
    pub regex: &'a str,
    pub re: Regex,  // Can't pre-compile these into constants
    pub priority: u16,
}

impl<'a> TMaker<'a> {
    fn new(token_type: TokenTypes,regex: &'a str, priority: u16) -> TMaker {
        let re = Regex::new(regex).unwrap();
        TMaker { token_type, regex ,re, priority}
    }
}
    
impl<'b> TMaker<'b> {
    pub fn peek(&'b self, hay: &'b str) -> Option<regex::Match> {
        //let re = Regex::new(self.regex).unwrap();
        // use the stored regex
        self.re.find(hay)
    }
}

// Consider sorting the array on priority before returning
// (will require switching to MUT)
fn generate_tmakers<'a>() -> [TMaker<'a>; 11] {

    let tmakers: [TMaker; 11] = [
        TMaker::new(TokenTypes::Whitespace, "\\s+", 100),
        TMaker::new(TokenTypes::Constant, "\\d+", 110),
        TMaker::new(TokenTypes::IntKeyword, "\\bint\\b", 110),
        TMaker::new(TokenTypes::VoidKeyword, "\\bvoid\\b", 110),
        TMaker::new(TokenTypes::ReturnKeyword, "\\breturn\\b", 110),
        TMaker::new(TokenTypes::Identifier, "\\bFIXME\\b", 110),
        TMaker::new(TokenTypes::OpenParen, "\\(", 110),
        TMaker::new(TokenTypes::CloseParen, "\\)", 110),
        TMaker::new(TokenTypes::OpenBrace, "\\{", 110),
        TMaker::new(TokenTypes::CloseBrace, "\\}", 110),
        TMaker::new(TokenTypes::Semicolon, ";", 110)
    ];

    tmakers
}

// takes a TokenMaker, looks for a match in the slice. If the begin
// of the match is 0, return the token, and length of the token.
// Otherwise return an empty token, with a 0 length
// TODO: Look for a way to make this more flexible, I don't like sending
// empty data in return.
fn get_token(source: &String, tmaker: &TMaker) -> (Token, usize) {

    // Define the Null Return
    let empty_token: Token = Token { token_type: TokenTypes::Empty, value:String::from("") };
    //(empty_token, 0)

    let Some(peek) = tmaker.peek(source)
        else { println!("{}: No match", tmaker.regex); 
        return (empty_token, 0);
    };

    if peek.start()!=0 {
        return (empty_token, 0);
    }

    let found_token: Token = Token { token_type: tmaker.token_type, value: String::from(peek.as_str())};
    let shift_marker: usize = peek.end();

    (found_token, shift_marker)
    //(empty_token, 0)
}


pub fn lexer() -> () {
    println!("Lexer Runs");
    
    // create the array of token makers
    let tmakers = generate_tmakers();

    // load source file
    let source: String = String::from("void main() { return; }");

    // initialize a token vector for the return
    let collected_tokens: Vec<Token>;

    // scan the source file with the token makers
    // as tokens are found, add to the vector
    // provide vector when end of source file 
    // test
    let t:(Token, usize)  = get_token(&source, &tmakers[3]);

    println!("Get_token test:{}", t.0.value);

    for tok in tmakers {
        //println!("{} ", tok.regex);
        let Some(rematch) = tok.peek(&source)
            else { println!("{}: No match", tok.regex); continue;};
        println!("{}: {}", tok.regex, rematch.as_str());
    }
     
}


// // Mechanism for grabbing tokens of different types, associating 
// // with a regex, and a priority for fast reordering
// pub struct TokenMaker<'a> {
//     pub token_type: TokenTypes,
//     pub regex: &'a str,
//     //pub re: Regex,  // Can't pre-compile these into constants
//     pub priority: u16,
// }

// // A peek function, which takes a haystack returns an Option<Match>.
// // In otherwords, if Regex finds a match, it returns it and the position
// // and the end (length will need to be calculated).
// // TODO: A re compilation currently occurs with every 
// // invocation of TMaker.peek()
// // Perhaps this needs to be compiled once and stored in the TMaker instead
// // of the string.
// // The Option needs to be addressed with a Some(x) ... else;
// impl<'b> TokenMaker<'b> {
//     pub fn peek(&self, hay: &'b str) -> Option<regex::Match> {
//         let re = Regex::new(self.regex).unwrap();
//         re.find(hay)
//     }
// }

// // impl<'b> TokenMaker<'b> {
// //     fn new( token_type: TokenTypes, regex: &'b str, priority: u16) -> TokenMaker
// //     {
// //         let re: Regex = Regex::new(regex).unwrap(); // TODO Do I need to address the unwrap???
// //         TokenMaker { token_type, regex, re, priority}
// //     }
// // }


// // I'm Trying to get the TokenMakers to have a compiled REGEX, but this
// // is proving difficult with the constant elements. Maybe these just don't 
// // need to be published to the main
// // The higher the priority the first applied.
// //pub const whitespace: TokenMaker = TokenMaker::new(TokenTypes::Whitespace, "\\s", 100); //
//      //TokenMaker { token_type: TokenTypes::Whitespace, regex: "\\s", priority: 100};
// pub const WHITESPACE: TokenMaker =
//     TokenMaker { token_type: TokenTypes::Whitespace, regex: "\\s+", priority: 100};
// pub const CONSTANT: TokenMaker = 
//     TokenMaker { token_type: TokenTypes::Constant, regex: "\\d+", priority: 110};
// pub const INTKEYWORD: TokenMaker = 
//     TokenMaker { token_type: TokenTypes::IntKeyword, regex: "int", priority: 90};
// pub const VOIDKEYWORD: TokenMaker = 
//     TokenMaker { token_type: TokenTypes::VoidKeyword, regex: "void", priority: 80};
// pub const RETURNKEYWORD: TokenMaker = 
//     TokenMaker { token_type: TokenTypes::ReturnKeyword, regex: "return", priority:70};
// pub const IDENTIFIER: TokenMaker = 
//     TokenMaker { token_type: TokenTypes::Identifier, regex: "FIXME", priority: 60};
// pub const OPENPAREN: TokenMaker =
//     TokenMaker { token_type: TokenTypes::OpenParen, regex: "\\(", priority:50};
// pub const CLOSEPAREN: TokenMaker = 
//     TokenMaker { token_type: TokenTypes::CloseParen, regex: "\\)", priority: 40};
// pub const OPENBRACE: TokenMaker = 
//     TokenMaker { token_type: TokenTypes::OpenBrace, regex: "{", priority: 30};
// pub const CLOSEBRACE: TokenMaker = 
//     TokenMaker { token_type: TokenTypes::CloseBrace, regex: "}", priority: 20};
// pub const SEMICOLON: TokenMaker = 
//     TokenMaker { token_type: TokenTypes::Semicolon, regex: ";", priority: 10};