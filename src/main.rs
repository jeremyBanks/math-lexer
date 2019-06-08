mod lexer;
mod tokens;
use lexer::Lexer;

use std::env;
use std::fs;

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let mut input: String;

    println!("input: {:?}", args);

    if args.len() < 2 || args.len() > 3 {
        println!("usage: lexer [--file] <filepath> | lexer <input>");
        return;
    }

    input = match args[1].as_ref() {
        "--file" => fs::read_to_string(args[2].clone()).expect("Unable to read the requested file."),
        _ => args[1].clone()
    };

    let mut lexer = Lexer::new(input);
    let tokens = lexer.run();

    println!("tokens: {:?}", tokens);
}