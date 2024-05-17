use super::Emitter;
use crate::emitter::Emit;
use ast::statement::*;

impl Emit for Modules {
    fn emit(&self, emitter: &mut Emitter) -> () {
        self.iter().for_each(|node| node.item.emit(emitter));
    }
}

#[warn(unreachable_patterns)]
impl Emit for Statement {
    fn emit(&self, emitter: &mut Emitter) -> () {
        match self {
            Statement::VariableStatement(stmt) => {
                stmt.emit(emitter);
            }
            Statement::TypeAliasDeclaration(stmt) => {
                stmt.emit(emitter);
            }
            Statement::FunctionDeclaration(stmt) => {
                stmt.emit(emitter);
            }
            Statement::ExpressionStatement(stmt) => {
                stmt.emit(emitter);
            }
        }
    }
}

impl Emit for VariableStatement {
    fn emit(&self, emitter: &mut Emitter) -> () {
        emitter.push("var ");
        self.declaration_list.item.emit(emitter)
    }
}
