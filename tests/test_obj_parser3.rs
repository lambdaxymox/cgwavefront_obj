extern crate wavefront_obj;


use wavefront_obj::obj::{
    Vertex, 
    NormalVertex,
    TextureVertex,
    Element, 
    VTNIndex, 
    ObjectSet, 
    Object,
    Group,
    SmoothingGroup,
    ShapeEntry,
    Geometry,
    Parser,
};
use std::fs::File;
use std::io::Read;


const SAMPLE_DATA: &str = "assets/cube_vt.obj";


struct Test {
    data: String,
    expected: ObjectSet,
}

#[rustfmt::skip]
fn test_case(file_path: &str) -> Test {
    let mut file = File::open(file_path).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let name = String::from("");
    let vertex_set = vec![
        Vertex { x: 0.0, y: 0.0, z: 0.0, w: 1.0 },
        Vertex { x: 0.0, y: 0.0, z: 1.0, w: 1.0 },
        Vertex { x: 0.0, y: 1.0, z: 0.0, w: 1.0 },
        Vertex { x: 0.0, y: 1.0, z: 1.0, w: 1.0 },
        Vertex { x: 1.0, y: 0.0, z: 0.0, w: 1.0 },
        Vertex { x: 1.0, y: 0.0, z: 1.0, w: 1.0 },
        Vertex { x: 1.0, y: 1.0, z: 0.0, w: 1.0 },
        Vertex { x: 1.0, y: 1.0, z: 1.0, w: 1.0 },
    ];
    let element_set = vec![
        Element::Face(VTNIndex::VTN(0, 1, 1),  VTNIndex::VTN(6, 2, 1),  VTNIndex::VTN(4, 0, 1)),
        Element::Face(VTNIndex::VTN(0, 1, 1),  VTNIndex::VTN(2, 6, 1),  VTNIndex::VTN(6, 2, 1)),
        Element::Face(VTNIndex::VTN(0, 4, 5),  VTNIndex::VTN(3, 1, 5),  VTNIndex::VTN(2, 3, 5)),
        Element::Face(VTNIndex::VTN(0, 4, 5),  VTNIndex::VTN(1, 5, 5),  VTNIndex::VTN(3, 1, 5)),
        Element::Face(VTNIndex::VTN(2, 5, 2),  VTNIndex::VTN(7, 6, 2),  VTNIndex::VTN(6, 1, 2)),
        Element::Face(VTNIndex::VTN(2, 5, 2),  VTNIndex::VTN(3, 7, 2),  VTNIndex::VTN(7, 6, 2)),
        Element::Face(VTNIndex::VTN(4, 7, 4),  VTNIndex::VTN(6, 13, 4), VTNIndex::VTN(7, 6, 4)),
        Element::Face(VTNIndex::VTN(4, 7, 4),  VTNIndex::VTN(7, 12, 4), VTNIndex::VTN(5, 13, 4)),
        Element::Face(VTNIndex::VTN(0, 8, 3),  VTNIndex::VTN(4, 7, 3),  VTNIndex::VTN(5, 5, 3)),
        Element::Face(VTNIndex::VTN(0, 8, 3),  VTNIndex::VTN(5, 9, 3),  VTNIndex::VTN(1, 7, 3)),
        Element::Face(VTNIndex::VTN(1, 10, 0), VTNIndex::VTN(5, 9, 0),  VTNIndex::VTN(7, 8, 0)),
        Element::Face(VTNIndex::VTN(1, 10, 0), VTNIndex::VTN(7, 11, 0), VTNIndex::VTN(3, 9, 0)),
    ];
    let texture_vertex_set = vec![
        TextureVertex { u: 0.0,   v: 0.66666666, w: 0.0 },
        TextureVertex { u: 0.25,  v: 0.66666666, w: 0.0 },
        TextureVertex { u: 0.0,   v: 0.33333333, w: 0.0 },
        TextureVertex { u: 0.25,  v: 1.0,        w: 0.0 },
        TextureVertex { u: 0.5,   v: 1.0,        w: 0.0 },
        TextureVertex { u: 0.5,   v: 0.66666666, w: 0.0 },
        TextureVertex { u: 0.25,  v: 0.33333333, w: 0.0 },
        TextureVertex { u: 0.5,   v: 0.33333333, w: 0.0 },
        TextureVertex { u: 0.75,  v: 0.66666666, w: 0.0 },
        TextureVertex { u: 0.75,  v: 0.33333333, w: 0.0 },
        TextureVertex { u: 1.0,   v: 0.66666666, w: 0.0 },
        TextureVertex { u: 1.0,   v: 0.33333333, w: 0.0 },
        TextureVertex { u: 0.5,   v: 0.0,        w: 0.0 },
        TextureVertex { u: 0.25,  v: 0.0,        w: 0.0 },
    ];
    let normal_vertex_set = vec![
        NormalVertex { x:  0.0, y:  0.0, z:  1.0 },
        NormalVertex { x:  0.0, y:  0.0, z: -1.0 },
        NormalVertex { x:  0.0, y:  1.0, z:  0.0 },
        NormalVertex { x:  0.0, y: -1.0, z:  0.0 },
        NormalVertex { x:  1.0, y:  0.0, z:  0.0 },
        NormalVertex { x: -1.0, y:  0.0, z:  0.0 },
    ];
    let group_set = vec![
        Group(String::from("cube")), 
    ];
    let smoothing_group_set = vec![SmoothingGroup(0)];
    let shape_set = vec![
        ShapeEntry { element: 0,    groups: vec![0], smoothing_group: 0 },
        ShapeEntry { element: 1,    groups: vec![0], smoothing_group: 0 },
        ShapeEntry { element: 2,    groups: vec![0], smoothing_group: 0 },
        ShapeEntry { element: 3,    groups: vec![0], smoothing_group: 0 },
        ShapeEntry { element: 4,    groups: vec![0], smoothing_group: 0 },
        ShapeEntry { element: 5,    groups: vec![0], smoothing_group: 0 },
        ShapeEntry { element: 6,    groups: vec![0], smoothing_group: 0 },
        ShapeEntry { element: 7,    groups: vec![0], smoothing_group: 0 },
        ShapeEntry { element: 8,    groups: vec![0], smoothing_group: 0 },
        ShapeEntry { element: 9,    groups: vec![0], smoothing_group: 0 },
        ShapeEntry { element: 10,   groups: vec![0], smoothing_group: 0 },
        ShapeEntry { element: 11,   groups: vec![0], smoothing_group: 0 },
    ];
    let material_name = None;
    let shapes = vec![
        0,    1,    2,    3,    4,    5,    6,    7,    8,    9,    10,   11,
    ];
    let geometry_set = vec![Geometry { material_name: material_name, shapes: shapes }];
    let object = Object {
        name: name,
        vertex_set: vertex_set,
        texture_vertex_set: texture_vertex_set,
        normal_vertex_set: normal_vertex_set,
        group_set: group_set,
        smoothing_group_set: smoothing_group_set,
        element_set: element_set,
        shape_set: shape_set,
        geometry_set: geometry_set,
    };
    let expected = ObjectSet { 
        material_libraries: vec![], 
        objects: vec![object]
    };

    Test {
        data: data,
        expected: expected,
    }
}

