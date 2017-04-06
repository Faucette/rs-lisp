use lexer::{Input, State, Reader, TokenMeta};

use super::super::utils;
use super::super::token::{Token, TokenKind};


#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct CommentReader;

impl Reader<TokenKind> for CommentReader {

    #[inline(always)]
    fn priority(&self) -> usize { 1usize }

    fn read(&self, input: &Input, current: &State, next: &mut State) -> Option<Token> {
        let ch = input.read(next);

        if ch == ';' && !input.done(next) {
            let mut comment = String::new();

            comment.push(ch);

            while !input.done(next) {
                let ch = input.peek(next, 0);

                if utils::is_newline(ch) {
                    break;
                } else {
                    input.read(next);
                    comment.push(ch);
                }
            }

            Some(Token::new(
                TokenMeta::new_state_meta(current, next),
                TokenKind::COMMENT,
                comment
            ))
        } else {
            None
        }
    }
}
