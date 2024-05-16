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
            Err(err) => panic!("Failed to parse module: {:?}", err),
        };

        let emitter = Emitter::new();
        let emit_result = emitter.emit_module(&modules).unwrap();

        assert_eq!(emit_result, expected.to_string());
    }
}
