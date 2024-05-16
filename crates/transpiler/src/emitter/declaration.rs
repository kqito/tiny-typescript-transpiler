use super::Emitter;
use crate::emitter::Emit;
use ast::declaration::*;

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