/// The parser should correctly parse a Wavefront OBJ file.
#[test]
fn test_parse_object_set() {
    let test = test_case(SAMPLE_DATA);
    let mut parser = Parser::new(&test.data);
    let result = parser.parse_objset().unwrap();

    assert_eq!(result, test.expected);
}

/// The parser should identify individual objects in a given 
/// object set parser.
#[test]
fn test_parse_object_set_should_parse_objects() {
    let test = test_case(SAMPLE_DATA);

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

/// The parser should correctly read object names.
#[test]
fn test_parse_object_set_should_parse_object_names() {
    let test = test_case(SAMPLE_DATA);
    let mut parser = Parser::new(&test.data);
    let result_set = parser.parse_objset().unwrap();
        
    for (result, expected) 
        in result_set.objects.iter().zip(test.expected.objects.iter()) {

        assert_eq!(result.name, expected.name);
    }
}

/// The parser should correctly parse vertex statements.
#[test]
fn test_parse_object_set_should_parse_vertices() {
    let test = test_case(SAMPLE_DATA);
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

/// The parser should correctly parse texture vertex statements.
#[test]
fn test_parse_object_set_should_parse_texture_vertices() {
    let test = test_case(SAMPLE_DATA);
    let mut parser = Parser::new(&test.data);
    let result_set = parser.parse_objset().unwrap();

    for (result, expected) 
        in result_set.objects.iter().zip(test.expected.objects.iter()) {
        assert_eq!(
            result.texture_vertex_set.len(), 
            expected.texture_vertex_set.len(), 
            "Length mismatch."
        );
        for (result_tv, expected_tv) in 
            result.texture_vertex_set.iter().zip(expected.texture_vertex_set.iter()) {
                
            assert_eq!(result_tv, expected_tv);
        }
    }
}

/// The parser should correctly parse normal vertex statements.
#[test]
fn test_parse_object_set_should_parse_normal_vertices() {
    let test = test_case(SAMPLE_DATA);
    let mut parser = Parser::new(&test.data);
    let result_set = parser.parse_objset().unwrap();

    for (result, expected) 
        in result_set.objects.iter().zip(test.expected.objects.iter()) {
        assert_eq!(
            result.normal_vertex_set.len(), 
            expected.normal_vertex_set.len(), 
            "Length mismatch."
        );
        for (result_nv, expected_nv) in 
            result.normal_vertex_set.iter().zip(expected.normal_vertex_set.iter()) {
            
            assert_eq!(
                result.vertex_set.len(), 
                expected.vertex_set.len(), 
                "Length mismatch."
            );
            assert_eq!(result_nv, expected_nv);
        }
    }
}

/// The parser should correctly group statements.
#[test]
fn test_parse_object_set_should_parse_groups() {
    let test = test_case(SAMPLE_DATA);
    let mut parser = Parser::new(&test.data);
    let result_set = parser.parse_objset().unwrap();

    for (result, expected) 
        in result_set.objects.iter().zip(test.expected.objects.iter()) {
        assert_eq!(
            result.group_set.len(), 
            expected.group_set.len(), 
            "Length mismatch."
        );
        for (result_g, expected_g) in 
            result.group_set.iter().zip(expected.group_set.iter()) {
                
            assert_eq!(result_g, expected_g);
        }
    }
}

/// The parser should correctly smoothing group statements.
#[test]
fn test_parse_object_set_should_parse_smoothing_groups() {
    let test = test_case(SAMPLE_DATA);
    let mut parser = Parser::new(&test.data);
    let result_set = parser.parse_objset().unwrap();

    for (result, expected) 
        in result_set.objects.iter().zip(test.expected.objects.iter()) {
        assert_eq!(
            result.smoothing_group_set.len(), 
            expected.smoothing_group_set.len(), 
            "Length mismatch."
        );
        for (result_sg, expected_sg) in 
            result.smoothing_group_set.iter().zip(expected.smoothing_group_set.iter()) {
                
            assert_eq!(result_sg, expected_sg);
        }
    }
}

/// The parser should correctly parse element statements.
#[test]
fn test_parse_object_set_should_parse_elements() {
    let test = test_case(SAMPLE_DATA);
    let mut parser = Parser::new(&test.data);
    let result_set = parser.parse_objset().unwrap();

    for (result, expected) 
        in result_set.objects.iter().zip(test.expected.objects.iter()) {
        assert_eq!(
            result.element_set.len(), 
            expected.element_set.len(), 
            "Length mismatch."
        );
        for (result_elem, expected_elem) in 
            result.element_set.iter().zip(expected.element_set.iter()) {
                
            assert_eq!(result_elem, expected_elem);
        }
    }
}

/// The parser should correctly derive shape entries from the contents of a
/// *.obj file.
#[test]
fn test_parse_object_set_should_parse_shape_entries() {
    let test = test_case(SAMPLE_DATA);
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

/// In a Wavefront OBJ file, vertices, elements, and grouping statements are
/// implicitly indexed in monotone increasing order. The resulting object set
/// should place the elements in monotone increasing order exactly as
/// they are found in the original file.
#[test]
fn test_parse_object_set_every_element_set_should_be_monotone_increasing() {
    let test = test_case(SAMPLE_DATA);
    let mut parser = Parser::new(&test.data);
    let result_set = parser.parse_objset().unwrap();

    for result in result_set.objects.iter() { 
        for (shape, next_shape) 
            in result.shape_set.iter().zip(result.shape_set[1..].iter()) {

            assert!(shape.element <= next_shape.element);
        }
    }  
}

/// Every element in a Wavefront OBJ belongs to at least one group. If no grouping
/// statements are used, it should belong to the default group `default`. Consequently,
/// every shape entry should have a nonempty group set.
#[test]
fn test_parse_object_every_element_belongs_to_a_group() {
    let test = test_case(SAMPLE_DATA);
    let mut parser = Parser::new(&test.data);
    let result_set = parser.parse_objset().unwrap();

    for result in result_set.objects.iter() { 
        for shape in result.shape_set.iter() {
            assert!(!shape.groups.is_empty());
        }
    }
}

/// Every element in a Wavefront OBJ belongs to at least one group. The parser
/// should correctly identify every group in the file; no nonexistent groups should
/// appear in the shape entry table. 
#[test]
fn test_parse_object_every_element_group_exists() {
    let test = test_case(SAMPLE_DATA);
    let mut parser = Parser::new(&test.data);
    let result_set = parser.parse_objset().unwrap();

    for result in result_set.objects.iter() { 
        for shape in result.shape_set.iter() {
            assert!(shape.groups.iter().all(|&group_index| {
                group_index <= result.group_set.len()
            }));
        }
    }
}

/// Every element in a Wavefront OBJ belongs to at least one smoothing group. The 
/// parser should correctly identify every group in the file; no nonexistent 
/// smoothing groups should appear in the shape entry table. 
#[test]
fn test_parse_object_every_element_smoothing_group_exists() {
    let test = test_case(SAMPLE_DATA);
    let mut parser = Parser::new(&test.data);
    let result_set = parser.parse_objset().unwrap();

    for result in result_set.objects.iter() { 
        for shape in result.shape_set.iter() {
            assert!(shape.smoothing_group < result.smoothing_group_set.len());
        }
    }   
}

