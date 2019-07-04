use std::fmt::Debug;

#[derive(Debug, PartialEq)]
pub enum ArithOperators {
    Plus,
    Minus,
    Times,
    Div,
}

#[derive(Debug, PartialEq)]
pub enum ComparisonOperators {
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    Equal,
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Identifier(String),
    // we'll use a String to represent potentially ints, floats, and exponents
    Number(String),
    ArithOperator(ArithOperators),
    ComparisonOperator(ComparisonOperators),
    Assignment,
    LeftParen,
    RightParen,
    EndOfInput,
}
