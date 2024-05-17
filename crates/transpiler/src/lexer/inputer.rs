use core::str::CharIndices;

/// References from: https://github.com/swc-project/swc/blob/main/crates/swc_common/src/input.rs
#[allow(unused)]
pub trait Input<'a>: Clone {
    fn peek(&mut self) -> Option<char>;
    fn peek_ref(&self) -> Option<char>;
    fn bump(&mut self);
    fn is_at_start(&self) -> bool;
    fn is_at_end(&self) -> bool;
    fn current_pos(&self) -> u32;
    fn current_end(&self) -> u32;

    fn slice(&mut self, start: u32, end: u32) -> &'a str;
    fn slice_ref(&mut self, start: u32, end: u32) -> &'a str;
    fn peek_while<F>(&mut self, f: F) -> &'a str
    where
        F: FnMut(char) -> bool;
    fn find<F>(&mut self, f: F) -> Option<u32>
    where
        F: FnMut(char) -> bool;
    fn find_ref<F>(&mut self, f: F) -> Option<u32>
    where
        F: FnMut(char) -> bool;

    fn reset_to(&mut self, pos: u32, end: u32);
}

#[derive(Debug, Clone)]
pub struct Inputer<'a> {
    pos: u32,
    end: u32,
    content: &'a str,
    content_iter: CharIndices<'a>,
}

impl<'a> From<&'a str> for Inputer<'a> {
    fn from(content: &'a str) -> Self {
        Inputer {
            pos: 0,
            end: 0,
            content,
            content_iter: content.char_indices(),
        }
    }
}

impl<'a> Inputer<'a> {
    fn calc_next_index(&self, index: usize, item: char) -> usize {
        index + item.len_utf8()
    }
}

impl<'a> Input<'a> for Inputer<'a> {
    fn peek(&mut self) -> Option<char> {
        let (_index, item) = match self.content_iter.next() {
            Some(v) => v,
            None => return None,
        };

        self.end += item.len_utf8() as u32;
        Some(item)
    }

    fn peek_ref(&self) -> Option<char> {
        let item = match self.content_iter.clone().next() {
            Some(v) => v.1,
            None => return None,
        };

        Some(item)
    }

    fn bump(&mut self) {
        self.pos = self.end;
    }

    fn is_at_start(&self) -> bool {
        self.pos == 0
    }

    fn is_at_end(&self) -> bool {
        self.end == self.content.len() as u32
    }

    fn current_pos(&self) -> u32 {
        self.pos
    }

    fn current_end(&self) -> u32 {
        self.end
    }

    fn slice(&mut self, pos: u32, end: u32) -> &'a str {
        self.pos = pos;
        self.end = end;

        let result: &str = &self.content[pos as usize..end as usize];

        let remain_content = &self.content[..end as usize];
        self.content_iter = remain_content.char_indices();

        result
    }

    fn slice_ref(&mut self, pos: u32, end: u32) -> &'a str {
        let mut s = self.clone();
        s.slice(pos, end)
    }

    fn peek_while<F>(&mut self, mut pred: F) -> &'a str
    where
        F: FnMut(char) -> bool,
    {
        let s = self.content_iter.as_str();
        let mut next = 0;

        for (index, item) in s.char_indices() {
            if pred(item) {
                next = self.calc_next_index(index, item);
            } else {
                break;
            }
        }

        let ret = &s[..next];
        self.end += next as u32;
        self.content_iter = s[next..].char_indices();

        ret
    }

    fn find<F>(&mut self, mut pred: F) -> Option<u32>
    where
        F: FnMut(char) -> bool,
    {
        let s = self.content_iter.as_str();
        let mut next = 0;

        for (index, item) in s.char_indices() {
            if pred(item) {
                next = self.calc_next_index(index, item);
                break;
            }
        }
        if next == 0 {
            return None;
        }

        self.end += next as u32;
        self.content_iter = s[next..].char_indices();

        Some(self.end)
    }

    fn find_ref<F>(&mut self, pred: F) -> Option<u32>
    where
        F: FnMut(char) -> bool,
    {
        let mut s = self.clone();
        s.find(pred)
    }

    fn reset_to(&mut self, pos: u32, end: u32) {
        self.pos = pos;
        self.end = end;
        self.content_iter = self.content[(pos as usize)..].char_indices();
    }
}

#[cfg(test)]
mod inputer {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    pub fn peek() {
        let mut inputer = Inputer::from("abc");
        assert_eq!(inputer.current_pos(), 0);
        assert_eq!(inputer.current_end(), 0);
        assert_eq!(inputer.is_at_start(), true);
        assert_eq!(inputer.is_at_end(), false);

        assert_eq!(inputer.peek(), Some('a'));
        assert_eq!(inputer.current_pos(), 0);
        assert_eq!(inputer.current_end(), 1);
        inputer.bump();
        assert_eq!(inputer.current_pos(), 1);
        assert_eq!(inputer.current_end(), 1);

