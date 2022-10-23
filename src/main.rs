mod calc;
pub mod lexer;
pub mod error;
pub mod parser;

use std::io::stdin;

fn main() {
    let stdin = stdin();
    let mut input: String = "".to_string();
    stdin.read_line(&mut input).expect("Shit happened");
    println!("You typed {}", input);
    let tokens = lexer::tokenize(&input).expect("Help!");
    println!("Tokens: {:#?}", tokens);
}
