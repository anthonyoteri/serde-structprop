//! Lexer (tokenizer) for the structprop format.
//!
//! The lexer converts a raw `&str` into a flat sequence of [`Token`]s, stripping
//! comments and collapsing insignificant whitespace.  The resulting token stream
//! is consumed by [`crate::parse`].
//!
//! # Token rules
//!
//! | Input | Token produced |
//! |---|---|
//! | `=` | [`Token::Eq`] |
//! | `{` | [`Token::Open`] |
//! | `}` | [`Token::Close`] |
//! | `# … \n` | *(discarded)* |
//! | `"…"` | [`Token::Term`] with the quoted content |
//! | any other non-whitespace run | [`Token::Term`] |
//! | end of input | [`Token::Eof`] |

/// A single token produced by the structprop lexer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    /// A bare or double-quoted string term.
    ///
    /// Bare terms are delimited by whitespace or the special characters
    /// `=`, `{`, `}`, and `#`.  Quoted terms may contain any character
    /// except an unescaped `"`.
    Term(String),

    /// The assignment operator `=`.
    Eq,

    /// An opening brace `{` that begins an array or object body.
    Open,

    /// A closing brace `}` that ends an array or object body.
    Close,

    /// A sentinel placed at the end of the token stream.
    Eof,
}

/// Lex a structprop `input` string into a flat [`Vec`] of [`Token`]s.
///
/// Comments (`# … \n`) and insignificant whitespace (spaces, tabs, carriage
/// returns, and newlines) are discarded.  The returned vector always ends with
/// [`Token::Eof`].
///
/// # Examples
///
/// ```
/// use serde_structprop::lexer::{tokenize, Token};
///
/// let tokens = tokenize("key = value");
/// assert_eq!(tokens, vec![
///     Token::Term("key".into()),
///     Token::Eq,
///     Token::Term("value".into()),
///     Token::Eof,
/// ]);
/// ```
#[must_use]
pub fn tokenize(input: &str) -> Vec<Token> {
    /// Internal lexer state machine states.
    #[derive(PartialEq)]
    enum State {
        /// Between tokens; skipping whitespace.
        Whitespace,
        /// Inside a `# …` line comment.
        Comment,
        /// Accumulating a bare (unquoted) term.
        Term,
        /// Accumulating a double-quoted term.
        Quoted,
    }

    let mut tokens = Vec::new();
    let mut state = State::Whitespace;
    let mut buf = String::new();

    for ch in input.chars() {
        match state {
            State::Whitespace => match ch {
                '#' => state = State::Comment,
                '"' => state = State::Quoted,
                ' ' | '\t' | '\r' | '\n' => {}
                '=' => tokens.push(Token::Eq),
                '{' => tokens.push(Token::Open),
                '}' => tokens.push(Token::Close),
                _ => {
                    buf.push(ch);
                    state = State::Term;
                }
            },
            State::Quoted => {
                if ch == '"' {
                    tokens.push(Token::Term(buf.clone()));
                    buf.clear();
                    state = State::Whitespace;
                } else {
                    buf.push(ch);
                }
            }
            State::Comment => {
                if ch == '\n' {
                    state = State::Whitespace;
                }
            }
            State::Term => match ch {
                '#' | '\n' | ' ' | '\t' | '\r' => {
                    let term = buf.trim().to_owned();
                    if !term.is_empty() {
                        tokens.push(Token::Term(term));
                    }
                    buf.clear();
                    state = if ch == '#' {
                        State::Comment
                    } else {
                        State::Whitespace
                    };
                }
                '=' => {
                    let term = buf.trim().to_owned();
                    if !term.is_empty() {
                        tokens.push(Token::Term(term));
                    }
                    buf.clear();
                    tokens.push(Token::Eq);
                    state = State::Whitespace;
                }
                '{' => {
                    let term = buf.trim().to_owned();
                    if !term.is_empty() {
                        tokens.push(Token::Term(term));
                    }
                    buf.clear();
                    tokens.push(Token::Open);
                    state = State::Whitespace;
                }
                '}' => {
                    let term = buf.trim().to_owned();
                    if !term.is_empty() {
                        tokens.push(Token::Term(term));
                    }
                    buf.clear();
                    tokens.push(Token::Close);
                    state = State::Whitespace;
                }
                _ => buf.push(ch),
            },
        }
    }

    // Flush any term that extends to the very end of the input.
    if state == State::Term {
        let term = buf.trim().to_owned();
        if !term.is_empty() {
            tokens.push(Token::Term(term));
        }
    }

    tokens.push(Token::Eof);
    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_kv() {
        let toks = tokenize("key = value");
        assert_eq!(
            toks,
            vec![
                Token::Term("key".into()),
                Token::Eq,
                Token::Term("value".into()),
                Token::Eof,
            ]
        );
    }

    #[test]
    fn quoted_value() {
        let toks = tokenize(r#"key = "hello world""#);
        assert_eq!(
            toks,
            vec![
                Token::Term("key".into()),
                Token::Eq,
                Token::Term("hello world".into()),
                Token::Eof,
            ]
        );
    }

    #[test]
    fn comment_stripped() {
        let toks = tokenize("# comment\nkey = val");
        assert_eq!(
            toks,
            vec![
                Token::Term("key".into()),
                Token::Eq,
                Token::Term("val".into()),
                Token::Eof,
            ]
        );
    }

    #[test]
    fn array() {
        let toks = tokenize("k = { 1 2 3 }");
        assert_eq!(
            toks,
            vec![
                Token::Term("k".into()),
                Token::Eq,
                Token::Open,
                Token::Term("1".into()),
                Token::Term("2".into()),
                Token::Term("3".into()),
                Token::Close,
                Token::Eof,
            ]
        );
    }
}
