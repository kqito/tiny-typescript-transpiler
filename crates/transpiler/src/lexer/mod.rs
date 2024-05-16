pub mod inputer;
mod reg;
mod tests;

use ast::char::CharacterCodes;
use ast::kind::SyntaxKind;
use inputer::Input;
use reg::is_match_char;
use std::str;

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
        let ret = self.pick_next();

        self.input.bump();

        ret
    }

    fn peek(&mut self) -> Lex<'a> {
        let reset_pos = self.input.current_pos();
        let ret = self.pop();

        self.input.reset_to(reset_pos);
        ret
    }

    fn pop_match<F>(&mut self, mut pred: F) -> Option<Lex<'a>>
    where
        F: FnMut(&Lex<'a>) -> bool,
    {
        let reset_pos = self.input.current_pos();
        let ret = self.pop();

        if pred(&ret) {
            return Some(ret);
        }

        self.input.reset_to(reset_pos);
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

        self.input.reset_to(reset_pos);
        target
    }

    fn peek_all(&mut self) -> Lexes<'a> {
        let reset_pos = self.input.current_pos();
        let mut lexes = Vec::new();
        loop {
            let lex = self.pop();
            if lex.kind == SyntaxKind::EOF {
                break;
            }

            lexes.push(lex);
        }

        self.input.reset_to(reset_pos);
        lexes
    }
}

impl<'a, I: Input<'a>> Lexer<'a, I> {
    fn pick_string_literal(&mut self) -> Lex<'a> {
        let quate_char = self.input.peek();
        let text = self.input.peek_while(|c| quate_char != Some(c));
        self.input.peek(); // Skip close quate

        Lex {
            pos: self.input.current_pos(),
            end: self.input.current_end(),
            text,
            kind: SyntaxKind::StringLiteral,
        }
    }

    fn pick_number(&mut self) -> Lex<'a> {
        let text = self.input.peek_while(|c| is_match_char(c, reg::NUMBER_REG));

        return Lex {
            pos: self.input.current_pos(),
            end: self.input.current_end(),
            text,
            kind: SyntaxKind::NumericLiteral,
        };
    }

    fn pick_string(&mut self) -> Lex<'a> {
        let text = self.input.peek_while(|c| is_match_char(c, reg::STRING_REG));

        return Lex {
            pos: self.input.current_pos(),
            end: self.input.current_end(),
            text,
            kind: SyntaxKind::try_from(text).unwrap_or(SyntaxKind::Identifier),
        };
    }

    fn pick_next(&mut self) -> Lex<'a> {
        self.input.peek_while(|c| is_match_char(c, reg::SPACE_REG));

        let current_char = match self.input.peek_ref() {
            Some(c) => c,
            // If the content has been reached at end
            None => {
                return Lex {
                    pos: self.input.current_pos(),
                    end: self.input.current_end(),
                    text: "",
                    kind: SyntaxKind::EOF,
                };
            }
        };

        if is_match_char(current_char, reg::NUMBER_REG) {
            return self.pick_number();
        }

        if is_match_char(current_char, reg::STRING_REG) {
            return self.pick_string();
        }

        match CharacterCodes::from(current_char as u8) {
            CharacterCodes::SingleQuote | CharacterCodes::DoubleQuote => self.pick_string_literal(),
            char => {
                self.input.peek();
                let kind = match char {
                    CharacterCodes::Unkown => {
                        println!("Unsupported character: {} ", &current_char.to_string()); // TODO: Should display only in test
                        SyntaxKind::Unknown
                    }
                    _ => SyntaxKind::try_from(char).unwrap_or(SyntaxKind::Unknown),
                };

                // Token
                let lex = Lex {
                    pos: self.input.current_pos(),
                    end: self.input.current_end(),
                    text: "",
                    kind,
                };

                lex
            }
        }
    }
}

#[cfg(test)]
mod lexer_test {
    use crate::lexer::inputer::Inputer;

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
