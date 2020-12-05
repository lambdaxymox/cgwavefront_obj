use crate::lexer::{
    Tokenizer,
    Lexer,
};
use std::error;
use std::fmt;
use std::slice;
use std::ops;


pub fn parse<T: AsRef<str>>(input: T) -> Result<MaterialSet, ParseError> {
    Parser::new(input.as_ref()).parse_mtlset()
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64
}

impl Color {
    #[inline]
    const fn new(r: f64, g: f64, b: f64) -> Color {
        Color { 
            r: r,
            g: g,
            b: b
        }
    }

    #[inline]
    const fn zero() -> Color {
        Color {
            r: 0_f64,
            g: 0_f64,
            b: 0_f64
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum IlluminationModel {
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
    pub illumination_model: IlluminationModel,
    pub map_ambient: Option<String>,
    pub map_diffuse: Option<String>,
    pub map_specular: Option<String>,
    pub map_emissive: Option<String>,
    pub map_specular_exponent: Option<String>,
    pub map_bump: Option<String>,
    pub map_displacement: Option<String>,
    pub map_dissolve: Option<String>,
}

impl Material {
    fn new() -> Material {
        Material {
            name: String::new(),
            color_ambient: Color::zero(),
            color_diffuse: Color::zero(),
            color_specular: Color::zero(),
            color_emissive: Color::zero(),
            specular_exponent: 0_f64,
            dissolve: 0_f64,
            optical_density: None,
            illumination_model: IlluminationModel::AmbientDiffuseSpecular,
            map_ambient: None,
            map_diffuse: None,
            map_specular: None,
            map_emissive: None,
            map_specular_exponent: None,
            map_bump: None,
            map_displacement: None,
            map_dissolve: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct MaterialSet {
    materials: Vec<Material>,
}

impl MaterialSet {
    pub fn new(materials: Vec<Material>) -> MaterialSet {
        MaterialSet {
            materials: materials,
        }    
    }

    pub fn iter(&self) -> MaterialSetIter {
        MaterialSetIter {
            inner: self.materials.iter(),
        }
    }

    pub fn len(&self) -> usize { 
        self.materials.len()
    }
}

pub struct MaterialSetIter<'a> {
    inner: slice::Iter<'a, Material>,   
}

impl<'a> Iterator for MaterialSetIter<'a> {
    type Item = &'a Material;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl ops::Index<usize> for MaterialSet {
    type Output = Material;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.materials[index]
    }
}


#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ErrorKind {
    EndOfFile,
    ExpectedTag,
    ExpectedFloat,
    ExpectedInteger,
    ExpectedEndOfInput,
    UnknownIlluminationModel,
    ErrorParsingMaterial,
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
    lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Parser<'a> {
        Parser {
            line_number: 1,
            lexer: Lexer::new(Tokenizer::new(input)),
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

        Ok(Color::new(r, g, b))
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

    fn parse_map_ambient(&mut self) -> Result<Option<&'a str>, ParseError> {
        match self.peek() {
            Some("map_Ka") => {}
            _ => return Ok(None)
        }

        self.expect_tag("map_Ka")?;
        match self.next() {
            Some(st) => Ok(Some(st)),
            None => error(
                self.line_number, 
                ErrorKind::EndOfFile, 
                format!("Expected texture map name but got end of input.")
            ),
        }
    }

    fn parse_map_diffuse(&mut self) -> Result<Option<&'a str>, ParseError> {
        match self.peek() {
            Some("map_Kd") => {}
            _ => return Ok(None)
        }

        self.expect_tag("map_Kd")?;
        match self.next() {
            Some(st) => Ok(Some(st)),
            None => error(
                self.line_number, 
                ErrorKind::EndOfFile, 
                format!("Expected texture map name but got end of input.")
            ),
        }
    }

    fn parse_map_specular(&mut self) -> Result<Option<&'a str>, ParseError> {
        match self.peek() {
            Some("map_Ks") => {}
            _ => return Ok(None)
        }

        self.expect_tag("map_Ks")?;
        match self.next() {
            Some(st) => Ok(Some(st)),
            None => error(
                self.line_number, 
                ErrorKind::EndOfFile, 
                format!("Expected texture map name but got end of input.")
            ),
        }
    }

    fn parse_map_emissive(&mut self) -> Result<Option<&'a str>, ParseError> {
        match self.peek() {
            Some("map_Ke") => {}
            _ => return Ok(None)
        }

        self.expect_tag("map_Ke")?;
        match self.next() {
            Some(st) => Ok(Some(st)),
            None => error(
                self.line_number, 
                ErrorKind::EndOfFile, 
                format!("Expected texture map name but got end of input.")
            ),
        }
    }

