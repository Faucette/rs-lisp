use lexer::{Input, State, Reader, TokenMeta};

use super::super::token::{Token, TokenKind};
use super::super::utils;


#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct IdentifierReader;

impl Reader<TokenKind> for IdentifierReader {

    #[inline(always)]
    fn priority(&self) -> usize { 3usize }

    fn read(&self, input: &Input, current: &State, next: &mut State) -> Option<Token> {
        let ch = input.read(next);

        if !utils::is_whitespace(ch) {
            let mut string = String::new();

            string.push(ch);

            while !input.done(next) {
                let ch = input.peek(next, 0);

                if !utils::is_whitespace(ch) {
                    input.read(next);
                    string.push(ch);
                } else {
                    break;
                }
            }


            let kind = match string.as_ref() {
                "true" => TokenKind::BOOL,
                "false" => TokenKind::BOOL,
                "nil" => TokenKind::NIL,
                _ => TokenKind::IDENTIFIER,
            };

            Some(Token::new(
                TokenMeta::new_state_meta(current, next),
                kind,
                string
            ))
        } else {
            None
        }
    }
}
