#[cfg(test)]
mod type_alias_declaration {
    use crate::lexer::tests::utils::{assert_lexes, LexCreateHelper as helper};
    use ast::kind::SyntaxKind;

    #[test]
    pub fn number_type_alias_declaration() {
        assert_lexes(
            "type num = number;",
            vec![
                helper::create_lex((0, 4), SyntaxKind::TypeKeyword, "type"),
                helper::create_lex((4, 8), SyntaxKind::Identifier, "num"),
                helper::create_lex((8, 10), SyntaxKind::EqualsToken, ""),
                helper::create_lex((10, 17), SyntaxKind::NumberKeyword, "number"),
                helper::create_lex((17, 18), SyntaxKind::SemicolonToken, ""),
            ],
        );
    }

    #[test]
    pub fn string_type_alias_declaration() {
        assert_lexes(
            "type str = string;",
            vec![
                helper::create_lex((0, 4), SyntaxKind::TypeKeyword, "type"),
                helper::create_lex((4, 8), SyntaxKind::Identifier, "str"),
                helper::create_lex((8, 10), SyntaxKind::EqualsToken, ""),
                helper::create_lex((10, 17), SyntaxKind::StringKeyword, "string"),
                helper::create_lex((17, 18), SyntaxKind::SemicolonToken, ""),
            ],
        );
    }
}
