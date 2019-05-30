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
    Number(i32),
    ArithOperator(ArithOperators),
    ComparisonOperator(ComparisonOperators),
    Assignment,
    LeftParen,
    RightParen,
    EndOfInput
}
