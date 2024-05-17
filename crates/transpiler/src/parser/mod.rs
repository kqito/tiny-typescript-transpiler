mod declaration;
pub mod error;
mod expression;
mod function;
mod statement;
mod tests;
mod types;

use ast::statement::*;

use crate::lexer::{inputer::Input, Lexer};

pub use self::error::{ParseError, ParseErrorContext};

trait Parse<'a, I: Input<'a>> {
    type Ast;

    fn parse(parser: &mut Parser<'a, I>) -> Self::Ast;
}

trait TryParse<'a, I: Input<'a>> {
    type Ast;

    fn try_parse(parser: &mut Parser<'a, I>) -> Result<Self::Ast, ParseError>;
}

#[macro_use]
mod macros;

pub struct Parser<'a, I: Input<'a>> {
    pub(crate) lexer: Lexer<'a, I>,
    pub(crate) error_context: error::ParseErrorContext,
}

impl<'a, I: Input<'a>> Parser<'a, I> {
    pub fn new(lexer: Lexer<'a, I>) -> Parser<'a, I> {
        Parser {
            lexer,
            error_context: error::ParseErrorContext::new(),
        }
    }

    pub fn parse_module(&mut self) -> Result<Modules, ParseErrorContext> {
        let mut statements = Vec::new();

        while !self.lexer.input.is_at_end() {
            if let Some(statement) = Statement::parse(self) {
                statements.push(statement);
            }
        }

        if self.error_context.has_errors() {
            return Err(self.error_context.clone());
        }

        Ok(statements)
    }
}
