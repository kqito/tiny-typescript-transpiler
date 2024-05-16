use crate::lexer::inputer::Input;
use crate::lexer::Controller;
use crate::Parser;
use ast::declaration::*;
use ast::expression::*;
use ast::kind::SyntaxKind;
use ast::*;

use super::Parse;

impl<'a, I: Input<'a>> Parse<'a, I> for Type {
    type Ast = Option<Node<Type>>;

    fn parse(parser: &mut Parser<'a, I>) -> Self::Ast {
        match parser.lexer.peek().kind {
            SyntaxKind::StringKeyword => {
                let lex = parser.lexer.pop();
                return Some(Node::new(Loc::new(lex.pos, lex.end), Type::StringKeyword));
            }

            SyntaxKind::NumberKeyword => {
                let lex = parser.lexer.pop();
                return Some(Node::new(Loc::new(lex.pos, lex.end), Type::NumberKeyword));
            }

            SyntaxKind::Identifier => {
                let identifier = match Identifier::parse(parser) {
                    Some(ident) => ident,
                    None => return None,
                };

                return Some(Node::new(
                    Loc::new(identifier.loc.pos, identifier.loc.end),
                    Type::TypeReference(TypeReference {
                        type_name: Some(identifier),
                    }),
                ));
            }

            _ => None,
        }
    }
}
