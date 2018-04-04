#[cfg(test)]
#[macro_use]
extern crate quickcheck;
extern crate wavefront;

use quickcheck::{Arbitrary, Gen};
use wavefront::obj::{
    Vertex, NormalVertex, Element, VTNIndex, ObjectSet, ObjectBuilder,
    GroupName, ShapeEntry,
};
use wavefront::obj::{Parser, ParseError};

use std::fmt;
use std::cmp;


#[derive(Clone, Debug)]
struct ObjectSetModel {

}

impl ObjectSetModel {
    fn new() -> ObjectSetModel {
        ObjectSetModel { }
    }

    fn parse(&self) -> Result<ObjectSet, ParseError> { 
        unimplemented!();
    }
}

impl fmt::Display for ObjectSetModel {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "")
    }
}

impl Arbitrary for ObjectSetModel {
    fn arbitrary<G: Gen>(g: &mut G) -> ObjectSetModel {
        ObjectSetModel::new()
    }
}

#[test]
fn prop_parser_correctly_parses_valid_obj_files() {
    fn property(model: ObjectSetModel) -> bool {
        let input = model.to_string();
        let result = Parser::new(input.chars()).parse();
        let expected = model.parse();

        result == expected
    }
}

#[test]
fn prop_parse_object_set_should_parse_objects() {
    unimplemented!();
}

#[test]
fn prop_parse_object_set_should_parse_object_names() {
    unimplemented!();
}

#[test]
fn prop_parse_object_set_should_parse_vertices() {
    unimplemented!();
}

#[test]
fn prop_parse_object_set_should_parse_texture_vertices() {
    unimplemented!();
}

#[test]
fn prop_parse_object_set_should_parse_normal_vertices() {
    unimplemented!();
}

#[test]
fn prop_parse_object_set_should_parse_groups() {
    unimplemented!();
}

#[test]
fn prop_parse_object_set_should_parse_smoothing_groups() {
    unimplemented!();
}

#[test]
fn prop_parse_object_set_should_parse_elements() {
    unimplemented!();
}

#[test]
fn prop_parse_object_set_should_parse_shape_entries() {
    unimplemented!();
}

