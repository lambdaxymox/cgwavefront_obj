use obj::object::{
    ObjectSet, Object, 
    Vertex, TextureVertex, NormalVertex,
    Element, VTNIndex,
};
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

    fn parse_statement(&mut self, tag: &str) -> Result<String, ParseError> {
        let st = try!(self.parse_string());
        match st == tag {
            true => Ok(st),
            false => self.error(format!("Expected `{}` statement but got: {}.", tag, st))
        }
    }

    fn parse_f32(&mut self) -> Result<f32, ParseError> {
        let st = try!(self.parse_string());
        match st.parse() {
            Ok(val) => Ok(val),
            Err(_) => self.error(
                format!("Expected `f32` but got `{}`.", st)
            ),
        }
    }

    fn parse_u32(&mut self) -> Result<u32, ParseError> {
        let st = try!(self.parse_string());
        match st.parse() {
            Ok(val) => Ok(val),
            Err(_) => self.error(format!("Expected integer but got `{}`.", st)),
        }
    }

    fn try_once<P, T>(&mut self, parser: P) -> Option<T> where P: FnOnce(&str) -> Option<T> {
        match self.peek() {
            Some(st) => {
                parser(&st).map(|got| { self.advance(); got })
            },
            None => None,
        }
    }

    fn parse_vertex(&mut self) -> Result<Vertex, ParseError> {
        try!(self.parse_statement("v"));
 
        let x = try!(self.parse_f32());
        let y = try!(self.parse_f32());
        let z = try!(self.parse_f32());
        let mw = self.try_once(|st| st.parse::<f32>().ok());
        let w = match mw {
            Some(val) => val,
            None => return Ok(Vertex { x: x, y: y, z: z, w: 1. })
        };

        Ok(Vertex { x: x, y: y, z: z, w: w })
    }

    fn parse_texture_vertex(&mut self) -> Result<TextureVertex, ParseError> {
        try!(self.parse_statement("vt"));

        let u = try!(self.parse_f32());
        let mv = self.try_once(|st| st.parse::<f32>().ok());
        let v = match mv {
            Some(val) => val,
            None => return Ok(TextureVertex { u: u, v: 0., w: 0. })
        };

        let mw = self.try_once(|st| st.parse::<f32>().ok());
        let w = match mw {
            Some(val) => val,
            None => return Ok(TextureVertex { u: u, v: v, w: 0. })
        };

        Ok(TextureVertex { u: u, v: v, w: w })
    }

    fn parse_normal_vertex(&mut self) -> Result<NormalVertex, ParseError> {
        try!(self.parse_statement("vn"));

        let i = try!(self.parse_f32());
        let j = try!(self.parse_f32());
        let k = try!(self.parse_f32());

        Ok(NormalVertex { i: i, j: j, k: k })
    }

    fn skip_zero_or_more_newlines(&mut self) {
        loop {
            match self.peek().as_ref().map(|st| &st[..]) {
                Some("\n") => {},
                Some(_) | None => break,
            }
            self.advance();
        }
    }

    fn skip_one_or_more_newlines(&mut self) -> Result<(), ParseError> {
        try!(self.parse_statement("\n"));
        self.skip_zero_or_more_newlines();
        Ok(())
    }

    fn parse_object_name(&mut self) -> Result<String, ParseError> {
        match self.peek().as_ref().map(|st| &st[..]) {
            Some("o") => {
                try!(self.parse_statement("o"));
                let object_name = self.parse_string();
                try!(self.skip_one_or_more_newlines());
                object_name
            }
            _ => Ok(String::from(""))
        }
    }

    fn parse_point(&mut self, elements: &mut Vec<Element>) -> Result<(), ParseError> {
        try!(self.parse_statement("p"));
        let v_index = try!(self.parse_u32());
        elements.push(Element::Point(VTNIndex::new(v_index as usize, None, None)));
        loop {
            match self.parse_string().as_ref().map(|st| &st[..]) {
                Ok("\n") | Err(_) => break,
                Ok(st) => match st.parse::<usize>() {
                    Ok(v_index) => elements.push(
                        Element::Point(VTNIndex::new(v_index, None, None))
                    ),
                    Err(_) => return self.error(format!("Expected integer but got `{}`.", st))
                }
            }
        }

        Ok(())
    }

    fn parse_line(&mut self, elements: &mut Vec<Element>) -> Result<(), ParseError> {
        unimplemented!();
    }

    fn parse_face(&mut self, elements: &mut Vec<Element>) -> Result<(), ParseError> {
        unimplemented!();
    }

    fn parse_element(&mut self) -> Result<Element, ParseError> {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use obj::object::{
        ObjectSet, Object, 
        TextureVertex, NormalVertex, Vertex,
        Element,
        VTNIndex
    };

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
             "v -6.207583 1.699077 8.466142
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

    #[test]
    fn test_parse_object_name1() {
        let mut parser = super::Parser::new("o object_name \n\n");
        assert_eq!(parser.parse_object_name(), Ok(String::from("object_name")));
    }

    #[test]
    fn test_parse_object_name2() {
        let mut parser = super::Parser::new("o object_name");
        assert!(parser.parse_object_name().is_err());
    }

    #[test]
    fn test_parse_point1() {
        let mut parser = super::Parser::new("p 1 2 3 4 \n");
        let mut result = Vec::new();
        parser.parse_point(&mut result).unwrap();
        let expected = vec![
            Element::Point(VTNIndex::new(1, None, None)), Element::Point(VTNIndex::new(2, None, None)),
            Element::Point(VTNIndex::new(3, None, None)), Element::Point(VTNIndex::new(4, None, None)),
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_line1() {
        let mut parser = super::Parser::new("l 297 38 118 108 \n");
        let mut result = Vec::new();
        parser.parse_line(&mut result).unwrap();
        let expected = vec![
            Element::Line(VTNIndex::new(297, None, None), VTNIndex::new(38,  None, None)), 
            Element::Line(VTNIndex::new(38,  None, None), VTNIndex::new(118, None, None)),
            Element::Line(VTNIndex::new(118, None, None), VTNIndex::new(4,   None, None)),
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_line2() {
        let mut parser = super::Parser::new("l 297/38 118/108 \n");
        let mut result = Vec::new();
        parser.parse_line(&mut result).unwrap();
        let expected = vec![
            Element::Line(VTNIndex::new(297, Some(38), None), VTNIndex::new(118, Some(108), None)),
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_object_set1() {
        let obj_file = r"
            o object1
            g cube
            v  0.0  0.0  0.0
            v  0.0  0.0  1.0
            v  0.0  1.0  0.0
            v  0.0  1.0  1.0
            v  1.0  0.0  0.0
            v  1.0  0.0  1.0
            v  1.0  1.0  0.0
            v  1.0  1.0  1.0

            vn  0.0  0.0  1.0
            vn  0.0  0.0 -1.0
            vn  0.0  1.0  0.0
            vn  0.0 -1.0  0.0
            vn  1.0  0.0  0.0
            vn -1.0  0.0  0.0
 
            f  1//2  7//2  5//2
            f  1//2  3//2  7//2 
            f  1//6  4//6  3//6
            f  1//6  2//6  4//6 
            f  3//3  8//3  7//3 
            f  3//3  4//3  8//3 
            f  5//5  7//5  8//5 
            f  5//5  8//5  6//5 
            f  1//4  5//4  6//4 
            f  1//4  6//4  2//4 
            f  2//1  6//1  8//1 
            f  2//1  8//1  4//1 
        ";
        assert!(false);
    }
}

