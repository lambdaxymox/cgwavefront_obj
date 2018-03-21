use obj::object::{ObjectSet, Object, Vertex, TextureVertex, NormalVertex};
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
        match self.peek() {
            Some(st) => {
                match st.parse() {
                    Ok(w) => Ok(Vertex { x: x, y: y, z: z, w: w }),
                    Err(_) => Ok(Vertex { x: x, y: y, z: z, w: 1.0 }),
                }
            },
            None => {
                Ok(Vertex { x: x, y: y, z: z, w: 1.0 })
            }
        }
    }

    fn parse_texture_vertex(&mut self) -> Result<TextureVertex, ParseError> {
        let st = try!(self.parse_string());
        match st.as_ref() {
            "vt" => {},
            _ => { 
                return self.error(format!("Expected TextureVertex tag but got: {}.", st));
            }
        }

        let u = try!(self.parse_f32());
        match self.peek() {
            Some(st1) => {
                match st1.parse() {
                    Ok(v) => { 
                        self.next();
                        match self.peek() {
                            Some(st2) => {
                                let w = st2.parse().unwrap_or(0.0);
                                self.next();
                                Ok(TextureVertex { u: u, v: v, w: w })
                            }
                            None => Ok(TextureVertex { u: u, v: v, w: 0.0 }),
                        }
                    },
                    Err(_) => Ok(TextureVertex { u: u, v: 0.0, w: 0.0 }),
                }
            },
            None => Ok(TextureVertex { u: u, v: 0.0, w: 0.0 }),
        }
    }

    fn parse_normal_vertex(&mut self) -> Result<NormalVertex, ParseError> {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use obj::object::{ObjectSet, Object, TextureVertex, NormalVertex, Vertex};

    #[test]
    fn test_parse_f32() {
        let mut parser = super::Parser::new("-1.929448");
        assert_eq!(parser.parse_f32(), Ok(-1.929448));
    }

    #[test]
    fn test_parse_u32() {
        let mut parser = super::Parser::new("    763   ");
        assert_eq!(parser.parse_u32(), Ok(763));
    }

    #[test]
    fn test_parse_vertex1() {
        let mut parser = super::Parser::new("v -1.929448 13.329624 -5.221914\n");
        let vertex = Vertex { x: -1.929448, y: 13.329624, z: -5.221914, w: 1.0 };
        assert_eq!(parser.parse_vertex(), Ok(vertex));
    }

    #[test]
    fn test_parse_vertex2() {
        let mut parser = super::Parser::new("v -1.929448 13.329624 -5.221914 1.329624\n");
        let vertex = Vertex { x: -1.929448, y: 13.329624, z: -5.221914, w: 1.329624 };
        assert_eq!(parser.parse_vertex(), Ok(vertex));
    }

    #[test]
    fn test_parse_vertex3() {
        let mut parser = super::Parser::new("v -1.929448 13.329624 \n");
        assert!(parser.parse_vertex().is_err());
    }

    #[test]
    fn test_parse_vertex4() {
        let mut parser = super::Parser::new("v -1.929448 13.329624 -5.221914 1.329624\n v");
        assert!(parser.parse_vertex().is_ok());
    }

    #[test]
    fn test_parse_vertex5() {
        let mut parser = super::Parser::new(
            r"v -6.207583 1.699077 8.466142
              v -14.299248 1.700244 8.468981 1.329624"
        );
        assert_eq!(
            parser.parse_vertex(), 
            Ok(Vertex { x: -6.207583, y: 1.699077, z: 8.466142, w: 1.0 })
        );
        assert_eq!(parser.next(), Some(String::from("\n")));
        assert_eq!(
            parser.parse_vertex(), 
            Ok(Vertex { x: -14.299248, y: 1.700244, z: 8.468981, w: 1.329624 })
        );
    }

    #[test]
    fn test_parse_texture_vertex1() {
        let mut parser = super::Parser::new("vt -1.929448");
        let vt = TextureVertex { u: -1.929448, v: 0.0, w: 0.0 };
        assert_eq!(parser.parse_texture_vertex(), Ok(vt));
    }

    #[test]
    fn test_parse_texture_vertex2() {
        let mut parser = super::Parser::new("vt -1.929448 13.329624 -5.221914");
        let vt = TextureVertex { u: -1.929448, v: 13.329624, w: -5.221914 };
        assert_eq!(parser.parse_texture_vertex(), Ok(vt));
    }

    #[test]
    fn test_parse_texture_vertex3() {
        let mut parser = super::Parser::new(
            "vt -1.929448 13.329624 -5.221914
             vt -27.6068  31.1438    27.2099"
        );
        assert_eq!(
            parser.parse_texture_vertex(), 
            Ok(TextureVertex { u: -1.929448, v: 13.329624, w: -5.221914 })
        );
        assert_eq!(parser.next(), Some(String::from("\n")));
        assert_eq!(
            parser.parse_texture_vertex(),
            Ok(TextureVertex { u: -27.6068, v: 31.1438, w: 27.2099 })
        );
    }

    #[test]
    fn test_parse_normal_vertex1() {
        let mut parser = super::Parser::new("vn  -0.966742  -0.255752  9.97231e-09");
        let vn = NormalVertex { i: -0.966742, j: -0.255752, k: 9.97231e-09 };
        assert_eq!(parser.parse_normal_vertex(), Ok(vn));
    }

    #[test]
    fn test_parse_normal_vertex2() {
        let mut parser = super::Parser::new(
            "vn -1.929448 13.329624 -5.221914
             vn -27.6068  31.1438    27.2099"
        );
        assert_eq!(
            parser.parse_normal_vertex(), 
            Ok(NormalVertex { i: -1.929448, j: 13.329624, k: -5.221914 })
        );
        assert_eq!(parser.next(), Some(String::from("\n")));
        assert_eq!(
            parser.parse_normal_vertex(),
            Ok(NormalVertex { i: -27.6068, j: 31.1438, k: 27.2099 })
        );        
    }
}

