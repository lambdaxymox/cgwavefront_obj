extern crate quickcheck;
extern crate wavefront;

use quickcheck::{Arbitrary, Gen};
use wavefront::obj::{
    Object, ObjectSet, ObjectBuilder,
    Vertex, TextureVertex, NormalVertex, Element, VTNIndex,
    GroupName, SmoothingGroup, ShapeEntry,
};
use wavefront::obj::{Parser, ParseError};

use std::fmt;
use std::cmp;
use std::str;
use std::convert;
use fmt::Write;


#[derive(Clone, Debug)]
enum MVertex {
    Vertex3(Vertex),
    Vertex4(Vertex),
}

impl MVertex {
    fn to_vertex(&self) -> Vertex { 
        match *self {
            MVertex::Vertex3(v) => v.clone(),
            MVertex::Vertex4(v) => v.clone(),
        }
    }
}

impl fmt::Display for MVertex {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            MVertex::Vertex3(v) => {
                write!(f, "v  {}  {}  {}", v.x, v.y, v.z)
            }
            MVertex::Vertex4(v) => {
                write!(f, "v  {}  {}  {}  {}", v.x, v.y, v.z, v.w)
            }
        }
    }
}

#[derive(Clone, Debug)]
enum MTextureVertex {
    VTU(TextureVertex),
    VTUV(TextureVertex),
    VTUVW(TextureVertex),
}

impl MTextureVertex {
    fn to_vertex(&self) -> TextureVertex {
        match *self {
            MTextureVertex::VTU(tv) => tv.clone(),
            MTextureVertex::VTUV(tv) => tv.clone(),
            MTextureVertex::VTUVW(tv) => tv.clone(),
        }
    }
}

impl fmt::Display for MTextureVertex {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            MTextureVertex::VTU(tv) => write!(f, "vt  {}", tv.u),
            MTextureVertex::VTUV(tv) => write!(f, "vt  {}  {}", tv.u, tv.v),
            MTextureVertex::VTUVW(tv) => write!(f, "vt  {}  {}  {}", tv.u, tv.v, tv.w),
        }
    }
}

impl cmp::PartialEq<TextureVertex> for MTextureVertex {
    fn eq(&self, other: &TextureVertex) -> bool {
        &self.to_vertex() == other
    }
}

impl<'a> cmp::PartialEq<&'a TextureVertex> for MTextureVertex {
    fn eq(&self, other: & &TextureVertex) -> bool { 
        &&self.to_vertex() == other
    }
}

impl quickcheck::Arbitrary for MTextureVertex {
    fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
        let tv_type = g.gen_range(0, 3);
        let u = quickcheck::Arbitrary::arbitrary(g);
        match tv_type {
            0 => {
                MTextureVertex::VTU(TextureVertex { u: u, v: 0.0, w: 0.0 })
            }
            1 => {
                let v = quickcheck::Arbitrary::arbitrary(g);
                MTextureVertex::VTUV(TextureVertex { u: u, v: v, w: 0.0 })
            }
            _ => {
                let v = quickcheck::Arbitrary::arbitrary(g);
                let w = quickcheck::Arbitrary::arbitrary(g);
                MTextureVertex::VTUVW(TextureVertex { u: u, v: v, w: w })
            }
        }
    }
}

#[derive(Clone, Debug)]
struct MNormalVertex(NormalVertex);

impl MNormalVertex {
    fn to_vertex(&self) -> NormalVertex { self.0 }
}

impl fmt::Display for MNormalVertex {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "vn  {}  {}  {}", self.0.i, self.0.j, self.0.k)
    }
}

impl cmp::PartialEq<NormalVertex> for MNormalVertex {
    fn eq(&self, other: &NormalVertex) -> bool { &self.0 == other }
}

impl<'a> cmp::PartialEq<&'a NormalVertex> for MNormalVertex {
    fn eq(&self, other: & &NormalVertex) -> bool { &&self.0 == other }
}

impl quickcheck::Arbitrary for MNormalVertex {
    fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
        let i = quickcheck::Arbitrary::arbitrary(g);
        let j = quickcheck::Arbitrary::arbitrary(g);
        let k = quickcheck::Arbitrary::arbitrary(g);

        MNormalVertex(NormalVertex { i: i, j: j, k: k })
    }
}

#[derive(Clone, Debug, PartialEq)]
struct MSmoothingGroup(SmoothingGroup);

