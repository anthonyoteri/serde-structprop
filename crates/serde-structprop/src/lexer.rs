//! Lexer (tokenizer) for the structprop format.
//!
//! The lexer converts a raw `&str` into a flat sequence of [`Token`]s paired
//! with their 1-indexed source line numbers.  Comments and insignificant
//! whitespace are stripped.  The resulting token stream is consumed by
//! [`crate::parse()`].
//!
//! # Token rules
//!
//! | Input | Token produced |
//! |---|---|
//! | `=` | `Token::Eq` |
//! | `{` | `Token::Open` |
//! | `}` | `Token::Close` |
//! | `# … \n` | *(discarded)* |
//! | `"…"` | `Token::Term` with the quoted content |
//! | any other non-whitespace run | `Token::Term` |
//! | end of input | `Token::Eof` |

/// A single token produced by the structprop lexer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    /// A bare or double-quoted string term.
    ///
    /// Bare terms are delimited by whitespace or the special characters
    /// `=`, `{`, `}`, and `#`.  Quoted terms may contain any character
    /// except `"` — the format has no escape sequences, so a `"` always terminates the quoted string.
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

/// Lex a structprop `input` string into a flat [`Vec`] of [`Token`]s, each
/// paired with its 1-indexed source line number.
///
/// Comments (`# … \n`) and insignificant whitespace (spaces, tabs, carriage
/// returns, and newlines) are discarded.  The returned vector always ends with
/// [`Token::Eof`].
///
/// # Errors
///
/// Returns [`crate::Error::Parse`] if the input contains [`u32::MAX`] or
/// more newlines (i.e. the file exceeds [`u32::MAX`] lines).
///
/// # Examples
///
/// ```
/// use serde_structprop::lexer::{tokenize, Token};
///
/// let tokens = tokenize("key = value").unwrap();
/// assert_eq!(tokens, vec![
///     (Token::Term("key".into()), 1),
///     (Token::Eq, 1),
///     (Token::Term("value".into()), 1),
///     (Token::Eof, 1),
/// ]);
/// ```
pub fn tokenize(input: &str) -> crate::error::Result<Vec<(Token, u32)>> {
    let mut tokens = Vec::new();
    let mut state = State::Whitespace;
    let mut buf = String::new();
    let mut line = 1u32;
    let mut token_line = 1u32;

    for ch in input.chars() {
        match state {
            State::Whitespace => match ch {
                '\n' => line = inc_line(line)?,
                ' ' | '\t' | '\r' => {}
                '#' => state = State::Comment,
                '"' => {
                    token_line = line;
                    state = State::Quoted;
                }
                '=' => tokens.push((Token::Eq, line)),
                '{' => tokens.push((Token::Open, line)),
                '}' => tokens.push((Token::Close, line)),
                _ => {
                    token_line = line;
                    buf.push(ch);
                    state = State::Term;
                }
            },
            State::Quoted => {
                if ch == '"' {
                    tokens.push((Token::Term(buf.clone()), token_line));
                    buf.clear();
                    state = State::Whitespace;
                } else {
                    if ch == '\n' {
                        line = inc_line(line)?;
                    }
                    buf.push(ch);
                }
            }
            State::Comment => {
                if ch == '\n' {
                    line = inc_line(line)?;
                    state = State::Whitespace;
                }
            }
            State::Term => {
                flush_term_char(
                    ch,
                    &mut buf,
                    &mut tokens,
                    &mut line,
                    &mut token_line,
                    &mut state,
                )?;
            }
        }
    }

    // Flush any term that extends to the very end of the input.
    if state == State::Term {
        let term = buf.trim().to_owned();
        if !term.is_empty() {
            tokens.push((Token::Term(term), token_line));
        }
    }

    tokens.push((Token::Eof, line));
    Ok(tokens)
}

/// Increment a line counter, returning an error if it would overflow.
fn inc_line(line: u32) -> crate::error::Result<u32> {
    line.checked_add(1).ok_or_else(|| {
        crate::error::Error::Parse("file exceeds maximum line count (u32::MAX)".to_owned())
    })
}

