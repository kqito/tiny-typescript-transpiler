pub mod char;
pub mod declaration;
pub mod expression;
pub mod kind;
pub mod statement;
pub mod token;

#[macro_use]
mod macros;

#[derive(Debug, Clone, PartialEq)]
pub struct Loc {
    pub pos: u32,
    pub end: u32,
}

impl Loc {
    pub fn new(pos: u32, end: u32) -> Self {
        Self { pos, end }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Node<T> {
    pub loc: Loc,
    pub item: T,
}

impl<T> Node<T> {
    pub fn new(loc: Loc, item: T) -> Self {
        Self { loc, item }
    }
}