impl MSmoothingGroup {
    fn new(smoothing_group: SmoothingGroup) -> MSmoothingGroup {
        MSmoothingGroup(smoothing_group)
    }

    fn to_smoothing_group(&self) -> SmoothingGroup {
        self.0.clone()
    }
}

impl fmt::Display for MSmoothingGroup {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "s  {}", self.0.as_int())
    }
}

#[derive(Clone, Debug, PartialEq)]
struct MComment(String);

impl MComment {
    fn new(comment: String) -> MComment {
        MComment(comment)
    }
}

impl fmt::Display for MComment {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "# {}", self.0)
    }
}

#[derive(Clone, Debug, PartialEq)]
enum MVTNIndex {
    V(u32),
    VT(u32, u32),
    VN(u32, u32),
    VTN(u32, u32, u32),
}

impl MVTNIndex {
    fn new(vtn_index: VTNIndex) -> MVTNIndex { 
        match vtn_index {
            VTNIndex::V(v) => MVTNIndex::V(v),
            VTNIndex::VT(v, vt) => MVTNIndex::VT(v, vt),
            VTNIndex::VN(v, vn) => MVTNIndex::VN(v, vn),
            VTNIndex::VTN(v, vt, vn) => MVTNIndex::VTN(v, vt, vn),
        }
    }

    fn to_vtn_index(&self) -> VTNIndex {
        match *self {
            MVTNIndex::V(v) => VTNIndex::V(v),
            MVTNIndex::VT(v, vt) => VTNIndex::VT(v, vt),
            MVTNIndex::VN(v, vn) => VTNIndex::VN(v, vn),
            MVTNIndex::VTN(v, vt, vn) => VTNIndex::VTN(v, vt, vn),
        }
    }
}

impl fmt::Display for MVTNIndex {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            MVTNIndex::V(v)           => write!(f, "{}", v),
            MVTNIndex::VT(v, vt)      => write!(f, "{}/{}", v, vt),
            MVTNIndex::VN(v, vn)      => write!(f, "{}//{}", v, vn),
            MVTNIndex::VTN(v, vt, vn) => write!(f, "{}/{}/{}", v, vt, vn),
        }
    }
}

#[derive(Clone, Debug)]
struct MObjectName(String);

impl fmt::Display for MObjectName {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "o {}", self.0)
    }
}

#[derive(Clone, Debug)]
struct MWhitespace(String);

impl MWhitespace {
    fn new(spaces: usize) -> MWhitespace {
        let line = (0..spaces % 79).fold(String::new(), |acc, _| acc + " ");
        MWhitespace(line)
    }
}

impl fmt::Display for MWhitespace {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Debug)]
struct MGroupName(GroupName);

impl MGroupName {
    fn new(group: GroupName) -> MGroupName { MGroupName(group) }

    fn to_group(&self) -> GroupName {
        self.0.clone()
    }
}

impl fmt::Display for MGroupName {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.to_group())
    }
}

#[derive(Clone, Debug)]
enum TextLine {
    V(MVertex),
    VT(MTextureVertex),
    VN(MNormalVertex),
    Comment(MComment),
    S(MSmoothingGroup),
    G(Vec<MGroupName>),
    P(Vec<MVTNIndex>),
    L(Vec<MVTNIndex>),
    F(Vec<MVTNIndex>),
    O(MObjectName),
    EmptyLine(MWhitespace),
}

impl fmt::Display for TextLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            TextLine::V(ref v) => { 
                v.fmt(f)
            }
            TextLine::VT(ref vt) => {
                vt.fmt(f)
            }
            TextLine::VN(ref vn) => { 
                vn.fmt(f)
            }
            TextLine::Comment(ref comment_line) => {
                comment_line.fmt(f)
            }
            TextLine::S(ref s_name) => {
                s_name.fmt(f)
            }
            TextLine::G(ref vec) => {
                let string = vec.iter().fold(String::from("g "), |acc, name| { 
                    acc + " " + &name.to_string() + " "
                });
                string.fmt(f)
            }
            TextLine::P(ref vec) => {
                let string = vec.iter().fold(String::from("p "), |acc, v_index| { 
                    acc + " " + &v_index.to_string() + " "
                });
                string.fmt(f)
            }
            TextLine::L(ref vec) => {
                let string = vec.iter().fold(String::from("l "), |acc, v_index| { 
                    acc + " " + &v_index.to_string() + " "
                });
                string.fmt(f)
            }
            TextLine::F(ref vec) => {
                let string = vec.iter().fold(String::from("f "), |acc, v_index| { 
                    acc + " " + &v_index.to_string() + " "
                });
                string.fmt(f)
            }
            TextLine::O(ref o_name) => {
                o_name.fmt(f)
            }
            TextLine::EmptyLine(ref empty_line) => {
                empty_line.fmt(f)
            }
        }
    }
}

