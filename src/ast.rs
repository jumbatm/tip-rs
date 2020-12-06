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
    Alloc(Box<Expression>),
    Record(Vec<(Ident, Expression)>),
    Projection(Box<Expression>, Vec<Ident>),
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
    Error(Expression),
    ExpressionStatement(Expression),
    Block(StatementList),
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

pub trait ASTVisitor {
    fn visit_program(&mut self, p: Program) {
        super_visit_program(self, p)
    }
    fn visit_function(&mut self, f: Function) {
        super_visit_function(self, f)
    }

    fn visit_statement_list(&mut self, s: StatementList) {
        super_visit_statement_list(self, s)
    }
    fn visit_statement(&mut self, s: Statement);
}

pub fn super_visit_program(vis: &mut (impl ASTVisitor + ?Sized), p: Program) {
    for f in p.functions {
        vis.visit_function(f)
    }
}

pub fn super_visit_function(vis: &mut (impl ASTVisitor + ?Sized), f: Function) {
    vis.visit_statement_list(f.body)
}

pub fn super_visit_statement_list(vis: &mut (impl ASTVisitor + ?Sized), sl: StatementList) {
    for s in sl {
        vis.visit_statement(s)
    }
}
