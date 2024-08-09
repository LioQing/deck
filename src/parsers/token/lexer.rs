use super::*;
use crate::parsers::SrcCode;
use crate::utils::NextRangePeek;
use crate::{utils::AdvanceIterExt, Span, SynParser};

/// Lexer.
#[derive(Debug, Clone)]
pub struct Lexer<Iter>
where
    Iter: Iterator<Item = SrcCode> + std::fmt::Debug + Clone,
{
    iter: NextRangePeek<Iter>,
}

impl<Iter> Lexer<Iter>
where
    Iter: Iterator<Item = SrcCode> + std::fmt::Debug + Clone,
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
    Iter: Iterator<Item = SrcCode> + std::fmt::Debug + Clone,
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
    let token = match iter.peek(1) {
        [c] if ['(', '{'].contains(&c.value) => Token {
            value: TokenKind::OpenBrac(c.value),
            span: c.span.clone(),
        },
        _ => return None,
    };

    iter.advance(1);
    Some(token)
}

/// Parse source code into [`TokenKind::CloseBrac`].
fn parse_close_brac<Iter>(iter: &mut NextRangePeek<Iter>) -> Option<Token>
where
    Iter: Iterator<Item = SrcCode>,
{
    let token = match iter.peek(1) {
        [c] if [')', '}'].contains(&c.value) => Token {
            value: TokenKind::CloseBrac(c.value),
            span: c.span.clone(),
        },
        _ => return None,
    };

    iter.advance(1);
    Some(token)
}

/// Parse source code into [`TokenKind::Newlines`].
fn parse_newlines<Iter>(iter: &mut NextRangePeek<Iter>) -> Option<Token>
where
    Iter: Iterator<Item = SrcCode>,
{
    let (token, len) = match iter.peek_while(|c| ['\n', '\r'].contains(&c.value)) {
        [] => return None,
        codes @ [c, ..] => (
            Token {
                value: TokenKind::Newlines,
                span: Span::new(c.span.start.clone(), codes.len()),
            },
            codes.len(),
        ),
    };

    iter.advance(len);
    Some(token)
}

/// Parse source code into [`TokenKind::Spaces`].
fn parse_spaces<Iter>(iter: &mut NextRangePeek<Iter>) -> Option<Token>
where
    Iter: Iterator<Item = SrcCode>,
{
    let (token, len) = match iter.peek_while(|c| c.value.is_whitespace()) {
        [] => return None,
        codes @ [c, ..] => (
            Token {
                value: TokenKind::Spaces,
                span: Span::new(c.span.start.clone(), codes.len()),
            },
            codes.len(),
        ),
    };

    iter.advance(len);
    Some(token)
}

/// Parse source code into [`TokenKind::Ident`].
fn parse_ident<Iter>(iter: &mut NextRangePeek<Iter>) -> Option<Token>
where
    Iter: Iterator<Item = SrcCode>,
{
    let (token, len) = match iter.peek_while(|c| {
        !c.value.is_whitespace() && !['(', ')', '{', '}', '\n', '\r'].contains(&c.value)
    }) {
        [] => return None,
        codes @ [c, ..] => (
            Token {
                value: TokenKind::Ident(codes.iter().map(|c| c.value).collect::<String>()),
                span: Span::new(c.span.start.clone(), codes.len()),
            },
            codes.len(),
        ),
    };

    iter.advance(len);
    Some(token)
}
