use lexer::{Input, State, Reader, TokenMeta};

use super::super::token::{Token, TokenKind};


#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct StringReader;

impl Reader<TokenKind> for StringReader {

    #[inline(always)]
    fn priority(&self) -> usize { 4usize }

    fn read(&self, input: &Input, current: &State, next: &mut State) -> Option<Token> {
        let ch = input.read(next);
        let quote = ch;

        if quote == '"' || quote == '\'' {
            let mut string = String::new();

            string.push(ch);

            while !input.done(next) {
                let ch = input.read(next);

                string.push(ch);

                if ch == quote {
                    break;
                }
            }

            Some(Token::new(
                TokenMeta::new_state_meta(current, next),
                if quote == '"' {
                    TokenKind::STRING
                } else {
                    TokenKind::CHAR
                },
                string
            ))
        } else {
            None
        }
    }
}
