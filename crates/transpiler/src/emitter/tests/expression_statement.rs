#[cfg(test)]
mod expression_statement {
    use crate::emitter::tests::utils::assert_emit_stmt;

    #[test]
    fn emit_call_expression() {
        assert_emit_stmt("alert();", "alert();");
        assert_emit_stmt("alert(100);", "alert(100);");
        assert_emit_stmt("alert(100, 200);", "alert(100, 200);");
        assert_emit_stmt("alert(100, 200, 300);", "alert(100, 200, 300);");
        assert_emit_stmt("alert(100, 200, 300, 400);", "alert(100, 200, 300, 400);");
        assert_emit_stmt(
            "alert(100, 200, 300, 400, 500);",
            "alert(100, 200, 300, 400, 500);",
        );
    }
}
