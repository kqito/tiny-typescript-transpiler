pub mod expression_statement;
pub mod function_declaration;
pub mod type_alias_declaration;
pub mod variable_statement;

#[cfg(test)]
pub mod utils {
    use crate::lexer::{inputer::Inputer, Lexer};
    use crate::Parser;
    use ast::declaration::{
        Block, FunctionDeclaration, Parameter, Type, TypeAliasDeclaration, TypeReference,
        VariableDeclaration, VariableDeclarationList,
    };
    use ast::expression::{CallExpression, Expression, Identifier, Literal};
    use ast::statement::*;
    use ast::*;
    use pretty_assertions::assert_eq;

    pub fn assert_parse(content: &str, expected: Vec<Node<Statement>>) {
        let mut inputer = Inputer::from(content);
        let lexer = Lexer::from(&mut inputer);
        let mut parse = Parser::new(lexer);
        let tree = parse.parse_module().unwrap();

        assert_eq!(tree, expected);
    }

    pub struct AstCreateHelper;
    impl AstCreateHelper {
        pub fn ident(&self, loc: (u32, u32), name: &str) -> Node<Identifier> {
            Node::new(
                Loc::new(loc.0, loc.1),
                Identifier {
                    text: String::from(name),
                },
            )
        }

        pub fn type_keyword(&self, loc: (u32, u32), keyword: Type) -> Node<Type> {
            Node::new(Loc::new(loc.0, loc.1), keyword)
        }

        pub fn type_ref(&self, loc: (u32, u32), ident: Node<Identifier>) -> Node<Type> {
            Node::new(
                Loc::new(loc.0, loc.1),
                Type::TypeReference(TypeReference {
                    type_name: Some(ident),
                }),
            )
        }

        pub fn literal(&self, loc: (u32, u32), lit: Literal) -> Node<Expression> {
            Node::new(Loc::new(loc.0, loc.1), Expression::Literal(lit))
        }

        pub fn var_decl(
            &self,
            loc: (u32, u32),
            ident: Node<Identifier>,
            init: Option<Node<Expression>>,
            r#type: Option<Node<Type>>,
        ) -> Node<VariableDeclaration> {
            Node::new(
                Loc::new(loc.0, loc.1),
                VariableDeclaration {
                    name: ident,
                    init,
                    r#type,
                },
            )
        }

        pub fn var_decl_list(
            &self,
            loc: (u32, u32),
            decls: Vec<Node<VariableDeclaration>>,
        ) -> Node<VariableDeclarationList> {
            Node::new(
                Loc::new(loc.0, loc.1),
                VariableDeclarationList {
                    declarations: decls,
                },
            )
        }

        pub fn var_stmt(
            &self,
            loc: (u32, u32),
            decl_list: Node<VariableDeclarationList>,
        ) -> Node<Statement> {
            Node::new(
                Loc::new(loc.0, loc.1),
                Statement::VariableStatement(VariableStatement {
                    declaration_list: decl_list,
                }),
            )
        }

        pub fn type_stmt(
            &self,
            loc: (u32, u32),
            name: Node<Identifier>,
            r#type: Node<Type>,
        ) -> Node<Statement> {
            Node::new(
                Loc::new(loc.0, loc.1),
                Statement::TypeAliasDeclaration(TypeAliasDeclaration {
                    name,
                    r#type: Some(r#type),
                }),
            )
        }

        pub fn param(
            &self,
            loc: (u32, u32),
            ident: Node<Identifier>,
            r#type: Option<Node<Type>>,
        ) -> Node<Parameter> {
            Node::new(
                Loc::new(loc.0, loc.1),
                Parameter {
                    name: ident,
                    r#type,
                },
            )
        }

        pub fn block(&self, loc: (u32, u32), stmts: Vec<Node<Statement>>) -> Node<Block> {
            Node::new(Loc::new(loc.0, loc.1), Block { statements: stmts })
        }

        pub fn func_decl(
            &self,
            loc: (u32, u32),
            ident: Node<Identifier>,
            r#type: Option<Node<Type>>,
            params: Option<Vec<Node<Parameter>>>,
            body: Node<Block>,
        ) -> Node<Statement> {
            Node::new(
                Loc::new(loc.0, loc.1),
                Statement::FunctionDeclaration(FunctionDeclaration {
                    name: ident,
                    r#type,
                    parameters: params,
                    body: Some(body),
                }),
            )
        }

        pub fn ident_expr(&self, loc: (u32, u32), name: &str) -> Node<Expression> {
            Node::new(
                Loc::new(loc.0, loc.1),
                Expression::Identifier(Identifier {
                    text: String::from(name),
                }),
            )
        }

        pub fn call_expr(
            &self,
            loc: (u32, u32),
            expr: Node<Expression>,
            args: Vec<Node<Expression>>,
        ) -> Node<Expression> {
            Node::new(
                Loc::new(loc.0, loc.1),
                Expression::CallExpression(CallExpression {
                    expression: Box::new(expr),
                    arguments: args,
                }),
            )
        }

        pub fn expr_stmt(&self, loc: (u32, u32), expr: Node<Expression>) -> Node<Statement> {
            Node::new(
                Loc::new(loc.0, loc.1),
                Statement::ExpressionStatement(ExpressionStatement { expression: expr }),
            )
        }
    }
}
