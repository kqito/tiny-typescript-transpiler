use crate::{impl_node_from, Node};

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier {
    pub text: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Numeric(String),
    String(String),
    RegEx(String),
    True,
    False,
    Undefined,
    Null,
}

pub type Arguments = Vec<Node<Expression>>;

#[derive(Debug, Clone, PartialEq)]
pub struct CallExpression {
    pub expression: Box<Node<Expression>>,
    pub arguments: Arguments,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Identifier(Identifier),
    Literal(Literal),
    CallExpression(CallExpression),
}

impl_node_from! {
  Expression,
  Identifier => Expression::Identifier,
  Literal => Expression::Literal,
  CallExpression => Expression::CallExpression
}
