use crate::lexer::inputer::Input;
use crate::lexer::Controller;
use crate::pop_match_kind;
use crate::Parser;
use ast::declaration::*;
use ast::expression::*;
use ast::kind::SyntaxKind;
use ast::*;

use super::Parse;

impl<'a, I: Input<'a>> Parse<'a, I> for VariableDeclaration {
    type Ast = Option<Node<VariableDeclaration>>;

    fn parse(parser: &mut Parser<'a, I>) -> Self::Ast {
        let name = match Identifier::parse(parser) {
            Some(name) => name,
            None => return None,
        };

        let r#type = match pop_match_kind!(parser, SyntaxKind::ColonToken) {
            Some(_) => Type::parse(parser),
            None => None,
        };

        let mut end = parser.lexer.input.current_pos();
        let init = match pop_match_kind!(parser, SyntaxKind::EqualsToken) {
            Some(_) => {
                let expr = Expression::parse(parser);
                end = parser.lexer.input.current_end();
                expr
            }
            None => None,
        };

        Some(Node::new(
            Loc::new(name.loc.pos, end),
            VariableDeclaration { name, init, r#type },
        ))
    }
}
