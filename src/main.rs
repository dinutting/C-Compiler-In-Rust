mod lexer;

use lexer::*;

fn main() {
    
    // get file path from arguments

    // read source from file path
    let source: String = String::from("void main() { return; }");

    // Use the lexer to tokenize the source (Should token be shifted to it's own module?)
    let tokens: Vec<Token> = lexer::lexer(&source);

    // Test showing the tokens are working
    for t in tokens {
         t.print();
    }

    // Parse the tokens into an AST
    // TODO, define AST nodes
    // TODO - build parser with recursive descent

    // Translate the AST into machine code
    // TODO iterate through AST nodes and convert

    // Write machine code to output

}
