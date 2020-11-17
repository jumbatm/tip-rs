
#[derive(Debug, PartialEq, Eq)]
pub struct Ident(pub String);

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
    Ident(Ident),
    Input,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Statement {
    VarDecl(Vec<Ident>), // TODO: Intern strings?
    Break,
    Return(Option<Expression>)
}
