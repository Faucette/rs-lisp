// Whitespace is defined as any unicode whitespace character,
// a comma, or a newline
#[inline(always)]
pub fn is_whitespace(ch: char) -> bool {
    ch.is_whitespace() || ch == ','
}

#[inline(always)]
pub fn is_newline(ch: char) -> bool {
    ch == '\n'
}
