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
    }

    #[test]
    fn emit_variable_declaration_with_type() {
        assert_emit_stmt("var a: number = 100;", "var a = 100;");
        assert_emit_stmt("let a: number = 100;", "var a = 100;");
        assert_emit_stmt("let a: number;", "var a;");
        assert_emit_stmt("let hoge: string;", "var hoge;");
        assert_emit_stmt("var hoge: str;", "var hoge;");
        assert_emit_stmt("var num: number = 100;", "var num = 100;");
        assert_emit_stmt(r#"const str: string = "test"#, r#"var str = "test";"#);
        assert_emit_stmt(r#"const hi: str = "hello";"#, r#"var hi = "hello";"#);
    }

    #[test]
    fn emit_variable_declaration_with_type_and_value() {
        assert_emit_stmt("var a: number = 100;", "var a = 100;");
        assert_emit_stmt("let a: number = 100;", "var a = 100;");
        assert_emit_stmt("let a: number;", "var a;");
        assert_emit_stmt("let hoge: string;", "var hoge;");
        assert_emit_stmt("var hoge: str;", "var hoge;");
        assert_emit_stmt("var num: number = 100;", "var num = 100;");
        assert_emit_stmt(r#"const str: string = "test"#, r#"var str = "test";"#);
        assert_emit_stmt(r#"const hi: str = "hello";"#, r#"var hi = "hello";"#);
    }

    #[test]
    fn emit_variable_declaration_with_literals() {
        assert_emit_stmt("  var a: number = 100;", "var a = 100;");
        assert_emit_stmt(r#"var a: string = "hello";"#, r#"var a = "hello";"#);
        assert_emit_stmt("  var a: RegExp = /test/;", "var a = /test/;");
        assert_emit_stmt("  var a: boolean = true;", "var a = true;");
        assert_emit_stmt("  var a: boolean = false;", "var a = false;");
        assert_emit_stmt("  var a = undefined;", "var a = undefined;");
        assert_emit_stmt("  var a = null;", "var a = null;");
        assert_emit_stmt("  var a = 0b101;", "var a = 0b101;");
    }
}
