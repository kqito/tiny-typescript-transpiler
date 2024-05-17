#[cfg(test)]
mod function_declaration {
    use crate::emitter::tests::utils::assert_emit_stmt;

    #[test]
    fn test_function_declaration() {
        assert_emit_stmt("function main() {}", "function main() {}");
        assert_emit_stmt(
            "function main() { let a: number = 1; }",
            "function main() { var a = 1; }",
        );
        assert_emit_stmt(
            r#"function main() { let a: number = 1; let b: string = "hello"; }"#,
            r#"function main() { var a = 1; var b = "hello"; }"#,
        );
        assert_emit_stmt("function main(index: number) {}", "function main(index) {}");
    }
}
