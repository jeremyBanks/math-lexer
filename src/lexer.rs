use std::fmt::Debug;
use std::iter::Enumerate;
use std::rc::Rc;
use std::str::Chars;

#[path = "tokens.rs"]
mod tokens;

use tokens::ArithOperators::*;
use tokens::ComparisonOperators::*;
use tokens::TokenType::*;

use crate::fsm::FSM;
use crate::number_state_rules::{NumberStateRules, NumberStates};

const WHITESPACE_CHARS: [char; 2] = [' ', '\n'];

#[derive(Debug)]
pub struct Token {
    token_type: tokens::TokenType,
    line: u32,
    column: u32,
}

impl Token {
    pub fn new(t: tokens::TokenType, line: u32, column: u32) -> Self {
        Token {
            token_type: t,
            line: line,
            column: column,
        }
    }
}

pub struct Lexer {
    input: Rc<String>,
    chars: Vec<char>,
    position: u64,
    line: u32,
    column: u32,
    skip_chars: u8,
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
            skip_chars: 0,
        }
    }

    pub fn next_token(&mut self, i: u64, c: char) -> Token {
        if c.is_alphabetic() {
            self.tokenize_identifier(i, c)
        } else if c.is_numeric() {
            self.tokenize_number(i, c)
        } else {
            match c {
                '(' | ')' => self.tokenize_paren(c),
                '>' | '<' | '=' => self.tokenize_comp_operator(i, c),
                '+' | '-' | '*' | '/' => self.tokenize_arith_operator(i, c),
                _ => panic!("Unexpected character at line {}: {}", self.line, c),
            }
        }
    }

    fn tokenize_identifier(&mut self, i: u64, first_char: char) -> Token {
        let mut output: String = first_char.to_string();
        let mut skip_chars: u8 = 1;
        let mut next_char = self.chars[i as usize + 1];

        while next_char.is_alphabetic() || next_char.is_digit(10) || next_char == '-' {
            output.push(next_char);

            skip_chars += 1;
            let next_index = i as usize + skip_chars as usize;
            if next_index >= self.chars.len() {
                break;
            }

            next_char = self.chars[next_index];
        }

        self.skip_chars = output.len() as u8;
        if output.is_empty() {
            panic!(
                "An error has occurred. Currently parsing at line {}: {}",
                self.line, first_char
            );
        }

        Token::new(Identifier(output), self.line, self.column)
    }

    fn tokenize_number(&mut self, i: u64, first_char: char) -> Token {
        // count the number of characters from i to the first whitespace char
        let mut index = (i + 1) as usize;
        let mut number_str = String::from(first_char.to_string());

        while let Some(c) = self.chars.get(index) {
            if WHITESPACE_CHARS.contains(c) {
                break;
            } else {
                number_str.push(*c);
                index += 1;
            }
        }

        let accepting_states = vec![
            NumberStates::Integer,
            NumberStates::NumberWithFractionalPart,
            NumberStates::NumberWithExponent,
        ];
        let fsm: FSM<NumberStateRules> = FSM::new(NumberStates::Initial, accepting_states);
        let input_clone = number_str.clone(); // in case we need to throw an error later
        let result = fsm.run(number_str);

        match result {
            Some(value) => {
                self.skip_chars = value.len() as u8;
                Token::new(Number(value), self.line, self.column)
            }
            None => panic!(
                "Unexpected token {}. Were you trying to define a number?",
                input_clone
            ),
        }
    }

    fn tokenize_comp_operator(&mut self, i: u64, c: char) -> Token {
        let next_char_is_equals = self.chars[i as usize + 1] == '=';
        match c {
            '>' => {
                if next_char_is_equals {
                    self.skip_chars += 1;
                    Token::new(
                        ComparisonOperator(GreaterThanOrEqual),
                        self.line,
                        self.column,
                    )
                } else {
                    Token::new(ComparisonOperator(GreaterThan), self.line, self.column)
                }
            }
            '<' => {
                if next_char_is_equals {
                    self.skip_chars += 1;
                    Token::new(ComparisonOperator(LessThanOrEqual), self.line, self.column)
                } else {
                    Token::new(ComparisonOperator(LessThan), self.line, self.column)
                }
            }
            '=' => {
                if next_char_is_equals {
                    self.skip_chars += 1;
                    Token::new(ComparisonOperator(Equal), self.line, self.column)
                } else {
                    Token::new(Assignment, self.line, self.column)
                }
            }
            _ => panic!(
                "An error has occurred. Currently parsing at line {}: {}",
                self.line, c
            ),
        }
    }

    fn tokenize_arith_operator(&mut self, i: u64, c: char) -> Token {
        match c {
            '+' => Token::new(ArithOperator(Plus), self.line, self.column),
            '-' => Token::new(ArithOperator(Minus), self.line, self.column),
            '*' => Token::new(ArithOperator(Times), self.line, self.column),
            '/' => Token::new(ArithOperator(Div), self.line, self.column),
            _ => panic!(
                "An error has occurred. Currently parsing at line {}: {}",
                self.line, c
            ),
        }
    }

    fn tokenize_paren(&mut self, c: char) -> Token {
        let line = self.line;
        let col = self.column;

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

        loop {
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
                        self.line += 1;
                    } else if c == ' ' {
                        self.position += 1;
                    } else {
                        self.position += 1;
                        self.column += 1;
                        tokens.push(self.next_token(self.position - 1 as u64, c));
                    }
                }
                None => {
                    tokens.push(Token::new(EndOfInput, self.line, self.column));
                    break tokens;
                }
            }
        }
    }
}
