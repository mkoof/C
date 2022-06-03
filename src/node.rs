#![allow(unused)]

mod defs;

use crate::span::Span;
use crate::token::{Cursor, Token, TokenKind};
use c_macros::*;
use defs::{define_default, define_keyword, define_symbol};
use defs::{Node, Parse, SemanticAnalyze, Split};

define_keyword!(Int, "int");
define_symbol!(Eq, "==");

#[derive(Parse, Node, SemanticAnalyze)]
pub struct A {
    a: Int,
    b: Eq,
    c: (Int, Eq),
    d: Split<Int, Eq>,
    span: Span,
}

#[derive(Parse, Node, SemanticAnalyze)]
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
