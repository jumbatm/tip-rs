use crate::ast::{BinOp, Expression, Function, Ident, Program, Statement, StatementList};
use peg;

peg::parser! {
    /// PEG grammar for TIP. Note that we wrap this parser in the `TipParser` struct. Most rules are `pub`
    /// to make them testable.
    pub(crate) grammar tip_parser() for str {

        rule ws() = [' ' | '\n' | '\t']+
        rule _() = ws()*

        rule statement_list() -> StatementList
            = stmt:(_ s:statement() _ { s })+ { stmt }

        rule statement_contents() -> Statement
            = "var" ws() first:ident() rest:("," _ id:ident() { id })* {
                let mut result = vec![first];
                result.extend(rest);
                Statement::VarDecl(result)
            }
            / "break" { Statement::Break }
            / "return" e:(ws() e:expression() { e })?  { Statement::Return(e) }
            / "output" ws() e:expression() { Statement::Output(e) }
            / i:ident() "=" e:expression() { Statement::Assign(i, e) }

        pub rule program() -> Program
            = fun:(_ f:function() _ { f })+ { Program { functions: fun }}

        pub rule function() -> Function
            = name:ident() _ "(" params:(i:(_ i:ident() _ { i }) ** "," { i })")" _ "{" _ body:statement_list()? _ "}" { Function { name, params, body: body.unwrap_or(vec![]) }}

        pub rule statement() -> Statement
            = s:statement_contents() ";" { s }
            / "while" _ "(" _ cond: expression() _ ")" _ "{" _ then:statement_list()? _ "}" { Statement::While { cond, then }}
            / "while" _ "(" _ cond: expression() _ ")" _ then: statement()? { Statement::While { cond, then: then.map(|t| vec![t]) }}
            / "if" _ "(" _ cond:expression() _")" _ "{" _ then:statement_list()? _ "}" otherwise:(_ "else" _ "{" _ s:statement_list()? _ "}" { s.unwrap_or(vec![]) })? {
                Statement::If { cond, then, otherwise }
            }
            / "if" _ "(" _ cond:expression() _")" _ then:(t:statement()? { t.map(|t| vec![t]) }) _ otherwise:(_ "else" _ s:statement()? { s.map(|s| vec![s] ).unwrap_or(vec![]) })? {
                Statement::If { cond, then, otherwise }
            }

        pub rule expression() -> Expression
            = precedence! {
                l:@ _ "+" _ r:(@)  { Expression::BinaryExpression(BinOp::Plus , Box::new(l), Box::new(r)) }
                l:@ _ "-" _ r:(@)  { Expression::BinaryExpression(BinOp::Minus , Box::new(l), Box::new(r)) }
                --
                l:@ _ "*" _ r:(@)  { Expression::BinaryExpression(BinOp::Times , Box::new(l), Box::new(r)) }
                l:@ _ "/" _ r:(@)  { Expression::BinaryExpression(BinOp::Divide , Box::new(l), Box::new(r)) }
                --
                a:atom() { a }

            }
        pub rule atom() -> Expression
            = number()
            / id:ident() { Expression::IdentReference(id) }
            / "(" e:expression() ")" { e }

        pub rule number() -> Expression
            = n:$(['0'..='9']+) { Expression::Number(n.parse().unwrap()) }

        pub rule ident() -> Ident
            = id:$(['A'..='Z' | 'a'..='z']['A'..='Z' | 'a'..='z' | '0'..='9' | '_' ]*) { Ident(id.into()) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_exprs() {
        assert_eq!(
            tip_parser::expression("1 + 2 * 3"),
            Ok(Expression::BinaryExpression(
                BinOp::Plus,
                Box::new(Expression::Number(1)),
                Box::new(Expression::BinaryExpression(
                    BinOp::Times,
                    Box::new(Expression::Number(2)),
                    Box::new(Expression::Number(3))
                ))
            ))
        );
        assert_eq!(
            tip_parser::expression("(1 + 2) * 3"),
            Ok(Expression::BinaryExpression(
                BinOp::Times,
                Box::new(Expression::BinaryExpression(
                    BinOp::Plus,
                    Box::new(Expression::Number(1)),
                    Box::new(Expression::Number(2))
                )),
                Box::new(Expression::Number(3)),
            ))
        );
        assert_eq!(
            tip_parser::expression("1 - 2 / 3"),
            Ok(Expression::BinaryExpression(
                BinOp::Minus,
                Box::new(Expression::Number(1)),
                Box::new(Expression::BinaryExpression(
                    BinOp::Divide,
                    Box::new(Expression::Number(2)),
                    Box::new(Expression::Number(3))
                ))
            ))
        );
        assert_eq!(
            tip_parser::expression("(1 - 2) / 3"),
            Ok(Expression::BinaryExpression(
                BinOp::Divide,
                Box::new(Expression::BinaryExpression(
                    BinOp::Minus,
                    Box::new(Expression::Number(1)),
                    Box::new(Expression::Number(2))
                )),
                Box::new(Expression::Number(3)),
            ))
        );

        assert_eq!(
            tip_parser::expression("1 * 2 * 3"),
            Ok(Expression::BinaryExpression(
                BinOp::Times,
                Box::new(Expression::Number(1)),
                Box::new(Expression::BinaryExpression(
                    BinOp::Times,
                    Box::new(Expression::Number(2)),
                    Box::new(Expression::Number(3)),
                )),
            ))
        );
    }
    #[test]
    fn test_parse_ident() {
        assert_eq!(tip_parser::ident("x"), Ok(Ident("x".to_string())));
        assert_eq!(tip_parser::ident("y"), Ok(Ident("y".to_string())));
        assert_eq!(tip_parser::ident("z"), Ok(Ident("z".to_string())));
        assert_eq!(tip_parser::ident("xyz"), Ok(Ident("xyz".to_string())));
        assert_eq!(tip_parser::ident("abc123"), Ok(Ident("abc123".to_string())));
        assert_eq!(
            tip_parser::ident("abc_123"),
            Ok(Ident("abc_123".to_string()))
        );
    }

    #[test]
    fn test_parse_var_decl() {
        assert_eq!(
            tip_parser::statement("var x;"),
            Ok(Statement::VarDecl(vec![Ident("x".to_string())]))
        );
        assert_eq!(
            tip_parser::statement("var x, y, z;"),
            Ok(Statement::VarDecl(
                vec!["x", "y", "z"]
                    .into_iter()
                    .map(|s| Ident(s.to_string()))
                    .collect()
            ))
        );
        assert_eq!(
            tip_parser::statement("var x,y,z;"),
            Ok(Statement::VarDecl(
                vec!["x", "y", "z"]
                    .into_iter()
                    .map(|s| Ident(s.to_string()))
                    .collect()
            ))
        );
        assert_eq!(
            tip_parser::statement("var a_complex_name,y, result123;"),
            Ok(Statement::VarDecl(
                vec!["a_complex_name", "y", "result123"]
                    .into_iter()
                    .map(|s| Ident(s.to_string()))
                    .collect()
            ))
        );
    }

    #[test]
    fn test_break() {
        assert_eq!(tip_parser::statement("break;"), Ok(Statement::Break));
    }

    #[test]
    fn test_return() {
        assert_eq!(
            tip_parser::statement("return;"),
            Ok(Statement::Return(None))
        );
        assert_eq!(
            tip_parser::statement("return 123;"),
            Ok(Statement::Return(Some(Expression::Number(123))))
        );
    }

    #[test]
    fn test_if() {
        assert_eq!(
            tip_parser::statement("if (1) { } else { }"),
            Ok(Statement::If {
                cond: Expression::Number(1),
                then: None,
                otherwise: Some(vec![])
            })
        );
        assert_eq!(
            tip_parser::statement("if (1) { var x; } else { var y; }"),
            Ok(Statement::If {
                cond: Expression::Number(1),
                then: Some(vec![Statement::VarDecl(vec![Ident("x".to_string())])]),
                otherwise: Some(vec![Statement::VarDecl(vec![Ident("y".to_string())])]),
            })
        );
        assert_eq!(
            tip_parser::statement("if (1) { var x; var y; } else { var a; var b; }"),
            Ok(Statement::If {
                cond: Expression::Number(1),
                then: Some(vec![
                    Statement::VarDecl(vec![Ident("x".to_string())]),
                    Statement::VarDecl(vec![Ident("y".to_string())])
                ]),
                otherwise: Some(vec![
                    Statement::VarDecl(vec![Ident("a".to_string())]),
                    Statement::VarDecl(vec![Ident("b".to_string())])
                ]),
            })
        );
        assert!(tip_parser::statement("if (1) var x; else var y;").is_ok());
        assert_eq!(
            tip_parser::statement("if (1) if (2) var x; else var y; else var z;"),
            Ok(Statement::If {
                cond: Expression::Number(1),
                then: Some(vec![Statement::If {
                    cond: Expression::Number(2),
                    then: Some(vec![Statement::VarDecl(vec![Ident("x".to_string())])]),
                    otherwise: Some(vec![Statement::VarDecl(vec![Ident("y".to_string())])]),
                }]),
                otherwise: Some(vec![Statement::VarDecl(vec![Ident("z".to_string())])]),
            })
        );
    }

    #[test]
    fn test_while() {
        assert_eq!(
            tip_parser::statement("while (1) { var x; }"),
            Ok(Statement::While {
                cond: Expression::Number(1),
                then: Some(vec![Statement::VarDecl(vec![Ident("x".to_string())])])
            })
        );
        assert_eq!(
            tip_parser::statement("while (1) { var x; var y; }"),
            Ok(Statement::While {
                cond: Expression::Number(1),
                then: Some(vec![
                    Statement::VarDecl(vec![Ident("x".to_string())]),
                    Statement::VarDecl(vec![Ident("y".to_string())])
                ],)
            })
        );
        assert_eq!(
            tip_parser::statement("while (1) var x;"),
            Ok(Statement::While {
                cond: Expression::Number(1),
                then: Some(vec![Statement::VarDecl(vec![Ident("x".to_string())])])
            })
        );
    }

    #[test]
    fn test_output() {
        assert_eq!(
            tip_parser::statement("output x;"),
            Ok(Statement::Output(Expression::IdentReference(Ident(
                "x".to_string()
            ))))
        );
    }

    #[test]
    fn test_function() {
        assert_eq!(
            tip_parser::function("f() { return 0; }"),
            Ok(Function {
                name: Ident("f".to_string()),
                params: vec![],
                body: vec![Statement::Return(Some(Expression::Number(0)))],
            })
        );
        assert_eq!(
            tip_parser::function("g(x, y, z) { return 1; }"),
            Ok(Function {
                name: Ident("g".to_string()),
                params: vec![
                    Ident("x".to_string()),
                    Ident("y".to_string()),
                    Ident("z".to_string())
                ],
                body: vec![Statement::Return(Some(Expression::Number(1)))],
            })
        );
    }
    #[test]
    fn test_program() {
        assert_eq!(
            tip_parser::program("f() { return 0; } g(x, y, z) { return 1; }"),
            Ok(Program {
                functions: vec![
                    Function {
                        params: vec![],
                        name: Ident("f".to_string()),
                        body: vec![Statement::Return(Some(Expression::Number(0)))],
                    },
                    Function {
                        name: Ident("g".to_string()),
                        params: vec![
                            Ident("x".to_string()),
                            Ident("y".to_string()),
                            Ident("z".to_string())
                        ],
                        body: vec![Statement::Return(Some(Expression::Number(1)))],
                    }
                ]
            })
        );
    }
}