/// Handle one character while in the `Term` state, flushing the accumulated
/// buffer and emitting punctuation tokens as needed.
fn flush_term_char(
    ch: char,
    buf: &mut String,
    tokens: &mut Vec<(Token, u32)>,
    line: &mut u32,
    token_line: &mut u32,
    state: &mut State,
) -> crate::error::Result<()> {
    match ch {
        '\n' => {
            flush_buf(buf, tokens, *token_line);
            *line = inc_line(*line)?;
            *state = State::Whitespace;
        }
        '#' | ' ' | '\t' | '\r' => {
            flush_buf(buf, tokens, *token_line);
            *state = if ch == '#' {
                State::Comment
            } else {
                State::Whitespace
            };
        }
        '=' => {
            flush_buf(buf, tokens, *token_line);
            tokens.push((Token::Eq, *line));
            *state = State::Whitespace;
        }
        '{' => {
            flush_buf(buf, tokens, *token_line);
            tokens.push((Token::Open, *line));
            *state = State::Whitespace;
        }
        '}' => {
            flush_buf(buf, tokens, *token_line);
            tokens.push((Token::Close, *line));
            *state = State::Whitespace;
        }
        _ => buf.push(ch),
    }
    Ok(())
}

/// Drain `buf` into a `Token::Term` if non-empty.
fn flush_buf(buf: &mut String, tokens: &mut Vec<(Token, u32)>, token_line: u32) {
    let term = buf.trim().to_owned();
    if !term.is_empty() {
        tokens.push((Token::Term(term), token_line));
    }
    buf.clear();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_kv() {
        let toks = tokenize("key = value").unwrap();
        assert_eq!(
            toks,
            vec![
                (Token::Term("key".into()), 1),
                (Token::Eq, 1),
                (Token::Term("value".into()), 1),
                (Token::Eof, 1),
            ]
        );
    }

    #[test]
    fn quoted_value() {
        let toks = tokenize(r#"key = "hello world""#).unwrap();
        assert_eq!(
            toks,
            vec![
                (Token::Term("key".into()), 1),
                (Token::Eq, 1),
                (Token::Term("hello world".into()), 1),
                (Token::Eof, 1),
            ]
        );
    }

    #[test]
    fn comment_stripped() {
        let toks = tokenize("# comment\nkey = val").unwrap();
        assert_eq!(
            toks,
            vec![
                (Token::Term("key".into()), 2),
                (Token::Eq, 2),
                (Token::Term("val".into()), 2),
                (Token::Eof, 2),
            ]
        );
    }

    #[test]
    fn array() {
        let toks = tokenize("k = { 1 2 3 }").unwrap();
        assert_eq!(
            toks,
            vec![
                (Token::Term("k".into()), 1),
                (Token::Eq, 1),
                (Token::Open, 1),
                (Token::Term("1".into()), 1),
                (Token::Term("2".into()), 1),
                (Token::Term("3".into()), 1),
                (Token::Close, 1),
                (Token::Eof, 1),
            ]
        );
    }

    #[test]
    fn multiline_line_numbers() {
        let toks = tokenize("a = 1\nb = 2\nc = 3\n").unwrap();
        assert_eq!(
            toks,
            vec![
                (Token::Term("a".into()), 1),
                (Token::Eq, 1),
                (Token::Term("1".into()), 1),
                (Token::Term("b".into()), 2),
                (Token::Eq, 2),
                (Token::Term("2".into()), 2),
                (Token::Term("c".into()), 3),
                (Token::Eq, 3),
                (Token::Term("3".into()), 3),
                (Token::Eof, 4),
            ]
        );
    }

    #[test]
    fn line_overflow_returns_error() {
        // Build a string with u32::MAX newlines — too large to actually
        // allocate, so we test with a saturated counter by constructing a
        // minimal reproduction using inc_line directly.
        assert!(inc_line(u32::MAX).is_err());
    }
}
