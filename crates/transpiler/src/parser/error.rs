use core::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq, Clone)]
pub struct ParseError {
    pub pos: u32,
    pub end: u32,
    pub message: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ParseErrorContext {
    pub errors: Vec<ParseError>,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} :{}\n", self.message, self.pos)
    }
}

impl ParseErrorContext {
    pub fn new() -> ParseErrorContext {
        ParseErrorContext { errors: Vec::new() }
    }
    pub fn push(&mut self, error: ParseError) {
        self.errors.push(error);
    }
}

impl Display for ParseErrorContext {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let write_result: Result = self
            .errors
            .iter()
            .map(|error| write!(f, "{}", error))
            .collect();

        write_result
    }
}

impl ParseErrorContext {
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
}
