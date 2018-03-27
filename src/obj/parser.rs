use obj::object::{
    ObjectSet, Object, ObjectBuilder,
    Vertex, TextureVertex, NormalVertex,
    GroupName, SmoothingGroupName, Element, VTNIndex, ShapeEntry,
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

    fn next_string(&mut self) -> Result<String, ParseError> {
        match self.next() {
            Some(st) => Ok(st),
            None => self.error(format!("Expected string but got `end of file`."))
        }
    }

    fn expect(&mut self, tag: &str) -> Result<String, ParseError> {
        let st = try!(self.next_string());
        match st == tag {
            true => Ok(st),
            false => self.error(format!("Expected `{}` statement but got: `{}`.", tag, st))
        }
    }

    fn parse_f32(&mut self) -> Result<f32, ParseError> {
        let st = try!(self.next_string());
        match st.parse::<f32>() {
            Ok(val) => Ok(val),
            Err(_) => self.error(
                format!("Expected `f32` but got `{}`.", st)
            ),
        }
    }

    fn parse_u32(&mut self) -> Result<u32, ParseError> {
        let st = try!(self.next_string());
        match st.parse::<u32>() {
            Ok(val) => Ok(val),
            Err(_) => self.error(format!("Expected integer but got `{}`.", st)),
        }
    }

    fn try_once<P, T>(&mut self, parser: P) -> Option<T> where P: FnOnce(&str) -> Option<T> {
        match self.peek() {
            Some(st) => parser(&st).map(|got| { self.advance(); got }),
            None => None,
        }
    }

    fn parse_vertex(&mut self) -> Result<Vertex, ParseError> {
        try!(self.expect("v"));
 
        let x = try!(self.parse_f32());
        let y = try!(self.parse_f32());
        let z = try!(self.parse_f32());
        let mw = self.try_once(|st| st.parse::<f32>().ok());
        let w = mw.unwrap_or(1.);

        Ok(Vertex { x: x, y: y, z: z, w: w })
    }

    fn parse_texture_vertex(&mut self) -> Result<TextureVertex, ParseError> {
        try!(self.expect("vt"));

        let u = try!(self.parse_f32());
        let mv = self.try_once(|st| st.parse::<f32>().ok());
        let v = mv.unwrap_or(0.);
        let mw = self.try_once(|st| st.parse::<f32>().ok());
        let w = mw.unwrap_or(0.);

        Ok(TextureVertex { u: u, v: v, w: w })
    }

    fn parse_normal_vertex(&mut self) -> Result<NormalVertex, ParseError> {
        try!(self.expect("vn"));

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
        try!(self.expect("\n"));
        self.skip_zero_or_more_newlines();
        Ok(())
    }

    fn parse_object_name(&mut self) -> Result<String, ParseError> {
        match self.peek().as_ref().map(|st| &st[..]) {
            Some("o") => {
                try!(self.expect("o"));
                let object_name = self.next_string();
                try!(self.skip_one_or_more_newlines());
                object_name
            }
            _ => Ok(String::from(""))
        }
    }

    fn parse_vn(&mut self, st: &str) -> Result<VTNIndex, ParseError> {
        if let Some(v_index_in_str) = st.find("//") {
            let v_index = match st[0..v_index_in_str].parse::<u32>() {
                Ok(val) => val,
                Err(_) => return self.error(format!("Expected `vertex` index but got `{}`", st))
            };
            let vn_index = match st[v_index_in_str+2..].parse::<u32>() {
                Ok(val) => val,
                Err(_) => return self.error(format!("Expected `normal` index but got `{}`", st))
            };

            return Ok(VTNIndex::VN(v_index, vn_index));
        } else {
            return self.error(format!("Expected `vertex//normal` index but got `{}`", st))
        }
    }

    fn parse_vt(&mut self, st: &str) -> Result<VTNIndex, ParseError> {
        if let Some(v_index_in_str) = st.find("/") {
            let v_index = match st[0..v_index_in_str].parse::<u32>() {
                Ok(val) => val,
                Err(_) => return self.error(format!("Expected `vertex` index but got `{}`", st))
            };
            let vt_index = match st[v_index_in_str+1..].parse::<u32>() {
                Ok(val) => val,
                Err(_) => return self.error(format!("Expected `texture` index but got `{}`", st))
            };

            return Ok(VTNIndex::VT(v_index, vt_index));
        } else {
            return self.error(format!("Expected `vertex/texture` index but got `{}`", st))
        }
    }

    fn parse_vtn(&mut self, st: &str) -> Result<VTNIndex, ParseError> {
        let v_index_in_str = match st.find("/") {
            Some(val) => val,
            None => return self.error(format!("Expected `vertex` index but got `{}`", st))
        };
        let v_index = match st[0..v_index_in_str].parse::<u32>() {
            Ok(val) => val,
            Err(_) => return self.error(format!("Expected `vertex` index but got `{}`", st))
        };
        let vt_index_in_str = match st[(v_index_in_str + 1)..].find("/") {
            Some(val) => v_index_in_str + 1 + val,
            None => return self.error(format!("Expected `texture` index but got `{}`", st))
        };
        let vt_index = match st[(v_index_in_str + 1)..vt_index_in_str].parse::<u32>() {
            Ok(val) => val,
            Err(_) => return self.error(format!("Expected `texture` index but got `{}`", st))
        };
        let vn_index = match st[(vt_index_in_str + 1)..].parse::<u32>() {
            Ok(val) => val,
            Err(_) => return self.error(format!("Expected `normal` index but got `{}`", st))
        };
   
        Ok(VTNIndex::VTN(v_index, vt_index, vn_index))
    }

    fn parse_v(&mut self, st: &str) -> Result<VTNIndex, ParseError> {
        match st.parse::<u32>() {
            Ok(val) => Ok(VTNIndex::V(val)),
            Err(_) => return self.error(format!("Expected `vertex` index but got `{}`", st))
        }
    }

    fn parse_vtn_index(&mut self) -> Result<VTNIndex, ParseError> {
        let st = try!(self.next_string());
        
        match self.parse_vn(&st) {
            Ok(val) => return Ok(val),
            Err(_) => {},
        }
        match self.parse_vtn(&st) {
            Ok(val) => return Ok(val),
            Err(_) => {},
        }
        match self.parse_vt(&st) {
            Ok(val) => return Ok(val),
            Err(_) => {},
        }
        match self.parse_v(&st) {
            Ok(val) => return Ok(val),
            Err(_) => {},
        }

        self.error(format!("Expected `vertex/texture/normal` index but got `{}`", st))
    }

    fn parse_point(&mut self, elements: &mut Vec<Element>) -> Result<u32, ParseError> {
        try!(self.expect("p"));

        let v_index = try!(self.parse_u32());
        elements.push(Element::Point(VTNIndex::V(v_index)));
        let mut elements_parsed = 1;
        loop {
            match self.next_string().as_ref().map(|st| &st[..]) {
                Ok("\n") | Err(_) => break,
                Ok(st) => match st.parse::<u32>() {
                    Ok(v_index) => { 
                        elements.push(Element::Point(VTNIndex::V(v_index)));
                        elements_parsed += 1;
                    }
                    Err(_) => return self.error(format!("Expected integer but got `{}`.", st))
                }
            }
        }

        Ok(elements_parsed)
    }

    fn parse_line(&mut self, elements: &mut Vec<Element>) -> Result<u32, ParseError> {
        try!(self.expect("l"));

        let current_vtn_index = try!(self.parse_vtn_index());
        let next_vtn_index = try!(self.parse_vtn_index());
        let mut vtn_indices = Vec::new();
        vtn_indices.push(current_vtn_index);
        vtn_indices.push(next_vtn_index);
        loop {
            match self.parse_vtn_index() {
                Ok(vtn_index) => {
                    vtn_indices.push(vtn_index);
                },
                Err(_) => {
                    break;
                }
            }
        }

        // Verify that each VTN index has the same type and if of a valid form.
        for i in 1..vtn_indices.len() {
            if !vtn_indices[i].has_same_type_as(&vtn_indices[0]) {
                return self.error(
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
        try!(self.expect("f"));

        let mut vtn_indices = Vec::new();
        loop {
            match self.parse_vtn_index() {
                Ok(vtn_index) => {
                    vtn_indices.push(vtn_index);
                },
                Err(_) => {
                    break;
                }
            }
        }

        // Check that there are enough vtn indices.
        if vtn_indices.len() < 3 {
            return self.error(
                format!("A face element must have at least three vertices.")
            );  
        }

        // Verify that each VTN index has the same type and if of a valid form.
        for i in 1..vtn_indices.len() {
            if !vtn_indices[i].has_same_type_as(&vtn_indices[0]) {
                return self.error(
                    format!("Every vertex/texture/normal index must have the same form.")
                );
            }
        }

        // Triangulate the polygon with a triangle fan.
        let vertex0 = vtn_indices[0];
        for i in 0..vtn_indices.len()-2 {
            elements.push(Element::Face(vertex0, vtn_indices[i+1], vtn_indices[i+2]));
        }

        Ok((vtn_indices.len() - 2) as u32)
    }

    fn parse_elements(&mut self, elements: &mut Vec<Element>) -> Result<u32, ParseError> {  
        match self.peek().as_ref().map(|st| &st[..]) {
            Some("p") => self.parse_point(elements),
            Some("l") => self.parse_line(elements),
            Some("f") => self.parse_face(elements),
            _ => self.error(format!("Parser error: Line must be a point, line, or face.")),
        }
    }

    fn parse_groups(&mut self, groups: &mut Vec<GroupName>) -> Result<u32, ParseError> {
        try!(self.expect("g"));
        let mut parsed = 0;
        loop {
            match self.next_string().as_ref().map(|st| &st[..]) {
                Ok("\n") | Err(_) => break,
                Ok(name) => groups.push(GroupName::new(name)),
            }
            parsed += 1;
        }

        Ok(parsed)
    }

    fn parse_smoothing_group(&mut self) -> Result<SmoothingGroupName, ParseError> {
        try!(self.expect("s"));
        match self.next_string() {
            Ok(name) => {
                if name == "off" {
                    return Ok(SmoothingGroupName::new(0));
                }

                match name.parse::<u32>() {
                    Ok(number) => Ok(SmoothingGroupName::new(number)),
                    Err(_) => self.error(format!(
                        "Expected integer or `off` for smoothing group name but got `{}`", name)
                    )
                }
            }
            Err(_) => self.error(format!("Parser error: Invalid smoothing group name.")),
        }
    }

    fn parse_shape_entries(&self,
        shape_entries: &mut Vec<ShapeEntry>,
        min_element_index: u32, max_element_index: u32,
        min_group_index: u32, max_group_index: u32, 
        min_smoothing_group_index: u32, max_smoothing_group_index: u32) {

        let mut groups = vec![];
        for i in min_group_index..max_group_index {
            groups.push(i);
        }
        let mut smoothing_groups = vec![];
        for i in min_smoothing_group_index..max_smoothing_group_index {
            smoothing_groups.push(i);
        }

        for i in min_element_index..max_element_index {
            shape_entries.push(ShapeEntry::new(i, &groups, &smoothing_groups));
        }
    }

    fn parse_object(&mut self) -> Result<Object, ParseError> {
        let mut vertices = vec![];
        let mut texture_vertices = vec![];
        let mut normal_vertices = vec![];
        
        let mut elements = vec![];
        let mut min_element_index = 1;
        let mut max_element_index = 1;
        
        let mut groups = vec![GroupName::new("default")];
        let mut min_group_index = 1;
        let mut max_group_index = 1;
        
        let mut shape_entries = vec![];
        //let mut smoothing_groups = vec![];
        let mut min_smoothing_group_index = 1;
        let mut max_smoothing_group_index = 1;
        loop {
            match self.peek().as_ref().map(|st| &st[..]) {
                Some("o")  => { 
                    self.parse_object_name();
                }
                Some("g")  => {
                    // Fill in the shape entries for the current group.
                    self.parse_shape_entries(
                        &mut shape_entries,
                        min_element_index, max_element_index,
                        min_group_index, max_group_index, 
                        min_smoothing_group_index, max_smoothing_group_index
                    );

                    // Fetch the new groups.
                    let amount_parsed = match self.parse_groups(&mut groups) {
                        Ok(got) => got,
                        Err(err) => return Err(err)
                    };
                    // Update range of group indices.
                    min_group_index = max_group_index;
                    max_group_index += amount_parsed;
                    // Update the element indices.
                    min_element_index = max_element_index;
                }
                Some("v")  => {
                    let vertex = match self.parse_vertex() {
                        Ok(got) => got,
                        Err(err) => return Err(err),
                    };
                    vertices.push(vertex);
                }
                Some("vt") => {
                    let texture_vertex = match self.parse_texture_vertex() {
                        Ok(got) => got,
                        Err(err) => return Err(err),
                    };
                    texture_vertices.push(texture_vertex);
                }
                Some("vn") => {
                    let normal_vertex = match self.parse_normal_vertex() {
                        Ok(got) => got,
                        Err(err) => return Err(err),
                    };
                    normal_vertices.push(normal_vertex);
                }
                Some("p") | Some("l") | Some("f") => {
                    let amount_parsed = match self.parse_elements(&mut elements) {
                        Ok(got) => got,
                        Err(err) => return Err(err)
                    };
                    max_element_index += amount_parsed;
                }
                Some("\n") => { 
                    self.skip_one_or_more_newlines();
                }
                Some(other_st) => {
                    return self.error(format!(
                        "Parse error: Invalid element declaration in obj file. Got `{}`", other_st
                    ));
                }
                None => {
                    break;
                }
            }
        }

        let mut builder = ObjectBuilder::new(vertices, elements);
        builder.with_texture_vertex_set(texture_vertices)
               .with_normal_vertex_set(normal_vertices)
               .with_group_set(groups)
               .with_shape_set(shape_entries);

        Ok(builder.build())
    }

    fn parse(&mut self) -> Result<ObjectSet, ParseError> {
        self.parse_object().map(|obj| ObjectSet::new(vec![obj]))
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
    use obj::object::Vertex;

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
}

#[cfg(test)]
mod texture_vertex_tests {
    use obj::object::TextureVertex;

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
}

#[cfg(test)]
mod normal_vertex_tests {
    use obj::object::NormalVertex;

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

#[cfg(test)]
mod object_tests {
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
}

#[cfg(test)]
mod vtn_index_tests {
    use obj::object::VTNIndex;

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
    use obj::object::{Element, VTNIndex};

    #[test]
    fn test_parse_point1() {
        let mut parser = super::Parser::new("p 1 2 3 4 \n");
        let mut result = Vec::new();
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
        let mut result = Vec::new();
        assert!(parser.parse_elements(&mut result).is_err());
    }

    #[test]
    fn test_parse_line1() {
        let mut parser = super::Parser::new("l 297 38 118 108 \n");
        let mut result = Vec::new();
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
        let mut result = Vec::new();
        let expected = vec![
            Element::Line(VTNIndex::VT(297, 38), VTNIndex::VT(118, 108)),
        ];
        assert!(parser.parse_elements(&mut result).is_ok());
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_line3() {
        let mut parser = super::Parser::new("l 297/38 118/108 324/398 \n");
        let mut result = Vec::new();
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
        let mut result = Vec::new();
        assert!(parser.parse_elements(&mut result).is_err());
    }

    #[test]
    fn test_parse_line5() {
        let mut parser = super::Parser::new("l 297 118/108 324/398 \n");
        let mut result = Vec::new();
        assert!(parser.parse_elements(&mut result).is_err());
    }

    #[test]
    fn test_parse_face1() {
        let mut parser = super::Parser::new("f 297 118 108\n");
        let mut result = Vec::new();
        let expected = vec![
            Element::Face(VTNIndex::V(297), VTNIndex::V(118), VTNIndex::V(108)),
        ];
        assert!(parser.parse_elements(&mut result).is_ok());
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_face2() {
        let mut parser = super::Parser::new("f 297 118 108 324\n");
        let mut result = Vec::new();
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
        let mut result = Vec::new();
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
        let mut result = Vec::new();
        assert!(parser.parse_face(&mut result).is_err());
    }

    #[test]
    fn test_parse_face5() {
        let mut parser = super::Parser::new(
            "f 34184//34184 34088//34088 34079//34079 34084//34084 34091//34091 34076//34076\n"
        );
        let mut result = Vec::new();
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
    use obj::object::GroupName;

    #[test]
    fn parse_group_name1() {
        let mut parser = super::Parser::new("g group");
        let mut result = Vec::new();
        let expected = vec![GroupName::new("group")];
        let parsed = parser.parse_groups(&mut result);

        assert!(parsed.is_ok());
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_group_name2() {
        let mut parser = super::Parser::new("g group1 group2 group3");
        let mut result = Vec::new();
        let parsed = parser.parse_groups(&mut result);
        let expected = vec![
            GroupName::new("group1"), GroupName::new("group2"), GroupName::new("group3")
        ];

        assert!(parsed.is_ok());
        assert_eq!(result, expected);
    }
}

#[cfg(test)]
mod smoothing_group_tests {
    use obj::object::SmoothingGroupName;

    #[test]
    fn test_smoothing_group_name1() {
        let mut parser = super::Parser::new("s off");
        let result = parser.parse_smoothing_group();
        let expected = SmoothingGroupName::new(0);
        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn test_smoothing_group_name2() {
        let mut parser = super::Parser::new("s 0");
        let result = parser.parse_smoothing_group();
        let expected = SmoothingGroupName::new(0);
        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn test_smoothing_group_name3() {
        let mut parser = super::Parser::new("s 3434");
        let result = parser.parse_smoothing_group();
        let expected = SmoothingGroupName::new(3434);
        assert_eq!(result, Ok(expected));
    }
}

#[cfg(test)]
mod objectset_tests {
    use obj::object::{
        ObjectSet, Object, ObjectBuilder,
        GroupName, Vertex, NormalVertex, Element, VTNIndex, ShapeEntry,
    };

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
        .with_group_set(vec![GroupName::new("cube")])
        .with_shape_set(vec![
            ShapeEntry { element: 1,  groups: vec![1], smoothing_groups: vec![] },
            ShapeEntry { element: 2,  groups: vec![1], smoothing_groups: vec![] },
            ShapeEntry { element: 3,  groups: vec![1], smoothing_groups: vec![] },
            ShapeEntry { element: 4,  groups: vec![1], smoothing_groups: vec![] },
            ShapeEntry { element: 5,  groups: vec![1], smoothing_groups: vec![] },
            ShapeEntry { element: 6,  groups: vec![1], smoothing_groups: vec![] },
            ShapeEntry { element: 7,  groups: vec![1], smoothing_groups: vec![] },
            ShapeEntry { element: 8,  groups: vec![1], smoothing_groups: vec![] },
            ShapeEntry { element: 9,  groups: vec![1], smoothing_groups: vec![] },
            ShapeEntry { element: 10, groups: vec![1], smoothing_groups: vec![] },
            ShapeEntry { element: 11, groups: vec![1], smoothing_groups: vec![] },
            ShapeEntry { element: 12, groups: vec![1], smoothing_groups: vec![] },
        ]);
        let expected = ObjectSet::new(vec![builder.build()]);
        let mut parser = super::Parser::new(obj_file);
        let result = parser.parse();

        assert_eq!(result, Ok(expected));
    }
}

