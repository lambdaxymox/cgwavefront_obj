use crate::lexer::{
    Lexer,
    ObjectLexer,
};
use std::error;
use std::fmt;


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Illumination {
    Ambient,
    AmbientDiffuse,
    AmbientDiffuseSpecular,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Material {
    pub name: String,
    pub color_ambient: Color,
    pub color_diffuse: Color,
    pub color_specular: Color,
    pub color_emissive: Color,
    pub specular_exponent: f64,
    pub dissolve: f64,
    pub optical_density: Option<f64>,
    pub illumination: Illumination,
    pub map_ambient: Option<String>,
    pub map_diffuse: Option<String>,
    pub map_specular: Option<String>,
    pub map_emissive: Option<String>,
    pub map_bump: Option<String>,
    pub map_displacement: Option<String>,
    pub map_dissolve: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MtlSet {
    pub materials: Vec<Material>,
}


#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ErrorKind {
    EndOfFile,
    ExpectedTag,
    ExpectedFloat,
    ExpectedInteger,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParseError {
    line_number: usize,
    kind: ErrorKind,
    message: String,
}

impl ParseError {
    fn new(
        line_number: usize, 
        kind: ErrorKind, 
        message: String) -> ParseError {
        
        ParseError {
            line_number: line_number,
            kind: kind,
            message: message,
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter, 
            "Parse error with kind {:?} at line {} with message \"{}\"", 
            self.kind, self.line_number, self.message
        )
    }
}

impl error::Error for ParseError {}

#[inline]
fn error<T>(
    line_number: usize, 
    kind: ErrorKind, 
    message: String) -> Result<T, ParseError> {
    
    Err(ParseError::new(line_number, kind, message))
}

/// A Wavefront MTL file parser.
pub struct Parser<'a> {
    line_number: usize,
    lexer: ObjectLexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Parser<'a> {
        Parser {
            line_number: 1,
            lexer: ObjectLexer::new(Lexer::new(input)),
        }
    }

    fn peek(&mut self) -> Option<&'a str> {
        self.lexer.peek()
    }

    fn next(&mut self) -> Option<&'a str> {
        let token = self.lexer.next();
        if let Some(val) = token {
            if val == "\n" {
                self.line_number += 1;
            }
        }

        token
    }

    fn advance(&mut self) {
        self.next();
    }

    fn next_string(&mut self) -> Result<&'a str, ParseError> {
        match self.next() {
            Some(st) => Ok(st),
            None => error(self.line_number, ErrorKind::EndOfFile, format!(""))
        }
    }

    fn expect_tag(&mut self, tag: &str) -> Result<(), ParseError> {
        match self.next() {
            None => error(self.line_number, ErrorKind::EndOfFile, format!("")),
            Some(st) if st != tag => error(
                self.line_number, 
                ErrorKind::ExpectedTag,
                format!("Expected statement {} but got statement {}", tag, st)
            ),
            _ => Ok(())
        }
    }

    fn skip_zero_or_more_newlines(&mut self) {
        while let Some("\n") = self.peek() {
            self.advance();
        }
    }

    fn skip_one_or_more_newlines(&mut self) -> Result<(), ParseError> {
        self.expect_tag("\n")?;
        self.skip_zero_or_more_newlines();
        Ok(())
    }

    fn parse_f64(&mut self) -> Result<f64, ParseError> {
        let st = self.next_string()?;
        match st.parse::<f64>() {
            Ok(val) => Ok(val),
            Err(_) => error(
                self.line_number, 
                ErrorKind::ExpectedFloat, 
                format!("Expected floating point number but got {}", st)
            ),
        }
    }

    fn parse_usize(&mut self) -> Result<usize, ParseError> {
        let st = self.next_string()?;
        match st.parse::<usize>() {
            Ok(val) => Ok(val),
            Err(_) => error(
                self.line_number, 
                ErrorKind::ExpectedInteger,
                format!("Expected integer but got {}", st)
            )
        }
    }

    fn parse_color(&mut self) -> Result<Color, ParseError> {
        let r = self.parse_f64()?;
        let g = self.parse_f64()?;
        let b = self.parse_f64()?;

        Ok(Color { 
            r: r, 
            g: g, 
            b: b
        })
    }

    fn parse_ambient_component(&mut self) -> Result<Color, ParseError> {
        self.expect_tag("Ka")?;
        self.parse_color()
    }

    fn parse_diffuse_component(&mut self) -> Result<Color, ParseError> {
        self.expect_tag("Kd")?;
        self.parse_color()
    }

    fn parse_specular_component(&mut self) -> Result<Color, ParseError> {
        self.expect_tag("Ks")?;
        self.parse_color()
    }

    fn parse_emissive_component(&mut self) -> Result<Color, ParseError> {
        self.expect_tag("Ke")?;
        self.parse_color()
    }

    fn parse_dissolve_component(&mut self) -> Result<f64, ParseError> {
        self.expect_tag("d")?;
        self.parse_f64()
    }

    fn parse_specular_exponent(&mut self) -> Result<f64, ParseError> {
        self.expect_tag("Ns")?;
        self.parse_f64()
    }

    fn parse_optical_density(&mut self) -> Result<f64, ParseError> {
        self.expect_tag("Ni")?;
        self.parse_f64()
    }
}


