use crate::lexer::{
    Lexer,
    PeekableLexer,
};
use std::error;
use std::fmt;


/// Parse a material library file from a string.
///
/// ## Example
///
/// ```
/// # use wavefront_obj::mtl;
/// # use wavefront_obj::mtl::{
/// #     MaterialSet,
/// #     Material,
/// #     IlluminationModel,
/// #     Color,
/// # };
/// #
/// let mtl_file = String::from(r"
///     newmtl my_material
///     Ka 0.0435 0.0435 0.0435
///     Kd 0.1086 0.1086 0.1086
///     Ks 0.0000 0.0000 0.0000
///     illum 2
///     d 0.6600
///     Ns 10.0000
///     Ni 1.19713
///     map_Ke emissive.jpg
///     map_Ka ambient.jpg
///     map_Kd diffuse.jpg
///     map_Ks specular.jpg
///     map_Ns specular_exponent.jpg
///     map_d dissolve.png
///     disp displacement.png
///     decal decal.jpg
///     bump height.png
/// ");
/// // let expected = ...;
/// # let expected = MaterialSet {
/// #     materials: vec![Material {
/// #         name: String::from("my_material"),
/// #         color_ambient: Color { r: 0.0435, g: 0.0435, b: 0.0435 },
/// #         color_diffuse: Color { r: 0.1086, g: 0.1086, b: 0.1086 },
/// #         color_specular: Color { r: 0.0000, g: 0.0000, b: 0.0000 },
/// #         color_emissive: Color { r: 0.0, g: 0.0, b: 0.0 },
/// #         specular_exponent: 10.0000,
/// #         dissolve: 0.6600,
/// #         optical_density: Some(1.19713),
/// #         illumination_model: IlluminationModel::AmbientDiffuseSpecular,
/// #         map_ambient: Some(String::from("ambient.jpg")),
/// #         map_diffuse: Some(String::from("diffuse.jpg")),
/// #         map_specular: Some(String::from("specular.jpg")),
/// #         map_emissive: Some(String::from("emissive.jpg")),
/// #         map_specular_exponent: Some(String::from("specular_exponent.jpg")),
/// #         map_bump: Some(String::from("height.png")),
/// #         map_displacement: Some(String::from("displacement.png")),
/// #         map_dissolve: Some(String::from("dissolve.png")),
/// #         map_decal: Some(String::from("decal.jpg")),
/// #     }]
/// # };
/// let result = mtl::parse(&mtl_file);
/// assert!(result.is_ok());
///
/// let result = result.unwrap();
/// assert_eq!(result, expected);
/// ```
pub fn parse<T: AsRef<str>>(input: T) -> Result<MaterialSet, ParseError> {
    Parser::new(input.as_ref()).parse_mtlset()
}

/// A representation of a material's color attributes, such as
/// the ambient color, diffuse color, specular color, and the emissive color.
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Color {
    /// The red component of a color.
    pub r: f64,
    /// The green component of a color.
    pub g: f64,
    /// The blue component of a color.
    pub b: f64
}

impl Color {
    #[inline]
    const fn zero() -> Color {
        Color {
            r: 0_f64,
            g: 0_f64,
            b: 0_f64
        }
    }
}

/// The illumination model describes how to illuminate an object with a given 
/// material.
/// 
/// The illumination model data is based on the original set Wavefront MTL spec 
/// illumination models. This parameter exists mostly for legacy reasons at this 
/// point.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum IlluminationModel {
    /// Apply an ambient color to the material only.
    Ambient,
    /// Apply a Lambertian shading model to a material.
    AmbientDiffuse,
    /// Apply a Phone shading model to a material.
    AmbientDiffuseSpecular,
}

