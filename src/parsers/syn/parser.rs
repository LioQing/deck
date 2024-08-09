use crate::parsers::Token;
use crate::utils::NextRangePeek;
// /// Syntactic parser.
// ///
// /// This module uses a different parsing strategy than other modules.
// /// This is because syntactic parsing is a bit more complex, so
// /// using a separate struct for each parser is more flexible.
// use paste::paste;
// use std::marker::PhantomData;
//
// use super::*;
// use crate::{SemParser, Span, Token, TokenKind};
use super::*;

/// Syntactic parser
#[derive(Debug, Clone)]
pub struct SynParser<Iter>
where
    Iter: Iterator<Item = Token>,
{
    iter: NextRangePeek<Iter>,
}

impl<Iter> SynParser<Iter>
where
    Iter: Iterator<Item = Token>,
{
    /// Create a new syntactic parser.
    pub fn new(iter: NextRangePeek<Iter>) -> Self {
        Self { iter }
    }
}

impl<Iter> Iterator for SynParser<Iter>
where
    Iter: Iterator<Item = Token>,
{
    type Item = SynNode;

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!();
    }
}

/// Parse open bracket.
fn parse_open_brac<Iter>(iter: &mut NextRangePeek<Iter>) -> Option<Token>
where
    Iter: Iterator<Item = Token>,
{
    let token = match iter.peek(1) {
        [t] if t.value.is_open_brac() => iter.next().unwrap(),
        _ => return None,
    };

    iter.advance(1);
    Some(token)
}

