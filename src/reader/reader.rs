use std::convert::From;
use std::io::Read;

use lexer::{Input, Lexer};

use super::readers::*;
use super::token::{Token, TokenKind};


pub struct Reader<I: Input> {
    lexer: Lexer<TokenKind, I>,
}

impl<I: Input> Reader<I> {
    #[inline]
    fn new(mut lexer: Lexer<TokenKind, I>) -> Self {
        lexer.readers
            .add(CommentReader)
            .add(IdentifierReader)
            .add(NumberReader)
            .add(SingleCharReader)
            .add(StringReader)
            .add(WhitespaceReader)
            .sort();

        Reader {
            lexer: lexer,
        }
    }
}

impl<'a> From<&'a str> for Reader<Vec<char>> {
    #[inline(always)]
    fn from(value: &'a str) -> Self {
        Reader::new(From::from(value))
    }
}

impl<'a> From<&'a String> for Reader<Vec<char>> {
    #[inline(always)]
    fn from(value: &'a String) -> Self {
        From::from(value.as_str())
    }
}

impl<'a, R: Read> From<&'a mut R> for Reader<Vec<char>> {
    #[inline]
    fn from(value: &'a mut R) -> Self {
        let mut string = String::new();
        let _ = value.read_to_string(&mut string);
        From::from(&string)
    }
}

impl<I: Input> Iterator for Reader<I> {
    type Item = Token;


    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        self.lexer.next()
    }
}
