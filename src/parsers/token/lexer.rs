use super::*;
use crate::parsers::SrcCode;
use crate::utils::NextRangePeek;
use crate::{Span, SynParser};

/// Lexer.
#[derive(Debug, Clone)]
pub struct Lexer<Iter>
where
    Iter: Iterator<Item = SrcCode>,
{
    iter: NextRangePeek<Iter>,
}

impl<Iter> Lexer<Iter>
where
    Iter: Iterator<Item = SrcCode>,
{
    /// Create a new lexer.
    pub fn new(iter: NextRangePeek<Iter>) -> Self {
        Self { iter }
    }

    /// Ignore spaces and newlines.
    pub fn ignore_spaces_and_newlines(self) -> impl Iterator<Item = Token> {
        self.filter(|t| !t.value.is_spaces() && !t.value.is_newlines())
    }

    /// Parse syntax.
    pub fn parse_syn(self) -> SynParser {
        SynParser::new(self.ignore_spaces_and_newlines().collect())
    }
}

impl<Iter> Iterator for Lexer<Iter>
where
    Iter: Iterator<Item = SrcCode>,
{
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        None.or_else(|| parse_open_brac(&mut self.iter))
            .or_else(|| parse_close_brac(&mut self.iter))
            .or_else(|| parse_newlines(&mut self.iter))
            .or_else(|| parse_spaces(&mut self.iter))
            .or_else(|| parse_ident(&mut self.iter))
    }
}

/// Parse source code into [`TokenKind::OpneBrac`].
fn parse_open_brac<Iter>(iter: &mut NextRangePeek<Iter>) -> Option<Token>
where
    Iter: Iterator<Item = SrcCode>,
{
    match iter.peek(1) {
        [c] if ['(', '{'].contains(&c.value) => {
            iter.next().map(|c| c.map(|ch| TokenKind::OpenBrac(ch)))
        }
        _ => None,
    }
}

/// Parse source code into [`TokenKind::CloseBrac`].
fn parse_close_brac<Iter>(iter: &mut NextRangePeek<Iter>) -> Option<Token>
where
    Iter: Iterator<Item = SrcCode>,
{
    match iter.peek(1) {
        [c] if [')', '}'].contains(&c.value) => {
            iter.next().map(|c| c.map(|ch| TokenKind::CloseBrac(ch)))
        }
        _ => None,
    }
}

/// Parse source code into [`TokenKind::Newlines`].
fn parse_newlines<Iter>(iter: &mut NextRangePeek<Iter>) -> Option<Token>
where
    Iter: Iterator<Item = SrcCode>,
{
    match iter.peek_while(|c| ['\n', '\r'].contains(&c.value)) {
        [] => None,
        codes => {
            let len = codes.len();
            let first = iter.next().unwrap();
            for _ in 1..len {
                iter.next();
            }

            Some(
                first
                    .map(|_| TokenKind::Newlines)
                    .map_span(|span| Span::new(span.start, len)),
            )
        }
    }
}

/// Parse source code into [`TokenKind::Spaces`].
fn parse_spaces<Iter>(iter: &mut NextRangePeek<Iter>) -> Option<Token>
where
    Iter: Iterator<Item = SrcCode>,
{
    match iter.peek_while(|c| c.value.is_whitespace()) {
        [] => None,
        codes => {
            let len = codes.len();
            let first = iter.next().unwrap();
            for _ in 1..len {
                iter.next();
            }

            Some(
                first
                    .map(|_| TokenKind::Spaces)
                    .map_span(|span| Span::new(span.start, len)),
            )
        }
    }
}

/// Parse source code into [`TokenKind::Ident`].
fn parse_ident<Iter>(iter: &mut NextRangePeek<Iter>) -> Option<Token>
where
    Iter: Iterator<Item = SrcCode>,
{
    match iter.peek_while(|c| {
        !c.value.is_whitespace() && !['(', ')', '{', '}', '\n', '\r'].contains(&c.value)
    }) {
        [] => None,
        codes => {
            let len = codes.len();
            let mut ident = String::with_capacity(len);

            let first = iter.next().unwrap();
            ident.push(first.value);
            for _ in 1..len {
                let code = iter.next().unwrap();
                ident.push(code.value);
            }

            Some(
                first
                    .map(|_| TokenKind::Ident(ident))
                    .map_span(|span| Span::new(span.start, len)),
            )
        }
    }
}
