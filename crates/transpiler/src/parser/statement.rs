use crate::lexer::inputer::Input;
use crate::lexer::Controller;
use crate::pop_match_kind;
use crate::Parser;
use ast::declaration::*;
use ast::expression::*;
use ast::kind::SyntaxKind;
use ast::statement::*;
use ast::*;

use super::Parse;

impl<'a, I: Input<'a>> Parse<'a, I> for Statement {
    type Ast = Option<Node<Statement>>;

    fn parse(parser: &mut Parser<'a, I>) -> Self::Ast {
        let prefix_lex = parser.lexer.peek();

        match prefix_lex.kind {
            SyntaxKind::VarKeyword | SyntaxKind::ConstKeyword | SyntaxKind::LetKeyword => {
                return VariableStatement::parse(parser).and_then(|stmt| Some(stmt.into()));
            }
            SyntaxKind::FunctionKeyword => {
                return FunctionDeclaration::parse(parser).and_then(|stmt| Some(stmt.into()));
            }
            SyntaxKind::TypeKeyword => {
                return TypeAliasDeclaration::parse(parser).and_then(|stmt| Some(stmt.into()));
            }
            SyntaxKind::AmpersandToken
            | SyntaxKind::AsteriskToken
            | SyntaxKind::AtToken
            | SyntaxKind::BackslashToken
            | SyntaxKind::BacktickToken
            | SyntaxKind::BarToken
            | SyntaxKind::CaretToken
            | SyntaxKind::CloseBraceToken
            | SyntaxKind::CloseBracketToken
            | SyntaxKind::CloseParenToken
            | SyntaxKind::ColonToken
            | SyntaxKind::CommaToken
            | SyntaxKind::DotToken
            | SyntaxKind::DoubleQuoteToken
            | SyntaxKind::EqualsToken
            | SyntaxKind::ExclamationToken
            | SyntaxKind::GreaterThanToken
            | SyntaxKind::HashToken
            | SyntaxKind::LessThanToken
            | SyntaxKind::MinusToken
            | SyntaxKind::OpenBraceToken
            | SyntaxKind::OpenBracketToken
            | SyntaxKind::OpenParenToken
            | SyntaxKind::PercentToken
            | SyntaxKind::PlusToken
            | SyntaxKind::QuestionToken
            | SyntaxKind::SemicolonToken
            | SyntaxKind::SingleQuoteToken
            | SyntaxKind::SlashToken
            | SyntaxKind::TildeToken => {
                parser.lexer.pop();
                return None;
            }
            SyntaxKind::BOF | SyntaxKind::EOF => {
                parser.lexer.pop();
                return None;
            }
            _ => {
                return ExpressionStatement::parse(parser).and_then(|stmt| Some(stmt.into()));
            }
        }
    }
}

impl<'a, I: Input<'a>> Parse<'a, I> for ExpressionStatement {
    type Ast = Option<Node<ExpressionStatement>>;

    fn parse(parser: &mut Parser<'a, I>) -> Self::Ast {
        if let Some(call_expr) = CallExpression::parse(parser) {
            pop_match_kind!(parser, SyntaxKind::SemicolonToken);
            let end = parser.lexer.input.current_end();

            return Some(Node::new(
                Loc::new(call_expr.loc.pos, end),
                ExpressionStatement {
                    expression: call_expr.into(),
                },
            ));
        }

        let name = match Identifier::parse(parser) {
            Some(i) => i,
            None => return None,
        };

        let peek = parser.lexer.peek();
        pop_match_kind!(parser, SyntaxKind::SemicolonToken);
        let end = parser.lexer.input.current_end();

        Some(Node::new(
            Loc::new(name.loc.pos, end),
            ExpressionStatement {
                expression: Node::new(name.loc, Expression::Identifier(name.item)),
            },
        ))
    }
}

impl<'a, I: Input<'a>> Parse<'a, I> for FunctionDeclaration {
    type Ast = Option<Node<FunctionDeclaration>>;

    fn parse(parser: &mut Parser<'a, I>) -> Self::Ast {
        let fn_key = match pop_match_kind!(parser, SyntaxKind::FunctionKeyword) {
            Some(lex) => lex,
            None => return None,
        };

        let name = match Identifier::parse(parser) {
            Some(name) => name,
            None => return None,
        };

        let parameters = Parameters::parse(parser);
        let body = Block::parse(parser);

        let end = parser.lexer.input.current_end();

        return Some(Node::new(
            Loc::new(fn_key.pos, end),
            FunctionDeclaration {
                name,
                r#type: None,
                parameters,
                body,
            },
        ));
    }
}

impl<'a, I: Input<'a>> Parse<'a, I> for TypeAliasDeclaration {
    type Ast = Option<Node<TypeAliasDeclaration>>;

    fn parse(parser: &mut Parser<'a, I>) -> Self::Ast {
        let type_key = match pop_match_kind!(parser, SyntaxKind::TypeKeyword) {
            Some(lex) => lex,
            None => return None,
        };

        let name = match Identifier::parse(parser) {
            Some(name) => name,
            None => return None,
        };

        let r#type = match pop_match_kind!(parser, SyntaxKind::EqualsToken) {
            Some(_) => Type::parse(parser),
            None => None,
        };

        pop_match_kind!(parser, SyntaxKind::SemicolonToken);
        let end = parser.lexer.input.current_end();

        return Some(Node::new(
            Loc::new(type_key.pos, end),
            TypeAliasDeclaration { name, r#type },
        ));
    }
}

impl<'a, I: Input<'a>> Parse<'a, I> for VariableStatement {
    type Ast = Option<Node<VariableStatement>>;

    fn parse(parser: &mut Parser<'a, I>) -> Self::Ast {
        let variable = match pop_match_kind!(
            parser,
            SyntaxKind::VarKeyword,
            SyntaxKind::ConstKeyword,
            SyntaxKind::LetKeyword
        ) {
            Some(lex) => lex,
            None => return None,
        };

        let dec = match VariableDeclaration::parse(parser) {
            Some(dec) => dec,
            None => return None,
        };

        pop_match_kind!(parser, SyntaxKind::SemicolonToken);
        let end = parser.lexer.input.current_end();

        return Some(Node::new(
            Loc::new(variable.pos, end),
            VariableStatement {
                declaration_list: Node::new(
                    Loc::new(variable.pos, dec.loc.end),
                    VariableDeclarationList {
                        declarations: vec![dec],
                    },
                ),
            },
        ));
    }
}
