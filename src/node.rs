#![allow(unused)]

mod parse;
mod semantic_analyze;

use std::marker::PhantomData;

use crate::span::Span;
use crate::token::{Cursor, Token, TokenKind};
use c_macros::*;
use parse::Parse;
use semantic_analyze::SemanticAnalyze;

pub trait Node: Parse + SemanticAnalyze {
    fn span(&self) -> Span;
}

pub struct Split<T, S> {
    data: Vec<T>,
    _marker: PhantomData<S>,
}

macro_rules! _default_node {
    ($name: ident) => {
        pub struct $name {
            span: Span,
        }

        impl $name {
            pub fn new(span: Span) -> Self {
                Self { span }
            }
        }

        impl Node for $name {
            fn span(&self) -> Span {
                self.span
            }
        }

        impl SemanticAnalyze for $name {
            fn semantic_analyze(&self) -> Result<(), Span> {
                Ok(())
            }
        }
    };
}

macro_rules! define_keyword {
    ($name: ident, $x: expr) => {
        _default_node!($name);

        impl Parse for $name {
            fn parse(cursor: &mut Cursor) -> Result<Self, Span> {
                if let Some(Token {
                    token: TokenKind::Ident(id),
                    ..
                }) = cursor.peek()
                {
                    if id == $x {
                        return Ok($name::new(cursor.next().unwrap().span));
                    }
                }
                Err(cursor.span())
            }
        }
    };
}

macro_rules! define_puncts {
    ($name: ident, $x: expr) => {
        _default_node!($name);

        impl Parse for $name {
            fn parse(cursor: &mut Cursor) -> Result<Self, Span> {
                let mut span = Span::empty();
                for ch in $x.chars() {
                    let b = ch as u8;
                    if let Some(Token {
                        token: TokenKind::Punct(p),
                        ..
                    }) = cursor.peek()
                    {
                        if *p == b {
                            span = span.merge(&cursor.span());
                            cursor.next();
                            continue;
                        }
                    }
                    return Err(cursor.span());
                }
                Ok($name::new(span))
            }
        }
    };
}

define_keyword!(Int, "int");
define_puncts!(Eq, "==");

#[derive(Parse)]
pub struct A {
    a: Int,
    b: Eq,
    c: (Int, Eq),
    d: Split<Int, Eq>,
}

#[derive(Parse)]
pub enum B {
    A(Int),
    B(Eq),
    C(Int, Eq),
}

#[cfg(test)]
mod tests {
    pub use super::*;

    #[test]
    fn test_define() {
        let code = "int ==";
        let tokens = crate::lex(code.to_owned());
        let mut cursor = tokens.cursor();
        assert!(Int::parse(&mut cursor).is_ok());
        assert!(Int::parse(&mut cursor).is_err());
        assert!(Eq::parse(&mut cursor).is_ok());
        cursor = tokens.cursor();
        assert!(<Int as Parse>::parse(&mut cursor).is_ok());
    }
}
