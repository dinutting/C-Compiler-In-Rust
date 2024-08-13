//#![allow(dead_code)]
// Add new tokens in the generate_tmakers function

use regex::Regex;

/// An enumeration used for tracking the different types of 
/// Tokens in the lexer. 
/// - When adding, or changing, the TokenTypes,
/// Make sure to update the generate_tmakers function.
/// 
/// I wanted to approach this as constant values, but the Regex
/// crate doesn't work for const functions. lazy_regex might work.
#[derive(Copy, Clone, PartialEq)]
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

impl std::fmt::Display for TokenTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TokenTypes::Whitespace => write!(f, "Whitespace"),
            TokenTypes::Constant => write!(f, "Constant"),
            TokenTypes::IntKeyword => write!(f, "IntKeyword"),
            TokenTypes::VoidKeyword => write!(f, "VoidKeyword"),
            TokenTypes::ReturnKeyword => write!(f, "ReturnKeyword"),
            TokenTypes::Identifier => write!(f, "Identifier"),
            TokenTypes::OpenParen => write!(f, "OpenParen"),
            TokenTypes::CloseParen => write!(f, "CloseParen"),
            TokenTypes::OpenBrace => write!(f, "OpenBrace"),
            TokenTypes::CloseBrace => write!(f, "CloseBrace"),
            TokenTypes::Semicolon => write!(f, "Semicolon"),
            TokenTypes::Empty => write!(f, "Empty")
        }
    }
}

/// Generates a fixed array of Token Makers (TMaker)
/// 
/// ## Example
/// ```
/// let tmakers = generate_tmakers();
/// ```
/// 
/// ## TODO
/// Consider sorting the array on priority before returning
/// (will require switching to MUT)
fn generate_tmakers<'a>() -> [TMaker<'a>; 11] {

    let tmakers: [TMaker; 11] = [
        TMaker::new(TokenTypes::Whitespace, "\\s+", 100),
        TMaker::new(TokenTypes::Constant, "\\d+", 110),
        TMaker::new(TokenTypes::IntKeyword, "\\bint\\b", 110),
        TMaker::new(TokenTypes::VoidKeyword, "\\bvoid\\b", 110),
        TMaker::new(TokenTypes::ReturnKeyword, "\\breturn\\b", 110),
        TMaker::new(TokenTypes::Identifier, "\\b\\w+\\b", 110),
        TMaker::new(TokenTypes::OpenParen, "\\(", 110),
        TMaker::new(TokenTypes::CloseParen, "\\)", 110),
        TMaker::new(TokenTypes::OpenBrace, "\\{", 110),
        TMaker::new(TokenTypes::CloseBrace, "\\}", 110),
        TMaker::new(TokenTypes::Semicolon, ";", 110)
    ];

    tmakers
}

/// Token struct for collecting tokens from a source files.
/// - I might not need this token and instead use the enum value thing
pub struct Token {
    pub token_type: TokenTypes,
    pub value: String,
}

impl Token {
    pub fn print(&self) {
        println!("{}: {}", self.token_type, self.value);
    }
}

/// TokenMaker struct as a tool for storing a compiled regex associated 
/// with the appropriate TokenType enum. This can be used to generate
/// tokens of it's specified type.
/// - Do these values need to be public?
struct TMaker<'a> {
    pub token_type: TokenTypes,
    pub regex: &'a str,
    pub re: Regex,  // Can't pre-compile these into constants
    pub priority: u16,
}

impl<'a> TMaker<'a> {
    /// Create a new Token maker. Compile the regex and store it in the
    /// TMaker struct.
    /// ## Example
    /// ```
    /// token_maker: TMaker = TMaker::new(TokenTypes::Whitespace, "\\s+", 100)
    /// ```
    fn new(token_type: TokenTypes,regex: &'a str, priority: u16) -> TMaker {
        let re = Regex::new(regex).unwrap();
        TMaker { token_type, regex ,re, priority}
    }
}
    
impl<'b> TMaker<'b> {
    /// Examines the haystack to see if a match exists
    /// 
    /// ## Example
    /// ```
    /// let Some(rematch) = tok.peek(&source)
    /// else { println!("{}: No match", tok.regex); continue;};
    /// println!("{}: {}", tok.regex, rematch.as_str());
    /// ```
    pub fn peek(&'b self, hay: &'b str) -> Option<regex::Match> {
        //let re = Regex::new(self.regex).unwrap();
        // use the stored regex
        self.re.find(hay)
    }
}

/// takes a TokenMaker, looks for a match in the slice. If the begin
/// of the match is 0, return the token, and length of the token.
/// Otherwise return an empty token, with a 0 length
/// 
/// - TODO: Look for a way to make this more flexible, I don't like sending
/// empty data in return.
fn get_token(source: &str, tmaker: &TMaker) -> (Token, usize) {

    // Define the Null Return
    let empty_token: Token = Token { token_type: TokenTypes::Empty, value:String::from("") };
    //(empty_token, 0)

    let Some(peek) = tmaker.peek(source)
        else { //println!("{}: No match", tmaker.regex); 
        return (empty_token, 0);
    };

    if peek.start()!=0 {
        return (empty_token, 0);
    }

    let found_token: Token = Token { token_type: tmaker.token_type, value: String::from(peek.as_str())};
    let shift_marker: usize = peek.end();

    (found_token, shift_marker)
   
}

/// Generates a Vector of Tokens
/// ## Panic
/// This function will panic if there's unaccounted for text in the source.
fn scan(source: &String, tmakers: &[TMaker]) -> Vec<Token> {
    let mut out: Vec<Token> = vec![];
    let mut i: usize = 0;
    let mut j: usize = 0;
    let length: usize = source.len();

    // Move through the source, using i as pointer along the text. Attempt each
    // token in tmakers array.  For each token, if there's no match, continue to the
    // next token. If there IS a match, push the found token to the vector, and move
    // the i along the source.
    while i<length {
        for t in tmakers {
            let slice = &source[i..length];
            let check = get_token(slice, &t);
            // If no match, go to next token
            if check.0.token_type == TokenTypes::Empty {
                continue;
            }
            // If whitespace, break out of the for loop without pushing a token
            if check.0.token_type == TokenTypes::Whitespace {
                i+=check.1;
                break;
            }
            out.push(check.0);
            i+=check.1;
            break;
        }
        // If all the tokens were visited, and no match was found, the i will not have moved. Panic
        if i==j { panic!("Token could not be found at position: {}", i); }
        else { j = i };
    }
    out
}

pub fn lexer(source: &String) -> Vec<Token> {
    println!("Lexer Runs");
    
    // create the array of token makers
    let tmakers = generate_tmakers();

    // load source file
    //let source: String = String::from("void main() { return; }");

    // initialize a token vector for the return
    let collected_tokens: Vec<Token>;

    // scan the source file with the token makers
    // as tokens are found, add to the vector
    // provide vector when end of source file 
    collected_tokens = scan(source, &tmakers);

    // Test - Print the collected tokens
    // for t in collected_tokens {
    //     t.print();
    // }
    collected_tokens
}