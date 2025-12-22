use std::{collections::VecDeque, iter::Peekable};

pub struct CharIter<I: Iterator<Item = char>> {
    pushed: VecDeque<char>,
    inner: Peekable<I>,
}

impl<I: Iterator<Item = char>> Iterator for CharIter<I> {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        self.pushed.pop_back().or_else(|| self.inner.next())
    }
}

impl<I: Iterator<Item = char>> CharIter<I> {
    pub fn new(inner: I) -> Self {
        let inner = inner.peekable();
        Self {
            pushed: VecDeque::new(),
            inner,
        }
    }

    pub fn peek(&mut self) -> Option<char> {
        self.pushed
            .iter()
            .last()
            .or_else(|| self.inner.peek())
            .copied()
    }

    pub fn push_front(&mut self, c: char) {
        self.pushed.push_front(c);
    }
    pub fn push_str_front(&mut self, str: &str) {
        self.pushed.reserve(str.len());
        str.chars().for_each(|c| self.pushed.push_front(c));
    }
}
