use std::iter;
use std::str;


struct Lexer {
    current_line_number: usize,
    stream: iter::Peakable<iter::Bytes>,
}

impl Lexer {
    fn new(stream: &str) -> Lexer {
        Lexer {
            current_line_number: 1,
            stream: stream.bytes().peekable(),
        }
    }
}