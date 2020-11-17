use crate::ast::{BinOp, Expression, Statement};
use peg;

peg::parser! {
    /// PEG grammar for TIP. Note that we wrap this parser in the `TipParser` struct. Most rules are `pub`
    /// to make them testable.
    grammar tip_parser() for str {

        rule ws() = [' ' | '\n' | '\t']+
        rule _() = ws()*

        rule statement_contents() -> Statement
            = "var" ws() first:ident() rest:("," _ id:ident() { id })* {
                let mut result = vec![first];
                result.extend(rest);
                Statement::VarDecl(result)
            }

        pub rule statement() -> Statement
            = s:statement_contents() ";" { s }

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
            / id:ident() { Expression::Ident(id) }
            / "(" e:expression() ")" { e }

        pub rule number() -> Expression
            = n:$(['0'..='9']+) { Expression::Number(n.parse().unwrap()) }

        pub rule ident() -> String
            = id:$(['A'..='Z' | 'a'..='z']['A'..='Z' | 'a'..='z' | '0'..='9' | '_' ]*) { id.into() }
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
        assert_eq!(tip_parser::ident("x"), Ok("x".to_string()));
        assert_eq!(tip_parser::ident("y"), Ok("y".to_string()));
        assert_eq!(tip_parser::ident("z"), Ok("z".to_string()));
        assert_eq!(tip_parser::ident("xyz"), Ok("xyz".to_string()));
        assert_eq!(tip_parser::ident("abc123"), Ok("abc123".to_string()));
        assert_eq!(tip_parser::ident("abc_123"), Ok("abc_123".to_string()));
    }

    #[test]
    fn test_parse_var_decl() {
        assert_eq!(
            tip_parser::statement("var x;"),
            Ok(Statement::VarDecl(vec!["x".to_string()]))
        );
        assert_eq!(
            tip_parser::statement("var x, y, z;"),
            Ok(Statement::VarDecl(
                vec!["x", "y", "z"]
                    .into_iter()
                    .map(|s| s.to_string())
                    .collect()
            ))
        );
        assert_eq!(
            tip_parser::statement("var x,y,z;"),
            Ok(Statement::VarDecl(
                vec!["x", "y", "z"]
                    .into_iter()
                    .map(|s| s.to_string())
                    .collect()
            ))
        );
        assert_eq!(
            tip_parser::statement("var a_complex_name,y, result123;"),
            Ok(Statement::VarDecl(
                vec!["a_complex_name", "y", "result123"]
                    .into_iter()
                    .map(|s| s.to_string())
                    .collect()
            ))
        );
    }
}
