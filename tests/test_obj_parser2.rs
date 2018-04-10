extern crate wavefront;

use wavefront::obj::{
    Vertex, TextureVertex, NormalVertex, Element, VTNIndex, ObjectSet, Object,
    Group, SmoothingGroup, ShapeEntry,
};
use wavefront::obj::Parser;

use std::slice;


struct Test {
    data: String,
    expected: ObjectSet,
}

struct TestSet { 
    data: Vec<Test>,
}

impl TestSet {
    fn iter(&self) -> TestSetIter {
        TestSetIter {
            inner: self.data.iter(),
        }
    }
}

struct TestSetIter<'a> {
    inner: slice::Iter<'a, Test>,
}

impl<'a> Iterator for TestSetIter<'a> {
    type Item = &'a Test;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

fn test_cases() -> TestSet {
    TestSet {
        data: vec![
            Test {
                data: String::from(r"
                    o  Object0
                    v  -36.84435  -31.289864  -23.619797  -8.21862 
                    # 1 vertices

                    vt  -44.275238  28.583176  -23.780418
                    # 1 texture vertices

                    vn  93.94331  -61.460472  -32.00753 
                    # 1 normal vertices

                    g  Group0
                    g  Group1
                    s  0
                    g  Group2
                    s  1
                    g  Group3
                    f 1/1/1 1/1/1 1/1/1

                    g  Group4                    
                    s  2 
                    #### End Object 0

                "),
                expected: ObjectSet::new(vec![
                    Object::new(
                        String::from("Object0"),
                        vec![Vertex { x: -36.84435, y: -31.289864, z: -23.619797, w: -8.21862 }],
                        vec![TextureVertex { u: -44.275238, v: 28.583176, w: -23.780418 }],
                        vec![NormalVertex { i: 93.94331, j: -61.460472, k: -32.00753 }],
                        vec![Group::new("Group0"), Group::new("Group1"), Group::new("Group2"), Group::new("Group3"), Group::new("Group4")],
                        vec![SmoothingGroup::new(0), SmoothingGroup::new(1), SmoothingGroup::new(2)],
                        vec![Element::Face(VTNIndex::VTN(1, 1, 1), VTNIndex::VTN(1, 1, 1), VTNIndex::VTN(1, 1, 1))], 
                        vec![ShapeEntry { element: 1, groups: vec![4], smoothing_group: 2 }],
                    )]
                )
            }
        ],
    }
    
}


#[test]
fn test_parse_object_set() {
    let tests = test_cases();
    
    for test in tests.iter() {
        let mut parser = Parser::new(test.data.chars());
        let result = parser.parse().unwrap();

        assert_eq!(result, test.expected);
    }
}

#[test]
fn test_parse_object_set_should_parse_objects() {
    let tests = test_cases();

    for test in tests.iter() {
        let mut parser = Parser::new(test.data.chars());
        let result_set = parser.parse().unwrap();
        for (result, expected) in result_set.iter().zip(test.expected.iter()) {
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

#[test]
fn test_parse_object_set_should_parse_object_names() {
    let tests = test_cases();

    for test in tests.iter() {
        let mut parser = Parser::new(test.data.chars());
        let result_set = parser.parse().unwrap();
        
        for (result, expected) in result_set.iter().zip(test.expected.iter()) {
            assert_eq!(result.name, expected.name);
        }
    }
}

#[test]
fn test_parse_object_set_should_parse_vertices() {
    let tests = test_cases();

    for test in tests.iter() {
        let mut parser = Parser::new(test.data.chars());
        let result_set = parser.parse().unwrap();
        for (result, expected) in result_set.iter().zip(test.expected.iter()) {
            assert_eq!(result.vertex_set.len(), expected.vertex_set.len(), "Length mismatch.");
            for (result_v, expected_v) in 
                result.vertex_set.iter().zip(expected.vertex_set.iter()) {
                
                assert_eq!(result_v, expected_v);
            }
        }
    }
}

#[test]
fn test_parse_object_set_should_parse_texture_vertices() {
    let tests = test_cases();

    for test in tests.iter() {
        let mut parser = Parser::new(test.data.chars());
        let result_set = parser.parse().unwrap();
        for (result, expected) in result_set.iter().zip(test.expected.iter()) {
            
            assert_eq!(result.texture_vertex_set.len(), expected.texture_vertex_set.len(), "Length mismatch.");
            for (result_tv, expected_tv) in 
                result.texture_vertex_set.iter().zip(expected.texture_vertex_set.iter()) {
                
                assert_eq!(result_tv, expected_tv);
            }
        }
    }
}

#[test]
fn test_parse_object_set_should_parse_normal_vertices() {
    let tests = test_cases();

    for test in tests.iter() {
        let mut parser = Parser::new(test.data.chars());
        let result_set = parser.parse().unwrap();
        for (result, expected) in result_set.iter().zip(test.expected.iter()) {

            assert_eq!(result.normal_vertex_set.len(), expected.normal_vertex_set.len(), "Length mismatch.");
            for (result_nv, expected_nv) in 
                result.normal_vertex_set.iter().zip(expected.normal_vertex_set.iter()) {
                
                assert_eq!(result.vertex_set.len(), expected.vertex_set.len(), "Length mismatch.");
                assert_eq!(result_nv, expected_nv);
            }
        }
    }
}

#[test]
fn test_parse_object_set_should_parse_groups() {
    let tests = test_cases();

    for test in tests.iter() {
        let mut parser = Parser::new(test.data.chars());
        let result_set = parser.parse().unwrap();
        for (result, expected) in result_set.iter().zip(test.expected.iter()) {
            assert_eq!(result.group_set.len(), expected.group_set.len(), "Length mismatch.");
            for (result_g, expected_g) in 
                result.group_set.iter().zip(expected.group_set.iter()) {
                
                assert_eq!(result_g, expected_g);
            }
        }
    }
}

#[test]
fn test_parse_object_set_should_parse_smoothing_groups() {
    let tests = test_cases();

    for test in tests.iter() {
        let mut parser = Parser::new(test.data.chars());
        let result_set = parser.parse().unwrap();
        for (result, expected) in result_set.iter().zip(test.expected.iter()) {
            assert_eq!(result.smoothing_group_set.len(), expected.smoothing_group_set.len(), "Length mismatch.");
            for (result_sg, expected_sg) in 
                result.smoothing_group_set.iter().zip(expected.smoothing_group_set.iter()) {
                
                assert_eq!(result_sg, expected_sg);
            }
        }
    }
}

#[test]
fn test_parse_object_set_should_parse_elements() {
    let tests = test_cases();

    for test in tests.iter() {
        let mut parser = Parser::new(test.data.chars());
        let result_set = parser.parse().unwrap();
        for (result, expected) in result_set.iter().zip(test.expected.iter()) {
            assert_eq!(result.element_set.len(), expected.element_set.len(), "Length mismatch.");
            for (result_elem, expected_elem) in 
                result.element_set.iter().zip(expected.element_set.iter()) {
                
                assert_eq!(result_elem, expected_elem);
            }
        }
    }
}

#[test]
fn test_parse_object_set_should_parse_shape_entries() {
    let tests = test_cases();

    for test in tests.iter() {
        let mut parser = Parser::new(test.data.chars());
        let result_set = parser.parse().unwrap();
        for (result, expected) in result_set.iter().zip(test.expected.iter()) { 
            assert_eq!(result.shape_set.len(), expected.shape_set.len(), "Length mismatch.");
            for (result_sh, expected_sh) in 
                result.shape_set.iter().zip(expected.shape_set.iter()) {
                
                assert_eq!(result_sh, expected_sh);
            }
        }
    }
}

