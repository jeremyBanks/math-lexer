use std::str::Chars;
use std::iter::Enumerate;

mod tokens;
mod char_utils;

use tokens::TokenType::{
    Identifier,
    Number,
    ArithOperator,
    ComparisonOperator,
    Assignment,
    LeftParen,
    RightParen,
    EndOfInput
};

struct Token {
    token_type: tokens::TokenType,
    line: u32,
    column: u32
}

struct Lexer {
    input: String,
    position: u64,
    line: u32,
    column: u32
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Lexer {
            input: input,
            position: 0,
            line: 0,
            column: 0
        }
    }

    pub fn next_token(&mut self, i: u64, c: char) -> Token {
        match c {
            '(' | ')' => self.tokenize_paren(c),
            '+' | '-' | '*' | '/' | '>' | '<' | '=' => self.tokenize_operator(c), // need to handle >=, =<, and ==
            _ => panic!("Unexpected character at line {}: {}", self.line, c)
        }
    }

    fn tokenize_identifier(&mut self, first_char: char) -> Token {
        unimplemented!()
    }

    fn tokenize_number(&mut self, first_char: char) -> Token {
        unimplemented!()
    }

    fn tokenize_operator(&mut self, first_char: char) -> Token {
        unimplemented!()
    }

    fn tokenize_paren(&mut self, c: char) -> Token {
        let pos = self.position;
        let line = self.line;
        let col = self.column;

        self.position += 1;
        self.column += 1;

        if c == '(' {
            Token { token_type: LeftParen, line: line, column: col }
        } else {
            Token { token_type: RightParen, line: line, column: col }
        }
    }

    pub fn run(&mut self) {
        let clone = self.input.clone();
        for (i, c) in clone.chars().enumerate() {
            self.next_token(i as u64, c);
        }
    }
}

fn main() {
    let s = String::from("Hello world");
    let e = s.chars().enumerate();
    println!("{}", s);

}
