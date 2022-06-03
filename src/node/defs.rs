use std::marker::PhantomData;

use crate::span::Span;
use crate::token::{Cursor, Token, TokenKind};

pub struct Split<T, S> {
    data: Vec<T>,
    _marker: PhantomData<S>,
}

pub trait Node: Parse {
    fn span(&self) -> Span;
}

pub trait Parse {
    fn parse(cursor: &mut Cursor) -> Result<Self, Span>
    where
        Self: Sized;
}

pub trait SemanticAnalyze {
    fn semantic_analyze(&self) -> Result<(), Span>;
}

impl<T> Node for Option<T>
where
    T: Node,
{
    fn span(&self) -> Span {
        match self {
            Some(t) => t.span(),
            None => Span::empty(),
        }
    }
}

impl<T> Parse for Option<T>
where
    T: Parse,
{
    fn parse(cursor: &mut Cursor) -> Result<Self, Span> {
        T::parse(cursor).map(|t| Some(t))
    }
}

impl<T> SemanticAnalyze for Option<T>
where
    T: SemanticAnalyze,
{
    fn semantic_analyze(&self) -> Result<(), Span> {
        match self {
            Some(t) => t.semantic_analyze(),
            None => Ok(()),
        }
    }
}

impl<T> Node for Vec<T>
where
    T: Node,
{
    fn span(&self) -> Span {
        let mut span = Span::empty();
        for t in self {
            span = span.merge(&t.span());
        }
        span
    }
}

impl<T> Parse for Vec<T>
where
    T: Parse,
{
    fn parse(cursor: &mut Cursor) -> Result<Self, Span> {
        let mut vec = Vec::new();
        while let Ok(t) = T::parse(cursor) {
            vec.push(t);
        }
        Ok(vec)
    }
}

impl<T> SemanticAnalyze for Vec<T>
where
    T: SemanticAnalyze,
{
    fn semantic_analyze(&self) -> Result<(), Span> {
        for t in self {
            t.semantic_analyze()?;
        }
        Ok(())
    }
}

impl<T, S> Node for Split<T, S>
where
    T: Node,
    S: Node,
{
    fn span(&self) -> Span {
        let mut span = Span::empty();
        for t in self.data.iter() {
            span = span.merge(&t.span());
        }
        span
    }
}

impl<T, S> Parse for Split<T, S>
where
    T: Parse,
    S: Parse,
{
    fn parse(cursor: &mut Cursor) -> Result<Self, Span> {
        let mut vec = Vec::new();
        if let Ok(t) = T::parse(cursor) {
            vec.push(t);
        } else {
            return Ok(Split {
                data: vec,
                _marker: Default::default(),
            });
        }
        while S::parse(cursor).is_ok() {
            vec.push(T::parse(cursor)?);
        }
        Ok(Split {
            data: vec,
            _marker: Default::default(),
        })
    }
}

impl<T, S> SemanticAnalyze for Split<T, S>
where
    T: SemanticAnalyze,
    S: SemanticAnalyze,
{
    fn semantic_analyze(&self) -> Result<(), Span> {
        for t in self.data.iter() {
            t.semantic_analyze()?;
        }
        Ok(())
    }
}

impl<T> Node for Box<T>
where
    T: Node,
{
    fn span(&self) -> Span {
        (**self).span()
    }
}

impl<T> Parse for Box<T>
where
    T: Parse,
{
    fn parse(cursor: &mut Cursor) -> Result<Self, Span> {
        T::parse(cursor).map(|t| Box::new(t))
    }
}

impl<T> SemanticAnalyze for Box<T>
where
    T: SemanticAnalyze,
{
    fn semantic_analyze(&self) -> Result<(), Span> {
        (**self).semantic_analyze()
    }
}

impl<T1, T2> Node for (T1, T2)
where
    T1: Node,
    T2: Node,
{
    fn span(&self) -> Span {
        let (ref t1, ref t2) = *self;
        t1.span().merge(&t2.span())
    }
}

impl<T1, T2> Parse for (T1, T2)
where
    T1: Parse,
    T2: Parse,
{
    fn parse(cursor: &mut Cursor) -> Result<Self, Span> {
        Ok((T1::parse(cursor)?, T2::parse(cursor)?))
    }
}

impl<T1, T2> SemanticAnalyze for (T1, T2)
where
    T1: SemanticAnalyze,
    T2: SemanticAnalyze,
{
    fn semantic_analyze(&self) -> Result<(), Span> {
        let (ref t1, ref t2) = *self;
        t1.semantic_analyze()?;
        t2.semantic_analyze()?;
        Ok(())
    }
}

