mod grammar;
use crate::ast::Program;

pub fn parse(src: &str) -> Result<Program, peg::error::ParseError<peg::str::LineCol>> {
    grammar::tip_parser::program(src)
}
