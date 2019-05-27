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

struct Lexer<'a> {
    input: Chars<'a>,
    position: u64,
    line: u32,
    column: u32
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a String) -> Self {
        let e = input.chars();
        Lexer {
            input: e,
            position: 0,
            line: 0,
            column: 0
        }
    }

    pub fn next_token(&mut self) -> Token {
        match self.input.next() {
            Some(c) => match c {
                '(' | ')' => self.tokenize_paren(c),
                _ => panic!("Unexpected character at line {}: {}", self.line, c)
            },
            None => Token { token_type: EndOfInput, line: self.line, column: self.column}
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

    pub fn run(&self) {
        
    }
}

fn main() {
    let s = String::from("Hello world");
    let e = s.chars().enumerate();
    println!("{}", s);

}
