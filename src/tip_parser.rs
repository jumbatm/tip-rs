mod grammar;
use crate::ast::Program;

pub type ParseError =  peg::error::ParseError<peg::str::LineCol>;

pub fn parse(src: String) -> Result<Program, ParseError> {
    grammar::tip_parser::program(&src)
}
