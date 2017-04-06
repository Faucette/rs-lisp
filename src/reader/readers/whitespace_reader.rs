use lexer::{Input, State, Reader, TokenMeta};

use super::super::utils;
use super::super::token::{Token, TokenKind};


#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct WhitespaceReader;

impl Reader<TokenKind> for WhitespaceReader {

    #[inline(always)]
    fn priority(&self) -> usize { 0usize }

    fn read(&self, input: &Input, current: &State, next: &mut State) -> Option<Token> {
        let ch = input.read(next);

        if utils::is_whitespace(ch) {
            let mut string = String::new();

            string.push(ch);

            while !input.done(next) {
                let ch = input.peek(next, 0);

                if utils::is_whitespace(ch) {
                    input.read(next);
                    string.push(ch);
                } else {
                    break;
                }
            }

            Some(Token::new(
                TokenMeta::new_state_meta(current, next),
                TokenKind::WHITESPACE,
                string
            ))
        } else {
            None
        }
    }
}
