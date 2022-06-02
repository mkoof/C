use super::Split;
use crate::span::Span;
use crate::token::Cursor;

pub trait Parse {
    fn parse(cursor: &mut Cursor) -> Result<Self, Span>
    where
        Self: Sized;
}

impl<T> Parse for Option<T>
where
    T: Parse,
{
    fn parse(cursor: &mut Cursor) -> Result<Self, Span> {
        T::parse(cursor).map(|t| Some(t))
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

impl<T> Parse for Box<T>
where
    T: Parse,
{
    fn parse(cursor: &mut Cursor) -> Result<Self, Span> {
        T::parse(cursor).map(|t| Box::new(t))
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
