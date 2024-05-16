pub mod function_declaration;
pub mod type_alias_declaration;
pub mod variable_statement;

#[cfg(test)]
mod utils {
    use crate::lexer::Controller;
    use crate::lexer::{inputer::Inputer, Lex, Lexer};
    use pretty_assertions::assert_eq;

    pub fn assert_lexes<'a>(content: &'a str, expected: Vec<Lex<'a>>) {
        let mut inputer = Inputer::from(content);
        let mut lexer = Lexer::from(&mut inputer);
        let lexes = lexer.pop_all();

        assert_eq!(lexes, expected);
    }
}
