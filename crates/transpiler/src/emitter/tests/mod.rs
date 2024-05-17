mod expression_statement;
mod function_declaration;
mod type_alias_declaration;
mod variable_statement;

#[cfg(test)]
mod utils {
    use crate::emitter::Emitter;
    use crate::lexer::inputer::Inputer;
    use crate::lexer::*;
    use crate::parser::*;
    use pretty_assertions::assert_eq;

    pub fn assert_emit_stmt<'a>(content: &'a str, expected: &'a str) {
        let mut inputer = Inputer::from(content);
        let lexer = Lexer::from(&mut inputer);
        let mut parse = Parser::new(lexer);
        let modules = match parse.parse_module() {
            Ok(modules) => modules,
            // highlight panic message
            Err(err) => panic!(
                "Failed to parse module: \x1b[31m{:?} => {:?}\x1b[0m",
                content, err.errors
            ),
        };

        let emitter = Emitter::new();
        let emit_result = match emitter.emit_module(&modules) {
            Ok(r) => r,
            Err(err) => panic!(
                "Failed to emit module: \x1b[31m{:?} => {:?}\x1b[0m",
                content, err.errors
            ),
        };

        assert_eq!(&emit_result, expected);
    }
}