    fn parse_map_bump(&mut self) -> Result<Option<&'a str>, ParseError> {
        match self.peek() {
            Some("map_Bump") => {
                self.expect_tag("map_Bump")?;
            }
            Some("bump") => {
                self.expect_tag("bump")?;
            }
            _ => return Ok(None)
        }

        match self.next() {
            Some(st) => Ok(Some(st)),
            None => error(
                self.line_number, 
                ErrorKind::EndOfFile, 
                format!("Expected texture map name but got end of input.")
            ),
        }
    }

    fn parse_map_displacement(&mut self) -> Result<Option<&'a str>, ParseError> {
        match self.peek() {
            Some("disp") => {}
            _ => return Ok(None)
        }

        self.expect_tag("disp")?;
        match self.next() {
            Some(st) => Ok(Some(st)),
            None => error(
                self.line_number, 
                ErrorKind::EndOfFile, 
                format!("Expected texture map name but got end of input.")
            ),
        }
    }

    fn parse_map_dissolve(&mut self) -> Result<Option<&'a str>, ParseError> {
        match self.peek() {
            Some("map_d") => {}
            _ => return Ok(None)
        }

        self.expect_tag("map_d")?;
        match self.next() {
            Some(st) => Ok(Some(st)),
            None => error(
                self.line_number, 
                ErrorKind::EndOfFile, 
                format!("Expected texture map name but got end of input.")
            ),
        }
    }

    fn parse_map_specular_exponent(&mut self) -> Result<Option<&'a str>, ParseError> {
        match self.peek() {
            Some("map_Ns") => {}
            _ => return Ok(None)
        }

        self.expect_tag("map_Ns")?;
        match self.next() {
            Some(st) => Ok(Some(st)),
            None => error(
                self.line_number, 
                ErrorKind::EndOfFile, 
                format!("Expected texture map name but got end of input.")
            ),
        }
    }

    fn parse_illumination_model(&mut self) -> Result<IlluminationModel, ParseError> {
        self.expect_tag("illum")?;
        let model_number = self.parse_usize()?;
        match model_number {
            0 => Ok(IlluminationModel::Ambient),
            1 => Ok(IlluminationModel::AmbientDiffuse),
            2 => Ok(IlluminationModel::AmbientDiffuseSpecular),
            n => error(
                self.line_number, 
                ErrorKind::UnknownIlluminationModel,
                format!("Unknown illumination model: {}.", n)
            )
        }
    }

    fn parse_newmtl(&mut self) -> Result<&'a str, ParseError> {
        match self.next() {
            Some("newmtl") => {}
            Some(st) => {
                return error(
                    self.line_number,
                    ErrorKind::ExpectedTag,
                    format!("Expected `newmtl` but got {}.", st)
                )
            }
            None => {
                return error(
                    self.line_number,
                    ErrorKind::EndOfFile,
                    format!("Expected `newmtl` but got end of input.")
                )
            }
        }

        match self.next() {
            Some(st) => Ok(st),
            None => {
                return error(
                    self.line_number,
                    ErrorKind::EndOfFile,
                    format!("Expected material name but got end of input.")
                )
            }
        }
    }

    fn parse_material(&mut self) -> Result<Material, ParseError> {
        let mut material = Material::new();
        let name = self.parse_newmtl()?;
        material.name = String::from(name);
        
        self.skip_zero_or_more_newlines();
        loop {
            match self.peek() {
                Some("Ka") => {
                    material.color_ambient = self.parse_ambient_component()?;
                }
                Some("Kd") => {
                    material.color_diffuse = self.parse_diffuse_component()?;
                }
                Some("Ks") => {
                    material.color_specular = self.parse_specular_component()?;
                }
                Some("Ke") => {
                    material.color_emissive = self.parse_emissive_component()?;
                }
                Some("d") => {
                    material.dissolve = self.parse_dissolve_component()?;
                }
                Some("illum") => {
                    material.illumination_model = self.parse_illumination_model()?;
                }
                Some("Ns") => {
                    material.specular_exponent = self.parse_specular_exponent()?;
                }
                Some("Ni") => {
                    let optical_density = self.parse_optical_density()?;
                    material.optical_density = Some(optical_density);
                }
                Some("map_Ka") => {
                    let name = self.parse_map_ambient()?;
                    material.map_ambient = name.map(|st| String::from(st));
                }
                Some("map_Kd") => {
                    let name = self.parse_map_diffuse()?;
                    material.map_diffuse = name.map(|st| String::from(st));
                }
                Some("map_Ks") => {
                    let name = self.parse_map_specular()?;
                    material.map_specular = name.map(|st| String::from(st));
                }
                Some("map_Ke") => {
                    let name = self.parse_map_emissive()?;
                    material.map_emissive = name.map(|st| String::from(st));
                }
                Some("map_Ns") => {
                    let name = self.parse_map_specular_exponent()?;
                    material.map_specular_exponent = name.map(|st| String::from(st));
                }
                Some("map_Bump") | Some("bump") => {
                    let map_bump = self.parse_map_bump()?;
                    material.map_bump = map_bump.map(|name| String::from(name));
                }
                Some("disp") => {
                    let map_displacement = self.parse_map_displacement()?;
                    material.map_displacement = map_displacement.map(|name| String::from(name));
                }
                Some("map_d") => {
                    let map_dissolve = self.parse_map_dissolve()?;
                    material.map_dissolve = map_dissolve.map(|name| String::from(name));
                }
                Some("newmtl") | None => {
                    break;
                }
                Some(other_st) => {
                    return error(
                        self.line_number, 
                        ErrorKind::ErrorParsingMaterial,
                        format!("Could not parse the token `{}`.", other_st) 
                    );
                }
            }
            self.skip_zero_or_more_newlines();
        }

        Ok(material)
    }

    fn parse_mtlset(&mut self) -> Result<MaterialSet, ParseError> {
        self.skip_zero_or_more_newlines();

        let mut materials = Vec::new();

        loop {
            match self.peek() {
                Some("newmtl") => {
                    let material = self.parse_material()?;
                    materials.push(material);
                }
                _ => break,
            }
        }
        
        match self.peek() {
            Some(st) => {
                return error(
                    self.line_number,
                    ErrorKind::ExpectedEndOfInput,
                    format!("Expeted end of input but got `{}`.", st)
                )
            }
            None => {}
        }

        Ok(MaterialSet { materials: materials })
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
        ErrorKind,
        IlluminationModel,
    };


    #[test]
    fn test_parse_ambient_component() {
        let mut parser = Parser::new("Ka 0.1345345 0.63453 0.982430");
        let expected = Ok(Color::new(0.1345345, 0.63453, 0.982430));
        let result = parser.parse_ambient_component();

        assert_eq!(result, expected);      
    }

    #[test]
    fn test_parse_diffuse_component() {
        let mut parser = Parser::new("Kd 0.1345345 0.63453 0.982430");
        let expected = Ok(Color::new(0.1345345, 0.63453, 0.982430));
        let result = parser.parse_diffuse_component();

        assert_eq!(result, expected);  
    }

    #[test]
    fn test_parse_specular_component() {
        let mut parser = Parser::new("Ks 0.1345345 0.63453 0.982430");
        let expected = Ok(Color::new(0.1345345, 0.63453, 0.982430));
        let result = parser.parse_specular_component();

        assert_eq!(result, expected);  
    }

    #[test]
    fn test_parse_emissive_component() {
        let mut parser = Parser::new("Ke 0.1345345 0.63453 0.982430");
        let expected = Ok(Color::new(0.1345345, 0.63453, 0.982430));
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

    #[test]
    fn test_parse_map_ambient() {
        let mut parser = Parser::new("map_Ka ambient.png");
        let expected = Ok(Some("ambient.png"));
        let result = parser.parse_map_ambient();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_map_diffuse() {
        let mut parser = Parser::new("map_Kd diffuse.png");
        let expected = Ok(Some("diffuse.png"));
        let result = parser.parse_map_diffuse();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_map_specular() {
        let mut parser = Parser::new("map_Ks specular.png");
        let expected = Ok(Some("specular.png"));
        let result = parser.parse_map_specular();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_map_emissive() {
        let mut parser = Parser::new("map_Ke emissive.png");
        let expected = Ok(Some("emissive.png"));
        let result = parser.parse_map_emissive();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_map_bump1() {
        let mut parser = Parser::new("map_Bump normal.png");
        let expected = Ok(Some("normal.png"));
        let result = parser.parse_map_bump();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_map_bump2() {
        let mut parser = Parser::new("bump normal.png");
        let expected = Ok(Some("normal.png"));
        let result = parser.parse_map_bump();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_map_displacement() {
        let mut parser = Parser::new("disp roughness.png");
        let expected = Ok(Some("roughness.png"));
        let result = parser.parse_map_displacement();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_map_dissolve() {
        let mut parser = Parser::new("map_d alpha.png");
        let expected = Ok(Some("alpha.png"));
        let result = parser.parse_map_dissolve();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_illumination_model0() {
        let mut parser = Parser::new("illum 0");
        let expected = Ok(IlluminationModel::Ambient);
        let result = parser.parse_illumination_model();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_illumination_model1() {
        let mut parser = Parser::new("illum 1");
        let expected = Ok(IlluminationModel::AmbientDiffuse);
        let result = parser.parse_illumination_model();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_illumination_model2() {
        let mut parser = Parser::new("illum 2");
        let expected = Ok(IlluminationModel::AmbientDiffuseSpecular);
        let result = parser.parse_illumination_model();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_illumination_model3() {
        let mut parser = Parser::new("illum 3");
        let expected_kind = ErrorKind::UnknownIlluminationModel;
        let result = parser.parse_illumination_model();
        assert!(result.is_err());

        let result_kind = result.unwrap_err().kind;
        assert_eq!(result_kind, expected_kind);
    }

    #[test]
    fn test_parse_newmtl1() {
        let mut parser = Parser::new("newmtl material_name");
        let expected = Ok("material_name");
        let result = parser.parse_newmtl();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_newmtl2() {
        let mut parser = Parser::new("newmtl    ");
        let result = parser.parse_newmtl();

        assert!(result.is_err());
    }
}


#[cfg(test)]
mod mtlset_parser_tests {
    use super::{
        Color,
        IlluminationModel,
        Material,
        MaterialSet,
    };


    #[test]
    fn test_parse() {
        let mtl_file = r"
        # Blender MTL File: 'None'      \
        # Material Count: 1             \
                                        \
        newmtl Scene_-_Root             \
        Ns 225.000000                   \
        Ka 1.000000 1.000000 1.000000   \
        Kd 0.800000 0.800000 0.800000   \
        Ks 0.500000 0.500000 0.500000   \
        Ke 0.0 0.0 0.0                  \
        Ni 1.450000                     \
        d 1.000000                      \
        illum 2                         \
        map_Kd diffuse.jpg              \
        map_Bump normal.png             \
        map_Ks specular.jpg             \
        disp roughness.jpg              \
        ";
        let expected = Ok(MaterialSet {
            materials: vec![
                Material {
                    name: String::from("Scene_-_Root"),
                    color_ambient: Color::new(1_f64, 1_f64, 1_f64),
                    color_diffuse: Color::new(0.8_f64, 0.8_f64, 0.8_f64),
                    color_specular: Color::new(0.5_f64, 0.5_f64, 0.5_f64),
                    color_emissive: Color::new(0_f64, 0_f64, 0_f64),
                    specular_exponent: 225_f64,
                    dissolve: 1_f64,
                    optical_density: Some(1.45_f64),
                    illumination_model: IlluminationModel::AmbientDiffuseSpecular,
                    map_ambient: None,
                    map_diffuse: Some(String::from("diffuse.jpg")),
                    map_specular: Some(String::from("specular.jpg")),
                    map_emissive: None,
                    map_specular_exponent: None,
                    map_bump: Some(String::from("normal.png")),
                    map_displacement: Some(String::from("roughness.jpg")),
                    map_dissolve: None,
                },
            ],
        });
        let result = super::parse(mtl_file);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_multiple_materials() {
        let mtl_file = r"
        # Blender MTL File: 'None'                                             \
        # Material Count: 1                                                    \
                                                                               \
        newmtl Scene_-_Root                                                    \
        Ns 225.000000                                                          \
        Ka 1.000000 1.000000 1.000000                                          \
        Kd 0.800000 0.800000 0.800000                                          \
        Ks 0.500000 0.500000 0.500000                                          \
        Ke 0.0 0.0 0.0                                                         \
        Ni 1.450000                                                            \
        d 1.000000                                                             \
        illum 2                                                                \
        map_Kd diffuse.jpg                                                     \
        map_Bump normal.png                                                    \
        map_Ks specular.jpg                                                    \
        disp roughness.jpg                                                     \
                                                                               \        
        # This is a bright green material.  When applied to an object, it will \ 
        # remain bright green regardless of any lighting in the scene.         \
        newmtl neon_green                                                      \
        Kd 0.0000 1.0000 0.0000                                                \
        illum 0                                                                \
                                                                               \
        # This is a flat green material.                                       \
        newmtl flat_green                                                      \
        Ka 0.0000 1.0000 0.0000                                                \
        Kd 0.0000 1.0000 0.0000                                                \
        illum 1                                                                \
                                                                               \
        # This is a flat green, partially dissolved material.                  \
        newmtl diss_green                                                      \
        Ka 0.0000 1.0000 0.0000                                                \
        Kd 0.0000 1.0000 0.0000                                                \
        d 0.8000                                                               \
        illum 1                                                                \
                                                                               \
        # This is a shiny green material.  When applied to an object, it       \
        # shows a white specular highlight.                                    \
        newmtl shiny_green                                                     \
        Ka 0.0000 1.0000 0.0000                                                \
        Kd 0.0000 1.0000 0.0000                                                \
        Ks 1.0000 1.0000 1.0000                                                \
        Ns 200.0000                                                            \
        illum 1                                                                \
        ";
        let expected = MaterialSet {
            materials: vec![
                Material {
                    name: String::from("Scene_-_Root"),
                    color_ambient: Color::new(1_f64, 1_f64, 1_f64),
                    color_diffuse: Color::new(0.8_f64, 0.8_f64, 0.8_f64),
                    color_specular: Color::new(0.5_f64, 0.5_f64, 0.5_f64),
                    color_emissive: Color::new(0_f64, 0_f64, 0_f64),
                    specular_exponent: 225_f64,
                    dissolve: 1_f64,
                    optical_density: Some(1.45_f64),
                    illumination_model: IlluminationModel::AmbientDiffuseSpecular,
                    map_ambient: None,
                    map_diffuse: Some(String::from("diffuse.jpg")),
                    map_specular: Some(String::from("specular.jpg")),
                    map_emissive: None,
                    map_specular_exponent: None,
                    map_bump: Some(String::from("normal.png")),
                    map_displacement: Some(String::from("roughness.jpg")),
                    map_dissolve: None,
                },
                Material {
                    name: String::from("neon_green"),
                    color_ambient: Color::zero(),
                    color_diffuse: Color::new(0_f64, 1_f64, 0_f64),
                    color_specular: Color::zero(),
                    color_emissive: Color::zero(),
                    specular_exponent: 0_f64,
                    dissolve: 0_f64,
                    optical_density: None,
                    illumination_model: IlluminationModel::Ambient,
                    map_ambient: None,
                    map_diffuse: None,
                    map_specular: None,
                    map_emissive: None,
                    map_specular_exponent: None,
                    map_bump: None,
                    map_displacement: None,
                    map_dissolve: None,
                },
                Material {
                    name: String::from("flat_green"),
                    color_ambient: Color::new(0_f64, 1_f64, 0_f64),
                    color_diffuse: Color::new(0_f64, 1_f64, 0_f64),
                    color_specular: Color::zero(),
                    color_emissive: Color::zero(),
                    specular_exponent: 0_f64,
                    dissolve: 0_f64,
                    optical_density: None,
                    illumination_model: IlluminationModel::AmbientDiffuse,
                    map_ambient: None,
                    map_diffuse: None,
                    map_specular: None,
                    map_emissive: None,
                    map_specular_exponent: None,
                    map_bump: None,
                    map_displacement: None,
                    map_dissolve: None,
                },
                Material {
                    name: String::from("diss_green"),
                    color_ambient: Color::new(0_f64, 1_f64, 0_f64),
                    color_diffuse: Color::new(0_f64, 1_f64, 0_f64),
                    color_specular: Color::zero(),
                    color_emissive: Color::zero(),
                    specular_exponent: 0_f64,
                    dissolve: 0.8_f64,
                    optical_density: None,
                    illumination_model: IlluminationModel::AmbientDiffuse,
                    map_ambient: None,
                    map_diffuse: None,
                    map_specular: None,
                    map_emissive: None,
                    map_specular_exponent: None,
                    map_bump: None,
                    map_displacement: None,
                    map_dissolve: None,
                },
                Material {
                    name: String::from("shiny_green"),
                    color_ambient: Color::new(0_f64, 1_f64, 0_f64),
                    color_diffuse: Color::new(0_f64, 1_f64, 0_f64),
                    color_specular: Color::new(1_f64, 1_f64, 1_f64),
                    color_emissive: Color::zero(),
                    specular_exponent: 200_f64,
                    dissolve: 0_f64,
                    optical_density: None,
                    illumination_model: IlluminationModel::AmbientDiffuse,
                    map_ambient: None,
                    map_diffuse: None,
                    map_specular: None,
                    map_emissive: None,
                    map_specular_exponent: None,
                    map_bump: None,
                    map_displacement: None,
                    map_dissolve: None,
                },
            ],
        };
        let result = super::parse(mtl_file);
        assert!(result.is_ok());
        let result = result.unwrap();

        for (result_i, expected_i) in result.iter().zip(expected.iter()) {
            assert_eq!(result_i, expected_i);
        }
    }
}

