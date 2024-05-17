#[cfg(test)]
mod expression_statement {
    use crate::parser::tests::utils::{assert_parse, AstCreateHelper as helper};
    use ast::expression::*;

    #[test]
    fn parse_expression_idenfier() {
        assert_parse(
            "fn;",
            vec![helper.expr_stmt((0, 3), helper.ident_expr((0, 2), "fn"))],
        );
    }

    #[test]
    fn parse_call_expression() {
        assert_parse(
            "fn();",
            vec![helper.expr_stmt(
                (0, 5),
                helper.call_expr((0, 4), helper.ident_expr((0, 2), "fn"), vec![]),
            )],
        );
    }

    #[test]
    fn parse_call_expression_with_argument() {
        assert_parse(
            "fn(0);",
            vec![helper.expr_stmt(
                (0, 6),
                helper.call_expr(
                    (0, 5),
                    helper.ident_expr((0, 2), "fn"),
                    vec![helper.literal((3, 4), Literal::Numeric(0.to_string()))],
                ),
            )],
        );
        assert_parse(
            "fn(arg1, arg2);",
            vec![helper.expr_stmt(
                (0, 15),
                helper.call_expr(
                    (0, 14),
                    helper.ident_expr((0, 2), "fn"),
                    vec![
                        helper.ident_expr((3, 7), "arg1"),
                        helper.ident_expr((8, 13), "arg2"),
                    ],
                ),
            )],
        );
    }

    #[test]
    fn parse_nest_call_expression() {
        assert_parse(
            "fn()();",
            vec![helper.expr_stmt(
                (0, 7),
                helper.call_expr(
                    (0, 6),
                    helper.call_expr((0, 4), helper.ident_expr((0, 2), "fn"), vec![]),
                    vec![],
                ),
            )],
        );
        assert_parse(
            "fn(arg1)(arg2);",
            vec![helper.expr_stmt(
                (0, 15),
                helper.call_expr(
                    (0, 14),
                    helper.call_expr(
                        (0, 8),
                        helper.ident_expr((0, 2), "fn"),
                        vec![helper.ident_expr((3, 7), "arg1")],
                    ),
                    vec![helper.ident_expr((9, 13), "arg2")],
                ),
            )],
        );
        assert_parse(
            "fn()()();",
            vec![helper.expr_stmt(
                (0, 9),
                helper.call_expr(
                    (0, 8),
                    helper.call_expr(
                        (0, 6),
                        helper.call_expr((0, 4), helper.ident_expr((0, 2), "fn"), vec![]),
                        vec![],
                    ),
                    vec![],
                ),
            )],
        );
    }
}
