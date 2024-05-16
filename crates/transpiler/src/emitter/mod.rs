mod declaration;
mod expression;
mod statement;
mod tests;

use ast::statement::Modules;

use crate::error::TranspileError;

trait Emit {
    fn emit(&self, emitter: &mut Emitter) -> ();
}

pub struct Emitter {
    result: String,
}

impl Emitter {
    pub fn new() -> Emitter {
        Self {
            result: String::from(""),
        }
    }

    pub fn emit_module(&self, module: &Modules) -> Result<String, TranspileError> {
        let mut emitter = Self::new();
        module.emit(&mut emitter);

        Ok(emitter.result.clone())
    }

    pub fn push(&mut self, part: &str) -> () {
        self.result.push_str(part);
    }
}
