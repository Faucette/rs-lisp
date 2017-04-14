

#[inline(always)]
pub fn is_whitespace(ch: char) -> bool {
    ch.is_whitespace() || ch == ','
}

#[inline]
pub fn is_symbol_char(ch: char) -> bool {
    !is_whitespace(ch) &&
    ch != '\'' && ch != '"' &&
    ch != '`' && ch != ';' &&
    ch != '(' && ch != ')' &&
    ch != '{' && ch != '}' &&
    ch != '[' && ch != ']' &&
    ch != '|' && ch != '|'
}