#[cfg(test)]
mod mtl_primitive_tests {
    use super::{
        Parser,
    };


    #[test]
    fn test_parse_f64() {
        let mut parser = Parser::new("-1.929448");
        assert_eq!(parser.parse_f64(), Ok(-1.929448));
    }

    #[test]
    fn test_parse_usize() {
        let mut parser = Parser::new("    763   ");
        assert_eq!(parser.parse_usize(), Ok(763));
    }
}

#[cfg(test)]
mod mtl_illumination_statement_tests {
    use super::{
        Color,
        Parser,
    };


    #[test]
    fn test_parse_ambient_component() {
        let mut parser = Parser::new("Ka 0.1345345 0.63453 0.982430");
        let expected = Ok(Color { r: 0.1345345, g: 0.63453, b: 0.982430 });
        let result = parser.parse_ambient_component();

        assert_eq!(result, expected);      
    }

    #[test]
    fn test_parse_diffuse_component() {
        let mut parser = Parser::new("Kd 0.1345345 0.63453 0.982430");
        let expected = Ok(Color { r: 0.1345345, g: 0.63453, b: 0.982430 });
        let result = parser.parse_diffuse_component();

        assert_eq!(result, expected);  
    }

    #[test]
    fn test_parse_specular_component() {
        let mut parser = Parser::new("Ks 0.1345345 0.63453 0.982430");
        let expected = Ok(Color { r: 0.1345345, g: 0.63453, b: 0.982430 });
        let result = parser.parse_specular_component();

        assert_eq!(result, expected);  
    }

    #[test]
    fn test_parse_emissive_component() {
        let mut parser = Parser::new("Ke 0.1345345 0.63453 0.982430");
        let expected = Ok(Color { r: 0.1345345, g: 0.63453, b: 0.982430 });
        let result = parser.parse_emissive_component();

        assert_eq!(result, expected); 
    }

    #[test]
    fn test_parse_dissolve_component() {
        let mut parser = Parser::new("d 0.24325634");
        let expected = Ok(0.24325634);
        let result = parser.parse_dissolve_component();

        assert_eq!(result, expected); 
    }

    #[test]
    fn test_parse_specular_exponent() {
        let mut parser = Parser::new("Ns 3.24325634");
        let expected = Ok(3.24325634);
        let result = parser.parse_specular_exponent();

        assert_eq!(result, expected); 
    }

    #[test]
    fn test_parse_optical_density() {
        let mut parser = Parser::new("Ni 1.24325634");
        let expected = Ok(1.24325634);
        let result = parser.parse_optical_density();

        assert_eq!(result, expected); 
    }
}

