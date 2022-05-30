#![allow(unused)]

use std::cmp;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Pos {
    pub line: usize,
    pub col: usize,
}

impl Pos {
    pub fn new(line: usize, col: usize) -> Pos {
        Pos { line, col }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Span {
    pub start: Pos,
    pub end: Pos,
}

impl Span {
    pub fn new(start: Pos, end: Pos) -> Span {
        Span { start, end }
    }

    pub fn merge(&self, other: &Span) -> Span {
        Span {
            start: Pos {
                line: cmp::min(self.start.line, other.start.line),
                col: cmp::min(self.start.col, other.start.col),
            },
            end: Pos {
                line: cmp::max(self.end.line, other.end.line),
                col: cmp::max(self.end.col, other.end.col),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    pub use super::*;

    #[test]
    fn merge() {
        let span1 = Span::new(Pos::new(1, 1), Pos::new(1, 2));
        let span2 = Span::new(Pos::new(1, 2), Pos::new(1, 3));
        let merged = span1.merge(&span2);
        assert_eq!(merged, Span::new(Pos::new(1, 1), Pos::new(1, 3)));
    }
}

