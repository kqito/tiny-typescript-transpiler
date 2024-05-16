use thiserror::Error;

use crate::{parser::ParseErrorContext, source_file::SourceFile};

#[derive(Error, Debug, PartialEq)]
pub enum TranspileError {
    #[error("Failed parsing: {context:?}")]
    ParseError {
        source_file: SourceFile,
        context: ParseErrorContext,
    },

    #[error("Failed transpiling: {message}")]
    EmittingError { message: String },

    #[error("Unimplemented")]
    Unimplemented,

    #[error("Not found entry file: {0}")]
    NotFoundEntryFile(String),
}