// /// Syntactic parser.
// #[derive(Debug, Clone)]
// pub struct SynParser {
//     tokens: Vec<Token>,
// }
//
// impl SynParser {
//     /// Create a new syntactic parser.
//     pub fn new(tokens: Vec<Token>) -> Self {
//         Self { tokens }
//     }
//
//     /// Parse the program.
//     pub fn parse<'a>(&'a self) -> Vec<SynNode> {
//         ProgramParser::<'a>::with_tokens(&self.tokens)
//             .next()
//             .map_or(Vec::new(), |x| x.item)
//     }
//
//     /// Parse semantic.
//     pub fn parse_sem(&self) -> SemParser<std::vec::IntoIter<SynNode>> {
//         SemParser::new(self.parse().into_iter())
//     }
// }
//
// /// Syntactic parser result.
// #[derive(Debug, PartialEq, Eq, Clone, Hash)]
// struct SynParserResult<'a, Item> {
//     pub item: Item,
//     pub tokens: &'a [Token],
// }
//
// /// Syntactic parser iterator trait.
// trait SynParserIter<'a, Item>: Iterator<Item = SynParserResult<'a, Item>> {
//     /// Supply the tokens to the parser.
//     fn with_tokens(tokens: &'a [Token]) -> Self;
// }
//
// macro_rules! token_parser {
//     ($name:ident, $patt:pat) => {
//         paste! {
//             /// Token parser.
//             #[derive(Debug, Clone)]
//             struct [< $name TokenParser >]<'a> {
//                 tokens: &'a [Token],
//             }
//
//             impl<'a> SynParserIter<'a, Token> for [< $name TokenParser >]<'a> {
//                 fn with_tokens(tokens: &'a [Token]) -> Self {
//                     Self { tokens }
//                 }
//             }
//
//             impl<'a> Iterator for [< $name TokenParser >]<'a> {
//                 type Item = SynParserResult<'a, Token>;
//
//                 fn next(&mut self) -> Option<Self::Item> {
//                     let token = self.tokens.get(0)?;
//                     if let TokenKind::$patt = token.value {
//                         Some(SynParserResult {
//                             item: token.clone(),
//                             tokens: &self.tokens[..1],
//                         })
//                     } else {
//                         None
//                     }
//                 }
//             }
//         }
//     };
// }
//
// token_parser!(OpenBrac, OpenBrac(_));
// token_parser!(CloseBrac, CloseBrac(_));
// token_parser!(Ident, Ident(_));
// token_parser!(Spaces, Spaces);
// token_parser!(Newlines, Newlines);
//
// /// Many parser
// #[derive(Debug, Clone)]
// struct ManyParser<'a, Parser, Item>
// where
//     Parser: SynParserIter<'a, Item>,
// {
//     tokens: &'a [Token],
//     parser: PhantomData<Parser>,
//     item: PhantomData<Item>,
// }
//
// impl<'a, Parser, Item> SynParserIter<'a, Vec<Item>> for ManyParser<'a, Parser, Item>
// where
//     Parser: SynParserIter<'a, Item>,
// {
//     fn with_tokens(tokens: &'a [Token]) -> Self {
//         Self {
//             tokens,
//             parser: PhantomData,
//             item: PhantomData,
//         }
//     }
// }
//
// impl<'a, Parser, Item> Iterator for ManyParser<'a, Parser, Item>
// where
//     Parser: SynParserIter<'a, Item>,
// {
//     type Item = SynParserResult<'a, Vec<Item>>;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         let mut v = Vec::new();
//         let mut count = 0;
//
//         while let Some(x) = Parser::with_tokens(&self.tokens[count..]).next() {
//             v.push(x.item);
//             count += x.tokens.len();
//         }
//
//         Some(SynParserResult {
//             item: v,
//             tokens: &self.tokens[..count],
//         })
//     }
// }
//
// /// Many1 parser
// #[derive(Debug, Clone)]
// struct Many1Parser<'a, Parser, Item>
// where
//     Parser: SynParserIter<'a, Item>,
// {
//     tokens: &'a [Token],
//     parser: PhantomData<Parser>,
//     item: PhantomData<Item>,
// }
//
// impl<'a, Parser, Item> SynParserIter<'a, Vec<Item>> for Many1Parser<'a, Parser, Item>
// where
//     Parser: SynParserIter<'a, Item>,
// {
//     fn with_tokens(tokens: &'a [Token]) -> Self {
//         Self {
//             tokens,
//             parser: PhantomData,
//             item: PhantomData,
//         }
//     }
// }
//
// impl<'a, Parser, Item> Iterator for Many1Parser<'a, Parser, Item>
// where
//     Parser: SynParserIter<'a, Item>,
// {
//     type Item = SynParserResult<'a, Vec<Item>>;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         let x = Parser::with_tokens(self.tokens).next()?;
//         let mut v = vec![x.item];
//         let mut count = x.tokens.len();
//
//         while let Some(y) = Parser::with_tokens(&self.tokens[count..]).next() {
//             v.push(y.item);
//             count += y.tokens.len();
//         }
//
//         Some(SynParserResult {
//             item: v,
//             tokens: &self.tokens[..count],
//         })
//     }
// }
//
// /// brackets parser.
// #[derive(Debug, Clone)]
// struct BracParser<'a, NodeParser>
// where
//     NodeParser: SynParserIter<'a, SynNode>,
// {
//     tokens: &'a [Token],
//     node_parser: PhantomData<NodeParser>,
// }
//
// impl<'a, NodeParser> SynParserIter<'a, SynNode> for BracParser<'a, NodeParser>
// where
//     NodeParser: SynParserIter<'a, SynNode>,
// {
//     fn with_tokens(tokens: &'a [Token]) -> Self {
//         Self {
//             tokens,
//             node_parser: PhantomData,
//         }
//     }
// }
//
// impl<'a, NodeParser> Iterator for BracParser<'a, NodeParser>
// where
//     NodeParser: SynParserIter<'a, SynNode>,
// {
//     type Item = SynParserResult<'a, SynNode>;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         // Open bracket
//         let open_brac = OpenBracTokenParser::with_tokens(self.tokens).next()?;
//
//         // Items
//         let items_start = open_brac.tokens.len();
//         let items = ManyParser::<'a, NodeParser, SynNode>::with_tokens(&self.tokens[items_start..])
//             .next()?;
//         let items_span_len = items.item.iter().fold(0, |acc, x| acc + x.span.len);
//
//         // Close bracket
//         let items_end = items_start + items.tokens.len();
//         let close_brac = CloseBracTokenParser::with_tokens(&self.tokens[items_end..]).next()?;
//
//         fn match_brac(open: char, close: char) -> bool {
//             match (open, close) {
//                 ('(', ')') | ('{', '}') => true,
//                 _ => false,
//             }
//         }
//
//         let value = match (open_brac.item.value, close_brac.item.value) {
//             (TokenKind::OpenBrac(open), TokenKind::CloseBrac(close)) if match_brac(open, close) => {
//                 SynNodeKind::Brac {
//                     open,
//                     close,
//                     children: items.item,
//                 }
//             }
//             (TokenKind::OpenBrac(open), TokenKind::CloseBrac(close)) => SynNodeKind::Error {
//                 msg: format!("Mismatched brackets: '{}' and '{}'", open, close),
//                 children: items.item,
//             },
//             _ => unreachable!(),
//         };
//         let end = items_end + close_brac.tokens.len();
//
//         Some(SynParserResult {
//             item: SynNode {
//                 value,
//                 span: Span {
//                     start: open_brac.item.span.start,
//                     len: open_brac.item.span.len + items_span_len + close_brac.item.span.len,
//                 },
//             },
//             tokens: &self.tokens[..end],
//         })
//     }
// }
//
// /// Idents parser.
// #[derive(Debug, Clone)]
// struct IdentParser<'a> {
//     tokens: &'a [Token],
// }
//
// impl<'a> SynParserIter<'a, SynNode> for IdentParser<'a> {
//     fn with_tokens(tokens: &'a [Token]) -> Self {
//         Self { tokens }
//     }
// }
//
// impl<'a> Iterator for IdentParser<'a> {
//     type Item = SynParserResult<'a, SynNode>;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         let token = IdentTokenParser::with_tokens(self.tokens).next()?;
//         let Token {
//             value: TokenKind::Ident(ident),
//             ..
//         } = token.item
//         else {
//             unreachable!()
//         };
//         Some(SynParserResult {
//             item: SynNode {
//                 value: SynNodeKind::Ident(ident),
//                 span: token.item.span,
//             },
//             tokens: token.tokens,
//         })
//     }
// }
//
// /// Syntactic node parser.
// #[derive(Debug, Clone)]
// struct SynNodeParser<'a> {
//     tokens: &'a [Token],
// }
//
// impl<'a> SynParserIter<'a, SynNode> for SynNodeParser<'a> {
//     fn with_tokens(tokens: &'a [Token]) -> Self {
//         Self { tokens }
//     }
// }
//
// impl<'a> Iterator for SynNodeParser<'a> {
//     type Item = SynParserResult<'a, SynNode>;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         if let Some(x) = IdentParser::with_tokens(self.tokens).next() {
//             Some(x)
//         } else if let Some(x) = BracParser::<'a, SynNodeParser>::with_tokens(self.tokens).next() {
//             Some(x)
//         } else {
//             None
//         }
//     }
// }
//
// /// Program parser.
// #[derive(Debug, Clone)]
// struct ProgramParser<'a> {
//     tokens: &'a [Token],
// }
//
// impl<'a> SynParserIter<'a, Vec<SynNode>> for ProgramParser<'a> {
//     fn with_tokens(tokens: &'a [Token]) -> Self {
//         Self { tokens }
//     }
// }
//
// impl<'a> Iterator for ProgramParser<'a> {
//     type Item = SynParserResult<'a, Vec<SynNode>>;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         ManyParser::<'a, SynNodeParser, SynNode>::with_tokens(self.tokens).next()
//     }
// }
