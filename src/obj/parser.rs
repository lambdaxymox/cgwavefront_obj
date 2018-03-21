use obj::object::{ObjectSet, Object, Vertex};
use lexer::Lexer;
use std::iter;


#[derive(Clone, Debug, PartialEq, Eq)]
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

    fn advance(&mut self) {
        self.next();
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

    fn parse_f32(&mut self) -> Result<f32, ParseError> {
        let st = try!(self.parse_string());
        match st.parse() {
            Ok(val) => Ok(val),
            Err(_) => self.error(
                format!("Expected floating point number but got: {}.", st)
            ),
        }
    }

    fn parse_u32(&mut self) -> Result<u32, ParseError> {
        let st = try!(self.parse_string());
        match st.parse() {
            Ok(val) => Ok(val),
            Err(_) => self.error(format!("Expected integer but got: {}.", st)),
        }
    }

    fn parse_vertex(&mut self) -> Result<Vertex, ParseError> {
        let st = try!(self.parse_string());
        match st.as_ref() {
            "v" => {},
            _ => { 
                return self.error(format!("Expected Vertex tag but got: {}.", st));
            }
        }

        let x = try!(self.parse_f32());
        let y = try!(self.parse_f32());
        let z = try!(self.parse_f32());

        Ok(Vertex { x: x, y: y, z: z, w: 1.0 })
    }
}

#[cfg(test)]
mod tests {
    use obj::object::{ObjectSet, Object, Vertex};

    #[test]
    fn test_parse_f32() {
        let mut parser = super::Parser::new("-1.929448");
        assert_eq!(parser.parse_f32(), Ok(-1.929448));
    }

    #[test]
    fn test_parse_vertex1() {
        let mut parser = super::Parser::new("v -1.929448 13.329624 -5.221914\n");
        let vertex = Vertex { x: -1.929448, y: 13.329624, z: -5.221914, w: 1.0 };
        assert_eq!(parser.parse_vertex(), Ok(vertex));
    }
}