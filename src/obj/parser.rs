use obj::object::{ObjectSet, Object};
use lexer::Lexer;
use std::iter;


#[derive(Clone, Debug)]
struct ParseError {
    line_number: usize,
    message: String,
}

impl ParseError {
    fn new(line_number: usize, message: String) -> ParseError {
        ParseError {
            line_number: line_number,
            message: message,
        }
    }
}

struct Parser<'a> {
    line_number: usize,
    lexer: iter::Peekable<Lexer<'a>>,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Parser<'a> {
        Parser {
            line_number: 1,
            lexer: Lexer::new(input).peekable(),
        }
    }

    fn peek(&mut self) -> Option<String> {
        self.lexer.peek().map(|s| s.clone())
    }

    fn next(&mut self) -> Option<String> {
        let token = self.lexer.next();

        match token {
            Some(ref val) => {
                if val == "\n" {
                    self.line_number += 1;
                }
            },
            None => {},
        }

        token
    }

    fn error<T>(&mut self, err: String) -> Result<T, ParseError> {
        Err(ParseError::new(self.line_number, err))
    }

    fn parse_string(&mut self) -> Result<String, ParseError> {
        match self.next() {
            Some(st) => Ok(st),
            None => self.error(format!("Expected string but got `end of line`."))
        }
    }

    fn parse_float(&mut self) -> Result<f32, ParseError> {
        let st = try!(self.parse_string());

        match st.parse() {
            Ok(val) => Ok(val),
            Err(_) => self.error(
                format!("Expected floating point number but got: {}.", st)
            ),
        }
    }

    fn parse_number(&mut self) -> Result<i32, ParseError> {
        let st = try!(self.parse_string());

        match st.parse() {
            Ok(val) => Ok(val),
            Err(_) => self.error(format!("Expected integer but got: {}.", st)),
        }
    }
}

#[cfg(test)]
mod tests {
}