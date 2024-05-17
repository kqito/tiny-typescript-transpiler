use crate::lexer::inputer::Input;
use crate::lexer::Controller;
use crate::parser::error::ParseError;
use crate::Parser;
use ast::expression::*;
use ast::kind::SyntaxKind;
use ast::*;

use super::{Parse, TryParse};

impl<'a, I: Input<'a>> Parse<'a, I> for Expression {
    type Ast = Option<Node<Expression>>;

    fn parse(parser: &mut Parser<'a, I>) -> Self::Ast {
        let lex = parser.lexer.pop();

        match lex.kind {
            SyntaxKind::NumericLiteral => {
                return Some(Node::new(
                    Loc::new(lex.pos, lex.end),
                    Expression::Literal(Literal::Numeric(String::from(lex.text))),
                ));
            }
            SyntaxKind::StringLiteral => {
                return Some(Node::new(
                    Loc::new(lex.pos, lex.end),
                    Expression::Literal(Literal::String(String::from(lex.text))),
                ));
            }
            SyntaxKind::Identifier => {
                return Some(Node::new(
                    Loc::new(lex.pos, lex.end),
                    Expression::Identifier(Identifier {
                        text: String::from(lex.text),
                    }),
                ));
            }
            SyntaxKind::RegularExpressionLiteral => {
                return Some(Node::new(
                    Loc::new(lex.pos, lex.end),
                    Expression::Literal(Literal::RegEx(String::from(lex.text))),
                ));
            }
            SyntaxKind::TrueKeyword => {
                return Some(Node::new(
                    Loc::new(lex.pos, lex.end),
                    Expression::Literal(Literal::True),
                ));
            }
            SyntaxKind::FalseKeyword => {
                return Some(Node::new(
                    Loc::new(lex.pos, lex.end),
                    Expression::Literal(Literal::False),
                ));
            }
            SyntaxKind::UndefinedKeyword => {
                return Some(Node::new(
                    Loc::new(lex.pos, lex.end),
                    Expression::Literal(Literal::Undefined),
                ));
            }
            SyntaxKind::NullKeyword => {
                return Some(Node::new(
                    Loc::new(lex.pos, lex.end),
                    Expression::Literal(Literal::Null),
                ));
            }
            _ => {
                let error_message = format!("Expected expression but got {:?}", lex.kind);
                parser.error_context.push(ParseError {
                    pos: lex.pos,
                    end: lex.end,
                    message: error_message,
                });

                None
            }
        }
    }
}

impl<'a, I: Input<'a>> Parse<'a, I> for Identifier {
    type Ast = Option<Node<Identifier>>;

    fn parse(parser: &mut Parser<'a, I>) -> Self::Ast {
        let lex = parser.lexer.pop();
        match lex.kind {
            SyntaxKind::Identifier => {
                let identifier = Identifier {
                    text: String::from(lex.text),
                };

                Some(Node::new(Loc::new(lex.pos, lex.end), identifier))
            }
            _ => {
                parser.error_context.push(ParseError {
                    pos: parser.lexer.peek().pos,
                    end: parser.lexer.peek().end,
                    message: format!("Expected identifier but got {:?}", lex.kind),
                });

                None
            }
        }
    }
}

impl<'a, I: Input<'a>> TryParse<'a, I> for Identifier {
    type Ast = Node<Identifier>;

    fn try_parse(parser: &mut Parser<'a, I>) -> Result<Self::Ast, ParseError> {
        let lex = parser.lexer.pop();
        match lex.kind {
            SyntaxKind::Identifier => {
                let identifier = Identifier {
                    text: String::from(lex.text),
                };

                Ok(Node::new(Loc::new(lex.pos, lex.end), identifier))
            }
            _ => Err(ParseError {
                pos: parser.lexer.peek().pos,
                end: parser.lexer.peek().end,
                message: format!("Expected identifier but got {:?}", lex.kind),
            }),
        }
    }
}
