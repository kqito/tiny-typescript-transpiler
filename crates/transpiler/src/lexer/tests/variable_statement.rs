#[cfg(test)]
mod variable_statement {
    use crate::lexer::{tests::utils::assert_lexes, Lex};
    use ast::kind::SyntaxKind;

    #[test]
    pub fn variable_declaration() {
        assert_lexes(
            "var a = 100;",
            vec![
                Lex {
                    pos: 0,
                    end: 3,
                    kind: SyntaxKind::VarKeyword,
                    text: "var",
                },
                Lex {
                    pos: 3,
                    end: 5,
                    kind: SyntaxKind::Identifier,
                    text: "a",
                },
                Lex {
                    pos: 5,
                    end: 7,
                    kind: SyntaxKind::EqualsToken,
                    text: "",
                },
                Lex {
                    pos: 7,
                    end: 11,
                    kind: SyntaxKind::NumericLiteral,
                    text: "100",
                },
                Lex {
                    pos: 11,
                    end: 12,
                    kind: SyntaxKind::SemicolonToken,
                    text: "",
                },
            ],
        );
    }

    #[test]
    pub fn string_variable_declaration() {
        assert_lexes(
            r#"var a = "hogehoge";"#,
            vec![
                Lex {
                    pos: 0,
                    end: 3,
                    kind: SyntaxKind::VarKeyword,
                    text: "var",
                },
                Lex {
                    pos: 3,
                    end: 5,
                    kind: SyntaxKind::Identifier,
                    text: "a",
                },
                Lex {
                    pos: 5,
                    end: 7,
                    kind: SyntaxKind::EqualsToken,
                    text: "",
                },
                Lex {
                    pos: 7,
                    end: 18,
                    kind: SyntaxKind::StringLiteral,
                    text: "hogehoge",
                },
                Lex {
                    pos: 18,
                    end: 19,
                    kind: SyntaxKind::SemicolonToken,
                    text: "",
                },
            ],
        );

        assert_lexes(
            "const b: string = 'piyo';",
            vec![
                Lex {
                    pos: 0,
                    end: 5,
                    kind: SyntaxKind::ConstKeyword,
                    text: "const",
                },
                Lex {
                    pos: 5,
                    end: 7,
                    kind: SyntaxKind::Identifier,
                    text: "b",
                },
                Lex {
                    pos: 7,
                    end: 8,
                    kind: SyntaxKind::ColonToken,
                    text: "",
                },
                Lex {
                    pos: 8,
                    end: 15,
                    kind: SyntaxKind::StringKeyword,
                    text: "string",
                },
                Lex {
                    pos: 15,
                    end: 17,
                    kind: SyntaxKind::EqualsToken,
                    text: "",
                },
                Lex {
                    pos: 17,
                    end: 24,
                    kind: SyntaxKind::StringLiteral,
                    text: "piyo",
                },
                Lex {
                    pos: 24,
                    end: 25,
                    kind: SyntaxKind::SemicolonToken,
                    text: "",
                },
            ],
        );

        assert_lexes(
            r#"let hoge: string;"#,
            vec![
                Lex {
                    pos: 0,
                    end: 3,
                    kind: SyntaxKind::LetKeyword,
                    text: "let",
                },
                Lex {
                    pos: 3,
                    end: 8,
                    kind: SyntaxKind::Identifier,
                    text: "hoge",
                },
                Lex {
                    pos: 8,
                    end: 9,
                    kind: SyntaxKind::ColonToken,
                    text: "",
                },
                Lex {
                    pos: 9,
                    end: 16,
                    kind: SyntaxKind::StringKeyword,
                    text: "string",
                },
                Lex {
                    pos: 16,
                    end: 17,
                    kind: SyntaxKind::SemicolonToken,
                    text: "",
                },
            ],
        );

        assert_lexes(
            r#"var a = "hogehoge";"#,
            vec![
                Lex {
                    pos: 0,
                    end: 3,
                    kind: SyntaxKind::VarKeyword,
                    text: "var",
                },
                Lex {
                    pos: 3,
                    end: 5,
                    kind: SyntaxKind::Identifier,
                    text: "a",
                },
                Lex {
                    pos: 5,
                    end: 7,
                    kind: SyntaxKind::EqualsToken,
                    text: "",
                },
                Lex {
                    pos: 7,
                    end: 18,
                    kind: SyntaxKind::StringLiteral,
                    text: "hogehoge",
                },
                Lex {
                    pos: 18,
                    end: 19,
                    kind: SyntaxKind::SemicolonToken,
                    text: "",
                },
            ],
        );
        assert_lexes(
            r#"const b: string = "piyo";"#,
            vec![
                Lex {
                    pos: 0,
                    end: 5,
                    kind: SyntaxKind::ConstKeyword,
                    text: "const",
                },
                Lex {
                    pos: 5,
                    end: 7,
                    kind: SyntaxKind::Identifier,
                    text: "b",
                },
                Lex {
                    pos: 7,
                    end: 8,
                    kind: SyntaxKind::ColonToken,
                    text: "",
                },
                Lex {
                    pos: 8,
                    end: 15,
                    kind: SyntaxKind::StringKeyword,
                    text: "string",
                },
                Lex {
                    pos: 15,
                    end: 17,
                    kind: SyntaxKind::EqualsToken,
                    text: "",
                },
                Lex {
                    pos: 17,
                    end: 24,
                    kind: SyntaxKind::StringLiteral,
                    text: "piyo",
                },
                Lex {
                    pos: 24,
                    end: 25,
                    kind: SyntaxKind::SemicolonToken,
                    text: "",
                },
            ],
        );
        assert_lexes(
            r#"const b: string = "'hello\n'";"#,
            vec![
                Lex {
                    pos: 0,
                    end: 5,
                    kind: SyntaxKind::ConstKeyword,
                    text: "const",
                },
                Lex {
                    pos: 5,
                    end: 7,
                    kind: SyntaxKind::Identifier,
                    text: "b",
                },
                Lex {
                    pos: 7,
                    end: 8,
                    kind: SyntaxKind::ColonToken,
                    text: "",
                },
                Lex {
                    pos: 8,
                    end: 15,
                    kind: SyntaxKind::StringKeyword,
                    text: "string",
                },
                Lex {
                    pos: 15,
                    end: 17,
                    kind: SyntaxKind::EqualsToken,
                    text: "",
                },
                Lex {
                    pos: 17,
                    end: 29,
                    kind: SyntaxKind::StringLiteral,
                    text: "'hello\\n'",
                },
                Lex {
                    pos: 29,
                    end: 30,
                    kind: SyntaxKind::SemicolonToken,
                    text: "",
                },
            ],
        );
        assert_lexes(
            r#"let hoge: string;"#,
            vec![
                Lex {
                    pos: 0,
                    end: 3,
                    kind: SyntaxKind::LetKeyword,
                    text: "let",
                },
                Lex {
                    pos: 3,
                    end: 8,
                    kind: SyntaxKind::Identifier,
                    text: "hoge",
                },
                Lex {
                    pos: 8,
                    end: 9,
                    kind: SyntaxKind::ColonToken,
                    text: "",
                },
                Lex {
                    pos: 9,
                    end: 16,
                    kind: SyntaxKind::StringKeyword,
                    text: "string",
                },
                Lex {
                    pos: 16,
                    end: 17,
                    kind: SyntaxKind::SemicolonToken,
                    text: "",
                },
            ],
        )
    }

