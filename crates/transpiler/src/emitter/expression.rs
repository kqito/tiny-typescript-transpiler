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
                call_expr.emit(emitter);
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
            Literal::True => emitter.push("true"),
            Literal::False => emitter.push("false"),
            Literal::Undefined => emitter.push("undefined"),
            Literal::Null => emitter.push("null"),
            Literal::RegEx(regex) => emitter.push(regex),
        }
    }
}

impl Emit for CallExpression {
    fn emit(&self, emitter: &mut Emitter) -> () {
        self.expression.item.emit(emitter);
        emitter.push("(");
        self.arguments.iter().enumerate().for_each(|(i, arg)| {
            if i != 0 {
                emitter.push(", ");
            }

            arg.item.emit(emitter);
        });
        emitter.push(")");
    }
}
