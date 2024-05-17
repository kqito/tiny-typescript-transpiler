use crate::lexer::inputer::Input;
use crate::lexer::Controller;
use crate::parser::TryParse;
use crate::pop_match_kind;
use crate::Parser;
use ast::declaration::*;
use ast::expression::*;
use ast::kind::SyntaxKind;
use ast::statement::*;
use ast::*;

use super::Parse;
use super::ParseError;

impl<'a, I: Input<'a>> Parse<'a, I> for Parameters {
    type Ast = Option<Parameters>;

    fn parse(parser: &mut Parser<'a, I>) -> Self::Ast {
        let has_open_paren = pop_match_kind!(parser, SyntaxKind::OpenParenToken);
        let has_close_paren = parser
            .lexer
            .find(|lex| lex.kind == SyntaxKind::CloseParenToken);

        if has_open_paren.is_none() || has_close_paren.is_none() {
            return None;
        }

        let mut parameters = Vec::new();

        while let None = pop_match_kind!(parser, SyntaxKind::CloseParenToken) {
            match Parameter::parse(parser) {
                Some(param) => {
                    parameters.push(param);
                    pop_match_kind!(parser, SyntaxKind::CommaToken);
                }
                None => break,
            }
        }

        if parameters.len() == 0 {
            return None;
        }

        Some(parameters)
    }
}

impl<'a, I: Input<'a>> Parse<'a, I> for Parameter {
    type Ast = Option<Node<Parameter>>;

    fn parse(parser: &mut Parser<'a, I>) -> Self::Ast {
        let name = match Identifier::parse(parser) {
            Some(name) => name,
            None => return None,
        };

        // Skip colon
        pop_match_kind!(parser, SyntaxKind::ColonToken);

        let r#type = Type::parse(parser);

        Some(Node::new(
            Loc::new(name.loc.pos, parser.lexer.input.current_end()),
            Parameter { name, r#type },
        ))
    }
}

impl<'a, I: Input<'a>> Parse<'a, I> for Arguments {
    type Ast = Option<Arguments>;

    fn parse(parser: &mut Parser<'a, I>) -> Self::Ast {
        let has_open_paren = pop_match_kind!(parser, SyntaxKind::OpenParenToken);
        let has_close_paren = parser
            .lexer
            .find(|lex| lex.kind == SyntaxKind::CloseParenToken);

        if has_open_paren.is_none() || has_close_paren.is_none() {
            return None;
        }

        let mut expressions = Vec::new();
        while let None = pop_match_kind!(parser, SyntaxKind::CloseParenToken) {
            if let Some(expr) = Expression::parse(parser) {
                expressions.push(expr);
                pop_match_kind!(parser, SyntaxKind::CommaToken);
            }
        }

        Some(expressions)
    }
}

impl<'a, I: Input<'a>> TryParse<'a, I> for Arguments {
    type Ast = Arguments;

    fn try_parse(parser: &mut Parser<'a, I>) -> Result<Self::Ast, ParseError> {
        let has_open_paren = pop_match_kind!(parser, SyntaxKind::OpenParenToken);
        let has_close_paren = parser
            .lexer
            .find(|lex| lex.kind == SyntaxKind::CloseParenToken);

        if has_open_paren.is_none() || has_close_paren.is_none() {
            return Err(ParseError {
                pos: parser.lexer.peek().pos,
                end: parser.lexer.peek().end,
                message: String::from("Expected '(' but got None"),
            });
        }

        let mut expressions = Vec::new();
        while let None = pop_match_kind!(parser, SyntaxKind::CloseParenToken) {
            if let Some(expr) = Expression::parse(parser) {
                expressions.push(expr);
                pop_match_kind!(parser, SyntaxKind::CommaToken);
            }
        }

        Ok(expressions)
    }
}

impl<'a, I: Input<'a>> Parse<'a, I> for CallExpression {
    type Ast = Option<Node<CallExpression>>;

    fn parse(parser: &mut Parser<'a, I>) -> Self::Ast {
        let save_pos = parser.lexer.input.current_pos();
        let save_end = parser.lexer.input.current_end();
        let ident = match Identifier::try_parse(parser) {
            Ok(ident) => ident,
            Err(_) => {
                parser.lexer.input.reset_to(save_pos, save_end);
                return None;
            }
        };

        let pos = ident.loc.pos;
        let args = match Arguments::try_parse(parser) {
            Ok(args) => args,
            Err(_) => {
                parser.lexer.input.reset_to(save_pos, save_end);
                return None;
            }
        };

        let mut call_expression: Option<Node<CallExpression>> = Some(Node::new(
            Loc::new(pos, parser.lexer.input.current_end()),
            CallExpression {
                expression: Box::new(Node::new(
                    ident.loc.clone(),
                    Expression::Identifier(ident.item.clone()),
                )),
                arguments: args,
            },
        ));

        while let Some(args) = Arguments::parse(parser) {
            let end = parser.lexer.input.current_end();
            call_expression = match call_expression {
                Some(c) => {
                    let call_expression = CallExpression {
                        expression: Box::new(Node::new(c.loc, Expression::CallExpression(c.item))),
                        arguments: args,
                    };

                    Some(Node::new(Loc::new(pos, end), call_expression))
                }
                None => {
                    let call_expression = CallExpression {
                        expression: Box::new(Node::new(
                            ident.loc.clone(),
                            Expression::Identifier(ident.item.clone()),
                        )),
                        arguments: args,
                    };

                    Some(Node::new(Loc::new(pos, end), call_expression))
                }
            }
        }

        call_expression
    }
}

impl<'a, I: Input<'a>> Parse<'a, I> for Block {
    type Ast = Option<Node<Block>>;

    fn parse(parser: &mut Parser<'a, I>) -> Self::Ast {
        let open_brace = match pop_match_kind!(parser, SyntaxKind::OpenBraceToken) {
            Some(lex) => lex,
            None => return None,
        };

        let mut option_statements = Vec::new();

        while let None = pop_match_kind!(parser, SyntaxKind::CloseBraceToken) {
            option_statements.push(Statement::parse(parser));
        }

        let statements: Vec<Node<Statement>> = option_statements
            .into_iter()
            .filter_map(|statement| statement)
            .collect();

        let end = parser.lexer.input.current_end();

        Some(Node::new(
            Loc::new(open_brace.pos, end),
            Block { statements },
        ))
    }
}