    #[test]
    pub fn variable_declaration_with_extra_space() {
        assert_lexes(
            "var  a =   100;",
            vec![
                Lex {
                    pos: 0,
                    end: 3,
                    kind: SyntaxKind::VarKeyword,
                    text: "var",
                },
                Lex {
                    pos: 3,
                    end: 6,
                    kind: SyntaxKind::Identifier,
                    text: "a",
                },
                Lex {
                    pos: 6,
                    end: 8,
                    kind: SyntaxKind::EqualsToken,
                    text: "",
                },
                Lex {
                    pos: 8,
                    end: 14,
                    kind: SyntaxKind::NumericLiteral,
                    text: "100",
                },
                Lex {
                    pos: 14,
                    end: 15,
                    kind: SyntaxKind::SemicolonToken,
                    text: "",
                },
            ],
        )
    }

    #[test]
    pub fn variable_declaration_with_type_annotation() {
        assert_lexes(
            "var a: number = 100;",
            vec![
                Lex {
                    pos: 0,
                    end: 3,
                    kind: SyntaxKind::VarKeyword,
                    text: "var",
                },
                Lex {
                    pos: 3,
                    end: 5,
                    kind: SyntaxKind::Identifier,
                    text: "a",
                },
                Lex {
                    pos: 5,
                    end: 6,
                    kind: SyntaxKind::ColonToken,
                    text: "",
                },
                Lex {
                    pos: 6,
                    end: 13,
                    kind: SyntaxKind::NumberKeyword,
                    text: "number",
                },
                Lex {
                    pos: 13,
                    end: 15,
                    kind: SyntaxKind::EqualsToken,
                    text: "",
                },
                Lex {
                    pos: 15,
                    end: 19,
                    kind: SyntaxKind::NumericLiteral,
                    text: "100",
                },
                Lex {
                    pos: 19,
                    end: 20,
                    kind: SyntaxKind::SemicolonToken,
                    text: "",
                },
            ],
        )
    }
}
