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
    CompareGt,
}

#[derive(Debug, PartialEq, Eq)]
pub enum UnOp {
    Negate,
    AddressOf,
    Dereference,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Expression {
    Number(i64),
    BinaryExpression(BinOp, Box<Expression>, Box<Expression>),
    IdentReference(Ident),
    Input,
    Call(Box<Expression>, Vec<Box<Expression>>),
    UnaryExpression(UnOp, Box<Expression>),
    Alloc(Box<Expression>)
}

#[derive(Debug, PartialEq, Eq)]
pub enum Statement {
    VarDecl(Vec<Ident>), // TODO: Intern strings?
    Assign(Expression, Expression),
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
    ExpressionStatement(Expression),
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
