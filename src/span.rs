const TAB_SIZE: usize = 4;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Pos {
    pub line: usize,
    pub col: usize,
}

impl Pos {
    pub fn new(line: usize, col: usize) -> Pos {
        Pos { line, col }
    }

    pub fn advance(&mut self, c: char) {
        if c == '\n' {
            self.line += 1;
            self.col = 0;
        } else if c == '\t' {
            self.col = (self.col + TAB_SIZE) & !(TAB_SIZE - 1);
        } else {
            self.col += 1;
        }
    }

    pub fn zero() -> Pos {
        Pos { line: 0, col: 0 }
    }

    pub fn eof() -> Pos {
        Pos {
            line: usize::max_value(),
            col: usize::max_value(),
        }
    }

    pub fn is_eof(&self) -> bool {
        self.line == usize::max_value()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Span {
    pub start: Pos,
    pub end: Pos,
}

impl Span {
    pub fn new(start: Pos, end: Pos) -> Span {
        Span { start, end }
    }

    pub fn merge(&self, other: &Span) -> Span {
        if self.is_eof() {
            *other
        } else if other.is_eof() {
            *self
        } else {
            Span {
                start: self.start,
                end: other.end,
            }
        }
    }

    pub fn zero() -> Span {
        Span {
            start: Pos::zero(),
            end: Pos::zero(),
        }
    }

    pub fn eof() -> Span {
        Span {
            start: Pos::eof(),
            end: Pos::eof(),
        }
    }

    pub fn empty() -> Span {
        Span::eof()
    }

    pub fn is_eof(&self) -> bool {
        self.start == Pos::eof()
    }
}

#[cfg(test)]
mod tests {
    pub use super::*;

    #[test]
    fn test_merge() {
        let span1 = Span::new(Pos::new(1, 1), Pos::new(1, 2));
        let span2 = Span::new(Pos::new(1, 2), Pos::new(1, 3));
        let merged = span1.merge(&span2);
        assert_eq!(merged, Span::new(Pos::new(1, 1), Pos::new(1, 3)));
    }
}
