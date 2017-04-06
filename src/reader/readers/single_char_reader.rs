use lexer::{Input, State, Reader, TokenMeta};

use super::super::utils;
use super::super::token::{Token, TokenKind};


#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct SingleCharReader;

impl Reader<TokenKind> for SingleCharReader {

    #[inline(always)]
    fn priority(&self) -> usize { 2usize }

    fn read(&self, input: &Input, current: &State, next: &mut State) -> Option<Token> {
        let ch = input.read(next);

        let kind = match ch {
            '(' => Some(TokenKind::L_PAREN),
            ')' => Some(TokenKind::R_PAREN),
            _ => None,
        };

        if let Some(kind) = kind {
            let mut value = String::new();

            value.push(ch);

            Some(Token::new(
                TokenMeta::new_state_meta(current, next),
                kind,
                value
            ))
        } else {
            None
        }
    }
}
