use std::fmt::Debug;

#[derive(Debug)]
pub enum ArithOperators {
    Plus,
    Minus,
    Times,
    Div
}

#[derive(Debug)]
pub enum ComparisonOperators {
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    Equal
}

#[derive(Debug)]
pub enum TokenType {
    Identifier(String),
    // we'll use a String to represent potentially ints, floats, and exponents
    Number(String), 
    ArithOperator(ArithOperators),
    ComparisonOperator(ComparisonOperators),
    Assignment,
    LeftParen,
    RightParen,
    EndOfInput
}
