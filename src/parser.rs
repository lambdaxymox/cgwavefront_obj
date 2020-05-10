use crate::object::{
    ObjectSet, 
    Object, 
    ObjectBuilder,
    Vertex, 
    TextureVertex, 
    NormalVertex,
    Group, 
    SmoothingGroup, 
    Element, 
    VTNIndex, 
    ShapeEntry,
};
use crate::lexer::{
    Lexer,
    ObjectLexer,
};
use std::error;
use std::fmt;
use std::io::{
    BufReader, 
    Read
};
use std::fs::File;
use std::path::Path;


#[derive(Clone, Debug)]
pub enum ObjError {
    Source,
    SourceDoesNotExist(String),
    Parse(ParseError),
}

impl fmt::Display for ObjError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ObjError::Source => {
                write!(f, "An error occurred in the OBJ source.")
            }
            ObjError::SourceDoesNotExist(source) => {
                write!(f, "Source could not be found: {}.", source)
            }
            ObjError::Parse(parse_error) => {
                write!(f, "{}", parse_error)
            }
        }
    }
}

impl error::Error for ObjError {}


/// Parse a wavefront object file from a file buffer or other `Read` instance.
pub fn parse<F: Read>(file: F) -> Result<ObjectSet, ObjError> {
    let mut reader = BufReader::new(file);
    let mut string = String::new();
    reader.read_to_string(&mut string).unwrap();

    let mut parser = Parser::new(&string);

    match parser.parse() {
        Ok(obj_set) => Ok(obj_set),
        Err(e) => Err(ObjError::Parse(e)),
    }
}


/// Parse a wavefront object file from a file path.
pub fn parse_file<P: AsRef<Path>>(path: P) -> Result<ObjectSet, ObjError> {
    if !path.as_ref().exists() {
        let disp = path.as_ref().display();

        return Err(ObjError::SourceDoesNotExist(format!("{}", disp)));
    }

    let file = match File::open(path) {
        Ok(val) => val,
        Err(_) => return Err(ObjError::Source)
    };

    parse(file)
}


/// Parse a wavefront object file from a string.
pub fn parse_str(st: &str) -> Result<ObjectSet, ParseError> {
    let mut parser = Parser::new(st);
    parser.parse()
}


#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ErrorKind {
    EndOfFile,
    ExpectedStatementButGot(String, String),
    ExpectedFloatButGot(String),
    ExpectedIntegerButGot(String),
    ExpectedVertexIndexButGot(String),
    ExpectedTextureIndexButGot(String),
    ExpectedNormalIndexButGot(String),
    ExpectedVertexNormalIndexButGot(String),
    ExpectedVertexTextureIndexButGot(String),
    ExpectedVertexTextureNormalIndexButGot(String),
    EveryFaceElementMustHaveAtLeastThreeVertices,
    EveryVTNIndexMustHaveTheSameFormForAGivenFace,
    InvalidElementDeclaration(String),
    InvalidElement,
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

/// An error that is returned from parsing an invalid *.obj file, or
/// another kind of error.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParseError {
    line_number: usize,
    kind: String,
}

impl ParseError {
    /// Generate a new parse error.
    fn new(line_number: usize, kind: String) -> ParseError {
        ParseError {
            line_number: line_number,
            kind: kind,
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "Parse error at line {}: {}", self.line_number, self.kind)
    }
}

impl error::Error for ParseError {}

#[inline]
fn error<T>(line_number: usize, kind: String) -> Result<T, ParseError> {
    Err(ParseError::new(line_number, kind))
}