#[derive(Clone, Debug)]
struct ObjectText {
    text: Vec<TextLine>,
}

impl ObjectText {
    fn new(text: Vec<TextLine>) -> ObjectText { 
        ObjectText { text: text }
    }
}

impl fmt::Display for ObjectText {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        self.text.iter().for_each(|line| { write!(f, "{}\n", line).unwrap(); });
        Ok(())
    }
}

impl Arbitrary for ObjectText {
    fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
        unimplemented!()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MParseError {
    message: String,
}

impl MParseError {
    fn new(message: String) -> MParseError { 
        MParseError { message: message }
    }
}

#[derive(Clone, Debug)]
struct ParserModel {
    text: ObjectText,
}

impl ParserModel {
    fn new(text: ObjectText) -> ParserModel {
        ParserModel { text: text }
    }

    fn error<T>(&mut self, err: String) -> Result<T, MParseError> {
        Err(MParseError::new(err))
    }

    fn parse_object_name(&self, index: usize) -> String {
        match self.text.text[index] {
            TextLine::O(ref m_object_name) => m_object_name.0.clone(),
            _ => String::from(""),
        }
    }

    fn parse_v(&mut self, m_vtn_index: MVTNIndex) -> Result<VTNIndex, MParseError> {
        match m_vtn_index {
            MVTNIndex::V(_) => { 
                Ok(m_vtn_index.to_vtn_index())
            }
            _ => { 
                self.error(format!(
                    "Expected `vertex` index but got `{:?}`", m_vtn_index.to_vtn_index())
                )
            }
        }
    } 

    fn parse_vt(&mut self, m_vtn_index: MVTNIndex) -> Result<VTNIndex, MParseError> {
        match m_vtn_index {
            MVTNIndex::VT(_,_) => {
                Ok(m_vtn_index.to_vtn_index())
            }
            _ => { 
                self.error(format!(
                    "Expected `vertex/texture` index but got `{:?}`", m_vtn_index.to_vtn_index())
                )
            }
        }
    }

    fn parse_vn(&mut self, m_vtn_index: MVTNIndex) -> Result<VTNIndex, MParseError> {
        match m_vtn_index {
            MVTNIndex::VN(_,_) => {
                Ok(m_vtn_index.to_vtn_index())
            }
            _ => { 
                self.error(format!(
                    "Expected `vertex//normal` index but got `{:?}`", m_vtn_index.to_vtn_index())
                )
            }
        }
    }

    fn parse_vtn(&mut self, m_vtn_index: MVTNIndex) -> Result<VTNIndex, MParseError> {
        match m_vtn_index {
            MVTNIndex::VN(_,_) => {
                Ok(m_vtn_index.to_vtn_index())
            }
            _ => { 
                self.error(format!(
                    "Expected `vertex/texture/normal` index but got `{:?}`", m_vtn_index.to_vtn_index())
                )
            }
        }
    }

    fn parse_point(&mut self, vec: &[MVTNIndex]) -> Result<Vec<Element>, MParseError> {
        let mut elements = vec![];

        if vec.is_empty() {
            return self.error(format!(
                "Expected at least one `vertex` index but got an empty line.")
            );
        }
         
        for m_vtn_index in vec {
            match m_vtn_index {
                &MVTNIndex::V(_) => {
                    elements.push(Element::Point(m_vtn_index.to_vtn_index()));
                }
                _ => {
                    return self.error(format!(
                        "Expected `vertex` index but got `{:?}`", m_vtn_index)
                    );
                }
            }
        }

        Ok(elements)
    }

    fn parse_line(&self) -> Result<Vec<Element>, MParseError> {
        unimplemented!()
    }

    fn parse_face(&self) -> Result<Vec<Element>, MParseError> {
        unimplemented!()
    }

    fn parse_elements(&self, vec: &[MVTNIndex]) -> Result<VTNIndex, MParseError> {
        unimplemented!()
    }

