use lexer::{Input, State, Reader, TokenMeta};

use super::super::token::{Token, TokenKind};


#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct NumberReader;

impl Reader<TokenKind> for NumberReader {

    #[inline(always)]
    fn priority(&self) -> usize { 5usize }

    fn read(&self, input: &Input, current: &State, next: &mut State) -> Option<Token> {
        let ch = input.read(next);

        if ch.is_numeric() {
            let mut string = String::new();

            string.push(ch);

            while !input.done(next) {
                let ch = input.peek(next, 0);

                if ch.is_numeric() {
                    input.read(next);
                    string.push(ch);
                } else {
                    break;
                }
            }

            Some(Token::new(
                TokenMeta::new_state_meta(current, next),
                TokenKind::NUMBER,
                string
            ))
        } else {
            None
        }
    }
}
