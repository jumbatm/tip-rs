
use crate::ast::{BinOp, Expression};
use peg;
peg::parser! {
    /// PEG grammar for TIP. Note that we wrap this parser in the `TipParser` struct. Most rules are `pub`
    /// to make them testable.
    grammar tip_parser() for str {

        rule _() = [' ' | '\n' | '\t' ]*

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
            / "(" e:expression() ")" { e }

        pub rule number() -> Expression
            = n:$(['0'..='9']+) { Expression::Number(n.parse().unwrap()) }
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
}
