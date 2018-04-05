extern crate quickcheck;
extern crate wavefront;

use quickcheck::{Arbitrary, Gen};
use wavefront::obj::{
    Vertex, TextureVertex, NormalVertex, Element, VTNIndex, ObjectSet, ObjectBuilder,
    GroupName, SmoothingGroup, ShapeEntry,
};
use wavefront::obj::{Parser, ParseError};

use std::fmt;
use std::cmp;
use std::str;


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

enum TextLine {
    V(MVertex),
    VT(MTextureVertex),
    VN(MNormalVertex),
    Comment(MComment),
    S(MSmoothingGroup),
    Group(Vec<GroupName>),

}

#[derive(Clone, Debug)]
struct ParserModel {

}

impl ParserModel {
    fn new() -> ParserModel {
        ParserModel { }
    }

    fn parse(&self) -> Result<ObjectSet, ParseError> { 
        unimplemented!();
    }
}

impl fmt::Display for ParserModel {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "")
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

    fn model(&self) -> ParserModel {
        self.model.clone()
    }
}

impl Arbitrary for Machine {
    fn arbitrary<G: Gen>(g: &mut G) -> Machine {
        unimplemented!();
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

