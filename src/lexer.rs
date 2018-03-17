use std::iter;
use std::str;


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

pub struct Lexer<'a> {
    current_line_number: usize,
    stream: iter::Peekable<str::Bytes<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(stream: &str) -> Lexer {
        Lexer {
            current_line_number: 1,
            stream: stream.bytes().peekable(),
        }
    }

    #[inline]
    fn peek(&mut self) -> Option<u8> {
        self.stream.peek().map(|&x| x)
    }

    fn advance(&mut self) {
        match self.stream.next() {
            Some(ch) if is_newline(ch) => {
                self.current_line_number += 1;
            },
            _ => {}
        }
    }

    fn skip_comment(&mut self) -> usize {
        let mut skipped: usize = 0;
        loop {
            match self.peek() {
                Some(ch) if ch == b'#' => {
                    self.advance();
                    skipped += 1;
                }
                Some(ch) if is_newline(ch) => {
                    self.advance();
                    skipped += 1;
                    break;
                }
                _ => {
                    break;
                }
            }
        }

        skipped
    }

    fn skip_whitespace(&mut self) -> usize {
        let mut skipped: usize = 0;
        loop {
            match self.peek() {
                Some(ch) if is_whitespace_or_newline(ch) => {
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

    fn next_token(&mut self) -> Option<Token> {
        let mut consumed: usize = self.skip_whitespace();
        let mut token: Vec<u8> = Vec::new();
        loop {
            match self.peek() {
                Some(ch) if ch == b'#' => {
                    consumed += self.skip_comment();
                }
                Some(ch) if is_whitespace_or_newline(ch) => {
                    consumed += self.skip_whitespace();
                    break;
                }
                Some(ch) => {
                    token.push(ch);
                    self.advance();
                    consumed += 1;
                }
                None => {
                    break;
                }
            }
        }

        if consumed == 0 {
            None
        } else {
            Some(token)
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

