#[cfg(test)]
mod function_declaration {
    use crate::lexer::tests::utils::{assert_lexes, LexCreateHelper as helper};
    use ast::kind::SyntaxKind;

    #[test]
    pub fn function_declaration() {
        assert_lexes(
            "function fn() {}",
            vec![
                helper::create_lex((0, 8), SyntaxKind::FunctionKeyword, "function"),
                helper::create_lex((8, 11), SyntaxKind::Identifier, "fn"),
                helper::create_lex((11, 12), SyntaxKind::OpenParenToken, ""),
                helper::create_lex((12, 13), SyntaxKind::CloseParenToken, ""),
                helper::create_lex((13, 15), SyntaxKind::OpenBraceToken, ""),
                helper::create_lex((15, 16), SyntaxKind::CloseBraceToken, ""),
            ],
        );

        assert_lexes(
            r#"function fn() { const hi = "hello"; }"#,
            vec![
                helper::create_lex((0, 8), SyntaxKind::FunctionKeyword, "function"),
                helper::create_lex((8, 11), SyntaxKind::Identifier, "fn"),
                helper::create_lex((11, 12), SyntaxKind::OpenParenToken, ""),
                helper::create_lex((12, 13), SyntaxKind::CloseParenToken, ""),
                helper::create_lex((13, 15), SyntaxKind::OpenBraceToken, ""),
                helper::create_lex((15, 21), SyntaxKind::ConstKeyword, "const"),
                helper::create_lex((21, 24), SyntaxKind::Identifier, "hi"),
                helper::create_lex((24, 26), SyntaxKind::EqualsToken, ""),
                helper::create_lex((26, 34), SyntaxKind::StringLiteral, "hello"),
                helper::create_lex((34, 35), SyntaxKind::SemicolonToken, ""),
                helper::create_lex((35, 37), SyntaxKind::CloseBraceToken, ""),
            ],
        );
    }

    #[test]
    pub fn function_declaration_with_args() {
        assert_lexes(
            "function fn(arg1: string) {}",
            vec![
                helper::create_lex((0, 8), SyntaxKind::FunctionKeyword, "function"),
                helper::create_lex((8, 11), SyntaxKind::Identifier, "fn"),
                helper::create_lex((11, 12), SyntaxKind::OpenParenToken, ""),
                helper::create_lex((12, 16), SyntaxKind::Identifier, "arg1"),
                helper::create_lex((16, 17), SyntaxKind::ColonToken, ""),
                helper::create_lex((17, 24), SyntaxKind::StringKeyword, "string"),
                helper::create_lex((24, 25), SyntaxKind::CloseParenToken, ""),
                helper::create_lex((25, 27), SyntaxKind::OpenBraceToken, ""),
                helper::create_lex((27, 28), SyntaxKind::CloseBraceToken, ""),
            ],
        );
    }
}
