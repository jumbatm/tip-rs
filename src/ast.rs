pub type StatementList = Vec<Statement>;

#[derive(Debug, PartialEq, Eq)]
pub struct Ident(pub String);

#[derive(Debug, PartialEq, Eq)]
pub enum BinOp {
    Plus,
    Minus,
    Times,
    Divide,
    CompareEq,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Expression {
    Number(i64),
    BinaryExpression(BinOp, Box<Expression>, Box<Expression>),
    IdentReference(Ident),
    Input,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Statement {
    VarDecl(Vec<Ident>), // TODO: Intern strings?
    Assign(Ident, Expression),
    If {
        cond: Expression,
        then: Option<StatementList>,
        otherwise: Option<StatementList>,
    },
    While {
        cond: Expression,
        then: Option<StatementList>,
    },
    Break,
    Output(Expression),
    Return(Option<Expression>),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Function {
    pub name: Ident,
    pub params: Vec<Ident>,
    pub body: StatementList,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Program {
    pub functions: Vec<Function>,
}
