use super::Emitter;
use crate::emitter::Emit;
use ast::statement::*;

impl Emit for Modules {
    fn emit(&self, emitter: &mut Emitter) -> () {
        self.iter().for_each(|node| match &node.item {
            Statement::VariableStatement(stmt) => {
                stmt.emit(emitter);
            }
            Statement::TypeAliasDeclaration(stmt) => {
                stmt.emit(emitter);
            }
            Statement::FunctionDeclaration(stmt) => {
                todo!();
            }
            Statement::ExpressionStatement(stmt) => {
                todo!();
            }
            _ => {}
        });
    }
}

impl Emit for VariableStatement {
    fn emit(&self, emitter: &mut Emitter) -> () {
        emitter.push("var ");
        self.declaration_list.item.emit(emitter)
    }
}
