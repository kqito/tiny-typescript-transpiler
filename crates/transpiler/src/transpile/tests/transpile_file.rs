#[cfg(test)]
mod type_alias_statement {
    use crate::{
        parser::{ParseError, ParseErrorContext},
        transpile::{tests::utils::assert_transpile_file, TranspileResult},
    };

    #[test]
    fn test_const_declaration_statement() {
        let content = "const a: number = 1;";
        let expected = Ok(TranspileResult {
            emit_result: "var a = 1;".to_string(),
        });
        assert_transpile_file(content, expected);
    }

    #[test]
    fn test_let_declaration_statement() {
        let content = "let a: number = 1;";
        let expected = Ok(TranspileResult {
            emit_result: "var a = 1;".to_string(),
        });
        assert_transpile_file(content, expected);
    }

    #[test]
    fn test_var_declaration_statement() {
        let content = "var a: number = 1;";
        let expected = Ok(TranspileResult {
            emit_result: "var a = 1;".to_string(),
        });
        assert_transpile_file(content, expected);
    }

    #[test]
    fn test_invalid_declaration_statement() {
        let content = "a: number = 1;";
        let expected = Err(ParseErrorContext {
            errors: vec![
                ParseError {
                    pos: 9,
                    end: 11,
                    message: "Expected identifier but got NumberKeyword".to_string(),
                },
                ParseError {
                    pos: 13,
                    end: 14,
                    message: "Expected identifier but got NumericLiteral".to_string(),
                },
            ],
        });
        assert_transpile_file(content, expected);
    }
}