/// A Wavefront OBJ file parser.
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

    fn error<T>(&mut self, err: String) -> Result<T, ParseError> {
        Err(ParseError::new(self.line_number, err))
    }

    fn next_string(&mut self) -> Result<&'a str, ParseError> {
        match self.next() {
            Some(st) => Ok(st),
            None => error(self.line_number, format!("Expected string but got `end of file`."))
        }
    }

    fn expect(&mut self, tag: &str) -> Result<&'a str, ParseError> {
        let st = self.next_string()?;
        match st == tag {
            true => Ok(st),
            false => error(self.line_number, format!("Expected `{}` statement but got: `{}`.", tag, st))
        }
    }

    fn parse_f32(&mut self) -> Result<f32, ParseError> {
        let st = self.next_string()?;
        match st.parse::<f32>() {
            Ok(val) => Ok(val),
            Err(_) => error(self.line_number, format!("Expected `f32` but got `{}`.", st)),
        }
    }

    fn parse_u32(&mut self) -> Result<u32, ParseError> {
        let st = self.next_string()?;
        match st.parse::<u32>() {
            Ok(val) => Ok(val),
            Err(_) => error(self.line_number, format!("Expected integer but got `{}`.", st)),
        }
    }

    fn try_once<P, T>(&mut self, parser: P) -> Option<T> where P: FnOnce(&str) -> Option<T> {
        match self.peek() {
            Some(st) => parser(&st).map(|got| { self.advance(); got }),
            None => None,
        }
    }

    fn parse_vertex(&mut self) -> Result<Vertex, ParseError> {
        self.expect("v")?;
 
        let x = self.parse_f32()?;
        let y = self.parse_f32()?;
        let z = self.parse_f32()?;
        let mw = self.try_once(|st| st.parse::<f32>().ok());
        let w = mw.unwrap_or(1.0);

        Ok(Vertex { x: x, y: y, z: z, w: w })
    }

    fn parse_texture_vertex(&mut self) -> Result<TextureVertex, ParseError> {
        self.expect("vt")?;

        let u = self.parse_f32()?;
        let mv = self.try_once(|st| st.parse::<f32>().ok());
        let v = mv.unwrap_or(0.0);
        let mw = self.try_once(|st| st.parse::<f32>().ok());
        let w = mw.unwrap_or(0.0);

        Ok(TextureVertex { u: u, v: v, w: w })
    }

    fn parse_normal_vertex(&mut self) -> Result<NormalVertex, ParseError> {
        self.expect("vn")?;

        let i = self.parse_f32()?;
        let j = self.parse_f32()?;
        let k = self.parse_f32()?;

        Ok(NormalVertex { i: i, j: j, k: k })
    }

    fn skip_zero_or_more_newlines(&mut self) {
        loop {
            match self.peek() {
                Some("\n") => self.advance(),
                _ => break
            }
        }
    }

    fn skip_one_or_more_newlines(&mut self) -> Result<(), ParseError> {
        self.expect("\n")?;
        self.skip_zero_or_more_newlines();
        Ok(())
    }

    fn parse_object_name(&mut self) -> Result<&'a str, ParseError> {
        match self.peek() {
            Some("o") => {
                self.expect("o")?;
                let object_name = self.next_string();
                self.skip_one_or_more_newlines()?;
                
                object_name
            }
            _ => Ok("")
        }
    }

    fn parse_vtn_index(&mut self) -> Result<VTNIndex, ParseError> {
        enum VTNErrorKind {
            ExpectedVertexIndexButGot,
            ExpectedNormalIndexButGot,
            ExpectedVertexNormalIndexButGot,
            ExpectedTextureIndexButGot,
            ExpectedVertexTextureIndexButGot,
        }

        fn parse_vn(st: &str) -> Result<VTNIndex, VTNErrorKind> {
            if let Some(v_index_in_str) = st.find("//") {
                let v_index = match st[0..v_index_in_str].parse::<u32>() {
                    Ok(val) => val,
                    Err(_) => return Err(VTNErrorKind::ExpectedVertexIndexButGot),
                };
                let vn_index = match st[v_index_in_str+2..].parse::<u32>() {
                    Ok(val) => val,
                    Err(_) => return Err(VTNErrorKind::ExpectedNormalIndexButGot),
                };
    
                return Ok(VTNIndex::VN(v_index, vn_index));
            } else {
                return Err(VTNErrorKind::ExpectedVertexNormalIndexButGot);
            }
        }
    
        fn parse_vt(st: &str) -> Result<VTNIndex, VTNErrorKind> {
            if let Some(v_index_in_str) = st.find("/") {
                let v_index = match st[0..v_index_in_str].parse::<u32>() {
                    Ok(val) => val,
                    Err(_) => return Err(VTNErrorKind::ExpectedVertexIndexButGot),
                };
                let vt_index = match st[v_index_in_str+1..].parse::<u32>() {
                    Ok(val) => val,
                    Err(_) => return Err(VTNErrorKind::ExpectedTextureIndexButGot)
                };
    
                return Ok(VTNIndex::VT(v_index, vt_index));
            } else {
                return Err(VTNErrorKind::ExpectedVertexTextureIndexButGot);
            }
        }
    
        fn parse_vtn(st: &str) -> Result<VTNIndex, VTNErrorKind> {
            let v_index_in_str = match st.find("/") {
                Some(val) => val,
                None => return Err(VTNErrorKind::ExpectedVertexIndexButGot),
            };
            let v_index = match st[0..v_index_in_str].parse::<u32>() {
                Ok(val) => val,
                Err(_) => return Err(VTNErrorKind::ExpectedVertexIndexButGot),
            };
            let vt_index_in_str = match st[(v_index_in_str + 1)..].find("/") {
                Some(val) => v_index_in_str + 1 + val,
                None => return Err(VTNErrorKind::ExpectedTextureIndexButGot),
            };
            let vt_index = match st[(v_index_in_str + 1)..vt_index_in_str].parse::<u32>() {
                Ok(val) => val,
                Err(_) => return Err(VTNErrorKind::ExpectedTextureIndexButGot),
            };
            let vn_index = match st[(vt_index_in_str + 1)..].parse::<u32>() {
                Ok(val) => val,
                Err(_) => return Err(VTNErrorKind::ExpectedNormalIndexButGot),
            };
       
            Ok(VTNIndex::VTN(v_index, vt_index, vn_index))
        }
    
        fn parse_v(st: &str) -> Result<VTNIndex, VTNErrorKind> {
            match st.parse::<u32>() {
                Ok(val) => Ok(VTNIndex::V(val)),
                Err(_) => return Err(VTNErrorKind::ExpectedVertexIndexButGot)
            }
        }


        let st = self.next_string()?;
        match parse_vn(&st) {
            Ok(val) => return Ok(val),
            Err(_) => {},
        }
        match parse_vtn(&st) {
            Ok(val) => return Ok(val),
            Err(_) => {},
        }
        match parse_vt(&st) {
            Ok(val) => return Ok(val),
            Err(_) => {},
        }
        match parse_v(&st) {
            Ok(val) => return Ok(val),
            Err(_) => {},
        }

        error(self.line_number, format!("Expected `vertex/texture/normal` index but got `{}`", st))
    }

    fn parse_vtn_indices(&mut self, vtn_indices: &mut Vec<VTNIndex>) -> Result<u32, ParseError> {
        let mut indices_parsed = 0;
        while let Ok(vtn_index) = self.parse_vtn_index() {
            vtn_indices.push(vtn_index);
            indices_parsed += 1;
        }

        Ok(indices_parsed)
    }

    fn parse_point(&mut self, elements: &mut Vec<Element>) -> Result<u32, ParseError> {
        self.expect("p")?;

        let v_index = self.parse_u32()?;
        elements.push(Element::Point(VTNIndex::V(v_index)));
        let mut elements_parsed = 1;
        loop {
            match self.next_string() {
                Ok(st) if st != "\n" => match st.parse::<u32>() {
                    Ok(v_index) => { 
                        elements.push(Element::Point(VTNIndex::V(v_index)));
                        elements_parsed += 1;
                    }
                    Err(_) => {
                        return error(self.line_number,format!("Expected integer but got `{}`.", st))
                    }
                }
                _ => break,
            }
        }

        Ok(elements_parsed)
    }

    fn parse_line(&mut self, elements: &mut Vec<Element>) -> Result<u32, ParseError> {
        self.expect("l")?;

        let mut vtn_indices = vec![];
        vtn_indices.push(self.parse_vtn_index()?);
        vtn_indices.push(self.parse_vtn_index()?);
        self.parse_vtn_indices(&mut vtn_indices)?;

        // Verify that each VTN index has the same type and if of a valid form.
        for i in 1..vtn_indices.len() {
            if !vtn_indices[i].has_same_type_as(&vtn_indices[0]) {
                return error(self.line_number,
                    format!("Every vertex/texture/normal index must have the same form.")
                );
            }
        }

        // Now that we have verified the indices, build the line elements.
        for i in 0..vtn_indices.len()-1 {
            elements.push(Element::Line(vtn_indices[i], vtn_indices[i + 1]));
        }

        Ok((vtn_indices.len() - 1) as u32)
    }

    fn parse_face(&mut self, elements: &mut Vec<Element>) -> Result<u32, ParseError> {
        self.expect("f")?;
        
        let mut vtn_indices = vec![];
        self.parse_vtn_indices(&mut vtn_indices)?;

        // Check that there are enough vtn indices.
        if vtn_indices.len() < 3 {
            return error(self.line_number,
                format!("A face element must have at least three vertices.")
            );  
        }

        // Verify that each VTN index has the same type and if of a valid form.
        for i in 1..vtn_indices.len() {
            if !vtn_indices[i].has_same_type_as(&vtn_indices[0]) {
                return error(self.line_number,
                    format!("Every vertex/texture/normal index must have the same form.")
                );
            }
        }

        // Triangulate the polygon with a triangle fan. Note that the OBJ specification
        // assumes that polygons are coplanar, and consequently the parser does not check
        // this. It is up to the model creator to ensure this.
        let vertex0 = vtn_indices[0];
        for i in 0..vtn_indices.len()-2 {
            elements.push(Element::Face(vertex0, vtn_indices[i+1], vtn_indices[i+2]));
        }

        Ok((vtn_indices.len() - 2) as u32)
    }

    fn parse_elements(&mut self, elements: &mut Vec<Element>) -> Result<u32, ParseError> {  
        match self.peek() {
            Some("p") => self.parse_point(elements),
            Some("l") => self.parse_line(elements),
            Some("f") => self.parse_face(elements),
            _ => error(self.line_number, format!("Parser error: Line must be a point, line, or face.")),
        }
    }

    fn parse_groups(&mut self, groups: &mut Vec<Group>) -> Result<u32, ParseError> {
        self.expect("g")?;
        let mut groups_parsed = 0;
        loop {
            match self.next_string() {
                Ok(name) if name != "\n" => {
                    groups.push(Group::new(name));
                    groups_parsed += 1;
                }
                _ => break,
            }
        }

        Ok(groups_parsed)
    }

    fn parse_smoothing_group(&mut self, 
        smoothing_groups: &mut Vec<SmoothingGroup>) -> Result<u32, ParseError> {

        self.expect("s")?;
        if let Ok(name) = self.next_string() {
            if name == "off" {
                smoothing_groups.push(SmoothingGroup::new(0));
            } else if let Ok(number) = name.parse::<u32>() {
                smoothing_groups.push(SmoothingGroup::new(number));
            } else {
                return error(self.line_number, format!(
                    "Expected integer or `off` for smoothing group name but got `{}`", name
                ));
            }
        } else {
            return error(self.line_number, format!("Parser error: Invalid smoothing group name."));
        }

        Ok(1)
    }

    fn parse_shape_entries(&self,
        shape_entry_table: &mut Vec<ShapeEntry>,
        elements: &[Element],
        group_entry_table: &[((u32, u32), (u32, u32))],
        smoothing_group_entry_table: &[((u32, u32), u32)]) {

        for &((min_element_index, max_element_index), 
              (min_group_index, max_group_index)) in group_entry_table { 
            
            let groups: Vec<u32> = (min_group_index..max_group_index).collect();
            for i in min_element_index..max_element_index {
                shape_entry_table.push(ShapeEntry::new(i, &groups, 1));
            }
        }
        debug_assert!(shape_entry_table.len() == elements.len());

        for &((min_element_index, max_element_index), 
               smoothing_group_index) in smoothing_group_entry_table {
 
            for i in min_element_index..max_element_index {
                shape_entry_table[(i - 1) as usize].smoothing_group = smoothing_group_index;
            }
        }
        debug_assert!(shape_entry_table.len() == elements.len());
    }

    fn parse_object(&mut self,
        min_vertex_index:  &mut usize,  max_vertex_index:  &mut usize,
        min_texture_index: &mut usize,  max_texture_index: &mut usize,
        min_normal_index:  &mut usize,  max_normal_index:  &mut usize) -> Result<Object, ParseError> {
        
        let object_name = self.parse_object_name()?;

        let mut vertices: Vec<Vertex> = vec![];
        let mut texture_vertices = vec![];
        let mut normal_vertices = vec![];        
        let mut elements = vec![];

        let mut group_entry_table = vec![];
        let mut groups = vec![];
        let mut min_element_group_index = 0;
        let mut max_element_group_index = 0;
        let mut min_group_index = 0;
        let mut max_group_index = 0;

        let mut smoothing_group_entry_table = vec![];        
        let mut smoothing_groups = vec![];
        let mut min_element_smoothing_group_index = 0;
        let mut max_element_smoothing_group_index = 0;
        let mut smoothing_group_index = 0;

        loop {
            match self.peek() {
                Some("g") if groups.is_empty() => {
                    min_element_group_index = 1;
                    max_element_group_index = 1;
                    min_group_index = 1;
                    max_group_index = 1;

                    // Fetch the new groups.
                    let amount_parsed = self.parse_groups(&mut groups)?;
                    // Update range of group indices.
                    max_group_index += amount_parsed;
                }
                Some("g") => {
                    // Save the shape entry ranges for the current group.
                    group_entry_table.push((
                        (min_element_group_index, max_element_group_index), 
                        (min_group_index, max_group_index)
                    ));

                    // Fetch the new groups.
                    let amount_parsed = self.parse_groups(&mut groups)?;
                    // Update range of group indices.
                    min_group_index = max_group_index;
                    max_group_index += amount_parsed;
                    // Update the element indices.
                    min_element_group_index = max_element_group_index;
                }
                Some("s") if smoothing_groups.is_empty() => {
                    min_element_smoothing_group_index = 1;
                    max_element_smoothing_group_index = 1;

                    // Fetch the next smoothing group.
                    self.parse_smoothing_group(&mut smoothing_groups)?;
                    // Update the smoothing group index.
                    smoothing_group_index = 1;
                }
                Some("s") => {
                    // Save the shape entry ranges for the current smoothing group.
                    smoothing_group_entry_table.push((
                        (min_element_smoothing_group_index, max_element_smoothing_group_index),
                        smoothing_group_index
                    ));

                    // Fetch the next smoothing group.
                    self.parse_smoothing_group(&mut smoothing_groups)?;
                    // Update the smoothing group index.
                    smoothing_group_index += 1;
                    //Update the element indices.
                    min_element_smoothing_group_index = max_element_smoothing_group_index;
                }
                Some("v")  => {
                    let vertex = self.parse_vertex()?;
                    vertices.push(vertex);
                }
                Some("vt") => {
                    let texture_vertex = self.parse_texture_vertex()?;
                    texture_vertices.push(texture_vertex);
                }
                Some("vn") => {
                    let normal_vertex = self.parse_normal_vertex()?;
                    normal_vertices.push(normal_vertex);
                }
                Some("p") | Some("l") | Some("f") => {
                    if groups.is_empty() {
                        groups.push(Default::default());
                        min_element_group_index = 1;
                        max_element_group_index = 1;
                        min_group_index = 1;
                        max_group_index = 2;
                    }

                    if smoothing_groups.is_empty() {
                        smoothing_groups.push(Default::default());
                        min_element_smoothing_group_index = 1;
                        max_element_smoothing_group_index = 1;
                        smoothing_group_index = 1;
                    }

                    let amount_parsed = self.parse_elements(&mut elements)?;
                    max_element_group_index += amount_parsed;
                    max_element_smoothing_group_index += amount_parsed;
                }
                Some("\n") => {
                    self.skip_one_or_more_newlines()?;
                }
                Some("o") | None => {
                    // At the end of file or object, collect any remaining shapes.
                    group_entry_table.push((
                        (min_element_group_index, max_element_group_index), 
                        (min_group_index, max_group_index)
                    ));
                    min_element_group_index = max_element_group_index;

                    smoothing_group_entry_table.push((
                        (min_element_smoothing_group_index, max_element_smoothing_group_index),
                        smoothing_group_index
                    ));
                    min_element_smoothing_group_index = max_element_smoothing_group_index;

                    break;
                }
                Some(other_st) => {
                    return error(self.line_number, format!(
                        "Parse error: Invalid element declaration in obj file. Got `{}`", other_st
                    ));
                }
            }
        }

        // At the end of file, collect any remaining shapes.
        // Fill in the shape entries for the current group.
        let mut shape_entries = vec![];
        self.parse_shape_entries(
            &mut shape_entries, &elements, &group_entry_table, &smoothing_group_entry_table
        );

        *min_vertex_index  += vertices.len();
        *max_vertex_index  += vertices.len();
        *min_texture_index += texture_vertices.len();
        *max_texture_index += texture_vertices.len();
        *min_normal_index  += normal_vertices.len();
        *max_normal_index  += normal_vertices.len();

        let mut builder = ObjectBuilder::new(vertices, elements);
        builder.with_name(object_name.into())
               .with_texture_vertex_set(texture_vertices)
               .with_normal_vertex_set(normal_vertices)
               .with_group_set(groups)
               .with_smoothing_group_set(smoothing_groups)
               .with_shape_set(shape_entries);

        Ok(builder.build())
    }

    fn parse_objects(&mut self) -> Result<Vec<Object>, ParseError> {
        let mut result = Vec::new();

        let mut min_vertex_index = 1;
        let mut max_vertex_index = 1;
        let mut min_tex_index    = 1;
        let mut max_tex_index    = 1;
        let mut min_normal_index = 1;
        let mut max_normal_index = 1;

        self.skip_zero_or_more_newlines();
        while let Some(_) = self.peek() {
            result.push(self.parse_object(
                &mut min_vertex_index, &mut max_vertex_index,
                &mut min_tex_index,    &mut max_tex_index,
                &mut min_normal_index, &mut max_normal_index
            )?);
            self.skip_zero_or_more_newlines();
        }

        Ok(result)
    }

    pub fn parse(&mut self) -> Result<ObjectSet, ParseError> {
        self.parse_objects().map(|objs| ObjectSet::new(objs))
    }
}

