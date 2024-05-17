#[cfg(test)]
mod variable_statement {
    use crate::parser::tests::utils::{assert_parse, AstCreateHelper as helper};
    use ast::declaration::*;
    use ast::expression::*;

    #[test]
    fn parse_var_variable_declaration() {
        let expected = vec![helper.var_stmt(
            (0, 12),
            helper.var_decl_list(
                (0, 11),
                vec![helper.var_decl(
                    (3, 11),
                    helper.ident((3, 5), "a"),
                    Some(helper.literal((7, 11), Literal::Numeric(100.to_string()))),
                    None,
                )],
            ),
        )];
        assert_parse("var a = 100;", expected);
    }

    #[test]
    fn parse_const_variable_declaration() {
        let expected = vec![helper.var_stmt(
            (0, 17),
            helper.var_decl_list(
                (0, 16),
                vec![helper.var_decl(
                    (5, 16),
                    helper.ident((5, 10), "hoge"),
                    Some(helper.literal((12, 16), Literal::Numeric(100.to_string()))),
                    None,
                )],
            ),
        )];
        assert_parse("const hoge = 100;", expected);
    }

    #[test]
    fn parse_let_variable_declaration() {
        let expected = vec![helper.var_stmt(
            (0, 12),
            helper.var_decl_list(
                (0, 11),
                vec![helper.var_decl(
                    (3, 11),
                    helper.ident((3, 5), "a"),
                    Some(helper.literal((7, 11), Literal::Numeric(100.to_string()))),
                    None,
                )],
            ),
        )];
        assert_parse("let a = 100;", expected);

        let expected = vec![helper.var_stmt(
            (0, 6),
            helper.var_decl_list(
                (0, 5),
                vec![helper.var_decl((3, 5), helper.ident((3, 5), "b"), None, None)],
            ),
        )];
        assert_parse("let b;", expected);
    }

    #[test]
    fn parse_variable_declaration_with_type() {
        assert_parse(
            "var num: number = 100;",
            vec![helper.var_stmt(
                (0, 22),
                helper.var_decl_list(
                    (0, 21),
                    vec![helper.var_decl(
                        (3, 21),
                        helper.ident((3, 7), "num"),
                        Some(helper.literal((17, 21), Literal::Numeric(100.to_string()))),
                        Some(helper.type_keyword((8, 15), Type::NumberKeyword)),
                    )],
                ),
            )],
        );
        assert_parse(
            r#"const str: string = "test";"#,
            vec![helper.var_stmt(
                (0, 27),
                helper.var_decl_list(
                    (0, 26),
                    vec![helper.var_decl(
                        (5, 26),
                        helper.ident((5, 9), "str"),
                        Some(helper.literal((19, 26), Literal::String("test".to_string()))),
                        Some(helper.type_keyword((10, 17), Type::StringKeyword)),
                    )],
                ),
            )],
        );
        assert_parse(
            r#"const hi: str = "hello";"#,
            vec![helper.var_stmt(
                (0, 24),
                helper.var_decl_list(
                    (0, 23),
                    vec![helper.var_decl(
                        (5, 23),
                        helper.ident((5, 8), "hi"),
                        Some(helper.literal((15, 23), Literal::String("hello".to_string()))),
                        Some(helper.type_ref((9, 13), helper.ident((9, 13), "str"))),
                    )],
                ),
            )],
        );
        assert_parse(
            "let hoge: string;",
            vec![helper.var_stmt(
                (0, 17),
                helper.var_decl_list(
                    (0, 16),
                    vec![helper.var_decl(
                        (3, 16),
                        helper.ident((3, 8), "hoge"),
                        None,
                        Some(helper.type_keyword((9, 16), Type::StringKeyword)),
                    )],
                ),
            )],
        );
        assert_parse(
            "var hoge: str;",
            vec![helper.var_stmt(
                (0, 14),
                helper.var_decl_list(
                    (0, 13),
                    vec![helper.var_decl(
                        (3, 13),
                        helper.ident((3, 8), "hoge"),
                        None,
                        Some(helper.type_ref((9, 13), helper.ident((9, 13), "str"))),
                    )],
                ),
            )],
        );
    }

    #[test]
    fn parse_variable_declaration_with_literrals() {
        assert_parse(
            "var a = 100;",
            vec![helper.var_stmt(
                (0, 12),
                helper.var_decl_list(
                    (0, 11),
                    vec![helper.var_decl(
                        (3, 11),
                        helper.ident((3, 5), "a"),
                        Some(helper.literal((7, 11), Literal::Numeric(100.to_string()))),
                        None,
                    )],
                ),
            )],
        );
        assert_parse(
            r#"const b = "test";"#,
            vec![helper.var_stmt(
                (0, 17),
                helper.var_decl_list(
                    (0, 16),
                    vec![helper.var_decl(
                        (5, 16),
                        helper.ident((5, 7), "b"),
                        Some(helper.literal((9, 16), Literal::String("test".to_string()))),
                        None,
                    )],
                ),
            )],
        );
        assert_parse(
            r#"let c = true;"#,
            vec![helper.var_stmt(
                (0, 13),
                helper.var_decl_list(
                    (0, 12),
                    vec![helper.var_decl(
                        (3, 12),
                        helper.ident((3, 5), "c"),
                        Some(helper.literal((7, 12), Literal::True)),
                        None,
                    )],
                ),
            )],
        );
        assert_parse(
            r#"let d = false;"#,
            vec![helper.var_stmt(
                (0, 14),
                helper.var_decl_list(
                    (0, 13),
                    vec![helper.var_decl(
                        (3, 13),
                        helper.ident((3, 5), "d"),
                        Some(helper.literal((7, 13), Literal::False)),
                        None,
                    )],
                ),
            )],
        );
        assert_parse(
            r#"let e = null;"#,
            vec![helper.var_stmt(
                (0, 13),
                helper.var_decl_list(
                    (0, 12),
                    vec![helper.var_decl(
                        (3, 12),
                        helper.ident((3, 5), "e"),
                        Some(helper.literal((7, 12), Literal::Null)),
                        None,
                    )],
                ),
            )],
        );

        assert_parse(
            r#"let f = undefined;"#,
            vec![helper.var_stmt(
                (0, 18),
                helper.var_decl_list(
                    (0, 17),
                    vec![helper.var_decl(
                        (3, 17),
                        helper.ident((3, 5), "f"),
                        Some(helper.literal((7, 17), Literal::Undefined)),
                        None,
                    )],
                ),
            )],
        );

        assert_parse(
            r#"let g = /test/;"#,
            vec![helper.var_stmt(
                (0, 15),
                helper.var_decl_list(
                    (0, 14),
                    vec![helper.var_decl(
                        (3, 14),
                        helper.ident((3, 5), "g"),
                        Some(helper.literal((7, 14), Literal::RegEx("/test/".to_string()))),
                        None,
                    )],
                ),
            )],
        );

        assert_parse(
            r#"let h = 0b1010;"#,
            vec![helper.var_stmt(
                (0, 15),
                helper.var_decl_list(
                    (0, 14),
                    vec![helper.var_decl(
                        (3, 14),
                        helper.ident((3, 5), "h"),
                        Some(helper.literal((7, 14), Literal::Numeric("0b1010".to_string()))),
                        None,
                    )],
                ),
            )],
        );
    }
}
