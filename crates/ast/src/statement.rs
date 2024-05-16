use crate::declaration::*;
use crate::expression::Expression;
use crate::impl_node_from;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct ExpressionStatement {
    pub expression: Node<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VariableStatement {
    pub declaration_list: Node<VariableDeclarationList>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    ExpressionStatement(ExpressionStatement),
    VariableStatement(VariableStatement),
    TypeAliasDeclaration(TypeAliasDeclaration),
    FunctionDeclaration(FunctionDeclaration),
}

impl_node_from! {
  Statement,
  ExpressionStatement => Statement::ExpressionStatement,
  VariableStatement => Statement::VariableStatement,
  TypeAliasDeclaration => Statement::TypeAliasDeclaration,
  FunctionDeclaration => Statement::FunctionDeclaration
}

pub type Modules = Vec<Node<Statement>>;
