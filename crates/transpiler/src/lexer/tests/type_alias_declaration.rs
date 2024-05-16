#[cfg(test)]
mod type_alias_declaration {
    use crate::lexer::{tests::utils::assert_lexes, Lex};
    use ast::kind::SyntaxKind;

    #[test]
    pub fn number_type_alias_declaration() {
        assert_lexes(
            "type num = number;",
            vec![
                Lex {
                    pos: 0,
                    end: 4,
                    kind: SyntaxKind::TypeKeyword,
                    text: "type",
                },
                Lex {
                    pos: 4,
                    end: 8,
                    kind: SyntaxKind::Identifier,
                    text: "num",
                },
                Lex {
                    pos: 8,
                    end: 10,
                    kind: SyntaxKind::EqualsToken,
                    text: "",
                },
                Lex {
                    pos: 10,
                    end: 17,
                    kind: SyntaxKind::NumberKeyword,
                    text: "number",
                },
                Lex {
                    pos: 17,
                    end: 18,
                    kind: SyntaxKind::SemicolonToken,
                    text: "",
                },
            ],
        );
    }

    #[test]
    pub fn string_type_alias_declaration() {
        assert_lexes(
            "type str = string;",
            vec![
                Lex {
                    pos: 0,
                    end: 4,
                    kind: SyntaxKind::TypeKeyword,
                    text: "type",
                },
                Lex {
                    pos: 4,
                    end: 8,
                    kind: SyntaxKind::Identifier,
                    text: "str",
                },
                Lex {
                    pos: 8,
                    end: 10,
                    kind: SyntaxKind::EqualsToken,
                    text: "",
                },
                Lex {
                    pos: 10,
                    end: 17,
                    kind: SyntaxKind::StringKeyword,
                    text: "string",
                },
                Lex {
                    pos: 17,
                    end: 18,
                    kind: SyntaxKind::SemicolonToken,
                    text: "",
                },
            ],
        );
    }
}
