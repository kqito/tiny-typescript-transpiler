#[cfg(test)]
mod variable_statement {
    use crate::emitter::tests::utils::assert_emit_stmt;

    #[test]
    fn emit_variable_declaration() {
        assert_emit_stmt("var a = 100;", "var a = 100;");
        assert_emit_stmt("const hoge = 100;", "var hoge = 100;");
        assert_emit_stmt("let a = 100;", "var a = 100;");
        assert_emit_stmt("let a;", "var a;");
        assert_emit_stmt("let hoge: string;", "var hoge;");
        assert_emit_stmt("var hoge: str;", "var hoge;");
        assert_emit_stmt("var num: number = 100;", "var num = 100;");
        assert_emit_stmt(r#"const str: string = "test"#, r#"var str = "test";"#);
        assert_emit_stmt(r#"const hi: str = "hello";"#, r#"var hi = "hello";"#);
        assert_emit_stmt("type Str = String;", "");
    }
}
