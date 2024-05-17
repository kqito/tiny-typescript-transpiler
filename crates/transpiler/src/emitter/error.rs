use core::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq, Clone)]
pub struct EmitError {
    pub message: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct EmitErrorContext {
    pub errors: Vec<EmitError>,
}

impl Display for EmitError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}\n", self.message)
    }
}

impl EmitErrorContext {
    pub fn new() -> EmitErrorContext {
        EmitErrorContext { errors: Vec::new() }
    }
    pub fn push(&mut self, error: EmitError) {
        self.errors.push(error);
    }
}

impl Display for EmitErrorContext {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let write_result: Result = self
            .errors
            .iter()
            .map(|error| write!(f, "{}", error))
            .collect();

        write_result
    }
}

impl EmitErrorContext {
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
}