#[cfg(test)]
mod primitive_tests {
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
}

#[cfg(test)]
mod vertex_tests {
    use crate::object::Vertex;


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
        assert_eq!(parser.next(), Some("\n"));
        assert_eq!(
            parser.parse_vertex(), 
            Ok(Vertex { x: -14.299248, y: 1.700244, z: 8.468981, w: 1.329624 })
        );
    }
}

#[cfg(test)]
mod texture_vertex_tests {
    use crate::object::TextureVertex;


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
        assert_eq!(parser.next(), Some("\n"));
        assert_eq!(
            parser.parse_texture_vertex(),
            Ok(TextureVertex { u: -27.6068, v: 31.1438, w: 27.2099 })
        );
    }
}

#[cfg(test)]
mod normal_vertex_tests {
    use crate::object::NormalVertex;


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
        assert_eq!(parser.next(), Some("\n"));
        assert_eq!(
            parser.parse_normal_vertex(),
            Ok(NormalVertex { i: -27.6068, j: 31.1438, k: 27.2099 })
        );        
    }
}

#[cfg(test)]
mod object_tests {
    #[test]
    fn test_parse_object_name1() {
        let mut parser = super::Parser::new("o object_name \n\n");
        assert_eq!(parser.parse_object_name(), Ok("object_name"));
    }

