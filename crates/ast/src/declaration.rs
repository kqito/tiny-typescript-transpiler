use crate::expression::*;
use crate::impl_node_from;
use crate::statement::Statement;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct VariableDeclarationList {
    pub declarations: Vec<Node<VariableDeclaration>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VariableDeclaration {
    pub name: Node<Identifier>,
    pub r#type: Option<Node<Type>>,
    pub init: Option<Node<Expression>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeReference {
    pub type_name: Option<Node<Identifier>>,
    // pub type_arguments
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeAliasDeclaration {
    pub name: Node<Identifier>,
    pub r#type: Option<Node<Type>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub statements: Vec<Node<Statement>>,
}

pub type Parameters = Vec<Node<Parameter>>;

#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    pub name: Node<Identifier>,
    pub r#type: Option<Node<Type>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDeclaration {
    pub name: Node<Identifier>,
    pub r#type: Option<Node<Type>>,
    pub parameters: Option<Parameters>,
    pub body: Option<Node<Block>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    NumberKeyword,
    StringKeyword,
    TypeReference(TypeReference),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Declaration {
    VariableDeclaration(VariableDeclaration),
    TypeAliasDeclaration(TypeAliasDeclaration),
}

impl Declaration {
    pub fn get_name(&self) -> &str {
        match self {
            Declaration::VariableDeclaration(var) => &var.name.item.text,
            Declaration::TypeAliasDeclaration(ty) => &ty.name.item.text,
        }
    }
}

impl_node_from! {
  Declaration,
  VariableDeclaration => Declaration::VariableDeclaration,
  TypeAliasDeclaration => Declaration::TypeAliasDeclaration
}
