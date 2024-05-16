pub mod transpile_file;

#[cfg(test)]
pub mod utils {
    use crate::error::TranspileError;
    use crate::parser::ParseErrorContext;
    use crate::source_file::SourceFile;
    use crate::transpile::{transpile_file, TranspileContext, TranspileResult};
    use crate::TranspileOptions;
    use pretty_assertions::assert_eq;

    pub fn assert_transpile_file(
        content: &str,
        expected: Result<TranspileResult, ParseErrorContext>,
    ) {
        let source_file = SourceFile {
            name: "test".to_string(),
            path: "test".to_string(),
            src: content.to_string(),
            node: None,
        };

        let expected = match expected {
            Ok(expected) => Ok(TranspileResult {
                emit_result: expected.emit_result,
            }),
            Err(err) => Err(TranspileError::ParseError {
                source_file: source_file.clone(),
                context: err,
            }),
        };

        assert_eq!(
            transpile_file(TranspileContext {
                source_file: &source_file,
                options: TranspileOptions {
                    entry_file_path: "test".to_string(),
                    debug: false,
                },
            }),
            expected
        );
    }
}
