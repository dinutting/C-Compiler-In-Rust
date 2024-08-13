// For some reason, if this file is named "parser.rs" vs code 
// dims all of the text. Unclear if this is from rust-analyzer
// something else....

use crate::lexer::*;
// TODO - Add the ASTNode trait to each of the Node types
// TODO - Add printing features for testing
pub fn parser () -> () {
    println!("Parser says hello");
    let t: TokenTypes = TokenTypes::Whitespace;

    let a = Box::new(NodeA { value: 10 }); //name: String::from("Node_a"), value: 10 };
    let b = Box::new(NodeB { value: 'a' }); // name: String::from("Node_b"), value: String::from("Sup bitches")};

    let mut t: Vec<Box<dyn PrintValue>> = vec![] ;
    t.push(a);
    t.push(b);

    for o in t {
        o.print();
    }
}

struct NodeA {
    //pub name: String,
    pub value: i32,
}

struct NodeB {
    //pub name: String,
    pub value: char,
}

trait PrintValue {
    fn print(&self);
}

impl PrintValue for NodeA {
    fn print(&self) {
        println!("{}", self.value);
    }
}

impl PrintValue for NodeB {
    fn print(&self) {
        println!("{}", self.value);
    }
}

trait ASTNode {}

struct ProgramNode<'a> {
    function_node: FunctionNode<'a>,
}

struct FunctionNode<'a> {
    identifier: &'a str,
    statement_node: StatementNode<'a>,
}

struct StatementNode<'a> {
    exp_node: ExpNode<'a>,
    value: &'a str,
}

struct ExpNode<'a> {
    int_node: IntNode<'a>,
    datatype: &'a str,
}

struct IdentifierNode<'a> {
    value: &'a str,
}

struct IntNode<'a> {
    value: &'a i32
}