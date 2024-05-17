pub mod inputer;
mod tests;

use ast::char::CharacterCodes;
use ast::kind::SyntaxKind;
use inputer::Input;
use std::str;

#[allow(unused)]
pub trait Controller<'a, T> {
    fn pop(&mut self) -> T;
    fn peek(&mut self) -> T;
    fn pop_match<F>(&mut self, pred: F) -> Option<T>
    where
        F: FnMut(&T) -> bool;
    fn pop_while<F>(&mut self, pred: F) -> Vec<T>
    where
        F: FnMut(&T) -> bool;
    fn find<F>(&mut self, pred: F) -> Option<T>
    where
        F: FnMut(&T) -> bool;
    fn peek_all(&mut self) -> Vec<T>;
}

type Lexes<'a> = Vec<Lex<'a>>;

#[derive(Debug, Clone, PartialEq)]
pub struct Lex<'a> {
    pub pos: u32,
    pub end: u32,
    pub text: &'a str,
    pub kind: SyntaxKind,
}

#[derive(Debug)]
pub struct Lexer<'a, I: Input<'a>> {
    pub input: &'a mut I,
}

impl<'a, I: Input<'a>> From<&'a mut I> for Lexer<'a, I> {
    fn from(input: &'a mut I) -> Lexer<'a, I> {
        Lexer { input }
    }
}

impl<'a, I: Input<'a>> Controller<'a, Lex<'a>> for Lexer<'a, I> {
    fn pop(&mut self) -> Lex<'a> {
        let ret = self.pick();

        self.input.bump();

        ret
    }

    fn peek(&mut self) -> Lex<'a> {
        let reset_pos = self.input.current_pos();
        let reset_end = self.input.current_end();
        let ret = self.pop();

        self.input.reset_to(reset_pos, reset_end);
        ret
    }

    fn pop_match<F>(&mut self, mut pred: F) -> Option<Lex<'a>>
    where
        F: FnMut(&Lex<'a>) -> bool,
    {
        let reset_pos = self.input.current_pos();
        let reset_end = self.input.current_end();
        let ret = self.pop();

        if pred(&ret) {
            return Some(ret);
        }

        self.input.reset_to(reset_pos, reset_end);
        None
    }

    fn pop_while<F>(&mut self, mut pred: F) -> Lexes<'a>
    where
        F: FnMut(&Lex<'a>) -> bool,
    {
        let mut ret = Vec::new();
        while let Some(lex) = self.pop_match(&mut pred) {
            ret.push(lex.clone());

            if lex.kind == SyntaxKind::EOF {
                break;
            }
        }

        ret
    }

    fn find<F>(&mut self, mut pred: F) -> Option<Lex<'a>>
    where
        F: FnMut(&Lex<'a>) -> bool,
    {
        let reset_pos = self.input.current_pos();
        let reset_end = self.input.current_end();
        let mut target: Option<Lex<'a>> = None;
        loop {
            let lex = self.pop();
            if lex.kind == SyntaxKind::EOF {
                break;
            }

            if !pred(&lex) {
                continue;
            }

            target = Some(lex);
            break;
        }

        self.input.reset_to(reset_pos, reset_end);
        target
    }

    fn peek_all(&mut self) -> Lexes<'a> {
        let reset_pos = self.input.current_pos();
        let reset_end = self.input.current_end();
        let mut lexes = Vec::new();
        loop {
            let lex = self.pop();
            if lex.kind == SyntaxKind::EOF {
                break;
            }

            lexes.push(lex);
        }

        self.input.reset_to(reset_pos, reset_end);
        lexes
    }
}

impl<'a, I: Input<'a>> Lexer<'a, I> {
    fn new_lex(&self, text: &'a str, kind: SyntaxKind) -> Lex<'a> {
        Lex {
            pos: self.input.current_pos(),
            end: self.input.current_end(),
            text,
            kind,
        }
    }

    fn pick(&mut self) -> Lex<'a> {
        self.input.peek_while(|c| c.is_ascii_whitespace());
        let current_char = match self.input.peek_ref() {
            Some(c) => c,
            // If the content has been reached at end
            None => {
                return self.new_lex("", SyntaxKind::EOF);
            }
        };

        match CharacterCodes::from(current_char as u8) {
            CharacterCodes::Numeric => {
                // Support 'b' character for binary number
                let text = self
                    .input
                    .peek_while(|c| c.is_ascii_digit() || matches!(c, 'b'));
                self.new_lex(text, SyntaxKind::NumericLiteral)
            }
            CharacterCodes::Alphabetic => {
                let text = self.input.peek_while(|c| c.is_ascii_alphanumeric());
                if let Ok(kind) = SyntaxKind::try_from(text) {
                    return self.new_lex(text, kind);
                }

                self.new_lex(text, SyntaxKind::Identifier)
            }
            CharacterCodes::SingleQuote | CharacterCodes::DoubleQuote => {
                let quate_char = self.input.peek();
                let text = self.input.peek_while(|c| quate_char != Some(c));
                self.input.peek(); // Skip close quate

                self.new_lex(text, SyntaxKind::StringLiteral)
            }
            // Reg
            CharacterCodes::Slash => {
                let pos = self.input.current_end();
                self.input.peek(); // Skip '/'

                // If the content has no '/' character
                if let None = self.input.find_ref(|c| c == '/') {
                    return self.new_lex("", SyntaxKind::SlashToken);
                }

                let text = self.input.peek_while(|c| c != '/');
                self.input.peek(); // Skip '/'
                let end = self.input.current_end();

                match text.len() {
                    // Infer the text is a comment
                    1 => {
                        let text = self.input.peek_while(|c| c != '\n');
                        return self.new_lex(text, SyntaxKind::Unknown); // TODO
                    }
                    // Infer the text is a reg ex
                    _ => {
                        let text = self.input.slice_ref(pos, end);
                        return self.new_lex(text, SyntaxKind::RegularExpressionLiteral);
                    }
                }
            }
            CharacterCodes::Unknown => {
                self.input.peek();
                self.new_lex("", SyntaxKind::Unknown)
            }
            // Maybe char is a pure syntax kind.
            c => {
                self.input.peek();
                self.new_lex("", SyntaxKind::try_from(c).unwrap_or(SyntaxKind::Unknown))
            }
        }
    }
}

