use super::Emitter;
use crate::emitter::Emit;
use ast::{declaration::*, statement::ExpressionStatement};

impl Emit for VariableDeclarationList {
    fn emit(&self, emitter: &mut Emitter) -> () {
        self.declarations
            .iter()
            .for_each(|dec| dec.item.emit(emitter));
    }
}

impl Emit for VariableDeclaration {
    fn emit(&self, emitter: &mut Emitter) -> () {
        self.name.item.emit(emitter);

        if let Some(expr) = &self.init {
            emitter.push(" = ");
            expr.item.emit(emitter);
        }

        emitter.push(";");
    }
}

impl Emit for TypeAliasDeclaration {
    fn emit(&self, _emitter: &mut Emitter) -> () {
        // nothing to do;
    }
}

impl Emit for FunctionDeclaration {
    fn emit(&self, emitter: &mut Emitter) -> () {
        emitter.push("function ");
        self.name.item.emit(emitter);

        emitter.push("(");
        if let Some(params) = &self.parameters {
            for (i, param) in params.iter().enumerate() {
                if i > 0 {
                    emitter.push(", ");
                }
                param.item.name.item.emit(emitter);
            }
        }
        emitter.push(") ");

        if let Some(body) = &self.body {
            body.item.emit(emitter);
        }
    }
}

impl Emit for ExpressionStatement {
    fn emit(&self, emitter: &mut Emitter) -> () {
        self.expression.item.emit(emitter);
        emitter.push(";");
    }
}
