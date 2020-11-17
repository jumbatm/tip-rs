#[derive(Debug, PartialEq, Eq)]
pub enum BinOp {
    Plus,
    Minus,
    Times,
    Divide,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Expression {
    Number(i64),
    BinaryExpression(BinOp, Box<Expression>, Box<Expression>),
    Input,
}