    #[test]
    fn test_parse_object_name2() {
        let mut parser = super::Parser::new("o object_name");
        assert!(parser.parse_object_name().is_err());
    }
}

#[cfg(test)]
mod vtn_index_tests {
    use crate::object::VTNIndex;
    use super::{Parser, ParseError};
    use quickcheck;
    use rand::Rng;


    #[derive(Clone, Debug)]
    struct VTNIndexParserModel(VTNIndex, String);

    impl VTNIndexParserModel {
        fn new(vtn_index: VTNIndex, string: String) -> VTNIndexParserModel {
            VTNIndexParserModel(vtn_index, string)
        }

        fn parse(&self) -> Result<VTNIndex, ParseError> { 
            Ok(self.0) 
        }
    }

    impl quickcheck::Arbitrary for VTNIndexParserModel {
        fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
            use quickcheck::Arbitrary;

            let mut rng = rand::thread_rng();
            let vtn_index_type = rng.gen_range(0, 4);
            let vtn_index = match vtn_index_type {
                0 => VTNIndex::V(Arbitrary::arbitrary(g)),
                1 => VTNIndex::VT(Arbitrary::arbitrary(g), Arbitrary::arbitrary(g)),
                2 => VTNIndex::VN(Arbitrary::arbitrary(g), Arbitrary::arbitrary(g)),
                _ => VTNIndex::VTN(
                    Arbitrary::arbitrary(g), Arbitrary::arbitrary(g), Arbitrary::arbitrary(g)
                ),
            };