/// A material description associated with an object in a scene describes
/// how to illuminate the object. 
/// 
/// A material can contain multiple texture maps including a diffuse map, a 
/// specular map, a specular exponent map, and an ambient map. This combination 
/// of maps allows one to implement some variation or other of a Phong shading 
/// model. Other maps include bump maps and displacement maps for varying the 
/// roughness of the material across the surface of an object, as well as 
/// a dissolve map that descibes how the variation changes across an object.
#[derive(Clone, Debug, PartialEq)]
pub struct Material {
    /// The material's name in the material library.
    pub name: String,
    /// The ambient color of the material. This parameter is typically used in
    /// some variation of Phong shading.
    pub color_ambient: Color,
    /// The diffuse color of the material. This parameter is typically used in
    /// some variation of Phong shading.
    pub color_diffuse: Color,
    /// The specular color of the material. This parameter is typically used in
    /// some variation of Phong shading.
    pub color_specular: Color,
    /// The emissive color of the material. When an object also happens to be
    /// a light source, the emissive color descibes the color of light the
    /// object emits.
    pub color_emissive: Color,
    /// The specular exponent of the material used in either Phong shading or
    /// Blinn-Phong shading.
    pub specular_exponent: f64,
    /// The dissolve paramter specifies the amount of opacity (alpha) of a material.
    /// A dissolve of 0.0 is a fully transparent material, and a dissolve of 1.0 is
    /// a fully opaque material.
    pub dissolve: f64,
    /// The index of refaction of the material. This enables transparency 
    /// effects.
    pub optical_density: Option<f64>,
    /// The illumination model used to render a material. This parameter is 
    /// based on the original models defined in the MTL spec, and it told the 
    /// Wavefront software's system how to render the object. Since we do this 
    /// with programmable shaders on modern workloads anyway, this parameter 
    /// may be considered a legacy item.
    pub illumination_model: IlluminationModel,
    /// A texture map describing the ambient color as it varies across an object.
    pub map_ambient: Option<String>,
    /// A texture map that describes the diffuse color as it varies across an object.
    pub map_diffuse: Option<String>,
    /// A texture map that describes the specular color as it varies across an object.
    pub map_specular: Option<String>,
    /// A texture map that describes the emissive color as it varies across an object. 
    pub map_emissive: Option<String>,
    /// A texture map that describes the specular exponent at different locations
    /// on an object.
    pub map_specular_exponent: Option<String>,
    /// A texture map that stores the height data that describes how a normal vector
    /// gets perturbed across a surface for providing extra surface detail at low 
    /// computational cost.
    pub map_bump: Option<String>,
    /// A texture map that describes the local deformation of the surface of an 
    /// object, creating surface roughness. Displacement mapping differs from bump 
    /// mapping in that a displacement map describes how to actually modify the 
    /// tesselation of an object's surface. A bump map merely perturbs the normal
    /// vector without modifying the geometry.
    pub map_displacement: Option<String>,
    /// A texture map that describes the opacity of a material as it varies across
    /// an object.
    pub map_dissolve: Option<String>,
    /// A texture map that replaces the main surface color with a color looked up
    /// from the decal map.
    pub map_decal: Option<String>,
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
            dissolve: 1_f64,
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
            map_decal: None,
        }
    }
}

/// A collection of materials that may be used by multiple parts of a single
/// object, or referenced when rendering a collection of objects.
#[derive(Clone, Debug, PartialEq)]
pub struct MaterialSet {
    pub materials: Vec<Material>,
}

/// A marker indicating the type of error generated during parsing of a 
/// Wavefront MTL file.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ErrorKind {
    /// The parser prematurely reached the end of the input.
    EndOfFile,
    /// The parser expected a tag statement that was not present.
    ExpectedTagStatement,
    /// The parser expected a floating point number but got something else
    /// instead.
    ExpectedFloat,
    /// The parser expected an integer but got something else instead.
    ExpectedInteger,
    /// The parser expected there to be no more input.
    ExpectedEndOfInput,
    /// The MTL file specified an unsupported or unknown illumination model. 
    UnknownIlluminationModel,
    /// A general parsing error occurred.
    ErrorParsingMaterial,
}

