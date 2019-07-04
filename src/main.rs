mod fsm;
mod lexer;
mod number_state_rules;
mod tokens;

use lexer::Lexer;

use std::env;
use std::fs;
use std::str::Chars;

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let mut input: String;

    println!("input: {:?}", args);

    if args.len() < 2 || args.len() > 3 {
        println!("usage: lexer [--file] <filepath> | lexer <input>");
        return;
    }

    input = match args[1].as_ref() {
        "--file" => {
            fs::read_to_string(args[2].clone()).expect("Unable to read the requested file.")
        }
        _ => args[1].clone(),
    };

    let mut lexer = Lexer::new(input);
    let tokens = lexer.run();

    println!("tokens: {:?}", tokens);
}

#[test]
fn example() {
    use lexer::{Token, tokens::TokenType::*};

    let input = "number = 12.34e-123";
    // JB: why does this take String instead of &str?
    let mut lexer = Lexer::new(input.to_string());
    let tokens = lexer.run();
    assert_eq!(
        tokens,
        vec![
            Token {
                token_type: Identifier("number".to_string()),
                line: 0,
                column: 1
            },
            Token {
                token_type: Assignment,
                line: 0,
                column: 2
            },
            Token {
                token_type: Number("12.34e-123".to_string()),
                line: 0,
                column: 3
            },
            Token {
                token_type: EndOfInput,
                line: 0,
                column: 3
            }
        ]
    );
}