    fn parse_object(&mut self,
        min_vertex_index:  &mut usize,  max_vertex_index:  &mut usize,
        min_texture_index: &mut usize,  max_texture_index: &mut usize,
        min_normal_index:  &mut usize,  max_normal_index:  &mut usize
    ) -> Result<Object, ParseError> { 
        let object_name = self.parse_object_name(0);

        let mut vertices = vec![];
        let mut texture_vertices = vec![];
        let mut normal_vertices = vec![];        
        let mut elements = vec![];
        
        let mut group_entry_table = vec![];
        let mut groups = vec![];
        let mut min_element_group_index = 1;
        let mut max_element_group_index = 1;
        let mut min_group_index = 1;
        let mut max_group_index = 1;

        let mut smoothing_group_entry_table = vec![];        
        let mut smoothing_groups = vec![];
        let mut min_element_smoothing_group_index = 1;
        let mut max_element_smoothing_group_index = 1;
        let mut min_smoothing_group_index = 1;
        let mut max_smoothing_group_index = 1;

        for text_line in &self.text.text {
            match text_line.clone() {
                TextLine::G(m_groups) => {            
                    // Save the shape entry ranges for the current group.
                    group_entry_table.push((
                        (min_element_group_index, max_element_group_index), 
                        (min_group_index, max_group_index)
                    ));
                    // Fetch the new groups.
                    let amount_parsed = m_groups.len();
                    groups.append(&mut m_groups.iter().map(|mg| mg.to_group()).collect());
                    // Update range of group indices.
                    min_group_index = max_group_index;
                    max_group_index += amount_parsed;
                    // Update the element indices.
                    min_element_group_index = max_element_group_index;
                }
                TextLine::S(m_smoothing_group) => {
                    // Save the shape entry ranges for the current smoothing group.
                    smoothing_group_entry_table.push((
                        (min_element_smoothing_group_index, max_element_smoothing_group_index),
                        (min_smoothing_group_index, max_smoothing_group_index)
                    ));
                    // Fetch the next smoothing group.
                    let amount_parsed = 1;
                    smoothing_groups.push(m_smoothing_group.to_smoothing_group());
                    // Update the range of the smoothing group indices.
                    min_smoothing_group_index = max_smoothing_group_index;
                    max_smoothing_group_index += amount_parsed;
                    //Update the element indices.
                    min_element_smoothing_group_index = max_element_smoothing_group_index;
                }
                TextLine::V(m_v)  => {
                    let vertex = m_v.to_vertex();
                    vertices.push(vertex);
                }
                TextLine::VT(m_vt) => {
                    let texture_vertex = m_vt.to_vertex();
                    texture_vertices.push(texture_vertex);
                }
                TextLine::VN(m_vn) => {
                    let normal_vertex = m_vn.to_vertex();
                    normal_vertices.push(normal_vertex);
                }
                TextLine::P(vec) | TextLine::L(vec) | TextLine::F(vec) => {
                    //let amount_parsed = try!(self.parse_elements(&vec));
                    //max_element_group_index += amount_parsed;
                    //max_element_smoothing_group_index += amount_parsed;
                }
                TextLine::EmptyLine(_) | TextLine::Comment(_) => { 
                    continue;
                }
                TextLine::O(m_object_name) => {
                    // At the end of file or object, collect any remaining shapes.
                    group_entry_table.push((
                        (min_element_group_index, max_element_group_index), 
                        (min_group_index, max_group_index)
                    ));
                    min_element_group_index = max_element_group_index;

                    smoothing_group_entry_table.push((
                        (min_element_smoothing_group_index, max_element_smoothing_group_index),
                        (min_smoothing_group_index, max_smoothing_group_index)
                    ));
                    min_element_smoothing_group_index = max_element_smoothing_group_index;

                    break;
                }
            }
        }

        if groups.is_empty() {
            groups.push(Default::default());
        }

        if smoothing_groups.is_empty() {
            smoothing_groups.push(SmoothingGroup::new(0));
        }

        // At the end of file, collect any remaining shapes.
        // Fill in the shape entries for the current group.
        let mut shape_entries = vec![];
        //self.parse_shape_entries(
        //    &mut shape_entries, &elements, &group_entry_table, &smoothing_group_entry_table
        //);

        *min_vertex_index  += vertices.len();
        *max_vertex_index  += vertices.len();
        *min_texture_index += texture_vertices.len();
        *max_texture_index += texture_vertices.len();
        *min_normal_index  += normal_vertices.len();
        *max_normal_index  += normal_vertices.len();

        let mut builder = ObjectBuilder::new(vertices, elements);
        builder.with_name(object_name)
               .with_texture_vertex_set(texture_vertices)
               .with_normal_vertex_set(normal_vertices)
               .with_group_set(groups)
               .with_smoothing_group_set(smoothing_groups)
               .with_shape_set(shape_entries);

        Ok(builder.build())
    }