/// An error that is returned from parsing an invalid `*.mtl` file, or
/// another kind of error.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParseError {
    /// The line number where the error occurred.
    line_number: usize,
    /// The kind of error that occurred.
    kind: ErrorKind,
    /// A message describing why the parse error was generated.
    message: String,
}

impl ParseError {
    /// Construct a new parse error.
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
            formatter, "Parse error at line {}: {}", self.line_number, self.message
        )
    }
}

impl error::Error for ParseError {}


/// A Wavefront MTL file parser.
pub struct Parser<'a> {
    /// the current line number in the input stream.
    line_number: usize,
    /// The underlying lexer that tokenizes the input stream.
    lexer: PeekableLexer<'a>,
}

impl<'a> Parser<'a> {
    /// Construct a new parser for an mtl file input as a string.
    pub fn new(input: &'a str) -> Parser<'a> {
        Parser {
            line_number: 1,
            lexer: PeekableLexer::new(Lexer::new(input)),
        }
    }

    /// Construct a new parse error.
    fn error<T>(&self, kind: ErrorKind, message: String) -> Result<T, ParseError> {
        Err(ParseError::new(self.line_number, kind, message))
    }

    /// Peek at the currently held token without advancing the token stream.
    fn peek(&mut self) -> Option<&'a str> {
        self.lexer.peek()
    }

    /// Advance the token stream one step returning the currently held string.
    fn next(&mut self) -> Option<&'a str> {
        let token = self.lexer.next();
        if let Some(val) = token {
            if val == "\n" {
                self.line_number += 1;
            }
        }

