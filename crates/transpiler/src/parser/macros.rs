#[macro_export]
macro_rules! pop_match_kind {
    ($self:expr, $kind: expr) => {{
        $self.lexer.pop_match(|lex| lex.kind == $kind)
    }};
    ($self:expr,  $($kind: expr), +) => {{
        let mut vec = Vec::new();
        $(
            vec.push($kind);
        )+

        $self.lexer.pop_match(|lex| vec.iter().any(|kind| &lex.kind == kind))
    }};
}

#[macro_export]
macro_rules! pop_while_kind {
    ($self:expr, $kind: expr) => {{
        $self.lexer.pop_while(|lex| lex.kind != $kind)
    }};
    ($self:expr,  $($kind: expr), +) => {{
        let mut vec = Vec::new();
        $(
            vec.push($kind);
        )+

        $self.lexer.pop_while(|lex| vec.iter().any(|kind| &lex.kind != kind))
    }};
}
