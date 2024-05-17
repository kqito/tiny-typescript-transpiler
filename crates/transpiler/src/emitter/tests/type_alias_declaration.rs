#[cfg(test)]
mod variable_statement {
    use crate::emitter::tests::utils::assert_emit_stmt;

    #[test]
    fn emit_type_alias_declaration() {
        assert_emit_stmt("type MyString = string;", "");
        assert_emit_stmt("type MyNumber = number;", "");
    }
}
