#![allow(unused)]

use crate::err::CompileError;

const TAB_SIZE: usize = 4;

pub struct Sequence {
    tokens: Vec<Token>,
    spans: Vec<Span>,
}

impl Sequence {
    pub fn new(tokens: Vec<Token>, spans: Vec<Span>) -> Sequence {
        assert_eq!(tokens.len(), spans.len());
        Sequence { tokens, spans }
    }

    pub fn len(&self) -> usize {
        self.tokens.len()
    }

    pub fn span(&self, idx: usize) -> Option<&Span> {
        self.spans.get(idx)
    }

    pub fn token(&self, idx: usize) -> Option<&Token> {
        self.tokens.get(idx)
    }

    pub fn cursor(&self) -> Cursor {
        Cursor {
            seq: self,
            offset: 0,
        }
    }
}

pub struct Cursor<'a> {
    seq: &'a Sequence,
    offset: usize,
}

impl<'a> Cursor<'a> {
    pub fn span(&self, idx: usize) -> Option<&Span> {
        self.seq.span(self.offset + idx)
    }

    pub fn token(&self, idx: usize) -> Option<&Token> {
        self.seq.token(self.offset + idx)
    }

    pub fn advance(&mut self, d: usize) {
        self.offset += d;
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
        let mut spans = Vec::new();
        let mut seq = Vec::new();
        for row in 0..self.m.len() {
            let line = &self.m[row];
            let len = line.len();
            let mut col = 0;
            while col < len {
                let col_backup = col;
                let c = self.m[row][col];
                if c == ' ' {
                    col += 1;
                    continue;
                }
                if !c.is_ascii() {
                    return Err(CompileError::InvalidChar(Span::single(row, col)));
                } else if c.is_digit(10) {
                    let mut num = (c as u8 - b'0') as u32;
                    col += 1;
                    while col < len && line[col].is_digit(10) {
                        num = num * 10 + (line[col] as u8 - b'0') as u32;
                        col += 1;
                    }
                    seq.push(Token::Int(num));
                } else if c.is_ascii_alphabetic() {
                    let mut s = String::new();
                    s.push(c);
                    col += 1;
                    while col < len && line[col].is_ascii_alphanumeric() {
                        s.push(line[col]);
                        col += 1;
                    }
                    seq.push(Token::Ident(s));
                } else {
                    col += 1;
                    seq.push(Token::Punct(c));
                }
                spans.push(Span::new(row, col_backup, row, col - 1));
            }
        }
        Ok(Sequence::new(seq, spans))
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

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Ident(String),
    Punct(char),
    Int(u32),
}

#[cfg(test)]
mod tests {
    use super::*;

    /// test get code from span
    #[test]
    fn frag() {
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

    /// test generating sequence and cursor
    #[test]
    fn seq() -> Result<(), CompileError> {
        let code = "int ab2 = 12 23;";
        let code = Code::new(code);
        let seq = code.lex()?;
        assert_eq!(seq.len(), 6);
        assert_eq!(seq.token(0).unwrap(), &Token::Ident("int".to_owned()));
        assert_eq!(seq.token(1).unwrap(), &Token::Ident("ab2".to_owned()));
        assert_eq!(seq.token(2).unwrap(), &Token::Punct('='));
        assert_eq!(seq.token(3).unwrap(), &Token::Int(12));
        assert_eq!(seq.token(4).unwrap(), &Token::Int(23));
        assert_eq!(seq.token(5).unwrap(), &Token::Punct(';'));
        // test one span is enough
        assert_eq!(seq.span(1).unwrap(), &Span::new(0, 4, 0, 6));
        let mut cursor = seq.cursor();
        assert_eq!(seq.token(0), cursor.token(0));
        cursor.advance(1);
        assert_eq!(seq.token(2), cursor.token(1));
        assert_eq!(None, cursor.token(5));
        Ok(())
    }
}