impl<T1, T2, T3> Node for (T1, T2, T3)
where
    T1: Node,
    T2: Node,
    T3: Node,
{
    fn span(&self) -> Span {
        let (ref t1, ref t2, ref t3) = *self;
        t1.span().merge(&t2.span()).merge(&t3.span())
    }
}

impl<T1, T2, T3> Parse for (T1, T2, T3)
where
    T1: Parse,
    T2: Parse,
    T3: Parse,
{
    fn parse(cursor: &mut Cursor) -> Result<Self, Span> {
        Ok((T1::parse(cursor)?, T2::parse(cursor)?, T3::parse(cursor)?))
    }
}

impl<T1, T2, T3> SemanticAnalyze for (T1, T2, T3)
where
    T1: SemanticAnalyze,
    T2: SemanticAnalyze,
    T3: SemanticAnalyze,
{
    fn semantic_analyze(&self) -> Result<(), Span> {
        let (ref t1, ref t2, ref t3) = *self;
        t1.semantic_analyze()?;
        t2.semantic_analyze()?;
        t3.semantic_analyze()?;
        Ok(())
    }
}

impl<T1, T2, T3, T4> Node for (T1, T2, T3, T4)
where
    T1: Node,
    T2: Node,
    T3: Node,
    T4: Node,
{
    fn span(&self) -> Span {
        let (ref t1, ref t2, ref t3, ref t4) = *self;
        t1.span()
            .merge(&t2.span())
            .merge(&t3.span())
            .merge(&t4.span())
    }
}

impl<T1, T2, T3, T4> Parse for (T1, T2, T3, T4)
where
    T1: Parse,
    T2: Parse,
    T3: Parse,
    T4: Parse,
{
    fn parse(cursor: &mut Cursor) -> Result<Self, Span> {
        Ok((
            T1::parse(cursor)?,
            T2::parse(cursor)?,
            T3::parse(cursor)?,
            T4::parse(cursor)?,
        ))
    }
}

impl<T1, T2, T3, T4> SemanticAnalyze for (T1, T2, T3, T4)
where
    T1: SemanticAnalyze,
    T2: SemanticAnalyze,
    T3: SemanticAnalyze,
    T4: SemanticAnalyze,
{
    fn semantic_analyze(&self) -> Result<(), Span> {
        let (ref t1, ref t2, ref t3, ref t4) = *self;
        t1.semantic_analyze()?;
        t2.semantic_analyze()?;
        t3.semantic_analyze()?;
        t4.semantic_analyze()?;
        Ok(())
    }
}

impl<T1, T2, T3, T4, T5> Node for (T1, T2, T3, T4, T5)
where
    T1: Node,
    T2: Node,
    T3: Node,
    T4: Node,
    T5: Node,
{
    fn span(&self) -> Span {
        let (ref t1, ref t2, ref t3, ref t4, ref t5) = *self;
        t1.span()
            .merge(&t2.span())
            .merge(&t3.span())
            .merge(&t4.span())
            .merge(&t5.span())
    }
}

impl<T1, T2, T3, T4, T5> Parse for (T1, T2, T3, T4, T5)
where
    T1: Parse,
    T2: Parse,
    T3: Parse,
    T4: Parse,
    T5: Parse,
{
    fn parse(cursor: &mut Cursor) -> Result<Self, Span> {
        Ok((
            T1::parse(cursor)?,
            T2::parse(cursor)?,
            T3::parse(cursor)?,
            T4::parse(cursor)?,
            T5::parse(cursor)?,
        ))
    }
}

impl<T1, T2, T3, T4, T5> SemanticAnalyze for (T1, T2, T3, T4, T5)
where
    T1: SemanticAnalyze,
    T2: SemanticAnalyze,
    T3: SemanticAnalyze,
    T4: SemanticAnalyze,
    T5: SemanticAnalyze,
{
    fn semantic_analyze(&self) -> Result<(), Span> {
        let (ref t1, ref t2, ref t3, ref t4, ref t5) = *self;
        t1.semantic_analyze()?;
        t2.semantic_analyze()?;
        t3.semantic_analyze()?;
        t4.semantic_analyze()?;
        t5.semantic_analyze()?;
        Ok(())
    }
}

macro_rules! define_default {
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
        define_default!($name);

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

macro_rules! define_symbol {
    ($name: ident, $x: expr) => {
        define_default!($name);

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

pub(crate) use define_default;
pub(crate) use define_keyword;
pub(crate) use define_symbol;
