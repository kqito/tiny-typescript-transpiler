mod emitter;
mod error;
mod lexer;
mod options;
mod parser;
mod source_file;
mod transpile;

use emitter::Emitter;
pub use error::TranspileError;
use lexer::Lexer;
use parser::Parser;
use transpile::{transpile_file, TranspileResult};

use crate::source_file::SourceFile;

pub use crate::options::TranspileOptions;

pub fn transpile(options: TranspileOptions) -> Result<TranspileResult, TranspileError> {
    let source_file = match SourceFile::load(&options.entry_file_path) {
        Ok(source_file) => source_file,
        Err(_) => {
            return Err(TranspileError::NotFoundEntryFile(
                options.entry_file_path.clone(),
            ))
        }
    };

    return transpile_file(transpile::TranspileContext {
        source_file: &source_file,
        options,
    });
}
