use ast::declaration::Block;

use super::{Emit, Emitter};

impl Emit for Block {
    fn emit(&self, emitter: &mut Emitter) -> () {
        emitter.push("{");
        self.statements.iter().enumerate().for_each(|(i, stmt)| {
            if i == 0 {
                // For braces
                emitter.push(" ");
            }

            stmt.item.emit(emitter);

            if i <= self.statements.len() - 1 {
                // For between statements
                emitter.push(" ");
            }
        });
        emitter.push("}");
    }
}
