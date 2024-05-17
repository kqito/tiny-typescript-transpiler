#[cfg(test)]
mod function_statement {
    use crate::parser::tests::utils::{assert_parse, AstCreateHelper as helper};
    use ast::declaration::*;
    use ast::expression::*;

    #[test]
    fn parse_function_declaration() {
        assert_parse(
            "function fn() {}",
            vec![helper.func_decl(
                (0, 16),
                helper.ident((8, 11), "fn"),
                None,
                None,
                helper.block((13, 16), vec![]),
            )],
        );
    }
    #[test]
    fn parse_function_declaration_with_arguments() {
        assert_parse(
            "function fn (a: string, b) {}",
            vec![helper.func_decl(
                (0, 29),
                helper.ident((8, 11), "fn"),
                None,
                Some(vec![
                    helper.param(
                        (13, 22),
                        helper.ident((13, 14), "a"),
                        Some(helper.type_keyword((15, 22), Type::StringKeyword)),
                    ),
                    helper.param((23, 25), helper.ident((23, 25), "b"), None),
                ]),
                helper.block((26, 29), vec![]),
            )],
        );
    }
    #[test]
    fn parse_function_declaration_with_body() {
        let var_stmt1 = helper.var_stmt(
            (16, 39),
            helper.var_decl_list(
                (16, 38),
                vec![helper.var_decl(
                    (20, 38),
                    helper.ident((20, 24), "num"),
                    Some(helper.literal((34, 38), Literal::Numeric(100.to_string()))),
                    Some(helper.type_keyword((25, 32), Type::NumberKeyword)),
                )],
            ),
        );

        let var_stmt2 = helper.var_stmt(
            (39, 59),
            helper.var_decl_list(
                (39, 58),
                vec![helper.var_decl(
                    (45, 58),
                    helper.ident((45, 48), "hi"),
                    Some(helper.literal((50, 58), Literal::String("hello".to_string()))),
                    None,
                )],
            ),
        );
        assert_parse(
            r#"function fn () { var num: number = 100; const hi = "hello"; }"#,
            vec![helper.func_decl(
                (0, 61),
                helper.ident((8, 11), "fn"),
                None,
                None,
                helper.block((14, 61), vec![var_stmt1, var_stmt2]),
            )],
        );
    }
}
