use super::*;
use crate::{utils::AdvanceIter, Span, SrcCodePoint, SynParser};

/// Lexer.
#[derive(Debug, Clone)]
pub struct Lexer<I>
where
    I: Iterator<Item = SrcCodePoint> + std::fmt::Debug + Clone + AdvanceIter,
{
    iter: I,
}

impl<I> Lexer<I>
where
    I: Iterator<Item = SrcCodePoint> + std::fmt::Debug + Clone + AdvanceIter,
{
    /// Create a new lexer.
    pub fn new(iter: I) -> Self {
        Self { iter }
    }

    /// Ignore spaces and newlines.
    pub fn ignore_spaces_and_newlines(self) -> impl Iterator<Item = Token> {
        self.filter(|t| !matches!(t.value, TokenKind::Spaces | TokenKind::Newlines))
    }

    /// Parse syntax.
    pub fn parse_syn(self) -> SynParser {
        SynParser::new(self.ignore_spaces_and_newlines().collect())
    }
}

impl<I> Iterator for Lexer<I>
where
    I: Iterator<Item = SrcCodePoint> + std::fmt::Debug + Clone + AdvanceIter,
{
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(point) => Some(match point.ch {
                ch @ ('(' | '{') => Token {
                    value: TokenKind::OpenBrac(ch),
                    span: Span {
                        start: point,
                        len: 1,
                    },
                },
                ch @ (')' | '}') => Token {
                    value: TokenKind::CloseBrac(ch),
                    span: Span {
                        start: point,
                        len: 1,
                    },
                },
                '\n' | '\r' => {
                    let len = 1 + self
                        .iter
                        .clone()
                        .take_while(|p| ['\n', '\r'].contains(&p.ch))
                        .count();
                    self.iter.advance(len - 1);
                    Token {
                        value: TokenKind::Newlines,
                        span: Span { start: point, len },
                    }
                }
                ch if ch.is_whitespace() => {
                    let len = 1 + self
                        .iter
                        .clone()
                        .take_while(|p| p.ch.is_whitespace())
                        .count();
                    self.iter.advance(len - 1);
                    Token {
                        value: TokenKind::Spaces,
                        span: Span { start: point, len },
                    }
                }
                ch => {
                    let ident = std::iter::once(ch)
                        .chain(self.iter.clone().map(|p| p.ch).take_while(|c| {
                            !c.is_whitespace() && !['(', ')', '\r', '\n'].contains(&c)
                        }))
                        .collect::<String>();
                    let len = ident.len();
                    self.iter.advance(len - 1);
                    Token {
                        value: TokenKind::Ident(ident),
                        span: Span { start: point, len },
                    }
                }
            }),
            None => None,
        }
    }
}
