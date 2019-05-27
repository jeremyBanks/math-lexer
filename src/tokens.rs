pub enum ArithOperators {
    Plus,
    Minus,
    Times,
    Div
}

pub enum ComparisonOperators {
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    Equal
}

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
