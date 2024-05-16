use std::any::Any;
use std::fmt::Debug;

use crate::lexer::inputer::Inputer;
use crate::lexer::Controller;
use crate::Emitter;
use crate::Lexer;
use crate::Parser;
use crate::TranspileError;

use crate::source_file::SourceFile;
use crate::TranspileOptions;

mod tests;

#[derive(Debug)]
pub struct TranspileContext<'a> {
    pub source_file: &'a SourceFile,
    pub options: TranspileOptions,
}

#[derive(Debug, PartialEq)]
pub struct TranspileResult {
    pub emit_result: String,
}

pub fn transpile_file(context: TranspileContext) -> Result<TranspileResult, TranspileError> {
    let source_file = context.source_file;
    let options = context.options;

    let debugger = move |any: &dyn Debug| {
        if options.debug {
            println!("{:#?}\n", any);
        }
    };

    let mut inputer = Inputer::from(&source_file.src as &str);
    let mut lexer = Lexer::from(&mut inputer);
    debugger(&lexer.peek_all());

    let mut parse = Parser::new(lexer);
    let modules = match parse.parse_module() {
        Ok(modules) => modules,
        Err(err) => {
            return Err(TranspileError::ParseError {
                source_file: source_file.clone(),
                context: err,
            })
        }
    };
    debugger(&modules);

    let emitter = Emitter::new();
    let emit_result = emitter.emit_module(&modules)?;

    let transpile_result = TranspileResult { emit_result };

    return Ok(transpile_result);
}
