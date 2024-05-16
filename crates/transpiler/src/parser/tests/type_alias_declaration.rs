#[cfg(test)]
mod type_alias_statement {
    use crate::parser::tests::utils::{assert_parse, AstCreateHelper as helper};
    use ast::declaration::*;

    #[test]
    fn parse_number_type_alias_declaration() {
        let expected = vec![helper.type_stmt(
            (0, 18),
            helper.ident((4, 8), "num"),
            helper.type_keyword((10, 17), Type::NumberKeyword),
        )];
        assert_parse("type num = number;", expected);
    }

    #[test]
    fn parse_string_type_alias_declaration() {
        let expected = vec![helper.type_stmt(
            (0, 18),
            helper.ident((4, 8), "str"),
            helper.type_keyword((10, 17), Type::StringKeyword),
        )];
        assert_parse("type str = string;", expected);
    }

    #[test]
    fn parse_type_alias_declaration_reference() {
        let expected = vec![helper.type_stmt(
            (0, 17),
            helper.ident((4, 9), "num2"),
            helper.type_ref((11, 16), helper.ident((11, 16), "num1")),
        )];
        assert_parse("type num2 = num1;", expected);
    }
}
