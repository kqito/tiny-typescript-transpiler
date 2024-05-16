use crate::lexer::Lex;
use ast::kind::SyntaxKind;

pub(crate) fn match_kind(lex: &Lex, kinds: impl IntoIterator<Item = SyntaxKind>) -> bool {
    kinds.into_iter().any(|kind| lex.kind == kind)
}
