extern crate wavefront_obj;


use wavefront_obj::obj::{
    Vertex, 
    TextureVertex, 
    NormalVertex, 
    Element, 
    VTNIndex, 
    ObjectSet, 
    Object,
    Group, 
    SmoothingGroup, 
    ShapeEntry,
    Geometry,
};
use wavefront_obj::obj::{
    Parser,
};

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
                    o  Object0                                      \
                    v  -36.84435  -31.289864  -23.619797  -8.21862  \
                    # 1 vertices                                    \
                                                                    \
                    vt  -44.275238  28.583176  -23.780418           \
                    # 1 texture vertices                            \
                                                                    \
                    vn  93.94331  -61.460472  -32.00753             \
                    # 1 normal vertices                             \
                                                                    \
                    g  Group0                                       \
                    g  Group1                                       \
                    s  0                                            \
                    g  Group2                                       \
                    s  1                                            \
                    g  Group3                                       \
                    f 1/1/1 1/1/1 1/1/1                             \
                    # 1 elements                                    \
                                                                    \
                    g  Group4                                       \
                    s  2                                            \
                    #### End Object 0                               \
                "),
                expected: ObjectSet::new(vec![], vec![
                    Object::new(
                        String::from("Object0"),
                        vec![Vertex::new(-36.84435, -31.289864, -23.619797, -8.21862)],
                        vec![TextureVertex::new(-44.275238, 28.583176, -23.780418)],
                        vec![NormalVertex::new(93.94331, -61.460472, -32.00753)],
                        vec![
                            Group::new("Group0"), Group::new("Group1"), 
                            Group::new("Group2"), Group::new("Group3"), Group::new("Group4")
                        ],
                        vec![SmoothingGroup::new(0), SmoothingGroup::new(1), SmoothingGroup::new(2)],
                        vec![Element::Face(VTNIndex::VTN(0, 0, 0), VTNIndex::VTN(0, 0, 0), VTNIndex::VTN(0, 0, 0))], 
                        vec![ShapeEntry::new(0, vec![3], 1)],
                        vec![Geometry::new(None, vec![0])]
                    )
                ])
            },
            Test {
                data: String::from(r"
                    o  Object0                                     \
                    v  -36.84435  -31.289864  -23.619797  -8.21862 \
                    # 1 vertices                                   \
                                                                   \
                    vt  -44.275238  28.583176  -23.780418          \
                    # 1 texture vertices                           \
                                                                   \
                    vn  93.94331  -61.460472  -32.00753            \
                    # 1 normal vertices                            \
                                                                   \
                    f 1/1/1 1/1/1 1/1/1                            \
                    # 1 elements                                   \
                                                                   \
                    #### End Object 0                              \
                "),
                expected: ObjectSet::new(vec![], vec![
                    Object::new(
                        String::from("Object0"),
                        vec![Vertex::new(-36.84435, -31.289864, -23.619797, -8.21862)],
                        vec![TextureVertex::new(-44.275238, 28.583176, -23.780418)],
                        vec![NormalVertex::new(93.94331, -61.460472, -32.00753)],
                        vec![Group::new("default")],
                        vec![SmoothingGroup::new(0)],
                        vec![Element::Face(VTNIndex::VTN(0, 0, 0), VTNIndex::VTN(0, 0, 0), VTNIndex::VTN(0, 0, 0))], 
                        vec![ShapeEntry::new(0, vec![0], 0)],
                        vec![Geometry::new(None, vec![0])]
                    )
                ])
            },
            Test {
                data: String::from(r"
                    # diamond.obj                           \
                                                            \
                    g Object001                             \
                                                            \
                    v 0.000000E+00 0.000000E+00 78.0000     \
                    v 45.0000 45.0000 0.000000E+00          \
                    v 45.0000 -45.0000 0.000000E+00         \
                    v -45.0000 -45.0000 0.000000E+00        \
                    v -45.0000 45.0000 0.000000E+00         \
                    v 0.000000E+00 0.000000E+00 -78.0000    \
                                                            \
                    f     1 2 3                             \
                    f     1 3 4                             \
                    f     1 4 5                             \
                    f     1 5 2                             \
                    f     6 5 4                             \
                    f     6 4 3                             \
                    f     6 3 2                             \
                    f     6 2 1                             \
                    f     6 1 5                             \
                "),
                expected: ObjectSet::new(vec![], vec![
                    Object::new(
                        String::from(""),
                        vec![
                            Vertex::new(  0.0,   0.0, 78.0, 1.0), Vertex::new( 45.0,  45.0,   0.0, 1.0),
                            Vertex::new( 45.0, -45.0,  0.0, 1.0), Vertex::new(-45.0, -45.0,   0.0, 1.0),
                            Vertex::new(-45.0,  45.0,  0.0, 1.0), Vertex::new(  0.0,   0.0, -78.0, 1.0),
                        ],
                        vec![],
                        vec![],
                        vec![Group::new("Object001")],
                        vec![SmoothingGroup::new(0)],
                        vec![
                            Element::Face(VTNIndex::V(0), VTNIndex::V(1), VTNIndex::V(2)),
                            Element::Face(VTNIndex::V(0), VTNIndex::V(2), VTNIndex::V(3)),
                            Element::Face(VTNIndex::V(0), VTNIndex::V(3), VTNIndex::V(4)),
                            Element::Face(VTNIndex::V(0), VTNIndex::V(4), VTNIndex::V(1)),
                            Element::Face(VTNIndex::V(5), VTNIndex::V(4), VTNIndex::V(3)),
                            Element::Face(VTNIndex::V(5), VTNIndex::V(3), VTNIndex::V(2)),
                            Element::Face(VTNIndex::V(5), VTNIndex::V(2), VTNIndex::V(1)),
                            Element::Face(VTNIndex::V(5), VTNIndex::V(1), VTNIndex::V(0)),
                            Element::Face(VTNIndex::V(5), VTNIndex::V(0), VTNIndex::V(4)),
                        ], 
                        vec![
                            ShapeEntry::new(0, vec![0], 0), ShapeEntry::new(1, vec![0], 0),
                            ShapeEntry::new(2, vec![0], 0), ShapeEntry::new(3, vec![0], 0),
                            ShapeEntry::new(4, vec![0], 0), ShapeEntry::new(5, vec![0], 0),
                            ShapeEntry::new(6, vec![0], 0), ShapeEntry::new(7, vec![0], 0),
                            ShapeEntry::new(8, vec![0], 0),
                        ],
                        vec![Geometry::new(None, vec![0, 1, 2, 3, 4, 5, 6, 7, 8])]
                    )
                ])
            },
            Test {
                data: String::from(r"
                    mtllib master.mtl             \
                    o Object001                   \
                    v 0.000000 2.000000 2.000000  \
                    v 0.000000 0.000000 2.000000  \
                    v 2.000000 0.000000 2.000000  \
                    v 2.000000 2.000000 2.000000  \
                    v 0.000000 2.000000 0.000000  \
                    v 0.000000 0.000000 0.000000  \
                    v 2.000000 0.000000 0.000000  \
                    v 2.000000 2.000000 0.000000  \
                    # 8 vertices                  \
                                                  \
                    g front                       \
                    usemtl red                    \
                    f 1 2 3 4                     \
                    g back                        \
                    usemtl blue                   \
                    f 8 7 6 5                     \
                    g right                       \
                    usemtl green                  \
                    f 4 3 7 8                     \
                    g top                         \
                    usemtl gold                   \
                    f 5 1 4 8                     \
                    g left                        \
                    usemtl orange                 \
                    f 5 6 2 1                     \
                    g bottom                      \
                    usemtl purple                 \
                    f 2 6 7 3                     \
                    # 6 elements                  \
                "),
                expected: ObjectSet::new(
                    vec![
                        String::from("master.mtl"),
                    ],
                    vec![
                        Object::new(
                            String::from("Object001"),
                            vec![
                                Vertex::new(0.000000, 2.000000, 2.000000, 1.0), 
                                Vertex::new(0.000000, 0.000000, 2.000000, 1.0),
                                Vertex::new(2.000000, 0.000000, 2.000000, 1.0),
                                Vertex::new(2.000000, 2.000000, 2.000000, 1.0),
                                Vertex::new(0.000000, 2.000000, 0.000000, 1.0),
                                Vertex::new(0.000000, 0.000000, 0.000000, 1.0),
                                Vertex::new(2.000000, 0.000000, 0.000000, 1.0),
                                Vertex::new(2.000000, 2.000000, 0.000000, 1.0),
                            ],
                            vec![],
                            vec![],
                            vec![
                                Group::new("front"), 
                                Group::new("back"), 
                                Group::new("right"), 
                                Group::new("top"),
                                Group::new("left"),
                                Group::new("bottom")
                            ],
                            vec![SmoothingGroup::new(0)],
                            vec![
                                Element::Face(VTNIndex::V(0), VTNIndex::V(1), VTNIndex::V(2)),
                                Element::Face(VTNIndex::V(0), VTNIndex::V(2), VTNIndex::V(3)),
                                Element::Face(VTNIndex::V(7), VTNIndex::V(6), VTNIndex::V(5)),
                                Element::Face(VTNIndex::V(7), VTNIndex::V(5), VTNIndex::V(4)),
                                Element::Face(VTNIndex::V(3), VTNIndex::V(2), VTNIndex::V(6)),
                                Element::Face(VTNIndex::V(3), VTNIndex::V(6), VTNIndex::V(7)),
                                Element::Face(VTNIndex::V(4), VTNIndex::V(0), VTNIndex::V(3)),
                                Element::Face(VTNIndex::V(4), VTNIndex::V(3), VTNIndex::V(7)),
                                Element::Face(VTNIndex::V(4), VTNIndex::V(5), VTNIndex::V(1)),
                                Element::Face(VTNIndex::V(4), VTNIndex::V(1), VTNIndex::V(0)),
                                Element::Face(VTNIndex::V(1), VTNIndex::V(5), VTNIndex::V(6)),
                                Element::Face(VTNIndex::V(1), VTNIndex::V(6), VTNIndex::V(2)),
                            ],
                            vec![
                                ShapeEntry::new(0,  vec![0], 0),
                                ShapeEntry::new(1,  vec![0], 0),
                                ShapeEntry::new(2,  vec![1], 0),
                                ShapeEntry::new(3,  vec![1], 0),
                                ShapeEntry::new(4,  vec![2], 0),
                                ShapeEntry::new(5,  vec![2], 0),
                                ShapeEntry::new(6,  vec![3], 0),
                                ShapeEntry::new(7,  vec![3], 0),
                                ShapeEntry::new(8,  vec![4], 0),
                                ShapeEntry::new(9,  vec![4], 0),
                                ShapeEntry::new(10, vec![5], 0),
                                ShapeEntry::new(11, vec![5], 0),
                            ],
                            vec![
                                Geometry::new(Some(String::from("red")),    vec![0,  1]),
                                Geometry::new(Some(String::from("blue")),   vec![2,  3]),
                                Geometry::new(Some(String::from("green")),  vec![4,  5]),
                                Geometry::new(Some(String::from("gold")),   vec![6,  7]),
                                Geometry::new(Some(String::from("orange")), vec![8,  9]),
                                Geometry::new(Some(String::from("purple")), vec![10, 11]),
                            ]
                        )
                    ],
                )
            }
        ],
    }
}

