#[cfg(test)]
mod function_declaration {
    use crate::lexer::{tests::utils::assert_lexes, Lex};
    use ast::kind::SyntaxKind;

    #[test]
    pub fn function_declaration() {
        assert_lexes(
            "function fn() {}",
            vec![
                Lex {
                    pos: 0,
                    end: 8,
                    kind: SyntaxKind::FunctionKeyword,
                    text: "function",
                },
                Lex {
                    pos: 8,
                    end: 11,
                    kind: SyntaxKind::Identifier,
                    text: "fn",
                },
                Lex {
                    pos: 11,
                    end: 12,
                    kind: SyntaxKind::OpenParenToken,
                    text: "",
                },
                Lex {
                    pos: 12,
                    end: 13,
                    kind: SyntaxKind::CloseParenToken,
                    text: "",
                },
                Lex {
                    pos: 13,
                    end: 15,
                    kind: SyntaxKind::OpenBraceToken,
                    text: "",
                },
                Lex {
                    pos: 15,
                    end: 16,
                    kind: SyntaxKind::CloseBraceToken,
                    text: "",
                },
            ],
        );

        assert_lexes(
            r#"function fn() { const hi = "hello"; }"#,
            vec![
                Lex {
                    pos: 0,
                    end: 8,
                    kind: SyntaxKind::FunctionKeyword,
                    text: "function",
                },
                Lex {
                    pos: 8,
                    end: 11,
                    kind: SyntaxKind::Identifier,
                    text: "fn",
                },
                Lex {
                    pos: 11,
                    end: 12,
                    kind: SyntaxKind::OpenParenToken,
                    text: "",
                },
                Lex {
                    pos: 12,
                    end: 13,
                    kind: SyntaxKind::CloseParenToken,
                    text: "",
                },
                Lex {
                    pos: 13,
                    end: 15,
                    kind: SyntaxKind::OpenBraceToken,
                    text: "",
                },
                Lex {
                    pos: 15,
                    end: 21,
                    kind: SyntaxKind::ConstKeyword,
                    text: "const",
                },
                Lex {
                    pos: 21,
                    end: 24,
                    kind: SyntaxKind::Identifier,
                    text: "hi",
                },
                Lex {
                    pos: 24,
                    end: 26,
                    kind: SyntaxKind::EqualsToken,
                    text: "",
                },
                Lex {
                    pos: 26,
                    end: 34,
                    kind: SyntaxKind::StringLiteral,
                    text: "hello",
                },
                Lex {
                    pos: 34,
                    end: 35,
                    kind: SyntaxKind::SemicolonToken,
                    text: "",
                },
                Lex {
                    pos: 35,
                    end: 37,
                    kind: SyntaxKind::CloseBraceToken,
                    text: "",
                },
            ],
        );
    }

    #[test]
    pub fn function_declaration_with_args() {
        assert_lexes(
            "function fn(arg1: string) {}",
            vec![
                Lex {
                    pos: 0,
                    end: 8,
                    kind: SyntaxKind::FunctionKeyword,
                    text: "function",
                },
                Lex {
                    pos: 8,
                    end: 11,
                    kind: SyntaxKind::Identifier,
                    text: "fn",
                },
                Lex {
                    pos: 11,
                    end: 12,
                    kind: SyntaxKind::OpenParenToken,
                    text: "",
                },
                Lex {
                    pos: 12,
                    end: 16,
                    kind: SyntaxKind::Identifier,
                    text: "arg1",
                },
                Lex {
                    pos: 16,
                    end: 17,
                    kind: SyntaxKind::ColonToken,
                    text: "",
                },
                Lex {
                    pos: 17,
                    end: 24,
                    kind: SyntaxKind::StringKeyword,
                    text: "string",
                },
                Lex {
                    pos: 24,
                    end: 25,
                    kind: SyntaxKind::CloseParenToken,
                    text: "",
                },
                Lex {
                    pos: 25,
                    end: 27,
                    kind: SyntaxKind::OpenBraceToken,
                    text: "",
                },
                Lex {
                    pos: 27,
                    end: 28,
                    kind: SyntaxKind::CloseBraceToken,
                    text: "",
                },
            ],
        );
    }
}
