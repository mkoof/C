#![allow(unused)]

use crate::err::CompileError;

const TAB_SIZE: usize = 4;

pub struct Sequence;

impl Sequence {
    pub fn new(tokens: &[Token]) -> Sequence {
        todo!()
    }
}

pub struct Code {
    m: Vec<Vec<char>>,
}

impl Code {
    pub fn new(code: &str) -> Code {
        let mut m = Vec::new();
        let mut line = Vec::<char>::new();
        for c in code.chars() {
            if c == '\n' {
                m.push(line);
                line = Vec::new();
            } else if c == '\t' {
                let space_count = TAB_SIZE - line.len() % TAB_SIZE;
                for _ in 0..space_count {
                    line.push(' ');
                }
            } else {
                line.push(c);
            }
        }
        m.push(line);
        Code { m }
    }

    pub fn lex(&self) -> Result<Sequence, CompileError> {
        todo!()
    }

    pub fn frag(&self, span: &Span) -> String {
        let mut res = String::new();
        for row in span.row1..=span.row2 {
            let i = if row == span.row1 { span.col1 } else { 0 };
            let j = if row == span.row2 {
                span.col2 + 1
            } else {
                self.m[row].len()
            };
            for k in i..j {
                res.push(self.m[row][k]);
            }
            if row < span.row2 {
                res.push('\n');
            }
        }
        res
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct Span {
    row1: usize,
    col1: usize,
    row2: usize,
    col2: usize,
}

impl Span {
    fn single(row: usize, col: usize) -> Span {
        Span {
            row1: row,
            col1: col,
            row2: row,
            col2: col,
        }
    }

    fn new(row1: usize, col1: usize, row2: usize, col2: usize) -> Span {
        Span {
            row1,
            col1,
            row2,
            col2,
        }
    }

    fn concat(&self, other: &Span) -> Span {
        if self.le(other) {
            Span {
                row1: self.row1,
                col1: self.col1,
                row2: other.row2,
                col2: other.col2,
            }
        } else {
            Span {
                row1: other.row1,
                col1: other.col1,
                row2: self.row2,
                col2: self.col2,
            }
        }
    }
}

pub enum Token {
    Ident(String),
    Punct(char),
    Lit(i32),
}

#[cfg(test)]
mod tests {
    use super::*;

    /// test get code from span
    #[test]
    fn code_frag() {
        let code = "1   23  4
        567";
        let code = Code::new(code);
        let span = Span::single(0, 0);
        assert_eq!(code.frag(&span), "1");
        let span = Span::new(0, 0, 0, 4);
        assert_eq!(code.frag(&span), "1   2");
        let span = Span::single(0, 8);
        assert_eq!(code.frag(&span), "4");
        let span = Span::single(1, 0);
        assert_eq!(code.frag(&span), " ");
        let span = Span::single(1, 8);
        assert_eq!(code.frag(&span), "5");
        let s1 = Span::single(0, 0);
        let s2 = Span::single(1, 10);
        let span = s1.concat(&s2);
        assert_eq!(code.frag(&span).len(), 21);
    }
}