            let string = match vtn_index {
                VTNIndex::V(v) => format!("{}", v),
                VTNIndex::VT(v, tv) => format!("{}/{}", v, tv),
                VTNIndex::VN(v, nv) => format!("{}//{}", v, nv),
                VTNIndex::VTN(v, tv, nv) => format!("{}/{}/{}", v, tv, nv),
            };

            VTNIndexParserModel::new(vtn_index, string)
        }
    }


    #[test]
    fn prop_parser_vertex_encode_decode_inverses() {
        fn property(vtn_model: VTNIndexParserModel) -> bool {
            let result = Parser::new(&vtn_model.1).parse_vtn_index();
            let expected = vtn_model.parse();

            result == expected
        }
        quickcheck::quickcheck(property as fn(VTNIndexParserModel) -> bool);
    }


    #[test]
    fn test_parse_vtn_index1() {
        let mut parser = super::Parser::new("1291");
        let expected = VTNIndex::V(1291);
        let result = parser.parse_vtn_index();
        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn test_parse_vtn_index2() {
        let mut parser = super::Parser::new("1291/1315");
        let expected = VTNIndex::VT(1291, 1315);
        let result = parser.parse_vtn_index();
        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn test_parse_vtn_index3() {
        let mut parser = super::Parser::new("1291/1315/1314");
        let expected = VTNIndex::VTN(1291, 1315, 1314);
        let result = parser.parse_vtn_index();
        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn test_parse_vtn_index4() {
        let mut parser = super::Parser::new("1291//1315");
        let expected = VTNIndex::VN(1291, 1315);
        let result = parser.parse_vtn_index();
        assert_eq!(result, Ok(expected));
    }

}

#[cfg(test)]
mod element_tests {
    use crate::object::{Element, VTNIndex};

    #[test]
    fn test_parse_point1() {
        let mut parser = super::Parser::new("p 1 2 3 4 \n");
        let mut result = vec![];
        let expected = vec![
            Element::Point(VTNIndex::V(1)), Element::Point(VTNIndex::V(2)),
            Element::Point(VTNIndex::V(3)), Element::Point(VTNIndex::V(4)),
        ];
        assert!(parser.parse_elements(&mut result).is_ok());
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_point2() {
        let mut parser = super::Parser::new("p 1 1/2 3 4/5");
        let mut result = vec![];
        assert!(parser.parse_elements(&mut result).is_err());
    }

    #[test]
    fn test_parse_line1() {
        let mut parser = super::Parser::new("l 297 38 118 108 \n");
        let mut result = vec![];
        let expected = vec![
            Element::Line(VTNIndex::V(297), VTNIndex::V(38)), 
            Element::Line(VTNIndex::V(38),  VTNIndex::V(118)),
            Element::Line(VTNIndex::V(118), VTNIndex::V(108)),
        ];
        assert!(parser.parse_elements(&mut result).is_ok());
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_line2() {
        let mut parser = super::Parser::new("l 297/38 118/108 \n");
        let mut result = vec![];
        let expected = vec![
            Element::Line(VTNIndex::VT(297, 38), VTNIndex::VT(118, 108)),
        ];
        assert!(parser.parse_elements(&mut result).is_ok());
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_line3() {
        let mut parser = super::Parser::new("l 297/38 118/108 324/398 \n");
        let mut result = vec![];
        let expected = vec![
            Element::Line(VTNIndex::VT(297, 38), VTNIndex::VT(118, 108)),
            Element::Line(VTNIndex::VT(118, 108), VTNIndex::VT(324, 398)),
        ];
        assert!(parser.parse_elements(&mut result).is_ok());
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_line4() {
        let mut parser = super::Parser::new("l 297/38 118 324 \n");
        let mut result = vec![];
        assert!(parser.parse_elements(&mut result).is_err());
    }

    #[test]
    fn test_parse_line5() {
        let mut parser = super::Parser::new("l 297 118/108 324/398 \n");
        let mut result = vec![];
        assert!(parser.parse_elements(&mut result).is_err());
    }

    #[test]
    fn test_parse_face1() {
        let mut parser = super::Parser::new("f 297 118 108\n");
        let mut result = vec![];
        let expected = vec![
            Element::Face(VTNIndex::V(297), VTNIndex::V(118), VTNIndex::V(108)),
        ];
        assert!(parser.parse_elements(&mut result).is_ok());
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_face2() {
        let mut parser = super::Parser::new("f 297 118 108 324\n");
        let mut result = vec![];
        let expected = vec![
            Element::Face(VTNIndex::V(297), VTNIndex::V(118), VTNIndex::V(108)),
            Element::Face(VTNIndex::V(297), VTNIndex::V(108), VTNIndex::V(324)),
        ];
        assert!(parser.parse_elements(&mut result).is_ok());
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_face3() {
        let mut parser = super::Parser::new("f 297 118 108 324 398 \n");
        let mut result = vec![];
        let expected = vec![
            Element::Face(VTNIndex::V(297), VTNIndex::V(118), VTNIndex::V(108)),
            Element::Face(VTNIndex::V(297), VTNIndex::V(108), VTNIndex::V(324)),
            Element::Face(VTNIndex::V(297), VTNIndex::V(324), VTNIndex::V(398)),
        ];
        assert!(parser.parse_elements(&mut result).is_ok());
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_face4() {
        let mut parser = super::Parser::new("f 297 118 \n");
        let mut result = vec![];
        assert!(parser.parse_face(&mut result).is_err());
    }

    #[test]
    fn test_parse_face5() {
        let mut parser = super::Parser::new(
            "f 34184//34184 34088//34088 34079//34079 34084//34084 34091//34091 34076//34076\n"
        );
        let mut result = vec![];
        let expected = vec![
            Element::Face(VTNIndex::VN(34184,34184), VTNIndex::VN(34088,34088), VTNIndex::VN(34079,34079)),
            Element::Face(VTNIndex::VN(34184,34184), VTNIndex::VN(34079,34079), VTNIndex::VN(34084,34084)),
            Element::Face(VTNIndex::VN(34184,34184), VTNIndex::VN(34084,34084), VTNIndex::VN(34091,34091)),
            Element::Face(VTNIndex::VN(34184,34184), VTNIndex::VN(34091,34091), VTNIndex::VN(34076,34076)),
        ];
        parser.parse_elements(&mut result).unwrap();
        assert_eq!(result, expected);
    }
}

#[cfg(test)]
mod group_tests {
    use crate::object::Group;

    #[test]
    fn parse_group_name1() {
        let mut parser = super::Parser::new("g group");
        let mut result = vec![];
        let expected = vec![Group::new("group")];
        let parsed = parser.parse_groups(&mut result);

        assert!(parsed.is_ok());
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_group_name2() {
        let mut parser = super::Parser::new("g group1 group2 group3");
        let mut result = vec![];
        let parsed = parser.parse_groups(&mut result);
        let expected = vec![
            Group::new("group1"), Group::new("group2"), Group::new("group3")
        ];

        assert!(parsed.is_ok());
        assert_eq!(result, expected);
    }
}

#[cfg(test)]
mod smoothing_group_tests {
    use crate::object::SmoothingGroup;

    #[test]
    fn test_smoothing_group_name1() {
        let mut parser = super::Parser::new("s off");
        let mut result = vec![];
        let parsed = parser.parse_smoothing_group(&mut result);
        let expected = vec![SmoothingGroup::new(0)];

        assert!(parsed.is_ok());
        assert_eq!(result, expected);
    }

    #[test]
    fn test_smoothing_group_name2() {
        let mut parser = super::Parser::new("s 0");
        let mut result = vec![];
        let parsed = parser.parse_smoothing_group(&mut result);
        let expected = vec![SmoothingGroup::new(0)];
        
        assert!(parsed.is_ok());
        assert_eq!(result, expected);
    }

    #[test]
    fn test_smoothing_group_name3() {
        let mut parser = super::Parser::new("s 3434");
        let mut result = vec![];
        let parsed = parser.parse_smoothing_group(&mut result);
        let expected = vec![SmoothingGroup::new(3434)];
        
        assert!(parsed.is_ok());
        assert_eq!(result, expected);
    }
}

#[cfg(test)]
mod objectset_tests {
    use crate::object::{
        ObjectSet, ObjectBuilder,
        Vertex, NormalVertex, Element, VTNIndex, 
        Group, SmoothingGroup, ShapeEntry,
    };

    fn test_case() -> (Result<ObjectSet, super::ParseError>, Result<ObjectSet, super::ParseError>){
        let obj_file =r"                      \
            o object1                         \
            g cube                            \
            v  0.0  0.0  0.0                  \
            v  0.0  0.0  1.0                  \
            v  0.0  1.0  0.0                  \
            v  0.0  1.0  1.0                  \
            v  1.0  0.0  0.0                  \
            v  1.0  0.0  1.0                  \
            v  1.0  1.0  0.0                  \
            v  1.0  1.0  1.0                  \
                                              \
            vn  0.0  0.0  1.0                 \
            vn  0.0  0.0 -1.0                 \
            vn  0.0  1.0  0.0                 \
            vn  0.0 -1.0  0.0                 \
            vn  1.0  0.0  0.0                 \
            vn -1.0  0.0  0.0                 \
                                              \
            f  1//2  7//2  5//2               \
            f  1//2  3//2  7//2               \
            f  1//6  4//6  3//6               \
            f  1//6  2//6  4//6               \
            f  3//3  8//3  7//3               \
            f  3//3  4//3  8//3               \
            f  5//5  7//5  8//5               \
            f  5//5  8//5  6//5               \
            f  1//4  5//4  6//4               \
            f  1//4  6//4  2//4               \
            f  2//1  6//1  8//1               \
            f  2//1  8//1  4//1               \
        ";
        let mut builder = ObjectBuilder::new(
            vec![
                Vertex { x: 0.0,  y: 0.0, z: 0.0, w: 1.0 },
                Vertex { x: 0.0,  y: 0.0, z: 1.0, w: 1.0 },
                Vertex { x: 0.0,  y: 1.0, z: 0.0, w: 1.0 },
                Vertex { x: 0.0,  y: 1.0, z: 1.0, w: 1.0 },
                Vertex { x: 1.0,  y: 0.0, z: 0.0, w: 1.0 },
                Vertex { x: 1.0,  y: 0.0, z: 1.0, w: 1.0 },
                Vertex { x: 1.0,  y: 1.0, z: 0.0, w: 1.0 },
                Vertex { x: 1.0,  y: 1.0, z: 1.0, w: 1.0 },
            ],
            vec![
                Element::Face(VTNIndex::VN(1,2), VTNIndex::VN(7,2), VTNIndex::VN(5,2)),
                Element::Face(VTNIndex::VN(1,2), VTNIndex::VN(3,2), VTNIndex::VN(7,2)),
                Element::Face(VTNIndex::VN(1,6), VTNIndex::VN(4,6), VTNIndex::VN(3,6)),
                Element::Face(VTNIndex::VN(1,6), VTNIndex::VN(2,6), VTNIndex::VN(4,6)),
                Element::Face(VTNIndex::VN(3,3), VTNIndex::VN(8,3), VTNIndex::VN(7,3)),
                Element::Face(VTNIndex::VN(3,3), VTNIndex::VN(4,3), VTNIndex::VN(8,3)),
                Element::Face(VTNIndex::VN(5,5), VTNIndex::VN(7,5), VTNIndex::VN(8,5)),
                Element::Face(VTNIndex::VN(5,5), VTNIndex::VN(8,5), VTNIndex::VN(6,5)),
                Element::Face(VTNIndex::VN(1,4), VTNIndex::VN(5,4), VTNIndex::VN(6,4)),
                Element::Face(VTNIndex::VN(1,4), VTNIndex::VN(6,4), VTNIndex::VN(2,4)),
                Element::Face(VTNIndex::VN(2,1), VTNIndex::VN(6,1), VTNIndex::VN(8,1)),
                Element::Face(VTNIndex::VN(2,1), VTNIndex::VN(8,1), VTNIndex::VN(4,1)),
            ],
        );
        builder
        .with_name(String::from("object1"))
        .with_normal_vertex_set(vec![
            NormalVertex { i:  0.0, j:  0.0, k:  1.0 },
            NormalVertex { i:  0.0, j:  0.0, k: -1.0 },
            NormalVertex { i:  0.0, j:  1.0, k:  0.0 },
            NormalVertex { i:  0.0, j: -1.0, k:  0.0 },
            NormalVertex { i:  1.0, j:  0.0, k:  0.0 },
            NormalVertex { i: -1.0, j:  0.0, k:  0.0 },
        ])
        .with_group_set(vec![Group::new("cube")])
        .with_smoothing_group_set(vec![SmoothingGroup::new(0)])
        .with_shape_set(vec![
            ShapeEntry { element: 1,  groups: vec![1], smoothing_group: 1 },
            ShapeEntry { element: 2,  groups: vec![1], smoothing_group: 1 },
            ShapeEntry { element: 3,  groups: vec![1], smoothing_group: 1 },
            ShapeEntry { element: 4,  groups: vec![1], smoothing_group: 1 },
            ShapeEntry { element: 5,  groups: vec![1], smoothing_group: 1 },
            ShapeEntry { element: 6,  groups: vec![1], smoothing_group: 1 },
            ShapeEntry { element: 7,  groups: vec![1], smoothing_group: 1 },
            ShapeEntry { element: 8,  groups: vec![1], smoothing_group: 1 },
            ShapeEntry { element: 9,  groups: vec![1], smoothing_group: 1 },
            ShapeEntry { element: 10, groups: vec![1], smoothing_group: 1 },
            ShapeEntry { element: 11, groups: vec![1], smoothing_group: 1 },
            ShapeEntry { element: 12, groups: vec![1], smoothing_group: 1 },
        ]);
        let expected = ObjectSet::new(vec![builder.build()]);
        let mut parser = super::Parser::new(obj_file);
        let result = parser.parse();

        (result, Ok(expected))
    }

    #[test]
    fn test_parse_object_set1() {
        let (result, expected) = test_case();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_object_set1_tokenwise() {
        let (result_set, expected_set) = test_case();
        let result_set = result_set.unwrap();
        let expected_set = expected_set.unwrap();

        for (result, expected) in result_set.iter().zip(expected_set.iter()) {
            assert_eq!(result.name, expected.name);
            assert_eq!(result.vertex_set, expected.vertex_set);
            assert_eq!(result.texture_vertex_set, expected.texture_vertex_set);
            assert_eq!(result.normal_vertex_set, expected.normal_vertex_set);
            assert_eq!(result.group_set, expected.group_set);
            assert_eq!(result.smoothing_group_set, expected.smoothing_group_set);
            assert_eq!(result.element_set, expected.element_set);
            assert_eq!(result.shape_set, expected.shape_set);
        }
    }
}