    fn parse(&self) -> Result<ObjectSet, MParseError> {
        unimplemented!()
    }
}

impl fmt::Display for ParserModel {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        self.text.fmt(f)
    }
}

impl Arbitrary for ParserModel {
    fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
        let text = Arbitrary::arbitrary(g);
        ParserModel::new(text)
    }
}

#[derive(Clone, Debug)]
struct Machine { 
    model: ParserModel, 
    text: String,
}

impl Machine {
    fn new(model: ParserModel) -> Machine {
        let text = model.to_string();
        Machine { model: model, text: text }
    }

    fn actual(&self) -> Parser<str::Chars> {
        let input = self.text.chars();
        Parser::new(input)
    }

    fn model(&self) -> &ParserModel {
        &self.model
    }
}

impl Arbitrary for Machine {
    fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Machine {
        let model = Arbitrary::arbitrary(g);
        Machine::new(model)
    }
}


#[test]
fn prop_parser_correctly_parses_valid_obj_files() {
    fn property(machine: Machine) -> bool {
        let result = machine.actual().parse();
        let expected = machine.model().parse();

        //result == expected
        unimplemented!();
    }
    quickcheck::quickcheck(property as fn(Machine) -> bool);
}

#[test]
fn prop_parse_object_set_should_parse_objects() {
    fn property(machine: Machine) -> bool {
        let result = machine.actual().parse();
        let expected = machine.model().parse();

        unimplemented!();
    }
    quickcheck::quickcheck(property as fn(Machine) -> bool);
}

#[test]
fn prop_parse_object_set_should_parse_object_names() {
    fn property(machine: Machine) -> bool {
        let result = machine.actual().parse();
        let expected = machine.model().parse();

        unimplemented!();
    }
    quickcheck::quickcheck(property as fn(Machine) -> bool);
}

#[test]
fn prop_parse_object_set_should_parse_vertices() {
    fn property(machine: Machine) -> bool {
        let result = machine.actual().parse();
        let expected = machine.model().parse();

        unimplemented!();
    }
    quickcheck::quickcheck(property as fn(Machine) -> bool);
}

#[test]
fn prop_parse_object_set_should_parse_texture_vertices() {
    fn property(machine: Machine) -> bool {
        let result = machine.actual().parse();
        let expected = machine.model().parse();

        unimplemented!();
    }
    quickcheck::quickcheck(property as fn(Machine) -> bool);
}

#[test]
fn prop_parse_object_set_should_parse_normal_vertices() {
    fn property(machine: Machine) -> bool {
        let result = machine.actual().parse();
        let expected = machine.model().parse();

        unimplemented!();
    }
    quickcheck::quickcheck(property as fn(Machine) -> bool);
}

#[test]
fn prop_parse_object_set_should_parse_groups() {
    fn property(machine: Machine) -> bool {
        let result = machine.actual().parse();
        let expected = machine.model().parse();

        unimplemented!();
    }
    quickcheck::quickcheck(property as fn(Machine) -> bool);
}

#[test]
fn prop_parse_object_set_should_parse_smoothing_groups() {
    fn property(machine: Machine) -> bool {
        let result = machine.actual().parse();
        let expected = machine.model().parse();

        unimplemented!();
    }
    quickcheck::quickcheck(property as fn(Machine) -> bool);
}

#[test]
fn prop_parse_object_set_should_parse_elements() {
    fn property(machine: Machine) -> bool {
        let result = machine.actual().parse();
        let expected = machine.model().parse();

        unimplemented!();
    }
    quickcheck::quickcheck(property as fn(Machine) -> bool);
}

#[test]
fn prop_parse_object_set_should_parse_shape_entries() {
    fn property(machine: Machine) -> bool {
        let result = machine.actual().parse();
        let expected = machine.model().parse();

        unimplemented!();
    }
    quickcheck::quickcheck(property as fn(Machine) -> bool);
}

