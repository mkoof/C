use crate::span::{Pos, Span};

pub struct CharSeq {
    chars: Vec<char>,
    spans: Vec<Span>,
}

impl CharSeq {
    pub fn new(s: String) -> Self {
        let cap = s.len();
        let mut pos = Pos::zero();
        let mut chars = Vec::with_capacity(cap);
        let mut spans = Vec::with_capacity(cap);
        for c in s.chars() {
            chars.push(c);
            spans.push(Span::new(pos, pos));
            pos.advance(c);
        }
        CharSeq { chars, spans }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&char, &Span)> {
        self.chars.iter().zip(self.spans.iter())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum TokenKind {
    Ident(String),
    Int(i64),
    // Float(f64),
    Punct(u8),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub token: TokenKind,
    pub span: Span,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Tokens {
    tokens: Vec<Token>,
}

impl Tokens {
    pub fn new(tokens: Vec<Token>) -> Self {
        Tokens { tokens }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Token> {
        self.tokens.iter()
    }

    pub fn get(&self, i: usize) -> Option<&Token> {
        self.tokens.get(i)
    }

    pub fn cursor(&self) -> Cursor {
        Cursor::new(self)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cursor<'a> {
    tokens: &'a Tokens,
    pos: usize,
}

impl<'a> Cursor<'a> {
    pub fn new(tokens: &'a Tokens) -> Self {
        Cursor { tokens, pos: 0 }
    }

    pub fn next(&mut self) -> Option<&Token> {
        self.pos += 1;
        self.tokens.get(self.pos - 1)
    }

    pub fn peek(&self) -> Option<&Token> {
        self.get(0)
    }

    pub fn get(&self, i: usize) -> Option<&Token> {
        self.tokens.get(self.pos + i)
    }

    pub fn span(&self) -> Span {
        if let Some(token) = self.peek() {
            token.span
        } else {
            Span::eof()
        }
    }
}

pub fn lex(seq: &CharSeq) -> Tokens {
    let mut kind = None::<TokenKind>;
    let mut span = Span::zero();
    let mut tokens = Vec::new();
    for (c, s) in seq.iter() {
        match c {
            '0'..='9' => {
                if let Some(TokenKind::Int(mut num)) = kind {
                    num = num * 10 + (*c as u8 - b'0') as i64;
                    kind = Some(TokenKind::Int(num));
                    span = span.merge(s);
                } else if let Some(TokenKind::Ident(mut id)) = kind {
                    id.push(*c);
                    kind = Some(TokenKind::Ident(id));
                    span = span.merge(s);
                } else {
                    if kind.is_some() {
                        tokens.push(Token {
                            token: kind.unwrap(),
                            span,
                        });
                    }
                    kind = Some(TokenKind::Int((*c as u8 - b'0') as i64));
                    span = *s;
                }
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                if let Some(TokenKind::Ident(mut id)) = kind {
                    id.push(*c);
                    kind = Some(TokenKind::Ident(id));
                    span = span.merge(s);
                } else {
                    if kind.is_some() {
                        tokens.push(Token {
                            token: kind.unwrap(),
                            span,
                        });
                    }
                    let mut id = String::new();
                    id.push(*c);
                    kind = Some(TokenKind::Ident(id));
                    span = *s;
                }
            }
            ' ' | '\t' | '\n' | '\r' => {
                if kind.is_some() {
                    tokens.push(Token {
                        token: kind.unwrap(),
                        span,
                    });
                    kind = None;
                }
            }
            _ => {
                if kind.is_some() {
                    tokens.push(Token {
                        token: kind.unwrap(),
                        span,
                    });
                }
                kind = Some(TokenKind::Punct(*c as u8));
                span = *s;
            }
        }
    }
    if kind.is_some() {
        tokens.push(Token {
            token: kind.unwrap(),
            span,
        });
    }
    Tokens { tokens }
}

#[cfg(test)]
mod tests {
    pub use super::*;

    #[test]
    fn test_lex() {
        let code = "int main() {
    int x = 1;
    int z = x + y;
    return 0;
}
        ";
        let seq = CharSeq::new(code.to_string());
        let tokens = lex(&seq);
        let mut cursor = tokens.cursor();
        assert_eq!(
            cursor.next(),
            Some(&Token {
                token: TokenKind::Ident("int".to_string()),
                span: Span::new(Pos::zero(), Pos::new(0, 2)),
            })
        );
        assert_eq!(
            cursor.next(),
            Some(&Token {
                token: TokenKind::Ident("main".to_string()),
                span: Span::new(Pos::new(0, 4), Pos::new(0, 7)),
            })
        );
        assert_eq!(
            cursor.next(),
            Some(&Token {
                token: TokenKind::Punct(b'('),
                span: Span::new(Pos::new(0, 8), Pos::new(0, 8)),
            })
        );
        assert_eq!(
            cursor.next(),
            Some(&Token {
                token: TokenKind::Punct(b')'),
                span: Span::new(Pos::new(0, 9), Pos::new(0, 9)),
            })
        );
        assert_eq!(
            cursor.next(),
            Some(&Token {
                token: TokenKind::Punct(b'{'),
                span: Span::new(Pos::new(0, 11), Pos::new(0, 11)),
            })
        );
        assert_eq!(
            cursor.next(),
            Some(&Token {
                token: TokenKind::Ident("int".to_string()),
                span: Span::new(Pos::new(1, 4), Pos::new(1, 6)),
            })
        );
        // -- snip --
    }
}
