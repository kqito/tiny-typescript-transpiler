#[cfg(test)]
mod variable_statement {
    use crate::lexer::tests::utils::{assert_lexes, LexCreateHelper as helper};
    use ast::kind::SyntaxKind;

    #[test]
    pub fn variable_declaration() {
        assert_lexes(
            "var a = 100;",
            vec![
                helper::create_lex((0, 3), SyntaxKind::VarKeyword, "var"),
                helper::create_lex((3, 5), SyntaxKind::Identifier, "a"),
                helper::create_lex((5, 7), SyntaxKind::EqualsToken, ""),
                helper::create_lex((7, 11), SyntaxKind::NumericLiteral, "100"),
                helper::create_lex((11, 12), SyntaxKind::SemicolonToken, ""),
            ],
        );

        assert_lexes(
            "var a = '100';",
            vec![
                helper::create_lex((0, 3), SyntaxKind::VarKeyword, "var"),
                helper::create_lex((3, 5), SyntaxKind::Identifier, "a"),
                helper::create_lex((5, 7), SyntaxKind::EqualsToken, ""),
                helper::create_lex((7, 13), SyntaxKind::StringLiteral, "100"),
                helper::create_lex((13, 14), SyntaxKind::SemicolonToken, ""),
            ],
        );

        assert_lexes(
            "var a = true;",
            vec![
                helper::create_lex((0, 3), SyntaxKind::VarKeyword, "var"),
                helper::create_lex((3, 5), SyntaxKind::Identifier, "a"),
                helper::create_lex((5, 7), SyntaxKind::EqualsToken, ""),
                helper::create_lex((7, 12), SyntaxKind::TrueKeyword, "true"),
                helper::create_lex((12, 13), SyntaxKind::SemicolonToken, ""),
            ],
        );

        assert_lexes(
            "var a = false;",
            vec![
                helper::create_lex((0, 3), SyntaxKind::VarKeyword, "var"),
                helper::create_lex((3, 5), SyntaxKind::Identifier, "a"),
                helper::create_lex((5, 7), SyntaxKind::EqualsToken, ""),
                helper::create_lex((7, 13), SyntaxKind::FalseKeyword, "false"),
                helper::create_lex((13, 14), SyntaxKind::SemicolonToken, ""),
            ],
        );

        assert_lexes(
            "var a = undefined;",
            vec![
                helper::create_lex((0, 3), SyntaxKind::VarKeyword, "var"),
                helper::create_lex((3, 5), SyntaxKind::Identifier, "a"),
                helper::create_lex((5, 7), SyntaxKind::EqualsToken, ""),
                helper::create_lex((7, 17), SyntaxKind::UndefinedKeyword, "undefined"),
                helper::create_lex((17, 18), SyntaxKind::SemicolonToken, ""),
            ],
        );

        assert_lexes(
            "var a = null;",
            vec![
                helper::create_lex((0, 3), SyntaxKind::VarKeyword, "var"),
                helper::create_lex((3, 5), SyntaxKind::Identifier, "a"),
                helper::create_lex((5, 7), SyntaxKind::EqualsToken, ""),
                helper::create_lex((7, 12), SyntaxKind::NullKeyword, "null"),
                helper::create_lex((12, 13), SyntaxKind::SemicolonToken, ""),
            ],
        );

        // Binary
        assert_lexes(
            "var a = 0b1010;",
            vec![
                helper::create_lex((0, 3), SyntaxKind::VarKeyword, "var"),
                helper::create_lex((3, 5), SyntaxKind::Identifier, "a"),
                helper::create_lex((5, 7), SyntaxKind::EqualsToken, ""),
                helper::create_lex((7, 14), SyntaxKind::NumericLiteral, "0b1010"),
                helper::create_lex((14, 15), SyntaxKind::SemicolonToken, ""),
            ],
        );

        assert_lexes(
            "var a = /ab+c/;",
            vec![
                helper::create_lex((0, 3), SyntaxKind::VarKeyword, "var"),
                helper::create_lex((3, 5), SyntaxKind::Identifier, "a"),
                helper::create_lex((5, 7), SyntaxKind::EqualsToken, ""),
                helper::create_lex((7, 14), SyntaxKind::RegularExpressionLiteral, "/ab+c/"),
                helper::create_lex((14, 15), SyntaxKind::SemicolonToken, ""),
            ],
        );
    }

