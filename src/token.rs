#![allow(unused)]
pub struct Sequence;

impl Sequence {
    pub fn new(tokens: &[Token]) -> Sequence {
        todo!()
    }
}

pub struct Code;

impl Code {
    pub fn new(code: &str) -> Code {
        todo!()
    }

    pub fn lex(&self) -> Sequence {
        todo!()
    }

    pub fn frag(&self, s: &Span) -> String {
        todo!()
    }
}

pub struct Span;

pub enum Token {
    Ident(String),
    Punct(char),
    Lit(i32),
}
