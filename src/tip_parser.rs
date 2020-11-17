use crate::ast::{BinOp, Expression};
use peg;

peg::parser! {
    grammar tip_parser() for str {
        rule _() = [' ' | '\n' | '\t' ]*
        pub rule expression() -> Expression
            = sum()
            / "input" { Expression::Input }

        // FIXME: Replace with precedence! block.
        rule sum() -> Expression
            = l:product() _ "+" _ r:sum() { Expression::BinaryExpression(BinOp::Plus , Box::new(l), Box::new(r)) }
            / l:product() _ "-" _ r:sum() { Expression::BinaryExpression(BinOp::Minus, Box::new(l), Box::new(r)) }
            / product()

        rule product() -> Expression
            = l:atom() _ "*" _ r:product() { Expression::BinaryExpression(BinOp::Times, Box::new(l), Box::new(r)) }
            / l:atom() _ "/" _ r:product() { Expression::BinaryExpression(BinOp::Divide, Box::new(l), Box::new(r)) }
            / atom()

        rule atom() -> Expression
            = number()
            / "(" e:expression() ")" { e }

        rule number() -> Expression
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
