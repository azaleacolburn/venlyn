use crate::lexer::lex;

mod ast;
mod lexer;
mod parser;
mod token;

fn main() {
    let string = "fn my_function(){\nlet my_int = 0;}";
    let tokens = lex(string);

    println!("{:?}", tokens);
}