/// The parser should correctly parse a Wavefront OBJ file.
#[test]
fn test_parse_object_set() {
    let tests = test_cases();
    
    for test in tests.iter() {
        let mut parser = Parser::new(&test.data);
        let result = parser.parse_objset().unwrap();

        assert_eq!(result, test.expected);
    }
}

/// The parser should identify individual objects in a given 
/// object set parser.
#[test]
fn test_parse_object_set_should_parse_objects() {
    let tests = test_cases();

    for test in tests.iter() {
        let mut parser = Parser::new(&test.data);
        let result_set = parser.parse_objset().unwrap();
        for (result, expected) 
            in result_set.objects.iter().zip(test.expected.objects.iter()) {

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

/// The parser should correctly read object names.
#[test]
fn test_parse_object_set_should_parse_object_names() {
    let tests = test_cases();

    for test in tests.iter() {
        let mut parser = Parser::new(&test.data);
        let result_set = parser.parse_objset().unwrap(); 
        for (result, expected) 
            in result_set.objects.iter().zip(test.expected.objects.iter()) {

            assert_eq!(result.name, expected.name);
        }
    }
}

/// The parser should correctly parse vertex statements.
#[test]
fn test_parse_object_set_should_parse_vertices() {
    let tests = test_cases();

    for test in tests.iter() {
        let mut parser = Parser::new(&test.data);
        let result_set = parser.parse_objset().unwrap();
        for (result, expected) 
            in result_set.objects.iter().zip(test.expected.objects.iter()) {
            assert_eq!(
                result.vertex_set.len(), 
                expected.vertex_set.len(), 
                "Length mismatch."
            );
            for (result_v, expected_v) in 
                result.vertex_set.iter().zip(expected.vertex_set.iter()) {
                
                assert_eq!(result_v, expected_v);
            }
        }
    }
}

/// The parser should correctly parse texture vertex statements.
#[test]
fn test_parse_object_set_should_parse_texture_vertices() {
    let tests = test_cases();

    for test in tests.iter() {
        let mut parser = Parser::new(&test.data);
        let result_set = parser.parse_objset().unwrap();
        for (result, expected) 
            in result_set.objects.iter().zip(test.expected.objects.iter()) {         
            assert_eq!(
                result.texture_vertex_set.len(), 
                expected.texture_vertex_set.len(), 
                "Length mismatch."
            );
            for (result_tv, expected_tv) 
                in result.texture_vertex_set.iter()
                    .zip(expected.texture_vertex_set.iter()) {
                
                assert_eq!(result_tv, expected_tv);
            }
        }
    }
}

/// The parser should correctly parse normal vertex statements.
#[test]
fn test_parse_object_set_should_parse_normal_vertices() {
    let tests = test_cases();

    for test in tests.iter() {
        let mut parser = Parser::new(&test.data);
        let result_set = parser.parse_objset().unwrap();
        for (result, expected) 
            in result_set.objects.iter().zip(test.expected.objects.iter()) {
            assert_eq!(
                result.normal_vertex_set.len(), 
                expected.normal_vertex_set.len(), 
                "Length mismatch."
            );
            for (result_nv, expected_nv)
                in result.normal_vertex_set.iter()
                    .zip(expected.normal_vertex_set.iter()) {
                
                assert_eq!(
                    result.vertex_set.len(), 
                    expected.vertex_set.len(), 
                    "Length mismatch."
                );
                assert_eq!(result_nv, expected_nv);
            }
        }
    }
}

/// The parser should correctly group statements.
#[test]
fn test_parse_object_set_should_parse_groups() {
    let tests = test_cases();

    for test in tests.iter() {
        let mut parser = Parser::new(&test.data);
        let result_set = parser.parse_objset().unwrap();
        for (result, expected) 
            in result_set.objects.iter().zip(test.expected.objects.iter()) {
            assert_eq!(
                result.group_set.len(), 
                expected.group_set.len(), 
                "Length mismatch."
            );
            for (result_g, expected_g) 
                in result.group_set.iter().zip(expected.group_set.iter()) {
                
                assert_eq!(result_g, expected_g);
            }
        }
    }
}

/// The parser should correctly smoothing group statements.
#[test]
fn test_parse_object_set_should_parse_smoothing_groups() {
    let tests = test_cases();

    for test in tests.iter() {
        let mut parser = Parser::new(&test.data);
        let result_set = parser.parse_objset().unwrap();
        for (result, expected) 
            in result_set.objects.iter().zip(test.expected.objects.iter()) {
            assert_eq!(
                result.smoothing_group_set.len(), 
                expected.smoothing_group_set.len(), 
                "Length mismatch."
            );
            for (result_sg, expected_sg)
                in result.smoothing_group_set.iter()
                    .zip(expected.smoothing_group_set.iter()) {
                
                assert_eq!(result_sg, expected_sg);
            }
        }
    }
}

/// The parser should correctly parse element statements.
#[test]
fn test_parse_object_set_should_parse_elements() {
    let tests = test_cases();

    for test in tests.iter() {
        let mut parser = Parser::new(&test.data);
        let result_set = parser.parse_objset().unwrap();
        for (result, expected) 
            in result_set.objects.iter().zip(test.expected.objects.iter()) {
            assert_eq!(
                result.element_set.len(), 
                expected.element_set.len(), 
                "Length mismatch."
            );
            for (result_elem, expected_elem) 
                in result.element_set.iter().zip(expected.element_set.iter()) {
                
                assert_eq!(result_elem, expected_elem);
            }
        }
    }
}

/// The parser should correctly derive shape entries from the contents of a
/// *.obj file.
#[test]
fn test_parse_object_set_should_parse_shape_entries() {
    let tests = test_cases();

    for test in tests.iter() {
        let mut parser = Parser::new(&test.data);
        let result_set = parser.parse_objset().unwrap();
        for (result, expected) 
            in result_set.objects.iter().zip(test.expected.objects.iter()) { 
            assert_eq!(
                result.shape_set.len(), 
                expected.shape_set.len(), 
                "Length mismatch."
            );
            for (result_sh, expected_sh) 
                in result.shape_set.iter().zip(expected.shape_set.iter()) {
                
                assert_eq!(result_sh, expected_sh);
            }
        }
    }
}

/// In a Wavefront OBJ file, vertices, elements, and grouping statements are
/// implicitly indexed in monotone increasing order. The resulting object set
/// should place the elements in monotone increasing order exactly as
/// they are found in the original file.
#[test]
fn test_parse_object_set_every_element_set_should_be_monotone_increasing() {
    let tests = test_cases();

    for test in tests.iter() {
        let mut parser = Parser::new(&test.data);
        let result_set = parser.parse_objset().unwrap();
        for result in result_set.objects.iter() { 
            for (shape, next_shape) 
                in result.shape_set.iter().zip(result.shape_set[1..].iter()) {

                assert!(shape.element <= next_shape.element);
            }
        }
    }    
}

/// Every element in a Wavefront OBJ belongs to at least one group. If no 
/// grouping statements are used, it should belong to the default group 
/// `default`. Consequently, every shape entry should have a nonempty group set.
#[test]
fn test_parse_object_every_element_belongs_to_a_group() {
    let tests = test_cases();

    for test in tests.iter() {
        let mut parser = Parser::new(&test.data);
        let result_set = parser.parse_objset().unwrap();
        for result in result_set.objects.iter() { 
            for shape in result.shape_set.iter() {
                assert!(!shape.groups.is_empty());
            }
        }
    }      
}

/// Every element in a Wavefront OBJ belongs to at least one group. The parser
/// should correctly identify every group in the file; no nonexistent groups 
/// should appear in the shape entry table. 
#[test]
fn test_parse_object_every_element_group_exists() {
    let tests = test_cases();

    for test in tests.iter() {
        let mut parser = Parser::new(&test.data);
        let result_set = parser.parse_objset().unwrap();
        for result in result_set.objects.iter() { 
            for shape in result.shape_set.iter() {
                assert!(shape.groups.iter().all(|&group_index| {
                    group_index < result.group_set.len()
                }));
            }
        }
    }      
}

/// Every element in a Wavefront OBJ belongs to at least one smoothing group. 
/// The parser should correctly identify every group in the file; no 
/// nonexistent smoothing groups should appear in the shape entry table. 
#[test]
fn test_parse_object_every_element_smoothing_group_exists() {
    let tests = test_cases();

    for test in tests.iter() {
        let mut parser = Parser::new(&test.data);
        let result_set = parser.parse_objset().unwrap();
        for result in result_set.objects.iter() { 
            for shape in result.shape_set.iter() {
                assert!(
                    shape.smoothing_group < result.smoothing_group_set.len()
                );
            }
        }
    }      
}

/// The Wavefront OBJ parser should associate the correct material with each
/// geometry element.
#[test]
fn test_parse_object_every_element_should_have_geometry() {
    let tests = test_cases();

    for test in tests.iter() {
        let mut parser = Parser::new(&test.data);
        let result_set = parser.parse_objset().unwrap();
        for (result, expected) 
            in result_set.objects.iter().zip(test.expected.objects.iter()) { 
                assert_eq!(result.geometry_set, expected.geometry_set);
        }
    }
}