#[cfg(test)]
mod lexer_test {
    use crate::lexer::inputer::Inputer;
    use pretty_assertions::assert_eq;

    use super::*;
    use ast::kind::SyntaxKind;

    #[test]
    pub fn pop() {
        let mut inputer = Inputer::from("abc/def");
        let mut lexer = Lexer::from(&mut inputer);

        assert_eq!(
            lexer.pop(),
            Lex {
                pos: 0,
                end: 3,
                text: "abc",
                kind: SyntaxKind::Identifier
            }
        );

        assert_eq!(
            lexer.pop(),
            Lex {
                pos: 3,
                end: 4,
                text: "",
                kind: SyntaxKind::SlashToken
            }
        );

        assert_eq!(
            lexer.pop(),
            Lex {
                pos: 4,
                end: 7,
                text: "def",
                kind: SyntaxKind::Identifier
            }
        );

        assert_eq!(
            lexer.pop(),
            Lex {
                pos: 7,
                end: 7,
                text: "",
                kind: SyntaxKind::EOF
            }
        );
    }

    #[test]
    pub fn pop_front_match() {
        let mut inputer = Inputer::from("abc/def");
        let mut lexer = Lexer::from(&mut inputer);

        assert_eq!(lexer.pop_match(|lex| lex.kind == SyntaxKind::EOF), None);
        assert_eq!(
            lexer.pop_match(|lex| lex.kind == SyntaxKind::Identifier),
            Some(Lex {
                pos: 0,
                end: 3,
                text: "abc",
                kind: SyntaxKind::Identifier
            })
        );
    }

    #[test]
    pub fn pop_while() {
        let mut inputer = Inputer::from("abc/def");
        let mut lexer = Lexer::from(&mut inputer);

        assert_eq!(
            lexer.pop_while(|lex| lex.kind != SyntaxKind::SlashToken),
            vec![Lex {
                pos: 0,
                end: 3,
                text: "abc",
                kind: SyntaxKind::Identifier
            }]
        );

        assert_eq!(
            lexer.pop_while(|lex| lex.kind == SyntaxKind::SlashToken),
            vec![Lex {
                pos: 3,
                end: 4,
                text: "",
                kind: SyntaxKind::SlashToken
            }]
        );
    }

    #[test]
    pub fn find() {
        let mut inputer = Inputer::from("abc/def");
        let mut lexer = Lexer::from(&mut inputer);

        assert_eq!(
            lexer.find(|lex| lex.kind == SyntaxKind::SlashToken),
            Some(Lex {
                pos: 3,
                end: 4,
                text: "",
                kind: SyntaxKind::SlashToken
            })
        );
        assert_eq!(lexer.input.current_pos(), 0);
        assert_eq!(lexer.find(|lex| lex.pos == 999), None);
        assert_eq!(lexer.input.current_pos(), 0);
    }
}
