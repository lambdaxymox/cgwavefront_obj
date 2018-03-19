use std::iter;
use std::str;


/// The return type from the lexer.
pub type Token = Vec<u8>;
 
#[inline]
fn is_whitespace(ch: u8) -> bool {
    ch == b' ' || ch == b'\\' || ch == b'\t'
}

#[inline]
fn is_newline(ch: u8) -> bool {
    ch == b'\n' || ch == b'\r'
}

#[inline]
fn is_whitespace_or_newline(ch: u8) -> bool {
    is_whitespace(ch) || is_newline(ch)
}

///
/// A OBJ file lexer tokenizes an input byte stream.
///
#[derive(Clone)]
pub struct Lexer<'a> {
    /// The current line position in the token stream.
    current_line_number: usize,
    /// The input stream.
    stream: iter::Peekable<str::Bytes<'a>>,
}

impl<'a> Lexer<'a> {
    /// Create a new lexer.
    pub fn new(stream: &str) -> Lexer {
        Lexer {
            current_line_number: 1,
            stream: stream.bytes().peekable(),
        }
    }

    ///
    /// The function `peek` looks at the character at the current position
    /// in the byte stream without advancing the stream.
    #[inline]
    fn peek(&mut self) -> Option<u8> {
        self.stream.peek().map(|&x| x)
    }

    ///
    /// the function `advance` advances the lexer by one
    /// character in the byte stream.
    ///
    fn advance(&mut self) {
        match self.stream.next() {
            Some(ch) if is_newline(ch) => {
                self.current_line_number += 1;
            },
            _ => {}
        }
    }

    ///
    /// The function `skip_comment` consumes a comment line
    /// without returning it.
    ///
    fn skip_comment(&mut self) -> usize {
        let mut skipped: usize = 0;
        loop {
            match self.peek() {
                Some(ch) => {
                    if is_newline(ch) {
                        break;
                    } else {
                        self.advance();
                        skipped += 1; 
                    }
                }
                None => {
                    break;
                }
            }
        }

        skipped
    }

    ///
    /// The function `skip_whitespace` consumes a string of whitespace
    /// characters without returning them.
    ///
    fn skip_whitespace(&mut self) -> usize {
        let mut skipped: usize = 0;
        loop {
            match self.peek() {
                Some(ch) if is_whitespace(ch) => {
                    self.advance();
                    skipped += 1;
                }
                _ => {
                    break;
                }
            }
        }

        skipped
    }

    ///
    /// The function `ship_newline` consumes one or more newline characters
    /// without returning them.
    ///
    fn skip_newline(&mut self) -> usize {
        let mut skipped = 0;
        loop {
            match self.peek() {
                Some(ch) if is_newline(ch) => {
                    self.advance();
                    skipped += 1;
                }
                _ => {
                    break;
                }
            }   
        }

        skipped
    }

    ///
    /// The method `next_token` fetches the next token from the input stream.
    ///
    fn next_token(&mut self) -> Option<Token> {
        // Count the number of bytes consumed for a token.
        let mut consumed: usize = 0;
        let mut token: Vec<u8> = Vec::new();
        loop {
            match self.peek() {
                Some(ch) if ch == b'#' => {
                    self.skip_comment();
                }
                Some(ch) if is_whitespace_or_newline(ch) => {
                    // If the cursor is pointing at a whitespace or newline character,
                    // there are two possible situations:
                    // (1) We are at the end of the token,
                    // (2) We have not encountered a token yet.
                    if consumed != 0 {
                        // We are at the end of a token.
                        break;
                    } else if is_newline(ch) {
                        // We are at the end of a line.
                        self.advance();
                        token.push(ch);
                        consumed += 1;
                        break;
                    } else {
                        // We have consumed only whitespace. No token has been found yet.
                        self.skip_whitespace();
                    }
                }
                Some(ch) => {
                    self.advance();
                    token.push(ch);
                    consumed += 1;
                }
                None => {
                    break;
                }
            }
        }

        if consumed != 0 {
            // We consumed a token.
            debug_assert!(token.len() != 0);
            Some(token)
        } else {
            debug_assert!(token.len() == 0);
            None
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token().map(|token| {
            match String::from_utf8(token) {
                Ok(st) => st,
                Err(_) => panic!(
                    "Lexical Error: Found invalid UTF-8 token on line {}.",
                    self.current_line_number
                )
            }
        })
    }
}
