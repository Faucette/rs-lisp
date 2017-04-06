use lexer::{Input, State, Reader, TokenMeta};

use super::super::token::{Token, TokenKind};


#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct IdentifierReader;

impl Reader<TokenKind> for IdentifierReader {

    #[inline(always)]
    fn priority(&self) -> usize { 3usize }

    fn read(&self, input: &Input, current: &State, next: &mut State) -> Option<Token> {
        let ch = input.read(next);

        if ch.is_alphabetic() {
            let mut string = String::new();

            string.push(ch);

            while !input.done(next) {
                let ch = input.peek(next, 0);

                if ch.is_alphanumeric() {
                    input.read(next);
                    string.push(ch);
                } else {
                    break;
                }
            }


            let kind = match string.as_ref() {
                "true" => TokenKind::BOOL,
                "false" => TokenKind::BOOL,
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