        token
    }

    /// Advance the token stream one step without returning the current token. 
    fn advance(&mut self) {
        self.next();
    }

    /// Advance the token stream one step, returning the next token in the 
    /// stream.
    ///
    /// This function generates an error is it runs out of input.
    fn next_string(&mut self) -> Result<&'a str, ParseError> {
        match self.next() {
            Some(st) => Ok(st),
            None => self.error(
                ErrorKind::EndOfFile, 
                "Expected token but reached end of input.".to_owned()
            )
        }
    }

    /// Advance the token stream if the next token in the stream matches the
    /// input tag.
    ///
    /// This functions returns an error if the expected tag is not present.
    fn expect_tag(&mut self, tag: &str) -> Result<(), ParseError> {
        match self.next() {
            None => self.error(ErrorKind::EndOfFile, format!("")),
            Some(st) if st != tag => self.error(
                ErrorKind::ExpectedTagStatement,
                format!("Expected statement {} but got statement {}", tag, st)
            ),
            _ => Ok(())
        }
    }

    /// Skip zero or more newlines in the input stream.
    fn skip_zero_or_more_newlines(&mut self) {
        while let Some("\n") = self.peek() {
            self.advance();
        }
    }

    /// Parse a floating point number from the current token in the stream.
    fn parse_f64(&mut self) -> Result<f64, ParseError> {
        let st = self.next_string()?;
        match st.parse::<f64>() {
            Ok(val) => Ok(val),
            Err(_) => self.error( 
                ErrorKind::ExpectedFloat, 
                format!("Expected floating point number but got {}", st)
            ),
        }
    }

    /// Parse an integer from the current token in the stream.
    fn parse_usize(&mut self) -> Result<usize, ParseError> {
        let st = self.next_string()?;
        match st.parse::<usize>() {
            Ok(val) => Ok(val),
            Err(_) => self.error( 
                ErrorKind::ExpectedInteger,
                format!("Expected integer but got {}", st)
            )
        }
    }

    /// Parse a RGB color from the input stream.
    fn parse_color(&mut self) -> Result<Color, ParseError> {
        let r = self.parse_f64()?;
        let g = self.parse_f64()?;
        let b = self.parse_f64()?;

        Ok(Color { r: r, g: g, b: b })
    }

    /// Parse a material's ambient component from the input stream.
    fn parse_ambient_component(&mut self) -> Result<Color, ParseError> {
        self.expect_tag("Ka")?;
        self.parse_color()
    }

    /// Parse a material's diffuse component from the input stream.
    fn parse_diffuse_component(&mut self) -> Result<Color, ParseError> {
        self.expect_tag("Kd")?;
        self.parse_color()
    }

    /// Parse a material's specular component from the input stream.
    fn parse_specular_component(&mut self) -> Result<Color, ParseError> {
        self.expect_tag("Ks")?;
        self.parse_color()
    }

    /// parse a material's emissive component from the input stream.
    fn parse_emissive_component(&mut self) -> Result<Color, ParseError> {
        self.expect_tag("Ke")?;
        self.parse_color()
    }

    /// Parse a material's dissolve (alpha) component from the input stream.
    fn parse_dissolve_component(&mut self) -> Result<f64, ParseError> {
        self.expect_tag("d")?;
        self.parse_f64()
    }

    /// Parse a material's specular exponent from the input stream.
    fn parse_specular_exponent(&mut self) -> Result<f64, ParseError> {
        self.expect_tag("Ns")?;
        self.parse_f64()
    }

    /// Parse a material's optical density componeont from the input stream.
    fn parse_optical_density(&mut self) -> Result<f64, ParseError> {
        self.expect_tag("Ni")?;
        self.parse_f64()
    }

    /// Parse the name of a material's ambient texture map from the input stream.
    fn parse_map_ambient(&mut self) -> Result<Option<&'a str>, ParseError> {
        match self.peek() {
            Some("map_Ka") => {}
            _ => return Ok(None)
        }

        self.expect_tag("map_Ka")?;
        match self.next() {
            Some(st) => Ok(Some(st)),
            None => self.error(
                ErrorKind::EndOfFile, 
                "Expected texture map name but got end of input.".to_owned()
            ),
        }
    }

    /// Parse the name of a material's diffuse texture map from the input stream.
    fn parse_map_diffuse(&mut self) -> Result<Option<&'a str>, ParseError> {
        match self.peek() {
            Some("map_Kd") => {}
            _ => return Ok(None)
        }

        self.expect_tag("map_Kd")?;
        match self.next() {
            Some(st) => Ok(Some(st)),
            None => self.error(
                ErrorKind::EndOfFile, 
                "Expected texture map name but got end of input.".to_owned()
            ),
        }
    }

    /// Parse the name of a material's specular texture map from the input stream.
    fn parse_map_specular(&mut self) -> Result<Option<&'a str>, ParseError> {
        match self.peek() {
            Some("map_Ks") => {}
            _ => return Ok(None)
        }

        self.expect_tag("map_Ks")?;
        match self.next() {
            Some(st) => Ok(Some(st)),
            None => self.error(
                ErrorKind::EndOfFile, 
                "Expected texture map name but got end of input.".to_owned()
            ),
        }
    }

    /// Parse the name of a material's emissive texture map from the input stream.
    fn parse_map_emissive(&mut self) -> Result<Option<&'a str>, ParseError> {
        match self.peek() {
            Some("map_Ke") => {}
            _ => return Ok(None)
        }

        self.expect_tag("map_Ke")?;
        match self.next() {
            Some(st) => Ok(Some(st)),
            None => self.error(
                ErrorKind::EndOfFile, 
                "Expected texture map name but got end of input.".to_owned()
            ),
        }
    }

    /// Parse the name of a material's bump texture map from the input stream.
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
            None => self.error(
                ErrorKind::EndOfFile, 
                "Expected texture map name but got end of input.".to_owned()
            ),
        }
    }

    /// Parse the name of a material's displacement texture map from the input stream.
    fn parse_map_displacement(&mut self) -> Result<Option<&'a str>, ParseError> {
        match self.peek() {
            Some("disp") => {}
            _ => return Ok(None)
        }

        self.expect_tag("disp")?;
        match self.next() {
            Some(st) => Ok(Some(st)),
            None => self.error(
                ErrorKind::EndOfFile, 
                "Expected texture map name but got end of input.".to_owned()
            ),
        }
    }

    /// Parse the name of a material's dissolve (alpha) texture map from the input stream.
    fn parse_map_dissolve(&mut self) -> Result<Option<&'a str>, ParseError> {
        match self.peek() {
            Some("map_d") => {}
            _ => return Ok(None)
        }

        self.expect_tag("map_d")?;
        match self.next() {
            Some(st) => Ok(Some(st)),
            None => self.error(
                ErrorKind::EndOfFile, 
                "Expected texture map name but got end of input.".to_owned()
            ),
        }
    }

    /// Parse the name of a material's decal texture map from the input stream.
    fn parse_map_decal(&mut self) -> Result<Option<&'a str>, ParseError> {
        match self.peek() {
            Some("decal") => {}
            _ => return Ok(None)
        }

        self.expect_tag("decal")?;
        match self.next() {
            Some(st) => Ok(Some(st)),
            None => self.error(
                ErrorKind::EndOfFile, 
                "Expected texture map name but got end of input.".to_owned()
            ),
        }
    }

    /// Parse the name of a material's specular exponent texture map from the input stream.
    fn parse_map_specular_exponent(&mut self) -> Result<Option<&'a str>, ParseError> {
        match self.peek() {
            Some("map_Ns") => {}
            _ => return Ok(None)
        }

        self.expect_tag("map_Ns")?;
        match self.next() {
            Some(st) => Ok(Some(st)),
            None => self.error(
                ErrorKind::EndOfFile, 
                "Expected texture map name but got end of input.".to_owned()
            ),
        }
    }

    /// Parse a material's illumination model.
    fn parse_illumination_model(&mut self) -> Result<IlluminationModel, ParseError> {
        self.expect_tag("illum")?;
        let model_number = self.parse_usize()?;
        match model_number {
            0 => Ok(IlluminationModel::Ambient),
            1 => Ok(IlluminationModel::AmbientDiffuse),
            2 => Ok(IlluminationModel::AmbientDiffuseSpecular),
            n => self.error(
                ErrorKind::UnknownIlluminationModel,
                format!("Unknown illumination model: {}.", n)
            )
        }
    }

    /// Parse the name of a material.
    fn parse_newmtl(&mut self) -> Result<&'a str, ParseError> {
        match self.next() {
            Some("newmtl") => {}
            Some(st) => {
                return self.error(
                    ErrorKind::ExpectedTagStatement,
                    format!("Expected `newmtl` but got {}.", st)
                )
            }
            None => {
                return self.error(
                    ErrorKind::EndOfFile,
                    "Expected `newmtl` but got end of input.".to_owned()
                )
            }
        }

        match self.next() {
            Some(st) => Ok(st),
            None => {
                self.error(
                    ErrorKind::EndOfFile,
                    "Expected material name but got end of input.".to_owned()
                )
            }
        }
    }

    /// Parse one material from a MTL file.
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
                    material.map_ambient = name.map(String::from);
                }
                Some("map_Kd") => {
                    let name = self.parse_map_diffuse()?;
                    material.map_diffuse = name.map(String::from);
                }
                Some("map_Ks") => {
                    let name = self.parse_map_specular()?;
                    material.map_specular = name.map(String::from);
                }
                Some("map_Ke") => {
                    let name = self.parse_map_emissive()?;
                    material.map_emissive = name.map(String::from);
                }
                Some("map_Ns") => {
                    let name = self.parse_map_specular_exponent()?;
                    material.map_specular_exponent = name.map(String::from);
                }
                Some("map_Bump") | Some("bump") => {
                    let map_bump = self.parse_map_bump()?;
                    material.map_bump = map_bump.map(String::from);
                }
                Some("disp") => {
                    let map_displacement = self.parse_map_displacement()?;
                    material.map_displacement = map_displacement.map(String::from);
                }
                Some("map_d") => {
                    let map_dissolve = self.parse_map_dissolve()?;
                    material.map_dissolve = map_dissolve.map(String::from);
                }
                Some("decal") => {
                    let map_decal = self.parse_map_decal()?;
                    material.map_decal = map_decal.map(String::from);
                }
                Some("newmtl") | None => {
                    break;
                }
                Some(other_st) => {
                    return self.error(
                        ErrorKind::ErrorParsingMaterial,
                        format!("Could not parse the token `{}`.", other_st) 
                    );
                }
            }
            self.skip_zero_or_more_newlines();
        }

        Ok(material)
    }

    /// Parse an MTL file.
    ///
    /// ## Example
    ///
    /// ```
    /// # use wavefront_obj::mtl::{
    /// #     MaterialSet,
    /// #     Material,
    /// #     IlluminationModel,
    /// #     Color,
    /// #     Parser,
    /// # };
    /// #
    /// let mtl_file = String::from(r"
    ///     newmtl my_material
    ///     Ka 0.0435 0.0435 0.0435
    ///     Kd 0.1086 0.1086 0.1086
    ///     Ks 0.0000 0.0000 0.0000
    ///     illum 2
    ///     d 0.6600
    ///     Ns 10.0000
    ///     Ni 1.19713
    ///     map_Ke emissive.jpg
    ///     map_Ka ambient.jpg
    ///     map_Kd diffuse.jpg
    ///     map_Ks specular.jpg
    ///     map_Ns specular_exponent.jpg
    ///     map_d dissolve.png
    ///     disp displacement.png
    ///     decal decal.jpg
    ///     bump height.png
    /// ");
    /// // let expected = ...;
    /// # let expected = MaterialSet {
    /// #     materials: vec![Material {
    /// #         name: String::from("my_material"),
    /// #         color_ambient: Color { r: 0.0435, g: 0.0435, b: 0.0435 },
    /// #         color_diffuse: Color { r: 0.1086, g: 0.1086, b: 0.1086 },
    /// #         color_specular: Color { r: 0.0000, g: 0.0000, b: 0.0000 },
    /// #         color_emissive: Color { r: 0.0, g: 0.0, b: 0.0 },
    /// #         specular_exponent: 10.0000,
    /// #         dissolve: 0.6600,
    /// #         optical_density: Some(1.19713),
    /// #         illumination_model: IlluminationModel::AmbientDiffuseSpecular,
    /// #         map_ambient: Some(String::from("ambient.jpg")),
    /// #         map_diffuse: Some(String::from("diffuse.jpg")),
    /// #         map_specular: Some(String::from("specular.jpg")),
    /// #         map_emissive: Some(String::from("emissive.jpg")),
    /// #         map_specular_exponent: Some(String::from("specular_exponent.jpg")),
    /// #         map_bump: Some(String::from("height.png")),
    /// #         map_displacement: Some(String::from("displacement.png")),
    /// #         map_dissolve: Some(String::from("dissolve.png")),
    /// #         map_decal: Some(String::from("decal.jpg")),
    /// #     }]
    /// # };
    /// let result = Parser::new(&mtl_file).parse_mtlset();
    /// assert!(result.is_ok());
    ///
    /// let result = result.unwrap();
    /// assert_eq!(result, expected);
    /// ```
    pub fn parse_mtlset(&mut self) -> Result<MaterialSet, ParseError> {
        self.skip_zero_or_more_newlines();

        let mut materials = Vec::new();

        while let Some("newmtl") = self.peek() {
            let material = self.parse_material()?;
            materials.push(material);
        }
        
        if let Some(st) = self.peek() {
            return self.error(
                ErrorKind::ExpectedEndOfInput,
                format!("Expected end of input but got `{}`.", st)
            )
        }

        Ok(MaterialSet { materials: materials })
    }
}


