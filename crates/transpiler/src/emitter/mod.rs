mod declaration;
mod error;
mod expression;
mod function;
mod statement;
mod tests;

use ast::statement::Modules;

pub use self::error::EmitErrorContext;

trait Emit {
    fn emit(&self, emitter: &mut Emitter) -> ();
}

pub struct Emitter {
    pub(crate) result: String,
    pub(crate) error_context: EmitErrorContext,
}

impl Emitter {
    pub fn new() -> Emitter {
        Self {
            result: String::from(""),
            error_context: EmitErrorContext::new(),
        }
    }

    pub fn emit_module(&self, module: &Modules) -> Result<String, EmitErrorContext> {
        let mut emitter = Self::new();
        module.emit(&mut emitter);

        if emitter.error_context.has_errors() {
            return Err(emitter.error_context);
        }

        Ok(emitter.result.clone())
    }

    pub fn push(&mut self, part: &str) -> () {
        self.result.push_str(part);
    }
}