    #[test]
    pub fn string_variable_declaration() {
        assert_lexes(
            r#"var a = "hogehoge";"#,
            vec![
                helper::create_lex((0, 3), SyntaxKind::VarKeyword, "var"),
                helper::create_lex((3, 5), SyntaxKind::Identifier, "a"),
                helper::create_lex((5, 7), SyntaxKind::EqualsToken, ""),
                helper::create_lex((7, 18), SyntaxKind::StringLiteral, "hogehoge"),
                helper::create_lex((18, 19), SyntaxKind::SemicolonToken, ""),
            ],
        );

        assert_lexes(
            "const b: string = 'piyo';",
            vec![
                helper::create_lex((0, 5), SyntaxKind::ConstKeyword, "const"),
                helper::create_lex((5, 7), SyntaxKind::Identifier, "b"),
                helper::create_lex((7, 8), SyntaxKind::ColonToken, ""),
                helper::create_lex((8, 15), SyntaxKind::StringKeyword, "string"),
                helper::create_lex((15, 17), SyntaxKind::EqualsToken, ""),
                helper::create_lex((17, 24), SyntaxKind::StringLiteral, "piyo"),
                helper::create_lex((24, 25), SyntaxKind::SemicolonToken, ""),
            ],
        );

        assert_lexes(
            r#"let hoge: string;"#,
            vec![
                helper::create_lex((0, 3), SyntaxKind::LetKeyword, "let"),
                helper::create_lex((3, 8), SyntaxKind::Identifier, "hoge"),
                helper::create_lex((8, 9), SyntaxKind::ColonToken, ""),
                helper::create_lex((9, 16), SyntaxKind::StringKeyword, "string"),
                helper::create_lex((16, 17), SyntaxKind::SemicolonToken, ""),
            ],
        );

        assert_lexes(
            r#"var a = "hogehoge";"#,
            vec![
                helper::create_lex((0, 3), SyntaxKind::VarKeyword, "var"),
                helper::create_lex((3, 5), SyntaxKind::Identifier, "a"),
                helper::create_lex((5, 7), SyntaxKind::EqualsToken, ""),
                helper::create_lex((7, 18), SyntaxKind::StringLiteral, "hogehoge"),
                helper::create_lex((18, 19), SyntaxKind::SemicolonToken, ""),
            ],
        );
        assert_lexes(
            r#"const b: string = "piyo";"#,
            vec![
                helper::create_lex((0, 5), SyntaxKind::ConstKeyword, "const"),
                helper::create_lex((5, 7), SyntaxKind::Identifier, "b"),
                helper::create_lex((7, 8), SyntaxKind::ColonToken, ""),
                helper::create_lex((8, 15), SyntaxKind::StringKeyword, "string"),
                helper::create_lex((15, 17), SyntaxKind::EqualsToken, ""),
                helper::create_lex((17, 24), SyntaxKind::StringLiteral, "piyo"),
                helper::create_lex((24, 25), SyntaxKind::SemicolonToken, ""),
            ],
        );
        assert_lexes(
            r#"const b: string = "'hello\n'";"#,
            vec![
                helper::create_lex((0, 5), SyntaxKind::ConstKeyword, "const"),
                helper::create_lex((5, 7), SyntaxKind::Identifier, "b"),
                helper::create_lex((7, 8), SyntaxKind::ColonToken, ""),
                helper::create_lex((8, 15), SyntaxKind::StringKeyword, "string"),
                helper::create_lex((15, 17), SyntaxKind::EqualsToken, ""),
                helper::create_lex((17, 29), SyntaxKind::StringLiteral, "'hello\\n'"),
                helper::create_lex((29, 30), SyntaxKind::SemicolonToken, ""),
            ],
        );
        assert_lexes(
            r#"let hoge: string;"#,
            vec![
                helper::create_lex((0, 3), SyntaxKind::LetKeyword, "let"),
                helper::create_lex((3, 8), SyntaxKind::Identifier, "hoge"),
                helper::create_lex((8, 9), SyntaxKind::ColonToken, ""),
                helper::create_lex((9, 16), SyntaxKind::StringKeyword, "string"),
                helper::create_lex((16, 17), SyntaxKind::SemicolonToken, ""),
            ],
        )
    }

    #[test]
    pub fn variable_declaration_with_extra_space() {
        assert_lexes(
            "var  a =   100;",
            vec![
                helper::create_lex((0, 3), SyntaxKind::VarKeyword, "var"),
                helper::create_lex((3, 6), SyntaxKind::Identifier, "a"),
                helper::create_lex((6, 8), SyntaxKind::EqualsToken, ""),
                helper::create_lex((8, 14), SyntaxKind::NumericLiteral, "100"),
                helper::create_lex((14, 15), SyntaxKind::SemicolonToken, ""),
            ],
        )
    }

    #[test]
    pub fn variable_declaration_with_type_annotation() {
        assert_lexes(
            "var a: number = 100;",
            vec![
                helper::create_lex((0, 3), SyntaxKind::VarKeyword, "var"),
                helper::create_lex((3, 5), SyntaxKind::Identifier, "a"),
                helper::create_lex((5, 6), SyntaxKind::ColonToken, ""),
                helper::create_lex((6, 13), SyntaxKind::NumberKeyword, "number"),
                helper::create_lex((13, 15), SyntaxKind::EqualsToken, ""),
                helper::create_lex((15, 19), SyntaxKind::NumericLiteral, "100"),
                helper::create_lex((19, 20), SyntaxKind::SemicolonToken, ""),
            ],
        )
    }
}
