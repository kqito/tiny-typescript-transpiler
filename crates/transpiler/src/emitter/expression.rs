use super::Emitter;
use crate::emitter::Emit;
use ast::expression::*;

impl Emit for Expression {
    fn emit(&self, emitter: &mut Emitter) -> () {
        match self {
            Expression::Identifier(ident) => {
                ident.emit(emitter);
            }
            Expression::Literal(literal) => literal.emit(emitter),
            Expression::CallExpression(call_expr) => {
                todo!();
            }
        };
    }
}

impl Emit for Identifier {
    fn emit(&self, emitter: &mut Emitter) -> () {
        emitter.push(&self.text);
    }
}

impl Emit for Literal {
    fn emit(&self, emitter: &mut Emitter) -> () {
        match self {
            Literal::Numeric(num) => emitter.push(&num.to_string()),
            Literal::String(str) => emitter.push(&format!(r#""{}""#, str)),
        }
    }
}