#[cfg(test)]
mod mtl_primitive_tests {
    use super::{
        Parser,
        Color,
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

    #[test]
    fn test_parse_color() {
        let mut parser = Parser::new("    0.1345345 0.63453 0.982430   ");
        assert_eq!(parser.parse_color(), Ok(Color { r: 0.1345345, g: 0.63453, b: 0.982430 }));
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
    fn test_parse_map_decal() {
        let mut parser = Parser::new("decal decal.png");
        let expected = Ok(Some("decal.png"));
        let result = parser.parse_map_decal();

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
        disp displacement.jpg           \
        ";
        let expected = Ok(MaterialSet {
            materials: vec![
                Material {
                    name: String::from("Scene_-_Root"),
                    color_ambient: Color { r: 1_f64, g: 1_f64, b: 1_f64 },
                    color_diffuse: Color { r: 0.8_f64, g: 0.8_f64, b: 0.8_f64 },
                    color_specular: Color { r: 0.5_f64, g: 0.5_f64, b: 0.5_f64 },
                    color_emissive: Color { r: 0_f64, g: 0_f64, b: 0_f64 },
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
                    map_displacement: Some(String::from("displacement.jpg")),
                    map_dissolve: None,
                    map_decal: None,
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
        disp displacement.jpg                                                  \
        decal decal.jpg                                                        \
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
                    color_ambient: Color { r: 1_f64, g: 1_f64, b: 1_f64 },
                    color_diffuse: Color { r: 0.8_f64, g: 0.8_f64, b: 0.8_f64 },
                    color_specular: Color { r: 0.5_f64, g: 0.5_f64, b: 0.5_f64 },
                    color_emissive: Color { r: 0_f64, g: 0_f64, b: 0_f64 },
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
                    map_displacement: Some(String::from("displacement.jpg")),
                    map_dissolve: None,
                    map_decal: Some(String::from("decal.jpg")),
                },
                Material {
                    name: String::from("neon_green"),
                    color_ambient: Color::zero(),
                    color_diffuse: Color { r: 0_f64, g: 1_f64, b: 0_f64 },
                    color_specular: Color::zero(),
                    color_emissive: Color::zero(),
                    specular_exponent: 0_f64,
                    dissolve: 1_f64,
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
                    map_decal: None,
                },
                Material {
                    name: String::from("flat_green"),
                    color_ambient: Color { r: 0_f64, g: 1_f64, b: 0_f64 },
                    color_diffuse: Color { r: 0_f64, g: 1_f64, b: 0_f64 },
                    color_specular: Color::zero(),
                    color_emissive: Color::zero(),
                    specular_exponent: 0_f64,
                    dissolve: 1_f64,
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
                    map_decal: None,
                },
                Material {
                    name: String::from("diss_green"),
                    color_ambient: Color { r: 0_f64, g: 1_f64, b: 0_f64 },
                    color_diffuse: Color { r: 0_f64, g: 1_f64, b: 0_f64 },
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
                    map_decal: None,
                },
                Material {
                    name: String::from("shiny_green"),
                    color_ambient: Color { r: 0_f64, g: 1_f64, b: 0_f64 },
                    color_diffuse: Color { r: 0_f64, g: 1_f64, b: 0_f64 },
                    color_specular: Color { r: 1_f64, g: 1_f64, b: 1_f64 },
                    color_emissive: Color::zero(),
                    specular_exponent: 200_f64,
                    dissolve: 1_f64,
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
                    map_decal: None,
                },
            ],
        };
        let result = super::parse(mtl_file);
        assert!(result.is_ok());
        let result = result.unwrap();

        for (result_i, expected_i) 
            in result.materials.iter().zip(expected.materials.iter()) {
            assert_eq!(result_i, expected_i);
        }
    }
}

