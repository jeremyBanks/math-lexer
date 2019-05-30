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
    arr: Vec<char>,
    position: u64,
    line: u32,
    column: u32
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let v: Vec<char> = input.clone().chars().collect();
        Lexer {
            input: input,
            arr: v,
            position: 0,
            line: 0,
            column: 0
        }
    }

    pub fn next_token(&mut self, i: u64, c: char) -> Token {
        match c {
            '(' | ')' => self.tokenize_paren(c),
            '>' | '<' | '=' => self.tokenize_comp_operator(i, c),
            '+' | '-' | '*' | '/' => self.tokenize_arith_operator(i, c),
            _ => panic!("Unexpected character at line {}: {}", self.line, c)
        }
    }

    fn tokenize_identifier(&mut self, first_char: char) -> Token {
        unimplemented!()
    }

    fn tokenize_number(&mut self, first_char: char) -> Token {
        unimplemented!()
    }

    fn tokenize_comp_operator(&mut self, i: u64, c: char) -> Token {
        unimplemented!()
    }

    fn tokenize_arith_operator(&mut self, i: u64, c: char) -> Token {
        unimplemented!()
    }

    fn tokenize_paren(&mut self, c: char) -> Token {
        let line = self.line;
        let col = self.column;

        self.column += 1;

        if c == '(' {
            Token { token_type: LeftParen, line: line, column: col }
        } else {
            Token { token_type: RightParen, line: line, column: col }
        }
    }

    pub fn run(&mut self) {
        let tokens: Vec<Token> = vec![];
        let clone = self.input.clone();
        let chars = clone.chars();

        while self.position <= chars.count() as u64 {
            match chars.next() {
                Some(c) => {
                    if c == '\n' {
                        self.position += 1;
                    } else {
                        tokens.push(self.next_token(self.position as u64, c));
                        self.position += 1;
                    }
                },
                None => tokens.push(Token { token_type: EndOfInput, line: self.line, column: self.column})
            }
        }
    }
}

fn main() {
    let s = String::from("Hello world");
    let e = s.chars().enumerate();
    println!("{}", s);

}
