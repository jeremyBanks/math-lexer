use std::str::Chars;
use std::iter::Enumerate;
use std::rc::Rc;
use std::fmt::Debug;

#[path = "tokens.rs"] mod tokens;

use tokens::TokenType::*;
use tokens::ComparisonOperators::*;
use tokens::ArithOperators::*;

#[derive(Debug)]
pub struct Token {
    token_type: tokens::TokenType,
    line: u32,
    column: u32
}

impl Token {
    pub fn new(t: tokens::TokenType, line: u32, column: u32) -> Self {
        Token {
            token_type: t,
            line: line,
            column: column
        }
    }
}

pub struct Lexer {
    input: Rc<String>,
    chars: Vec<char>,
    position: u64,
    line: u32,
    column: u32,
    skip_chars: u8
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let v: Vec<char> = input.clone().chars().collect();
        Lexer {
            input: Rc::new(input),
            chars: v,
            position: 0,
            line: 0,
            column: 0,
            skip_chars: 0
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
        let next_char_is_equals = self.chars[i as usize + 1] == '=';
        match c {
            '>' => {
                if next_char_is_equals {
                    self.skip_chars += 1;
                    Token::new(ComparisonOperator(GreaterThanOrEqual), self.line, self.column) 
                } else { 
                    Token::new(ComparisonOperator(GreaterThan), self.line, self.column) 
                }
            },
            '<' => {
                if next_char_is_equals {
                    self.skip_chars += 1;
                    Token::new(ComparisonOperator(LessThanOrEqual), self.line, self.column) 
                } else { 
                    Token::new(ComparisonOperator(LessThan), self.line, self.column) 
                }
            },
            '=' => {
                if next_char_is_equals {
                    self.skip_chars += 1;
                    Token::new(ComparisonOperator(Equal), self.line, self.column) 
                } else { 
                    Token::new(Assignment, self.line, self.column)
                }
            },
            _ => panic!("An error has occurred. Currently parsing at line {}: {}", self.line, c)
        }
    }

    fn tokenize_arith_operator(&mut self, i: u64, c: char) -> Token {
        match c {
            '+' => {
                Token::new(ArithOperator(Plus), self.line, self.column)
            },
            '-' => {
                Token::new(ArithOperator(Minus), self.line, self.column)
            },
            '*' => {
                Token::new(ArithOperator(Times), self.line, self.column)
            },
            '/' => {
                Token::new(ArithOperator(Div), self.line, self.column)
            },
            _ => panic!("An error has occurred. Currently parsing at line {}: {}", self.line, c)
        }
    }

    fn tokenize_paren(&mut self, c: char) -> Token {
        let line = self.line;
        let col = self.column;

        self.column += 1;

        if c == '(' {
            Token::new(LeftParen, self.line, self.column)
        } else {
            Token::new(RightParen, self.line, self.column)
        }
    }

    pub fn run(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        let input: Rc<String> = self.input.clone();
        let mut chars = input.chars();
        let mut count = input.chars().count();

        while self.position <= count as u64 {
            if self.skip_chars > 0 {
                for _ in 0..self.skip_chars {
                    chars.next();
                }
                self.position += self.skip_chars as u64;
                self.skip_chars = 0;
                continue;
            }

            match chars.next() {
                Some(c) => {
                    if c == '\n' {
                        self.position += 1;
                    } else {
                        tokens.push(self.next_token(self.position as u64, c));
                        self.position += 1;
                    }

                    count -= 1;
                },
                None => tokens.push(Token::new(EndOfInput, self.line, self.column))
            }
        }

        tokens
    }
}