        assert_eq!(inputer.peek(), Some('b'));
        assert_eq!(inputer.current_pos(), 1);
        assert_eq!(inputer.current_end(), 2);
        inputer.bump();
        assert_eq!(inputer.current_pos(), 2);
        assert_eq!(inputer.current_end(), 2);

        assert_eq!(inputer.peek(), Some('c'));
        assert_eq!(inputer.current_pos(), 2);
        assert_eq!(inputer.current_end(), 3);
        inputer.bump();
        assert_eq!(inputer.current_pos(), 3);
        assert_eq!(inputer.current_end(), 3);
        assert_eq!(inputer.is_at_start(), false);
        assert_eq!(inputer.is_at_end(), true);

        assert_eq!(inputer.peek(), None);
        assert_eq!(inputer.current_pos(), 3);
        assert_eq!(inputer.current_end(), 3);
    }

    #[test]
    pub fn peek_ref() {
        let mut inputer = Inputer::from("abc");
        assert_eq!(inputer.peek_ref(), Some('a'));
        assert_eq!(inputer.peek(), Some('a'));

        assert_eq!(inputer.peek_ref(), Some('b'));
        assert_eq!(inputer.peek(), Some('b'));

        assert_eq!(inputer.peek_ref(), Some('c'));
        assert_eq!(inputer.peek(), Some('c'));

        assert_eq!(inputer.peek_ref(), None);
        assert_eq!(inputer.peek(), None);
    }

    #[test]
    pub fn slice() {
        let mut inputer = Inputer::from("abc/def");
        assert_eq!(inputer.slice(0, 3), "abc");
        assert_eq!(inputer.current_pos(), 0);
        assert_eq!(inputer.current_end(), 3);
        inputer.bump();

        assert_eq!(inputer.slice(4, 7), "def");
        assert_eq!(inputer.current_pos(), 4);
        assert_eq!(inputer.current_end(), 7);
    }

    #[test]
    pub fn slice_ref() {
        let mut inputer = Inputer::from("abc/def");
        assert_eq!(inputer.slice_ref(0, 3), "abc");
        assert_eq!(inputer.current_pos(), 0);
        assert_eq!(inputer.current_end(), 0);

        assert_eq!(inputer.slice_ref(4, 7), "def");
        assert_eq!(inputer.current_pos(), 0);
        assert_eq!(inputer.current_end(), 0);
    }

    #[test]
    pub fn peek_while() {
        let mut inputer = Inputer::from("abc/def");
        assert_eq!(inputer.peek_while(|c| c != '/'), "abc");
        assert_eq!(inputer.current_pos(), 0);
        assert_eq!(inputer.current_end(), 3);
        inputer.bump();

        assert_eq!(inputer.peek_while(|c| c != 'z'), "/def");
        assert_eq!(inputer.current_pos(), 3);
        assert_eq!(inputer.current_end(), 7);
        inputer.bump();
    }

    #[test]
    pub fn find() {
        let mut inputer = Inputer::from("abc/def");
        assert_eq!(inputer.find(|c| c == '/'), Some(4));
        assert_eq!(inputer.current_pos(), 0);
        assert_eq!(inputer.current_end(), 4);
        inputer.bump();

        assert_eq!(inputer.find(|c| c == 'z'), None);
        assert_eq!(inputer.current_pos(), 4);
        assert_eq!(inputer.current_end(), 4);
        inputer.bump();

        assert_eq!(inputer.find(|c| c == 'e'), Some(6));
        assert_eq!(inputer.current_pos(), 4);
        assert_eq!(inputer.current_end(), 6);
    }

    #[test]
    pub fn find_ref() {
        let mut inputer = Inputer::from("abc/def");
        assert_eq!(inputer.find_ref(|c| c == '/'), Some(4));
        assert_eq!(inputer.current_pos(), 0);
        assert_eq!(inputer.current_end(), 0);

        assert_eq!(inputer.find_ref(|c| c == 'z'), None);
        assert_eq!(inputer.current_pos(), 0);
        assert_eq!(inputer.current_end(), 0);

        assert_eq!(inputer.find_ref(|c| c == 'e'), Some(6));
        assert_eq!(inputer.current_pos(), 0);
        assert_eq!(inputer.current_end(), 0);
    }

    #[test]
    pub fn reset_to() {
        let mut inputer = Inputer::from("abc");
        assert_eq!(inputer.peek(), Some('a'));
        assert_eq!(inputer.current_pos(), 0);
        assert_eq!(inputer.current_end(), 1);
        inputer.bump();
        assert_eq!(inputer.current_pos(), 1);
        assert_eq!(inputer.current_end(), 1);

        inputer.reset_to(0, 0);
        assert_eq!(inputer.current_pos(), 0);
        assert_eq!(inputer.current_end(), 0);

        assert_eq!(inputer.peek(), Some('a'));
        assert_eq!(inputer.current_pos(), 0);
        assert_eq!(inputer.current_end(), 1);
        inputer.bump();
        assert_eq!(inputer.current_pos(), 1);
        assert_eq!(inputer.current_end(), 1);
    }
}
