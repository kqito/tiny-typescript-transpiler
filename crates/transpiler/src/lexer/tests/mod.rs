pub mod function_declaration;
pub mod type_alias_declaration;
pub mod variable_statement;

#[cfg(test)]
mod utils {
    use crate::lexer::Controller;
    use crate::lexer::{inputer::Inputer, Lex, Lexer};
    use ast::kind::SyntaxKind;
    use pretty_assertions::assert_eq;

    pub fn assert_lexes<'a>(content: &'a str, expected: Vec<Lex<'a>>) {
        let mut inputer = Inputer::from(content);
        let mut lexer = Lexer::from(&mut inputer);
        let lexes = lexer.peek_all();

        assert_eq!(lexes, expected);
    }

    pub struct LexCreateHelper;

    impl LexCreateHelper {
        pub fn create_lex<'a>(loc: (u32, u32), kind: SyntaxKind, text: &'a str) -> Lex<'a> {
            Lex {
                pos: loc.0,
                end: loc.1,
                kind,
                text,
            }
        }
    }
}
