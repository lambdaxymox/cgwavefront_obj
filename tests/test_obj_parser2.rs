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

#[rustfmt::skip]
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
                expected: ObjectSet { 
                    material_libraries: vec![], 
                    objects: vec![
                        Object {
                            name: String::from("Object0"),
                            vertex_set: vec![
                                Vertex { x: -36.84435, y: -31.289864, z: -23.619797, w: -8.21862 }
                            ],
                            texture_vertex_set: vec![
                                TextureVertex { u: -44.275238, v: 28.583176, w: -23.780418 }
                            ],
                            normal_vertex_set: vec![
                                NormalVertex { x: 93.94331, y: -61.460472, z: -32.00753 }
                            ],
                            group_set: vec![
                                Group(String::from("Group0")), 
                                Group(String::from("Group1")), 
                                Group(String::from("Group2")), 
                                Group(String::from("Group3")), 
                                Group(String::from("Group4"))
                            ],
                            smoothing_group_set: vec![
                                SmoothingGroup(0), 
                                SmoothingGroup(1), 
                                SmoothingGroup(2)
                            ],
                            element_set: vec![
                                Element::Face(VTNIndex::VTN(0, 0, 0), VTNIndex::VTN(0, 0, 0), VTNIndex::VTN(0, 0, 0))
                            ], 
                            shape_set: vec![
                                ShapeEntry { element: 0, groups: vec![3], smoothing_group: 1 }
                            ],
                            geometry_set: vec![
                                Geometry { material_name: None, shapes: vec![0] }
                            ]
                        }
                    ]
                }
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
                expected: ObjectSet {
                    material_libraries: vec![], 
                    objects: vec![
                        Object {
                            name: String::from("Object0"),
                            vertex_set: vec![
                                Vertex { x: -36.84435, y: -31.289864, z: -23.619797, w: -8.21862 }
                            ],
                            texture_vertex_set: vec![
                                TextureVertex { u: -44.275238, v: 28.583176, w: -23.780418 }
                            ],
                            normal_vertex_set: vec![
                                NormalVertex { x: 93.94331, y: -61.460472, z: -32.00753 }
                            ],
                            group_set: vec![
                                Group(String::from("default"))
                            ],
                            smoothing_group_set: vec![
                                SmoothingGroup(0)
                            ],
                            element_set: vec![
                                Element::Face(VTNIndex::VTN(0, 0, 0), VTNIndex::VTN(0, 0, 0), VTNIndex::VTN(0, 0, 0))
                            ], 
                            shape_set: vec![
                                ShapeEntry { element: 0, groups: vec![0], smoothing_group: 0 }
                            ],
                            geometry_set: vec![
                                Geometry { material_name: None, shapes: vec![0] }
                            ]
                        }
                    ]
                }
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
                expected: ObjectSet { 
                    material_libraries: vec![], 
                    objects: vec![
                        Object {
                            name: String::from(""),
                            vertex_set: vec![
                                Vertex { x:  0.0,  y:  0.0,  z:  78.0, w: 1.0 }, 
                                Vertex { x:  45.0, y:  45.0, z:  0.0,  w: 1.0 },
                                Vertex { x:  45.0, y: -45.0, z:  0.0,  w: 1.0 }, 
                                Vertex { x: -45.0, y: -45.0, z:  0.0,  w: 1.0 },
                                Vertex { x: -45.0, y:  45.0, z:  0.0,  w: 1.0 }, 
                                Vertex { x:  0.0,  y:  0.0,  z: -78.0, w: 1.0 },
                            ],
                            texture_vertex_set: vec![],
                            normal_vertex_set: vec![],
                            group_set: vec![
                                Group(String::from("Object001"))
                            ],
                            smoothing_group_set: vec![
                                SmoothingGroup(0)
                            ],
                            element_set: vec![
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
                            shape_set: vec![
                                ShapeEntry { element: 0, groups: vec![0], smoothing_group: 0 }, 
                                ShapeEntry { element: 1, groups: vec![0], smoothing_group: 0 },
                                ShapeEntry { element: 2, groups: vec![0], smoothing_group: 0 }, 
                                ShapeEntry { element: 3, groups: vec![0], smoothing_group: 0 },
                                ShapeEntry { element: 4, groups: vec![0], smoothing_group: 0 },
                                ShapeEntry { element: 5, groups: vec![0], smoothing_group: 0 },
                                ShapeEntry { element: 6, groups: vec![0], smoothing_group: 0 }, 
                                ShapeEntry { element: 7, groups: vec![0], smoothing_group: 0 },
                                ShapeEntry { element: 8, groups: vec![0], smoothing_group: 0 },
                            ],
                            geometry_set: vec![
                                Geometry { material_name: None, shapes: vec![0, 1, 2, 3, 4, 5, 6, 7, 8] },
                            ]
                        }
                    ]
                }
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
                expected: ObjectSet {
                    material_libraries: vec![
                        String::from("master.mtl"),
                    ],
                    objects: vec![
                        Object {
                            name: String::from("Object001"),
                            vertex_set: vec![
                                Vertex { x: 0.000000, y: 2.000000, z: 2.000000, w: 1.0 }, 
                                Vertex { x: 0.000000, y: 0.000000, z: 2.000000, w: 1.0 },
                                Vertex { x: 2.000000, y: 0.000000, z: 2.000000, w: 1.0 },
                                Vertex { x: 2.000000, y: 2.000000, z: 2.000000, w: 1.0 },
                                Vertex { x: 0.000000, y: 2.000000, z: 0.000000, w: 1.0 },
                                Vertex { x: 0.000000, y: 0.000000, z: 0.000000, w: 1.0 },
                                Vertex { x: 2.000000, y: 0.000000, z: 0.000000, w: 1.0 },
                                Vertex { x: 2.000000, y: 2.000000, z: 0.000000, w: 1.0 },
                            ],
                            texture_vertex_set: vec![],
                            normal_vertex_set: vec![],
                            group_set: vec![
                                Group(String::from("front")), 
                                Group(String::from("back")), 
                                Group(String::from("right")), 
                                Group(String::from("top")),
                                Group(String::from("left")),
                                Group(String::from("bottom"))
                            ],
                            smoothing_group_set: vec![
                                SmoothingGroup(0)
                            ],
                            element_set: vec![
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
                            shape_set: vec![
                                ShapeEntry { element: 0,  groups: vec![0], smoothing_group: 0 },
                                ShapeEntry { element: 1,  groups: vec![0], smoothing_group: 0 },
                                ShapeEntry { element: 2,  groups: vec![1], smoothing_group: 0 },
                                ShapeEntry { element: 3,  groups: vec![1], smoothing_group: 0 },
                                ShapeEntry { element: 4,  groups: vec![2], smoothing_group: 0 },
                                ShapeEntry { element: 5,  groups: vec![2], smoothing_group: 0 },
                                ShapeEntry { element: 6,  groups: vec![3], smoothing_group: 0 },
                                ShapeEntry { element: 7,  groups: vec![3], smoothing_group: 0 },
                                ShapeEntry { element: 8,  groups: vec![4], smoothing_group: 0 },
                                ShapeEntry { element: 9,  groups: vec![4], smoothing_group: 0 },
                                ShapeEntry { element: 10, groups: vec![5], smoothing_group: 0 },
                                ShapeEntry { element: 11, groups: vec![5], smoothing_group: 0 },
                            ],
                            geometry_set: vec![
                                Geometry { material_name: Some(String::from("red")),    shapes: vec![0,  1]  },
                                Geometry { material_name: Some(String::from("blue")),   shapes: vec![2,  3]  },
                                Geometry { material_name: Some(String::from("green")),  shapes: vec![4,  5]  },
                                Geometry { material_name: Some(String::from("gold")),   shapes: vec![6,  7]  },
                                Geometry { material_name: Some(String::from("orange")), shapes: vec![8,  9]  },
                                Geometry { material_name: Some(String::from("purple")), shapes: vec![10, 11] },
                            ]
                        }
                    ],
                }
            },
            Test {
                data: String::from(r"
                    mtllib material_library.mtl   \
                    o Object001                   \
                    v 0.000000 2.000000 0.000000  \
                    v 0.000000 0.000000 0.000000  \
                    v 2.000000 0.000000 0.000000  \
                    v 2.000000 2.000000 0.000000  \
                    v 4.000000 0.000000 -1.255298 \
                    v 4.000000 2.000000 -1.255298 \
                    vn 0.000000 0.000000 1.000000 \
                    vn 0.000000 0.000000 1.000000 \
                    vn 0.276597 0.000000 0.960986 \
                    vn 0.276597 0.000000 0.960986 \
                    vn 0.531611 0.000000 0.846988 \
                    vn 0.531611 0.000000 0.846988 \
                    # 6 vertices                  \
                    # 6 normals                   \
                                                  \
                    usemtl material1              \
                    g all                         \
                    s 1                           \
                    f 1//1 2//2 3//3 4//4         \
                    f 4//4 3//3 5//5 6//6         \
                    # 2 elements                  \
                                                  \
                    #### End Object001            \
                                                  \
                    o Object002                   \
                    v 0.000000 2.000000 0.000000  \
                    v 0.000000 0.000000 0.000000  \
                    v 2.000000 0.000000 0.000000  \
                    v 2.000000 2.000000 0.000000  \
                    v 4.000000 0.000000 -1.255298 \
                    v 4.000000 2.000000 -1.255298 \
                    vn 0.000000 0.000000 1.000000 \
                    vn 0.000000 0.000000 1.000000 \
                    vn 0.276597 0.000000 0.960986 \
                    vn 0.276597 0.000000 0.960986 \
                    vn 0.531611 0.000000 0.846988 \
                    vn 0.531611 0.000000 0.846988 \
                    # 6 vertices                  \
                    # 6 normals                   \
                                                  \
                    usemtl material2              \
                    g all                         \
                    s 1                           \
                    f 7//7   8//8 9//9   10//10   \
                    f 10//10 9//9 11//11 12//12   \
                    # 2 elements                  \
                                                  \
                    #### End Object002            \
                                                  \
                    o Object003                   \
                    v 0.000000 2.000000 0.000000  \
                    v 0.000000 0.000000 0.000000  \
                    v 2.000000 0.000000 0.000000  \
                    v 2.000000 2.000000 0.000000  \
                    v 4.000000 0.000000 -1.255298 \
                    v 4.000000 2.000000 -1.255298 \
                    vn 0.000000 0.000000 1.000000 \
                    vn 0.000000 0.000000 1.000000 \
                    vn 0.276597 0.000000 0.960986 \
                    vn 0.276597 0.000000 0.960986 \
                    vn 0.531611 0.000000 0.846988 \
                    vn 0.531611 0.000000 0.846988 \
                    # 6 vertices                  \
                    # 6 normals                   \
                                                  \
                    usemtl material3              \
                    g all                         \
                    s 1                           \
                    f 13//13 14//14 15//15 16//16 \
                    f 16//16 15//15 17//17 18//18 \
                    # 2 elements                  \
                                                  \
                    #### End Object003            \
                "),
                expected: ObjectSet {
                    material_libraries: vec![
                        String::from("material_library.mtl"),
                    ],
                    objects: vec![
                        Object {
                            name: String::from("Object001"),
                            vertex_set: vec![
                                Vertex { x: 0.000000, y: 2.000000, z:  0.000000, w: 1.0 }, 
                                Vertex { x: 0.000000, y: 0.000000, z:  0.000000, w: 1.0 },
                                Vertex { x: 2.000000, y: 0.000000, z:  0.000000, w: 1.0 },
                                Vertex { x: 2.000000, y: 2.000000, z:  0.000000, w: 1.0 },
                                Vertex { x: 4.000000, y: 0.000000, z: -1.255298, w: 1.0 },
                                Vertex { x: 4.000000, y: 2.000000, z: -1.255298, w: 1.0 },
                            ],
                            texture_vertex_set: vec![],
                            normal_vertex_set: vec![
                                NormalVertex { x: 0.000000, y: 0.000000, z: 1.000000 },
                                NormalVertex { x: 0.000000, y: 0.000000, z: 1.000000 },
                                NormalVertex { x: 0.276597, y: 0.000000, z: 0.960986 },
                                NormalVertex { x: 0.276597, y: 0.000000, z: 0.960986 },
                                NormalVertex { x: 0.531611, y: 0.000000, z: 0.846988 },
                                NormalVertex { x: 0.531611, y: 0.000000, z: 0.846988 },
                            ],
                            group_set: vec![
                                Group(String::from("all")), 
                            ],
                            smoothing_group_set: vec![
                                SmoothingGroup(1),
                            ],
                            element_set: vec![
                                Element::Face(VTNIndex::VN(0, 0), VTNIndex::VN(1, 1), VTNIndex::VN(2, 2)),
                                Element::Face(VTNIndex::VN(0, 0), VTNIndex::VN(2, 2), VTNIndex::VN(3, 3)),
                                Element::Face(VTNIndex::VN(3, 3), VTNIndex::VN(2, 2), VTNIndex::VN(4, 4)),
                                Element::Face(VTNIndex::VN(3, 3), VTNIndex::VN(4, 4), VTNIndex::VN(5, 5)),
                            ],
                            shape_set: vec![
                                ShapeEntry { element: 0,  groups: vec![0], smoothing_group: 0 },
                                ShapeEntry { element: 1,  groups: vec![0], smoothing_group: 0 },
                                ShapeEntry { element: 2,  groups: vec![0], smoothing_group: 0 },
                                ShapeEntry { element: 3,  groups: vec![0], smoothing_group: 0 },
                            ],
                            geometry_set: vec![
                                Geometry { material_name: Some(String::from("material1")), shapes: vec![0, 1, 2, 3] },
                            ]
                        },
                        Object {
                            name: String::from("Object002"),
                            vertex_set: vec![
                                Vertex { x: 0.000000, y: 2.000000, z:  0.000000, w: 1.0 }, 
                                Vertex { x: 0.000000, y: 0.000000, z:  0.000000, w: 1.0 },
                                Vertex { x: 2.000000, y: 0.000000, z:  0.000000, w: 1.0 },
                                Vertex { x: 2.000000, y: 2.000000, z:  0.000000, w: 1.0 },
                                Vertex { x: 4.000000, y: 0.000000, z: -1.255298, w: 1.0 },
                                Vertex { x: 4.000000, y: 2.000000, z: -1.255298, w: 1.0 },
                            ],
                            texture_vertex_set: vec![],
                            normal_vertex_set: vec![
                                NormalVertex { x: 0.000000, y: 0.000000, z: 1.000000 },
                                NormalVertex { x: 0.000000, y: 0.000000, z: 1.000000 },
                                NormalVertex { x: 0.276597, y: 0.000000, z: 0.960986 },
                                NormalVertex { x: 0.276597, y: 0.000000, z: 0.960986 },
                                NormalVertex { x: 0.531611, y: 0.000000, z: 0.846988 },
                                NormalVertex { x: 0.531611, y: 0.000000, z: 0.846988 },
                            ],
                            group_set: vec![
                                Group(String::from("all")), 
                            ],
                            smoothing_group_set: vec![
                                SmoothingGroup(1),
                            ],
                            element_set: vec![
                                Element::Face(VTNIndex::VN(0, 0), VTNIndex::VN(1, 1), VTNIndex::VN(2, 2)),
                                Element::Face(VTNIndex::VN(0, 0), VTNIndex::VN(2, 2), VTNIndex::VN(3, 3)),
                                Element::Face(VTNIndex::VN(3, 3), VTNIndex::VN(2, 2), VTNIndex::VN(4, 4)),
                                Element::Face(VTNIndex::VN(3, 3), VTNIndex::VN(4, 4), VTNIndex::VN(5, 5)),
                            ],
                            shape_set: vec![
                                ShapeEntry { element: 0,  groups: vec![0], smoothing_group: 0 },
                                ShapeEntry { element: 1,  groups: vec![0], smoothing_group: 0 },
                                ShapeEntry { element: 2,  groups: vec![0], smoothing_group: 0 },
                                ShapeEntry { element: 3,  groups: vec![0], smoothing_group: 0 },
                            ],
                            geometry_set: vec![
                                Geometry { material_name: Some(String::from("material2")), shapes: vec![0, 1, 2, 3] },
                            ]
                        },
                        Object {
                            name: String::from("Object003"),
                            vertex_set: vec![
                                Vertex { x: 0.000000, y: 2.000000, z:  0.000000, w: 1.0 }, 
                                Vertex { x: 0.000000, y: 0.000000, z:  0.000000, w: 1.0 },
                                Vertex { x: 2.000000, y: 0.000000, z:  0.000000, w: 1.0 },
                                Vertex { x: 2.000000, y: 2.000000, z:  0.000000, w: 1.0 },
                                Vertex { x: 4.000000, y: 0.000000, z: -1.255298, w: 1.0 },
                                Vertex { x: 4.000000, y: 2.000000, z: -1.255298, w: 1.0 },
                            ],
                            texture_vertex_set: vec![],
                            normal_vertex_set: vec![
                                NormalVertex { x: 0.000000, y: 0.000000, z: 1.000000 },
                                NormalVertex { x: 0.000000, y: 0.000000, z: 1.000000 },
                                NormalVertex { x: 0.276597, y: 0.000000, z: 0.960986 },
                                NormalVertex { x: 0.276597, y: 0.000000, z: 0.960986 },
                                NormalVertex { x: 0.531611, y: 0.000000, z: 0.846988 },
                                NormalVertex { x: 0.531611, y: 0.000000, z: 0.846988 },
                            ],
                            group_set: vec![
                                Group(String::from("all")), 
                            ],
                            smoothing_group_set: vec![
                                SmoothingGroup(1),
                            ],
                            element_set: vec![
                                Element::Face(VTNIndex::VN(0, 0), VTNIndex::VN(1, 1), VTNIndex::VN(2, 2)),
                                Element::Face(VTNIndex::VN(0, 0), VTNIndex::VN(2, 2), VTNIndex::VN(3, 3)),
                                Element::Face(VTNIndex::VN(3, 3), VTNIndex::VN(2, 2), VTNIndex::VN(4, 4)),
                                Element::Face(VTNIndex::VN(3, 3), VTNIndex::VN(4, 4), VTNIndex::VN(5, 5)),
                            ],
                            shape_set: vec![
                                ShapeEntry { element: 0,  groups: vec![0], smoothing_group: 0 },
                                ShapeEntry { element: 1,  groups: vec![0], smoothing_group: 0 },
                                ShapeEntry { element: 2,  groups: vec![0], smoothing_group: 0 },
                                ShapeEntry { element: 3,  groups: vec![0], smoothing_group: 0 },
                            ],
                            geometry_set: vec![
                                Geometry { material_name: Some(String::from("material3")), shapes: vec![0, 1, 2, 3] },
                            ]
                        },
                    ]
                }
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

