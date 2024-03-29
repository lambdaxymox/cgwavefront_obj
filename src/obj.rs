use crate::lexer::{
    Lexer,
    PeekableLexer,
};
use std::error;
use std::fmt;


/// Parse a set of objects and material library names from a string.
///
/// ## Example
///
/// ```
/// # use wavefront_obj::obj;
/// # use wavefront_obj::obj::{
/// #      Vertex,
/// #      NormalVertex,
/// #      Group,
/// #      SmoothingGroup,
/// #      Element,
/// #      ShapeEntry,
/// #      Geometry,
/// #      VTNIndex,
/// #      Object,
/// #      ObjectSet,
/// # };
/// #
/// let obj_file = String::from(r"
///     mtllib material_library.mtl   \
///     o Object001                   \
///     v 0.000000 2.000000 0.000000  \
///     v 0.000000 0.000000 0.000000  \
///     v 2.000000 0.000000 0.000000  \
///     v 2.000000 2.000000 0.000000  \
///     v 4.000000 0.000000 -1.255298 \
///     v 4.000000 2.000000 -1.255298 \
///     vn 0.000000 0.000000 1.000000 \
///     vn 0.000000 0.000000 1.000000 \
///     vn 0.276597 0.000000 0.960986 \
///     vn 0.276597 0.000000 0.960986 \
///     vn 0.531611 0.000000 0.846988 \
///     vn 0.531611 0.000000 0.846988 \
///     ## 6 vertices                 \
///     ## 6 normals                  \
///                                   \
///     usemtl material               \
///     g all                         \
///     s 1                           \
///     f 1//1 2//2 3//3 4//4         \
///     f 4//4 3//3 5//5 6//6         \
///     ## 2 elements                 \
///                                   \
///     #### End Object001            \
///                                   \
/// ");
/// // let expected = ...;
/// # let expected = ObjectSet {
/// #     material_libraries: vec![
/// #         String::from("material_library.mtl"),
/// #     ],
/// #     objects: vec![
/// #         Object {
/// #             name: String::from("Object001"),
/// #             vertex_set: vec![
/// #                 Vertex { x: 0.000000, y: 2.000000, z:  0.000000, w: 1.0 },
/// #                 Vertex { x: 0.000000, y: 0.000000, z:  0.000000, w: 1.0 },
/// #                 Vertex { x: 2.000000, y: 0.000000, z:  0.000000, w: 1.0 },
/// #                 Vertex { x: 2.000000, y: 2.000000, z:  0.000000, w: 1.0 },
/// #                 Vertex { x: 4.000000, y: 0.000000, z: -1.255298, w: 1.0 },
/// #                 Vertex { x: 4.000000, y: 2.000000, z: -1.255298, w: 1.0 },
/// #             ],
/// #             texture_vertex_set: vec![],
/// #             normal_vertex_set: vec![
/// #                 NormalVertex { x: 0.000000, y: 0.000000, z: 1.000000 },
/// #                 NormalVertex { x: 0.000000, y: 0.000000, z: 1.000000 },
/// #                 NormalVertex { x: 0.276597, y: 0.000000, z: 0.960986 },
/// #                 NormalVertex { x: 0.276597, y: 0.000000, z: 0.960986 },
/// #                 NormalVertex { x: 0.531611, y: 0.000000, z: 0.846988 },
/// #                 NormalVertex { x: 0.531611, y: 0.000000, z: 0.846988 },
/// #             ],
/// #             group_set: vec![
/// #                 Group(String::from("all")),
/// #             ],
/// #             smoothing_group_set: vec![
/// #                 SmoothingGroup(1),
/// #             ],
/// #             element_set: vec![
/// #                 Element::Face(VTNIndex::VN(0, 0), VTNIndex::VN(1, 1), VTNIndex::VN(2, 2)),
/// #                 Element::Face(VTNIndex::VN(0, 0), VTNIndex::VN(2, 2), VTNIndex::VN(3, 3)),
/// #                 Element::Face(VTNIndex::VN(3, 3), VTNIndex::VN(2, 2), VTNIndex::VN(4, 4)),
/// #                 Element::Face(VTNIndex::VN(3, 3), VTNIndex::VN(4, 4), VTNIndex::VN(5, 5)),
/// #             ],
/// #             shape_set: vec![
/// #                 ShapeEntry { element: 0,  groups: vec![0], smoothing_group: 0 },
/// #                 ShapeEntry { element: 1,  groups: vec![0], smoothing_group: 0 },
/// #                 ShapeEntry { element: 2,  groups: vec![0], smoothing_group: 0 },
/// #                 ShapeEntry { element: 3,  groups: vec![0], smoothing_group: 0 },
/// #             ],
/// #             geometry_set: vec![
/// #                 Geometry { material_name: Some(String::from("material")), shapes: vec![0, 1, 2, 3] },
/// #             ]
/// #         }
/// #     ]
/// # };
/// let result = obj::parse(&obj_file);
/// assert!(result.is_ok());
///
/// let result = result.unwrap();
/// assert_eq!(result.material_libraries, expected.material_libraries);
/// ```
pub fn parse<T: AsRef<str>>(input: T) -> Result<ObjectSet, ParseError> {
    Parser::new(input.as_ref()).parse_objset()
}


/// A single three dimensional point in an object, or a single
/// three-dimensional point of an object in homogeneous coordinates
/// when the w-component is one.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vertex {
    /// The **x-axis** component of a vertex.
    pub x: f64,
    /// The **y-axis** component of a vertex.
    pub y: f64,
    /// The **z-axis** component of a vertex.
    pub z: f64,
    /// The **w-axis** (homogeneous) component of a vertex. The default value
    /// of this field is 0 when the w coordinate is not present.
    pub w: f64,
}

impl fmt::Display for Vertex {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(formatter, "v  {}  {}  {}  {}", self.x, self.y, self.z, self.w)
    }
}


/// A single three-dimensional coordinate in a texture.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TextureVertex {
    /// The horizontal coordinate of a texture vertex.
    pub u: f64,
    /// The vertical coordinate of a texture vertex.
    pub v: f64,
    /// The depth coordinate of a texture vertex.
    pub w: f64,
}

impl fmt::Display for TextureVertex {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(formatter, "vt  {}  {}  {}", self.u, self.v, self.w)
    }
}


/// A normal vector at a vertex in an object.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NormalVertex {
    /// The **x-axis** component of a normal vector.
    pub x: f64,
    /// The **y-axis** component of a normal vector.
    pub y: f64,
    /// The **z-axis** componont of a normal vector.
    pub z: f64,
}

impl fmt::Display for NormalVertex {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(formatter, "vn  {}  {}  {}", self.x, self.y, self.z)
    }
}


/// A general vertex/texture/normal index representing the indices
/// of a vertex, texture vertex, and normal vector in an element
/// of a geometry figure.
///
/// A VTN index has the forms of **vertex**, **vertex/texture**,
/// **vertex//normal**, or **vertex/texture/normal** indices,
/// which indicates which data of vertices, texture vertices, and
/// normal vectors are bound to each vertex in a shape element.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum VTNIndex {
    V(VertexIndex),
    VT(VertexIndex, TextureVertexIndex),
    VN(VertexIndex, NormalVertexIndex),
    VTN(VertexIndex, TextureVertexIndex, NormalVertexIndex),
}

impl VTNIndex {
    /// Determine whether two VTN indices have the same form.
    ///
    /// The function returns true if both VTN indices are of the form
    /// vertex, vertex/texture/ vertex//normal, or vertex/texture/normal.
    /// Otherwise, the function returns false.
    ///
    /// ## Example
    ///
    /// ```
    /// # use wavefront_obj::obj::{
    /// #     VTNIndex,
    /// # };
    /// #
    /// let v_index1 = VTNIndex::V(0);
    /// let v_index2 = VTNIndex::V(1);
    /// assert!(v_index1.has_same_type_as(&v_index2));
    ///
    /// let vt_index1 = VTNIndex::VT(2, 3);
    /// let vt_index2 = VTNIndex::VT(4, 5);
    /// assert!(vt_index1.has_same_type_as(&vt_index2));
    ///
    /// let vn_index1 = VTNIndex::VN(6, 7);
    /// let vn_index2 = VTNIndex::VN(8, 9);
    /// assert!(vn_index1.has_same_type_as(&vn_index2));
    ///
    /// let vtn_index1 = VTNIndex::VTN(10, 11, 12);
    /// let vtn_index2 = VTNIndex::VTN(13, 14, 15);
    /// assert!(vtn_index1.has_same_type_as(&vtn_index2));
    ///
    /// assert!(!v_index1.has_same_type_as(&vt_index1));
    /// assert!(!v_index1.has_same_type_as(&vn_index1));
    /// assert!(!v_index1.has_same_type_as(&vtn_index1));
    /// assert!(!vt_index1.has_same_type_as(&vn_index1));
    /// assert!(!vt_index1.has_same_type_as(&vtn_index1));
    /// assert!(!vn_index1.has_same_type_as(&vtn_index1));
    /// ```
    pub fn has_same_type_as(&self, other: &VTNIndex) -> bool {
        matches!(
            (self, other),
            (&VTNIndex::V(_), &VTNIndex::V(_))
                | (&VTNIndex::VT(_, _), &VTNIndex::VT(_, _))
                | (&VTNIndex::VN(_, _), &VTNIndex::VN(_, _))
                | (&VTNIndex::VTN(_, _, _), &VTNIndex::VTN(_, _, _))
        )
    }
}

impl fmt::Display for VTNIndex {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        // NOTE: The library represented VTN indices starting form 0, whereas
        // *.obj files index starting from 1, so we must add one to each index
        // when displaying the data back in a form that looks like the original
        // file.
        match *self {
            VTNIndex::V(v) => {
                write!(formatter, "{}", v + 1)
            }
            VTNIndex::VT(v, vt) => {
                write!(formatter, "{}/{}", v + 1, vt + 1)
            }
            VTNIndex::VN(v, vn) => {
                write!(formatter, "{}//{}", v + 1, vn + 1)
            }
            VTNIndex::VTN(v, vt, vn) => {
                write!(formatter, "{}/{}/{}", v + 1, vt + 1, vn + 1)
            }
        }
    }
}


type ElementIndex = usize;
type VertexIndex = usize;
type TextureVertexIndex = usize;
type NormalVertexIndex = usize;
type GroupIndex = usize;
type SmoothingGroupIndex = usize;
type ShapeEntryIndex = usize;


/// An element is the smallest component of a more complex geometric figure.
///
/// An element can be either a point, line, or a face (triangle). A geometric figures
/// is a collection of elements. Typically, a geometric figure consists of elements that
/// are all the same type, i.e. a three-dimensional object is composed of all faces,
/// or a line is composed of all line elements.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Element {
    Point(VTNIndex),
    Line(VTNIndex, VTNIndex),
    Face(VTNIndex, VTNIndex, VTNIndex),
}

impl fmt::Display for Element {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            Element::Point(vtn) => {
                write!(formatter, "p  {}", vtn)
            }
            Element::Line(vtn1, vtn2) => {
                write!(formatter, "l  {}  {}", vtn1, vtn2)
            }
            Element::Face(vtn1, vtn2, vtn3) => {
                write!(formatter, "f  {}  {}  {}", vtn1, vtn2, vtn3)
            }
        }
    }
}

/// A group is a label for a collection of elements within an object.
///
/// A collection of groups enables one to organize collections of elements
/// by group.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Group(pub String);

impl fmt::Display for Group {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(formatter, "{}", self.0)
    }
}

impl Default for Group {
    fn default() -> Group {
        Group(String::from("default"))
    }
}

/// A smoothing group is a label providing information on which collections
/// of elements should have their normal vectors interpolated over give
/// those elements a non-faceted appearance.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct SmoothingGroup(pub usize);

impl fmt::Display for SmoothingGroup {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        if self.0 == 0 {
            write!(formatter, "off")
        } else {
            write!(formatter, "{}", self.0)
        }
    }
}

/// A shape entry is a collection of indices grouping together all the
/// organizational information about each element in an object.
#[derive(Clone, Debug, PartialEq)]
pub struct ShapeEntry {
    /// The index of the element in the element set that the shape entry describes.
    pub element: ElementIndex,
    /// The groups that a particular element belongs to.
    pub groups: Vec<GroupIndex>,
    /// The smoothing group that a particular element belongs to.
    pub smoothing_group: SmoothingGroupIndex,
}

/// A shape is a collection of data grouping together all the organizational
/// information about each element in an object.
#[derive(Clone, Debug, PartialEq)]
pub struct Shape {
    /// The element from the element set.
    pub element: Element,
    /// The names of the groups that a particular element belongs to.
    pub groups: Vec<Group>,
    /// The smoothing group that a particular element belongs to.
    pub smoothing_groups: Vec<SmoothingGroup>,
}

/// The geometry inside an object is a collection of elements along with their
/// material description data.
///
/// The material description data describes which material from a corresponding
/// material library contains the data for rendering each primitive in the set of
/// shapes.
#[derive(Clone, Debug, PartialEq)]
pub struct Geometry {
    /// The material name that contains the material and lighting properties
    /// of each shape in this collection.
    pub material_name: Option<String>,
    /// The shapes associated with a particular material and geometry.
    pub shapes: Vec<ShapeEntryIndex>,
}

/// A VTN triple contains the actual data of each element in an object.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum VTNTriple<'a> {
    V(&'a Vertex),
    VT(&'a Vertex, &'a TextureVertex),
    VN(&'a Vertex, &'a NormalVertex),
    VTN(&'a Vertex, &'a TextureVertex, &'a NormalVertex),
}

/// An object is a collection of vertices, texture vertices, normal vectors,
/// and geometric primitives composing a unit of geometry in a scene to
/// be rendered.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Object {
    /// The name of the object.
    pub name: String,
    /// The set of vertices in an object.
    pub vertex_set: Vec<Vertex>,
    /// The set of texture coordinates in an object for mapping materials onto
    /// an object.
    pub texture_vertex_set: Vec<TextureVertex>,
    /// The set of normal vectors defined at each vertex in an object.
    pub normal_vertex_set: Vec<NormalVertex>,
    /// The set of names of groups of elements in an object.
    pub group_set: Vec<Group>,
    /// The set of names of smoothing groups of elements in an object.
    pub smoothing_group_set: Vec<SmoothingGroup>,
    /// The set of primitives (i.e. points, lines, and faces) in an object.
    pub element_set: Vec<Element>,
    /// The set of grouping data associated with each element in an object.
    pub shape_set: Vec<ShapeEntry>,
    /// The set of elements associated with each material used in an object.
    pub geometry_set: Vec<Geometry>,
}

impl Object {
    /// Fetch the vertex/texture/normal of a vertex in an object.
    ///
    /// The function returns `None` if any of the VTN indices are not found
    /// inside the object.
    ///
    /// ## Example
    ///
    /// ```
    /// # use wavefront_obj::obj;
    /// # use wavefront_obj::obj::{
    /// #    VTNIndex,
    /// #    VTNTriple,
    /// #    Vertex,
    /// #    TextureVertex,
    /// #    NormalVertex,
    /// # };
    /// #
    /// let obj_file = String::from(r"
    ///     o quad                    \
    ///     v -0.5 -0.5 0.0           \
    ///     v  0.5 -0.5 0.0           \
    ///     v  0.5  0.5 0.0           \
    ///     v -0.5  0.5 0.0           \
    ///     ## 4 vertices             \
    ///                               \
    ///     vt 0.0 0.0 0.0            \
    ///     vt 1.0 0.0 0.0            \
    ///     vt 1.0 1.0 0.0            \
    ///     vt 0.0 1.0 0.0            \
    ///     ## 4 texture vertices     \
    ///                               \
    ///     vn 0.0 0.0 1.0            \
    ///     vn 0.0 0.0 1.0            \
    ///     vn 0.0 0.0 1.0            \
    ///     vn 0.0 0.0 1.0            \
    ///     ## 4 normal vertices      \
    ///                               \
    ///     f 1/1/1 2/2/2 3/3/3 4/4/4 \
    ///     ## 2 faces                \
    ///     ## end quad               \
    /// ");
    /// let obj_set = obj::parse(&obj_file).unwrap();
    ///
    /// // The vertex data of an obj file are stored 1-indexed, but the library stores
    /// // the vertex data 0-indexed, so one must add one to each index to get the indices
    /// // as they would appear in a *.obj file.
    /// let vtn_index0 = VTNIndex::VTN(0, 0, 0);
    /// let vtn_index1 = VTNIndex::VTN(1, 1, 1);
    /// let vtn_index2 = VTNIndex::VTN(2, 2, 2);
    /// let vtn_index3 = VTNIndex::VTN(3, 3, 3);
    ///
    /// let object = &obj_set.objects[0];
    /// let vtn_triple0 = object.get_vtn_triple(vtn_index0);
    /// let vtn_triple1 = object.get_vtn_triple(vtn_index1);
    /// let vtn_triple2 = object.get_vtn_triple(vtn_index2);
    /// let vtn_triple3 = object.get_vtn_triple(vtn_index3);
    ///
    /// // Explicitly construct the vertex data so we can make comparisons.
    /// # let vertex0 = Vertex { x: -0.5, y: -0.5, z: 0.0, w: 1.0 };
    /// # let vertex1 = Vertex { x:  0.5, y: -0.5, z: 0.0, w: 1.0 };
    /// # let vertex2 = Vertex { x:  0.5, y:  0.5, z: 0.0, w: 1.0 };
    /// # let vertex3 = Vertex { x: -0.5, y:  0.5, z: 0.0, w: 1.0 };
    /// # let texture_vertex0 = TextureVertex { u: 0.0, v: 0.0, w: 0.0 };
    /// # let texture_vertex1 = TextureVertex { u: 1.0, v: 0.0, w: 0.0 };
    /// # let texture_vertex2 = TextureVertex { u: 1.0, v: 1.0, w: 0.0 };
    /// # let texture_vertex3 = TextureVertex { u: 0.0, v: 1.0, w: 0.0 };
    /// # let normal_vertex0 = NormalVertex { x: 0.0, y: 0.0, z: 1.0 };
    /// # let normal_vertex1 = NormalVertex { x: 0.0, y: 0.0, z: 1.0 };
    /// # let normal_vertex2 = NormalVertex { x: 0.0, y: 0.0, z: 1.0 };
    /// # let normal_vertex3 = NormalVertex { x: 0.0, y: 0.0, z: 1.0 };
    /// let expected0 = Some(VTNTriple::VTN(&vertex0, &texture_vertex0, &normal_vertex0));
    /// let expected1 = Some(VTNTriple::VTN(&vertex1, &texture_vertex1, &normal_vertex1));
    /// let expected2 = Some(VTNTriple::VTN(&vertex2, &texture_vertex2, &normal_vertex2));
    /// let expected3 = Some(VTNTriple::VTN(&vertex3, &texture_vertex3, &normal_vertex3));
    ///
    /// assert_eq!(vtn_triple0, expected0);
    /// assert_eq!(vtn_triple1, expected1);
    /// assert_eq!(vtn_triple2, expected2);
    /// assert_eq!(vtn_triple3, expected3);
    ///
    /// // VTN indices lying outside the ones stored in the oject should return nothing.
    /// assert!(object.get_vtn_triple(VTNIndex::VTN(4, 4, 4)).is_none());
    /// ```
    pub fn get_vtn_triple(&self, index: VTNIndex) -> Option<VTNTriple> {
        match index {
            VTNIndex::V(v_index) => {
                let vertex = self.vertex_set.get(v_index)?;

                Some(VTNTriple::V(vertex))
            }
            VTNIndex::VT(v_index, vt_index) => {
                let vertex = self.vertex_set.get(v_index)?;
                let texture_vertex = self.texture_vertex_set.get(vt_index)?;

                Some(VTNTriple::VT(vertex, texture_vertex))
            }
            VTNIndex::VN(v_index, vn_index) => {
                let vertex = self.vertex_set.get(v_index)?;
                let normal_vertex = self.normal_vertex_set.get(vn_index)?;

                Some(VTNTriple::VN(vertex, normal_vertex))
            }
            VTNIndex::VTN(v_index, vt_index, vn_index) => {
                let vertex = self.vertex_set.get(v_index)?;
                let texture_vertex = self.texture_vertex_set.get(vt_index)?;
                let normal_vertex = self.normal_vertex_set.get(vn_index)?;

                Some(VTNTriple::VTN(vertex, texture_vertex, normal_vertex))
            }
        }
    }
}

struct DisplayObjectCompositor {}

impl DisplayObjectCompositor {
    fn new() -> Self {
        Self {}
    }

    fn compose_set<T: fmt::Display>(&self, set: &[T], name: &str) -> String {
        let mut string = format!("    {} set:\n", name);
        if set.is_empty() {
            string += "        data: []\n";
        } else {
            string += &format!("        data: [({}) ... ({})]\n", set[0], set[set.len() - 1]);
        }
        string += &format!("        length: {}\n", set.len());

        string
    }

    fn compose(&self, object: &Object) -> String {
        let mut string = String::from("Object {\n");

        string += &format!("    name: {}\n", object.name);
        string += &self.compose_set(&object.vertex_set, "vertex");
        string += &self.compose_set(&object.texture_vertex_set, "texture vertex");
        string += &self.compose_set(&object.normal_vertex_set, "normal vertex");
        string += &self.compose_set(&object.group_set, "group");
        string += &self.compose_set(&object.smoothing_group_set, "smoothing group");
        string += &self.compose_set(&object.element_set, "element");
        string += "}}\n";

        string
    }
}

impl fmt::Display for Object {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let string = DisplayObjectCompositor::new().compose(self);
        write!(formatter, "{}", string)
    }
}

/// An object set is a collection of objects and material library named obtained
/// from parsing an `*.obj` file. An `*.obj` file may contain more that one object.
#[derive(Clone, Debug, PartialEq)]
pub struct ObjectSet {
    /// The set of material libraries associated with the object set.
    pub material_libraries: Vec<String>,
    /// The set of objects in an object set.
    pub objects: Vec<Object>,
}

impl fmt::Display for ObjectSet {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let compositor = DisplayObjectCompositor::new();
        let mut string = String::from("ObjectSet {\n");

        for object in self.objects.iter() {
            string += &compositor.compose(object);
            string += &"\n";
        }

        string += &"}\n";

        write!(formatter, "{}", string)
    }
}

/// A marker indicating the type of error generated during parsing of a
/// Wavefront OBJ file.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ErrorKind {
    /// The parser reached the end of the input early.
    EndOfFile,
    /// The parser expected a tag statement that was not present.
    ExpectedTagStatement,
    /// The parser expected a floating point number but found something else.
    ExpectedFloat,
    /// The parser expected an integer but found something else.
    ExpectedInteger,
    /// The parser expected a vertex/texture/normal index but found something else.
    ExpectedVTNIndex,
    /// the parser encountered an object element index that is out of range.
    VTNIndexOutOfRange,
    /// The parser encountered a face element that did not have enough vertices.
    EveryFaceElementMustHaveAtLeastThreeVertices,
    /// An element had VTN indices with different forms.
    EveryVTNIndexMustHaveTheSameFormForAGivenElement,
    /// A statement in a wavefront obj file that is either unsupported or does not exist.
    InvalidObjectStatement,
    /// The parser encountered an invalid or unsupported element type.
    ElementMustBeAPointLineOrFace,
    /// The smoothing group name is something other than an integer or the default
    /// value `off`.
    SmoothingGroupNameMustBeOffOrInteger,
    /// The smoothing group declaration is missing a name.
    SmoothingGroupDeclarationHasNoName,
    /// The `usemtl` statement has no corresponding material name.
    MaterialStatementHasNoName,
}

/// An error that is returned from parsing an invalid `*.obj` file, or
/// another kind of error.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParseError {
    /// The line number where the error occurred.
    pub line_number: usize,
    /// The kind of error that occurred.
    pub kind: ErrorKind,
    /// A message describing why the parse error was generated.
    pub message: String,
}

impl ParseError {
    /// Construct a new parse error.
    fn new(line_number: usize, kind: ErrorKind, message: String) -> ParseError {
        ParseError {
            line_number: line_number,
            kind: kind,
            message: message,
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            formatter,
            "Parse error at line {}: {}",
            self.line_number, self.message
        )
    }
}

impl error::Error for ParseError {}


/// A Wavefront OBJ file parser extracts three-dimensional geometric data
/// from a `*.obj` file.
#[derive(Clone)]
pub struct Parser<'a> {
    /// The current line position of the parser in the input stream.
    line_number: usize,
    /// the underlying lexer that generates tokens.
    lexer: PeekableLexer<'a>,
}

/// Triangulate a polygon with a triangle fan.
///
/// NOTE: the OBJ specification assumes that polygons are coplanar, and
/// consequently the parser does not check this. It is up to the model creator
/// to ensure this.
#[inline]
fn triangulate(elements: &mut Vec<Element>, vtn_indices: &[VTNIndex]) -> usize {
    let vertex0 = vtn_indices[0];
    for i in 0..(vtn_indices.len() - 2) {
        elements.push(Element::Face(vertex0, vtn_indices[i + 1], vtn_indices[i + 2]));
    }

    vtn_indices.len() - 2
}

/// Verify that each VTN index has the same type and has a valid form.
#[inline]
fn verify_vtn_indices(vtn_indices: &[VTNIndex]) -> bool {
    for i in 1..vtn_indices.len() {
        if !vtn_indices[i].has_same_type_as(&vtn_indices[0]) {
            return false;
        }
    }

    true
}

impl<'a> Parser<'a> {
    /// Construct a new Wavefront OBJ file parser.
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
                "Reached the end of the input in the process of getting the next token.".to_owned(),
            ),
        }
    }

    /// Advance the token stream if the next token in the stream matches the
    /// input tag.
    ///
    /// This functions returns an error if the expected tag is not present.
    fn expect_tag(&mut self, tag: &str) -> Result<(), ParseError> {
        match self.next() {
            None => self.error(
                ErrorKind::EndOfFile,
                "Reached the end of the input in the process of getting the next token.".to_owned(),
            ),
            Some(st) if st != tag => self.error(
                ErrorKind::ExpectedTagStatement,
                format!("Expected `{}` but got `{}` instead.", tag, st),
            ),
            _ => Ok(()),
        }
    }

    /// Parse a floating point number from the current token in the stream.
    fn parse_f64(&mut self) -> Result<f64, ParseError> {
        let st = self.next_string()?;
        match st.parse::<f64>() {
            Ok(val) => Ok(val),
            Err(_) => self.error(
                ErrorKind::ExpectedFloat,
                format!("Expected a floating point number but got `{}` instead.", st),
            ),
        }
    }

    /// Parse an integer from the current token in the stream.
    fn parse_isize(&mut self) -> Result<isize, ParseError> {
        let st = self.next_string()?;
        match st.parse::<isize>() {
            Ok(val) => Ok(val),
            Err(_) => self.error(
                ErrorKind::ExpectedInteger,
                format!("Expected an integer but got `{}` instead.", st),
            ),
        }
    }

    /// Apply a parser to the input stream.
    ///
    /// If the parser `parser` fails to parse the current token in the stream,
    /// it returns nothing and the stream state does not change. Otherwise, the
    /// stream advances and the corresponding result is returned.
    fn try_once<P, T>(&mut self, parser: P) -> Option<T>
    where
        P: FnOnce(&str) -> Option<T>,
    {
        match self.peek() {
            Some(st) => parser(st).map(|got| {
                self.advance();
                got
            }),
            None => None,
        }
    }

    /// Parse a vertex from the input.
    fn parse_vertex(&mut self) -> Result<Vertex, ParseError> {
        self.expect_tag("v")?;

        let x = self.parse_f64()?;
        let y = self.parse_f64()?;
        let z = self.parse_f64()?;
        let mw = self.try_once(|st| st.parse::<f64>().ok());
        let w = mw.unwrap_or(1_f64);

        Ok(Vertex {
            x: x,
            y: y,
            z: z,
            w: w,
        })
    }

    /// Parse a texture vertex from the input.
    fn parse_texture_vertex(&mut self) -> Result<TextureVertex, ParseError> {
        self.expect_tag("vt")?;

        let u = self.parse_f64()?;
        let mv = self.try_once(|st| st.parse::<f64>().ok());
        let v = mv.unwrap_or(0_f64);
        let mw = self.try_once(|st| st.parse::<f64>().ok());
        let w = mw.unwrap_or(0_f64);

        Ok(TextureVertex { u: u, v: v, w: w })
    }

    /// Parse a normal vector from the input.
    fn parse_normal_vertex(&mut self) -> Result<NormalVertex, ParseError> {
        self.expect_tag("vn")?;

        let x = self.parse_f64()?;
        let y = self.parse_f64()?;
        let z = self.parse_f64()?;

        Ok(NormalVertex { x: x, y: y, z: z })
    }

    /// Skip over any number of newlines in the input stream.
    fn skip_zero_or_more_newlines(&mut self) {
        while let Some("\n") = self.peek() {
            self.advance();
        }
    }

    /// Skip over at least one newline in the input stream.
    ///
    /// The function returns an error if no newline tokens are present.
    fn skip_one_or_more_newlines(&mut self) -> Result<(), ParseError> {
        self.expect_tag("\n")?;
        self.skip_zero_or_more_newlines();
        Ok(())
    }

    /// Parse the name of an object.
    fn parse_object_name(&mut self) -> Result<&'a str, ParseError> {
        match self.peek() {
            Some("o") => {
                self.expect_tag("o")?;
                let object_name = self.next_string();
                self.skip_one_or_more_newlines()?;

                object_name
            }
            _ => Ok(""),
        }
    }

    #[inline(always)]
    fn calculate_index(&self, value_range: (usize, usize), parsed_value: isize) -> Result<usize, ParseError> {
        let (min_value, max_value) = value_range;
        let actual_value = if parsed_value <= 0 {
            max_value as isize - parsed_value
        } else {
            parsed_value - 1
        };

        if (actual_value >= min_value as isize) && (actual_value < max_value as isize) {
            debug_assert!(actual_value >= 0);
            Ok((actual_value - min_value as isize) as usize)
        } else {
            self.error(
                ErrorKind::VTNIndexOutOfRange,
                format!(
                    "Expected index in range [{}, {}), but got {}.",
                    min_value, max_value, actual_value
                ),
            )
        }
    }

    /// Parse a vertex/texture/normal index.
    fn parse_vtn_index(
        &mut self,
        vertex_index_range: (usize, usize),
        texture_index_range: (usize, usize),
        normal_index_range: (usize, usize),
    ) -> Result<VTNIndex, ParseError> {
        let st = self.next_string()?;
        let process_split = |split: &str, value_range: (usize, usize)| -> Result<Option<usize>, ParseError> {
            if !split.is_empty() {
                let parsed_value = split.parse::<isize>().or_else(|_| {
                    self.error(
                        ErrorKind::ExpectedInteger,
                        format!("Expected an integer but got `{}` instead.", split),
                    )
                })?;
                let index = self.calculate_index(value_range, parsed_value)?;
                Ok(Some(index))
            } else {
                Ok(None)
            }
        };

        let mut splits_iter = st.split('/');
        let split1 = splits_iter
            .next()
            .and_then(|s| process_split(s, vertex_index_range).transpose())
            .transpose()?;
        let split2 = splits_iter
            .next()
            .and_then(|s| process_split(s, texture_index_range).transpose())
            .transpose()?;
        let split3 = splits_iter
            .next()
            .and_then(|s| process_split(s, normal_index_range).transpose())
            .transpose()?;
        if split1.is_none() || splits_iter.next().is_some() {
            return self.error(
                ErrorKind::ExpectedVTNIndex,
                format!(
                    "Expected a `vertex/texture/normal` index but got `{}` instead.",
                    st
                ),
            );
        }

        match (split1, split2, split3) {
            (Some(v), None, None) => Ok(VTNIndex::V(v)),
            (Some(v), None, Some(vn)) => Ok(VTNIndex::VN(v, vn)),
            (Some(v), Some(vt), None) => Ok(VTNIndex::VT(v, vt)),
            (Some(v), Some(vt), Some(vn)) => Ok(VTNIndex::VTN(v, vt, vn)),
            _ => self.error(
                ErrorKind::ExpectedVTNIndex,
                format!(
                    "Expected a `vertex/texture/normal` index but got `{}` instead.",
                    st
                ),
            ),
        }
    }

    /// Parse one more more VTN indices.
    ///
    /// Return the number of VTN indices parsed if no errors occurred.
    fn parse_vtn_indices(
        &mut self,
        vtn_indices: &mut Vec<VTNIndex>,
        vertex_index_range: (usize, usize),
        texture_index_range: (usize, usize),
        normal_index_range: (usize, usize),
    ) -> Result<usize, ParseError> {
        let mut indices_parsed = 0;
        while let Ok(vtn_index) =
            self.parse_vtn_index(vertex_index_range, texture_index_range, normal_index_range)
        {
            vtn_indices.push(vtn_index);
            indices_parsed += 1;
        }

        Ok(indices_parsed)
    }

    /// Parse one or more point from the current line in the input stream.
    ///
    /// There can be more than one point in a single line of input, so
    /// this parsing rule will attempt to read all of them.
    fn parse_point(
        &mut self,
        elements: &mut Vec<Element>,
        vertex_index_range: (usize, usize),
    ) -> Result<usize, ParseError> {
        self.expect_tag("p")?;

        let parsed_value = self.parse_isize()?;
        let v_index = self.calculate_index(vertex_index_range, parsed_value)?;
        elements.push(Element::Point(VTNIndex::V(v_index)));
        let mut elements_parsed = 1;
        loop {
            match self.next() {
                Some(st) if st != "\n" => match st.parse::<isize>() {
                    Ok(val) => {
                        let v_index = self.calculate_index(vertex_index_range, val)?;
                        elements.push(Element::Point(VTNIndex::V(v_index)));
                        elements_parsed += 1;
                    }
                    Err(_) => {
                        return self.error(
                            ErrorKind::ExpectedInteger,
                            format!("Expected an integer but got `{}` instead.", st),
                        )
                    }
                },
                _ => break,
            }
        }

        Ok(elements_parsed)
    }

    /// Parse one more more line elements from a line of text input from the input.
    ///
    /// If the parser cannot parse each line element from a line of text input, the
    /// parser returns an error.
    fn parse_line(
        &mut self,
        elements: &mut Vec<Element>,
        vertex_index_range: (usize, usize),
        texture_index_range: (usize, usize),
        normal_index_range: (usize, usize),
    ) -> Result<usize, ParseError> {
        self.expect_tag("l")?;

        let mut vtn_indices = vec![];
        vtn_indices.push(self.parse_vtn_index(
            vertex_index_range,
            texture_index_range,
            normal_index_range,
        )?);
        vtn_indices.push(self.parse_vtn_index(
            vertex_index_range,
            texture_index_range,
            normal_index_range,
        )?);
        self.parse_vtn_indices(
            &mut vtn_indices,
            vertex_index_range,
            texture_index_range,
            normal_index_range,
        )?;

        if !verify_vtn_indices(&vtn_indices) {
            return self.error(
                ErrorKind::EveryVTNIndexMustHaveTheSameFormForAGivenElement,
                "Every VTN index for a line must have the same form.".to_owned(),
            );
        }

        // Now that we have verified the indices, build the line elements.
        for i in 0..(vtn_indices.len() - 1) {
            elements.push(Element::Line(vtn_indices[i], vtn_indices[i + 1]));
        }

        Ok(vtn_indices.len() - 1)
    }

    /// Parse one or more faces from a single line of text input.
    ///
    /// All face vertices must have the same vertex/texture/normal form on
    /// a line of input. If they do not, the parser will return an error. Otherwise,
    /// it succeeds. The face parser unpacks the face elements by treating the line
    /// of face indices as a triangle fan.
    ///
    /// The parser returns the number of triangles generated.
    fn parse_face(
        &mut self,
        elements: &mut Vec<Element>,
        vertex_index_range: (usize, usize),
        texture_index_range: (usize, usize),
        normal_index_range: (usize, usize),
    ) -> Result<usize, ParseError> {
        self.expect_tag("f")?;

        let mut vtn_indices = vec![];

        self.parse_vtn_indices(
            &mut vtn_indices,
            vertex_index_range,
            texture_index_range,
            normal_index_range,
        )?;

        // Check that there are enough vtn indices.
        if vtn_indices.len() < 3 {
            return self.error(
                ErrorKind::EveryFaceElementMustHaveAtLeastThreeVertices,
                "A face primitive must have at least three vertices.".to_owned(),
            );
        }

        if !verify_vtn_indices(&vtn_indices) {
            return self.error(
                ErrorKind::EveryVTNIndexMustHaveTheSameFormForAGivenElement,
                "Every VTN index for a face must have the same form.".to_owned(),
            );
        }

        let face_count = triangulate(elements, &vtn_indices);

        Ok(face_count)
    }

    /// Parse all the elements of a givne type from a line of text input.
    fn parse_elements(
        &mut self,
        elements: &mut Vec<Element>,
        vertex_index_range: (usize, usize),
        texture_index_range: (usize, usize),
        normal_index_range: (usize, usize),
    ) -> Result<usize, ParseError> {
        match self.peek() {
            Some("p") => self.parse_point(elements, vertex_index_range),
            Some("l") => self.parse_line(
                elements,
                vertex_index_range,
                texture_index_range,
                normal_index_range,
            ),
            Some("f") => self.parse_face(
                elements,
                vertex_index_range,
                texture_index_range,
                normal_index_range,
            ),
            _ => self.error(
                ErrorKind::ElementMustBeAPointLineOrFace,
                "An element must be a point (`p`), line (`l`), or face (`f`).".to_owned(),
            ),
        }
    }

    /// Parse group names from a line of text input.
    fn parse_groups(&mut self, groups: &mut Vec<Group>) -> Result<usize, ParseError> {
        self.expect_tag("g")?;
        let mut groups_parsed = 0;
        loop {
            match self.next() {
                Some(name) if name != "\n" => {
                    groups.push(Group(String::from(name)));
                    groups_parsed += 1;
                }
                _ => break,
            }
        }

        Ok(groups_parsed)
    }

    /// Parse a smoothing group name from a line of text input.
    fn parse_smoothing_group(
        &mut self,
        smoothing_groups: &mut Vec<SmoothingGroup>,
    ) -> Result<usize, ParseError> {
        self.expect_tag("s")?;
        if let Some(name) = self.next() {
            if name == "off" {
                smoothing_groups.push(SmoothingGroup(0));
            } else if let Ok(number) = name.parse::<usize>() {
                smoothing_groups.push(SmoothingGroup(number));
            } else {
                return self.error(
                    ErrorKind::SmoothingGroupNameMustBeOffOrInteger,
                    format!(
                        "A smoothing group name must either be `off`, which denotes that an \
                        object has no smoothing groups, or an integer. The parser got `{}` instead.",
                        name
                    ),
                );
            }
        } else {
            return self.error(
                ErrorKind::SmoothingGroupDeclarationHasNoName,
                "Got a smoothing group declaration without a smoothing group name.".to_owned(),
            );
        }

        Ok(1)
    }

    /// Parse a material name from a line of text input.
    fn parse_material_name(
        &mut self,
        material_names: &mut Vec<Option<&'a str>>,
    ) -> Result<usize, ParseError> {
        self.expect_tag("usemtl")?;
        if let Some(name) = self.next() {
            material_names.push(Some(name));
        } else {
            return self.error(
                ErrorKind::MaterialStatementHasNoName,
                "Got a `usemtl` material declaration without a material name.".to_owned(),
            );
        }

        Ok(1)
    }

    /// Construct a set of shape entries for each element in the element set.
    #[allow(clippy::type_complexity)]
    #[allow(clippy::needless_range_loop)]
    fn parse_shape_entries(
        &self,
        shape_entry_table: &mut Vec<ShapeEntry>,
        elements: &[Element],
        group_entry_table: &[((usize, usize), (usize, usize))],
        smoothing_group_entry_table: &[((usize, usize), usize)],
    ) {
        for &((min_element_index, max_element_index), (min_group_index, max_group_index)) in group_entry_table
        {
            let groups: Vec<usize> = (min_group_index..max_group_index).collect();
            for i in min_element_index..max_element_index {
                shape_entry_table.push(ShapeEntry {
                    element: i,
                    groups: groups.clone(),
                    smoothing_group: 0,
                });
            }
        }
        debug_assert!(shape_entry_table.len() == elements.len());

        for &((min_element_index, max_element_index), smoothing_group_index) in smoothing_group_entry_table {
            for i in min_element_index..max_element_index {
                shape_entry_table[i].smoothing_group = smoothing_group_index;
            }
        }
        debug_assert!(shape_entry_table.len() == elements.len());
    }

    /// Construct a set of geometries for reach material in an object.
    fn parse_geometries(
        &self,
        geometries: &mut Vec<Geometry>,
        material_name_entry_table: &[((usize, usize), usize)],
        material_names: &[Option<&'a str>],
    ) {
        for &((min_element_index, max_element_index), material_name_index) in material_name_entry_table {
            let shapes: Vec<ShapeEntryIndex> = (min_element_index..max_element_index).collect();
            let material_name = material_names[material_name_index].map(String::from);
            let geometry = Geometry {
                material_name: material_name,
                shapes: shapes,
            };
            geometries.push(geometry);
        }
    }

    /*
    fn calculate_index_ranges(
        &self,
        max_vertex_index:  &mut usize,
        max_texture_index: &mut usize,
        max_normal_index:  &mut usize
    ) {
        let mut cloned = self.clone();
        loop {
            match cloned.peek() {
                Some("v")  => {
                    *max_vertex_index += 1;
                    cloned.advance();
                }
                Some("vt") => {
                    *max_texture_index += 1;
                    cloned.advance();
                }
                Some("vn") => {
                    *max_normal_index += 1;
                    cloned.advance();
                }
                Some("o") | None => {
                    break;
                }
                _ => {
                    cloned.advance();
                }
            }
        }
    }
    */

    /// Parse one object from a Wavefront OBJ file.
    fn parse_object(
        &mut self,
        min_vertex_index: &mut usize,
        max_vertex_index: &mut usize,
        min_texture_index: &mut usize,
        max_texture_index: &mut usize,
        min_normal_index: &mut usize,
        max_normal_index: &mut usize,
    ) -> Result<Object, ParseError> {
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

        let mut material_name_entry_table = vec![];
        let mut material_names = vec![];
        let mut min_element_material_name_index = 0;
        let mut max_element_material_name_index = 0;
        let mut material_name_index = 0;

        // self.calculate_index_ranges(max_vertex_index, max_texture_index, max_normal_index);

        loop {
            match self.peek() {
                Some("g") if groups.is_empty() => {
                    let amount_parsed = self.parse_groups(&mut groups)?;
                    max_group_index += amount_parsed;
                }
                Some("g") => {
                    // Save the shape entry ranges for the current group.
                    group_entry_table.push((
                        (min_element_group_index, max_element_group_index),
                        (min_group_index, max_group_index),
                    ));

                    let amount_parsed = self.parse_groups(&mut groups)?;
                    min_group_index = max_group_index;
                    max_group_index += amount_parsed;
                    min_element_group_index = max_element_group_index;
                }
                Some("s") if smoothing_groups.is_empty() => {
                    self.parse_smoothing_group(&mut smoothing_groups)?;
                    smoothing_group_index = 0;
                }
                Some("s") => {
                    // Save the shape entry ranges for the current smoothing group.
                    smoothing_group_entry_table.push((
                        (
                            min_element_smoothing_group_index,
                            max_element_smoothing_group_index,
                        ),
                        smoothing_group_index,
                    ));

                    self.parse_smoothing_group(&mut smoothing_groups)?;
                    smoothing_group_index += 1;
                    min_element_smoothing_group_index = max_element_smoothing_group_index;
                }
                Some("usemtl") => {
                    if min_element_material_name_index == max_element_material_name_index {
                        if material_names.is_empty() {
                            self.parse_material_name(&mut material_names)?;
                        } else {
                            self.parse_material_name(&mut material_names)?;
                            material_name_index += 1;
                        }
                    } else {
                        material_name_entry_table.push((
                            (min_element_material_name_index, max_element_material_name_index),
                            material_name_index,
                        ));

                        if material_names.is_empty() {
                            self.parse_material_name(&mut material_names)?;
                        } else {
                            self.parse_material_name(&mut material_names)?;
                            material_name_index += 1;
                        }
                    }

                    min_element_material_name_index = max_element_material_name_index;
                }
                Some("v") => {
                    let vertex = self.parse_vertex()?;
                    vertices.push(vertex);
                    *max_vertex_index += 1;
                }
                Some("vt") => {
                    let texture_vertex = self.parse_texture_vertex()?;
                    texture_vertices.push(texture_vertex);
                    *max_texture_index += 1;
                }
                Some("vn") => {
                    let normal_vertex = self.parse_normal_vertex()?;
                    normal_vertices.push(normal_vertex);
                    *max_normal_index += 1;
                }
                Some("p") | Some("l") | Some("f") => {
                    if groups.is_empty() {
                        groups.push(Default::default());
                        min_group_index = 0;
                        max_group_index = 1;
                    }

                    if smoothing_groups.is_empty() {
                        smoothing_groups.push(Default::default());
                        smoothing_group_index = 0;
                    }

                    if material_names.is_empty() {
                        material_names.push(None);
                        material_name_index = 0;
                    }

                    let elements_parsed = self.parse_elements(
                        &mut elements,
                        (*min_vertex_index, *max_vertex_index),
                        (*min_texture_index, *max_texture_index),
                        (*min_normal_index, *max_normal_index),
                    )?;
                    max_element_group_index += elements_parsed;
                    max_element_smoothing_group_index += elements_parsed;
                    max_element_material_name_index += elements_parsed;
                }
                Some("\n") => {
                    self.skip_one_or_more_newlines()?;
                }
                Some("o") | None => {
                    // At the end of file or object, collect any remaining shapes.
                    group_entry_table.push((
                        (min_element_group_index, max_element_group_index),
                        (min_group_index, max_group_index),
                    ));

                    smoothing_group_entry_table.push((
                        (
                            min_element_smoothing_group_index,
                            max_element_smoothing_group_index,
                        ),
                        smoothing_group_index,
                    ));

                    material_name_entry_table.push((
                        (min_element_material_name_index, max_element_material_name_index),
                        material_name_index,
                    ));

                    break;
                }
                Some(other_st) => {
                    return self.error(
                        ErrorKind::InvalidObjectStatement,
                        format!("Unsupported or invalid object statement `{}`.", other_st),
                    );
                }
            }
        }

        let mut shape_entries = vec![];
        self.parse_shape_entries(
            &mut shape_entries,
            &elements,
            &group_entry_table,
            &smoothing_group_entry_table,
        );

        let mut geometries = vec![];
        self.parse_geometries(&mut geometries, &material_name_entry_table, &material_names);

        *min_vertex_index += vertices.len();
        *min_texture_index += texture_vertices.len();
        *min_normal_index += normal_vertices.len();

        Ok(Object {
            name: object_name.into(),
            vertex_set: vertices,
            texture_vertex_set: texture_vertices,
            normal_vertex_set: normal_vertices,
            group_set: groups,
            smoothing_group_set: smoothing_groups,
            element_set: elements,
            shape_set: shape_entries,
            geometry_set: geometries,
        })
    }

    /// Parse a set of objects in a wavefront OBJ file.
    fn parse_objects(&mut self) -> Result<Vec<Object>, ParseError> {
        let mut result = Vec::new();

        let mut min_vertex_index = 0;
        let mut max_vertex_index = 0;
        let mut min_texture_index = 0;
        let mut max_texture_index = 0;
        let mut min_normal_index = 0;
        let mut max_normal_index = 0;

        self.skip_zero_or_more_newlines();
        while self.peek().is_some() {
            result.push(self.parse_object(
                &mut min_vertex_index,
                &mut max_vertex_index,
                &mut min_texture_index,
                &mut max_texture_index,
                &mut min_normal_index,
                &mut max_normal_index,
            )?);
            self.skip_zero_or_more_newlines();
        }

        Ok(result)
    }

    /// Parse a set of material library file names from a line of text input.
    fn parse_material_library_line(
        &mut self,
        material_libraries: &mut Vec<String>,
    ) -> Result<usize, ParseError> {
        self.expect_tag("mtllib")?;
        let mut number_of_libraries_found = 0;
        loop {
            match self.next() {
                Some(st) if st != "\n" => {
                    material_libraries.push(String::from(st));
                    number_of_libraries_found += 1;
                }
                _ => break,
            }
        }

        Ok(number_of_libraries_found)
    }

    /// Parse a set of material library names from a Wavefront OBJ file.
    fn parse_material_libraries(&mut self) -> Result<Vec<String>, ParseError> {
        let mut material_libraries = vec![];
        self.skip_zero_or_more_newlines();
        while let Some("mtllib") = self.peek() {
            self.parse_material_library_line(&mut material_libraries)?;
            self.skip_zero_or_more_newlines();
        }

        Ok(material_libraries)
    }

    /// Parse the object set in the wavefront obj file.
    ///
    /// ## Example
    ///
    /// ```
    /// # use wavefront_obj::obj;
    /// # use wavefront_obj::obj::{
    /// #      Vertex,
    /// #      NormalVertex,
    /// #      Group,
    /// #      SmoothingGroup,
    /// #      Element,
    /// #      ShapeEntry,
    /// #      Geometry,
    /// #      VTNIndex,
    /// #      Object,
    /// #      ObjectSet,
    /// #      Parser,
    /// # };
    /// #
    /// let obj_file = String::from(r"
    ///     mtllib material_library.mtl    \
    ///     o object1                      \
    ///     v 0.000000  2.000000  0.000000 \
    ///     v 0.000000  0.000000  0.000000 \
    ///     v 2.000000  0.000000  0.000000 \
    ///     v 2.000000  2.000000  0.000000 \
    ///     v 4.000000  0.000000 -1.255298 \
    ///     v 4.000000  2.000000 -1.255298 \
    ///     ## 6 vertices                  \
    ///                                    \
    ///     g all                          \
    ///     s 1                            \
    ///     usemtl material1               \
    ///     f 1 2 3 4                      \
    ///     f 4 3 5 6                      \
    ///     ## 2 elements                  \
    ///                                    \
    ///     o object2                      \
    ///     v 0.000000  2.000000  0.000000 \
    ///     v 0.000000  0.000000  0.000000 \
    ///     v 2.000000  0.000000  0.000000 \
    ///     v 2.000000  2.000000  0.000000 \
    ///     v 4.000000  0.000000 -1.255298 \
    ///     v 4.000000  2.000000 -1.255298 \
    ///     ## 6 vertices                  \
    ///                                    \
    ///     g all                          \
    ///     s 1                            \
    ///     usemtl material2               \
    ///     f 7 8 9 10                     \
    ///     f 10 9 11 12                   \
    ///     ## 2 elements                  \
    ///                                    \
    /// ");
    /// // let expected = ...;
    /// # let expected = ObjectSet {
    /// #     material_libraries: vec![
    /// #         String::from("material_library.mtl"),
    /// #     ],
    /// #     objects: vec![
    /// #         Object {
    /// #             name: String::from("object1"),
    /// #             vertex_set: vec![
    /// #                 Vertex { x: 0.000000, y: 2.000000, z:  0.000000, w: 1.0 },
    /// #                 Vertex { x: 0.000000, y: 0.000000, z:  0.000000, w: 1.0 },
    /// #                 Vertex { x: 2.000000, y: 0.000000, z:  0.000000, w: 1.0 },
    /// #                 Vertex { x: 2.000000, y: 2.000000, z:  0.000000, w: 1.0 },
    /// #                 Vertex { x: 4.000000, y: 0.000000, z: -1.255298, w: 1.0 },
    /// #                 Vertex { x: 4.000000, y: 2.000000, z: -1.255298, w: 1.0 },
    /// #             ],
    /// #             texture_vertex_set: vec![],
    /// #             normal_vertex_set: vec![],
    /// #             group_set: vec![
    /// #                 Group(String::from("all")),
    /// #             ],
    /// #             smoothing_group_set: vec![
    /// #                 SmoothingGroup(1),
    /// #             ],
    /// #             element_set: vec![
    /// #                 Element::Face(VTNIndex::V(0), VTNIndex::V(1), VTNIndex::V(2)),
    /// #                 Element::Face(VTNIndex::V(0), VTNIndex::V(2), VTNIndex::V(3)),
    /// #                 Element::Face(VTNIndex::V(3), VTNIndex::V(2), VTNIndex::V(4)),
    /// #                 Element::Face(VTNIndex::V(3), VTNIndex::V(4), VTNIndex::V(5)),
    /// #             ],
    /// #             shape_set: vec![
    /// #                 ShapeEntry { element: 0,  groups: vec![0], smoothing_group: 0 },
    /// #                 ShapeEntry { element: 1,  groups: vec![0], smoothing_group: 0 },
    /// #                 ShapeEntry { element: 2,  groups: vec![0], smoothing_group: 0 },
    /// #                 ShapeEntry { element: 3,  groups: vec![0], smoothing_group: 0 },
    /// #             ],
    /// #             geometry_set: vec![
    /// #                 Geometry { material_name: Some(String::from("material1")), shapes: vec![0, 1, 2, 3] },
    /// #             ]
    /// #         },
    /// #         Object {
    /// #             name: String::from("object2"),
    /// #             vertex_set: vec![
    /// #                 Vertex { x: 0.000000, y: 2.000000, z:  0.000000, w: 1.0 },
    /// #                 Vertex { x: 0.000000, y: 0.000000, z:  0.000000, w: 1.0 },
    /// #                 Vertex { x: 2.000000, y: 0.000000, z:  0.000000, w: 1.0 },
    /// #                 Vertex { x: 2.000000, y: 2.000000, z:  0.000000, w: 1.0 },
    /// #                 Vertex { x: 4.000000, y: 0.000000, z: -1.255298, w: 1.0 },
    /// #                 Vertex { x: 4.000000, y: 2.000000, z: -1.255298, w: 1.0 },
    /// #             ],
    /// #             texture_vertex_set: vec![],
    /// #             normal_vertex_set: vec![],
    /// #             group_set: vec![
    /// #                 Group(String::from("all")),
    /// #             ],
    /// #             smoothing_group_set: vec![
    /// #                 SmoothingGroup(1),
    /// #             ],
    /// #             element_set: vec![
    /// #                 Element::Face(VTNIndex::V(0), VTNIndex::V(1), VTNIndex::V(2)),
    /// #                 Element::Face(VTNIndex::V(0), VTNIndex::V(2), VTNIndex::V(3)),
    /// #                 Element::Face(VTNIndex::V(3), VTNIndex::V(2), VTNIndex::V(4)),
    /// #                 Element::Face(VTNIndex::V(3), VTNIndex::V(4), VTNIndex::V(5)),
    /// #             ],
    /// #             shape_set: vec![
    /// #                 ShapeEntry { element: 0,  groups: vec![0], smoothing_group: 0 },
    /// #                 ShapeEntry { element: 1,  groups: vec![0], smoothing_group: 0 },
    /// #                 ShapeEntry { element: 2,  groups: vec![0], smoothing_group: 0 },
    /// #                 ShapeEntry { element: 3,  groups: vec![0], smoothing_group: 0 },
    /// #             ],
    /// #             geometry_set: vec![
    /// #                 Geometry { material_name: Some(String::from("material2")), shapes: vec![0, 1, 2, 3] },
    /// #             ]
    /// #         }
    /// #     ]
    /// # };
    /// let mut parser = Parser::new(&obj_file);
    /// let result = parser.parse_objset();
    /// assert!(result.is_ok());
    ///
    /// let result = result.unwrap();
    /// assert_eq!(result, expected)
    /// ```
    pub fn parse_objset(&mut self) -> Result<ObjectSet, ParseError> {
        let material_libraries = self.parse_material_libraries()?;
        let objects = self.parse_objects()?;

        Ok(ObjectSet {
            material_libraries: material_libraries,
            objects: objects,
        })
    }
}


#[cfg(test)]
mod primitive_tests {
    use super::Parser;


    #[test]
    fn test_parse_f64() {
        let mut parser = Parser::new("-1.929448");
        assert_eq!(parser.parse_f64(), Ok(-1.929448));
    }

    #[test]
    fn test_parse_isize() {
        let mut parser = Parser::new("    763   ");
        assert_eq!(parser.parse_isize(), Ok(763));
    }
}

#[cfg(test)]
mod vertex_tests {
    use super::{
        Parser,
        Vertex,
    };


    #[test]
    fn test_parse_vertex1() {
        let mut parser = Parser::new("v -1.929448 13.329624 -5.221914\n");
        let vertex = Vertex {
            x: -1.929448,
            y: 13.329624,
            z: -5.221914,
            w: 1.0,
        };
        assert_eq!(parser.parse_vertex(), Ok(vertex));
    }

    #[test]
    fn test_parse_vertex2() {
        let mut parser = Parser::new("v -1.929448 13.329624 -5.221914 1.329624\n");
        let vertex = Vertex {
            x: -1.929448,
            y: 13.329624,
            z: -5.221914,
            w: 1.329624,
        };
        assert_eq!(parser.parse_vertex(), Ok(vertex));
    }

    #[test]
    fn test_parse_vertex3() {
        let mut parser = Parser::new("v -1.929448 13.329624 \n");
        assert!(parser.parse_vertex().is_err());
    }

    #[test]
    fn test_parse_vertex4() {
        let mut parser = Parser::new("v -1.929448 13.329624 -5.221914 1.329624\n v");
        assert!(parser.parse_vertex().is_ok());
    }

    #[test]
    fn test_parse_vertex5() {
        let mut parser = Parser::new(
            "v -6.207583 1.699077 8.466142
              v -14.299248 1.700244 8.468981 1.329624",
        );
        assert_eq!(
            parser.parse_vertex(),
            Ok(Vertex {
                x: -6.207583,
                y: 1.699077,
                z: 8.466142,
                w: 1.0,
            })
        );
        assert_eq!(parser.next(), Some("\n"));
        assert_eq!(
            parser.parse_vertex(),
            Ok(Vertex {
                x: -14.299248,
                y: 1.700244,
                z: 8.468981,
                w: 1.329624,
            })
        );
    }

    #[test]
    fn test_parse_vertex6() {
        let mut parser = Parser::new("v -6.207583 1.699077 8.466142 v -14.299248 1.700244 8.468981 1.329624");
        assert_eq!(
            parser.parse_vertex(),
            Ok(Vertex {
                x: -6.207583,
                y: 1.699077,
                z: 8.466142,
                w: 1.0,
            })
        );
        assert_eq!(parser.peek(), Some("v"));
        assert_eq!(
            parser.parse_vertex(),
            Ok(Vertex {
                x: -14.299248,
                y: 1.700244,
                z: 8.468981,
                w: 1.329624,
            })
        );
    }
}

#[cfg(test)]
mod texture_vertex_tests {
    use super::{
        Parser,
        TextureVertex,
    };


    #[test]
    fn test_parse_texture_vertex1() {
        let mut parser = Parser::new("vt -1.929448");
        let vt = TextureVertex {
            u: -1.929448,
            v: 0.0,
            w: 0.0,
        };
        assert_eq!(parser.parse_texture_vertex(), Ok(vt));
    }

    #[test]
    fn test_parse_texture_vertex2() {
        let mut parser = Parser::new("vt -1.929448 13.329624 -5.221914");
        let vt = TextureVertex {
            u: -1.929448,
            v: 13.329624,
            w: -5.221914,
        };
        assert_eq!(parser.parse_texture_vertex(), Ok(vt));
    }

    #[test]
    fn test_parse_texture_vertex3() {
        let mut parser = Parser::new(
            "vt -1.929448 13.329624 -5.221914
             vt -27.6068  31.1438    27.2099",
        );
        assert_eq!(
            parser.parse_texture_vertex(),
            Ok(TextureVertex {
                u: -1.929448,
                v: 13.329624,
                w: -5.221914,
            })
        );
        assert_eq!(parser.next(), Some("\n"));
        assert_eq!(
            parser.parse_texture_vertex(),
            Ok(TextureVertex {
                u: -27.6068,
                v: 31.1438,
                w: 27.2099,
            })
        );
    }

    #[test]
    fn test_parse_texture_vertex4() {
        let mut parser = Parser::new("vt -1.929448 13.329624 -5.221914 vt -27.6068  31.1438    27.2099");
        assert_eq!(
            parser.parse_texture_vertex(),
            Ok(TextureVertex {
                u: -1.929448,
                v: 13.329624,
                w: -5.221914,
            })
        );
        assert_eq!(parser.peek(), Some("vt"));
        assert_eq!(
            parser.parse_texture_vertex(),
            Ok(TextureVertex {
                u: -27.6068,
                v: 31.1438,
                w: 27.2099,
            })
        );
    }
}

#[cfg(test)]
mod normal_vertex_tests {
    use super::{
        NormalVertex,
        Parser,
    };


    #[test]
    fn test_parse_normal_vertex1() {
        let mut parser = Parser::new("vn  -0.966742  -0.255752  9.97231e-09");
        let vn = NormalVertex {
            x: -0.966742,
            y: -0.255752,
            z: 9.97231e-09,
        };
        assert_eq!(parser.parse_normal_vertex(), Ok(vn));
    }

    #[test]
    fn test_parse_normal_vertex2() {
        let mut parser = Parser::new(
            "vn -1.929448 13.329624 -5.221914
             vn -27.6068  31.1438    27.2099",
        );
        assert_eq!(
            parser.parse_normal_vertex(),
            Ok(NormalVertex {
                x: -1.929448,
                y: 13.329624,
                z: -5.221914,
            })
        );
        assert_eq!(parser.next(), Some("\n"));
        assert_eq!(
            parser.parse_normal_vertex(),
            Ok(NormalVertex {
                x: -27.6068,
                y: 31.1438,
                z: 27.2099,
            })
        );
    }

    #[test]
    fn test_parse_normal_vertex3() {
        let mut parser = Parser::new("vn -1.929448 13.329624 -5.221914 vn -27.6068  31.1438    27.2099");
        assert_eq!(
            parser.parse_normal_vertex(),
            Ok(NormalVertex {
                x: -1.929448,
                y: 13.329624,
                z: -5.221914,
            })
        );
        assert_eq!(parser.peek(), Some("vn"));
        assert_eq!(
            parser.parse_normal_vertex(),
            Ok(NormalVertex {
                x: -27.6068,
                y: 31.1438,
                z: 27.2099,
            })
        );
    }
}

#[cfg(test)]
mod object_tests {
    use super::Parser;


    #[test]
    fn test_parse_object_name1() {
        let mut parser = Parser::new("o object_name \n\n");
        assert_eq!(parser.parse_object_name(), Ok("object_name"));
    }

    #[test]
    fn test_parse_object_name2() {
        let mut parser = Parser::new("o object_name");
        assert!(parser.parse_object_name().is_err());
    }
}

#[cfg(test)]
mod vtn_index_tests {
    use super::{
        Parser,
        VTNIndex,
    };


    #[test]
    fn test_parse_vtn_index1() {
        let mut parser = Parser::new("1291");
        let expected = VTNIndex::V(1290);
        let result = parser.parse_vtn_index((0, 1300), (0, 1300), (0, 1300));
        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn test_parse_vtn_index2() {
        let mut parser = Parser::new("1291/1315");
        let expected = VTNIndex::VT(1290, 1314);
        let result = parser.parse_vtn_index((0, 1316), (0, 1316), (0, 1316));
        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn test_parse_vtn_index3() {
        let mut parser = Parser::new("1291/1315/1314");
        let expected = VTNIndex::VTN(1290, 1314, 1313);
        let result = parser.parse_vtn_index((0, 1316), (0, 1316), (0, 1316));
        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn test_parse_vtn_index4() {
        let mut parser = Parser::new("1291//1315");
        let expected = VTNIndex::VN(1290, 1314);
        let result = parser.parse_vtn_index((0, 1316), (0, 1316), (0, 1316));
        assert_eq!(result, Ok(expected));
    }
}

#[cfg(test)]
mod element_tests {
    use super::{
        Element,
        Parser,
        VTNIndex,
    };


    #[test]
    fn test_parse_point1() {
        let mut parser = Parser::new("p 1 2 3 4 \n");
        let mut result = vec![];
        let expected = vec![
            Element::Point(VTNIndex::V(0)),
            Element::Point(VTNIndex::V(1)),
            Element::Point(VTNIndex::V(2)),
            Element::Point(VTNIndex::V(3)),
        ];
        assert!(parser.parse_elements(&mut result, (0, 5), (0, 5), (0, 5)).is_ok());
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_point2() {
        let mut parser = Parser::new("p 1 1/2 3 4/5");
        let mut result = vec![];
        assert!(parser
            .parse_elements(&mut result, (0, 6), (0, 6), (0, 6))
            .is_err());
    }

    #[test]
    fn test_parse_line1() {
        let mut parser = Parser::new("l 297 38 118 108 \n");
        let mut result = vec![];
        let expected = vec![
            Element::Line(VTNIndex::V(296), VTNIndex::V(37)),
            Element::Line(VTNIndex::V(37), VTNIndex::V(117)),
            Element::Line(VTNIndex::V(117), VTNIndex::V(107)),
        ];
        assert!(parser
            .parse_elements(&mut result, (0, 300), (0, 300), (0, 300))
            .is_ok());
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_line2() {
        let mut parser = Parser::new("l 297/38 118/108 \n");
        let mut result = vec![];
        let expected = vec![Element::Line(VTNIndex::VT(296, 37), VTNIndex::VT(117, 107))];
        assert!(parser
            .parse_elements(&mut result, (0, 300), (0, 300), (0, 300))
            .is_ok());
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_line3() {
        let mut parser = Parser::new("l 297/38 118/108 324/398 \n");
        let mut result = vec![];
        let expected = vec![
            Element::Line(VTNIndex::VT(296, 37), VTNIndex::VT(117, 107)),
            Element::Line(VTNIndex::VT(117, 107), VTNIndex::VT(323, 397)),
        ];
        assert!(parser
            .parse_elements(&mut result, (0, 400), (0, 400), (0, 400))
            .is_ok());
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_line4() {
        let mut parser = Parser::new("l 297/38 118 324 \n");
        let mut result = vec![];
        assert!(parser
            .parse_elements(&mut result, (0, 340), (0, 340), (0, 340))
            .is_err());
    }

    #[test]
    fn test_parse_line5() {
        let mut parser = Parser::new("l 297 118/108 324/398 \n");
        let mut result = vec![];
        assert!(parser
            .parse_elements(&mut result, (0, 400), (0, 400), (0, 400))
            .is_err());
    }

    #[test]
    fn test_parse_face1() {
        let mut parser = Parser::new("f 297 118 108\n");
        let mut result = vec![];
        let expected = vec![Element::Face(
            VTNIndex::V(296),
            VTNIndex::V(117),
            VTNIndex::V(107),
        )];
        assert!(parser
            .parse_elements(&mut result, (0, 340), (0, 340), (0, 340))
            .is_ok());
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_face2() {
        let mut parser = Parser::new("f 297 118 108 324\n");
        let mut result = vec![];
        let expected = vec![
            Element::Face(VTNIndex::V(296), VTNIndex::V(117), VTNIndex::V(107)),
            Element::Face(VTNIndex::V(296), VTNIndex::V(107), VTNIndex::V(323)),
        ];
        assert!(parser
            .parse_elements(&mut result, (0, 340), (0, 340), (0, 340))
            .is_ok());
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_face3() {
        let mut parser = Parser::new("f 297 118 108 324 398 \n");
        let mut result = vec![];
        let expected = vec![
            Element::Face(VTNIndex::V(296), VTNIndex::V(117), VTNIndex::V(107)),
            Element::Face(VTNIndex::V(296), VTNIndex::V(107), VTNIndex::V(323)),
            Element::Face(VTNIndex::V(296), VTNIndex::V(323), VTNIndex::V(397)),
        ];
        assert!(parser
            .parse_elements(&mut result, (0, 400), (0, 400), (0, 400))
            .is_ok());
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_face4() {
        let mut parser = Parser::new("f 297 118 \n");
        let mut result = vec![];
        assert!(parser
            .parse_face(&mut result, (0, 400), (0, 400), (0, 400))
            .is_err());
    }

    #[test]
    fn test_parse_face5() {
        let min_index = 320;
        let max_index = 35000;
        let vertex_index_range = (min_index, max_index);
        let texture_index_range = (min_index, max_index);
        let normal_index_range = (min_index, max_index);
        let mut parser =
            Parser::new("f 34184//34184 34088//34088 34079//34079 34084//34084 34091//34091 34076//34076\n");
        let mut result = vec![];
        /*
        let expected = vec![
            Element::Face(VTNIndex::VN(34183, 34183), VTNIndex::VN(34087, 34087), VTNIndex::VN(34078, 34078)),
            Element::Face(VTNIndex::VN(34183, 34183), VTNIndex::VN(34078, 34078), VTNIndex::VN(34083, 34083)),
            Element::Face(VTNIndex::VN(34183, 34183), VTNIndex::VN(34083, 34083), VTNIndex::VN(34090, 34090)),
            Element::Face(VTNIndex::VN(34183, 34183), VTNIndex::VN(34090, 34090), VTNIndex::VN(34075, 34075)),
        ];
        */
        let expected = vec![
            Element::Face(
                VTNIndex::VN(33863, 33863),
                VTNIndex::VN(33767, 33767),
                VTNIndex::VN(33758, 33758),
            ),
            Element::Face(
                VTNIndex::VN(33863, 33863),
                VTNIndex::VN(33758, 33758),
                VTNIndex::VN(33763, 33763),
            ),
            Element::Face(
                VTNIndex::VN(33863, 33863),
                VTNIndex::VN(33763, 33763),
                VTNIndex::VN(33770, 33770),
            ),
            Element::Face(
                VTNIndex::VN(33863, 33863),
                VTNIndex::VN(33770, 33770),
                VTNIndex::VN(33755, 33755),
            ),
        ];
        parser
            .parse_elements(
                &mut result,
                vertex_index_range,
                texture_index_range,
                normal_index_range,
            )
            .unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_face6() {
        let mut parser = Parser::new("f 297/13/12 118/124/45 108/93/7\n");
        let mut result = vec![];
        let expected = vec![Element::Face(
            VTNIndex::VTN(296, 12, 11),
            VTNIndex::VTN(117, 123, 44),
            VTNIndex::VTN(107, 92, 6),
        )];
        assert!(parser
            .parse_elements(&mut result, (0, 340), (0, 340), (0, 340))
            .is_ok());
        assert_eq!(result, expected);
    }
}

#[cfg(test)]
mod group_tests {
    use super::{
        Group,
        Parser,
    };


    #[test]
    fn parse_group_name1() {
        let mut parser = Parser::new("g group");
        let mut result = vec![];
        let expected = vec![Group(String::from("group"))];
        let parsed = parser.parse_groups(&mut result);

        assert!(parsed.is_ok());
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_group_name2() {
        let mut parser = Parser::new("g group1 group2 group3");
        let mut result = vec![];
        let parsed = parser.parse_groups(&mut result);
        let expected = vec![
            Group(String::from("group1")),
            Group(String::from("group2")),
            Group(String::from("group3")),
        ];

        assert!(parsed.is_ok());
        assert_eq!(result, expected);
    }
}

#[cfg(test)]
mod smoothing_group_tests {
    use super::{
        Parser,
        SmoothingGroup,
    };


    #[test]
    fn test_smoothing_group_name1() {
        let mut parser = Parser::new("s off");
        let mut result = vec![];
        let parsed = parser.parse_smoothing_group(&mut result);
        let expected = vec![SmoothingGroup(0)];

        assert!(parsed.is_ok());
        assert_eq!(result, expected);
    }

    #[test]
    fn test_smoothing_group_name2() {
        let mut parser = Parser::new("s 0");
        let mut result = vec![];
        let parsed = parser.parse_smoothing_group(&mut result);
        let expected = vec![SmoothingGroup(0)];

        assert!(parsed.is_ok());
        assert_eq!(result, expected);
    }

    #[test]
    fn test_smoothing_group_name3() {
        let mut parser = Parser::new("s 3434");
        let mut result = vec![];
        let parsed = parser.parse_smoothing_group(&mut result);
        let expected = vec![SmoothingGroup(3434)];

        assert!(parsed.is_ok());
        assert_eq!(result, expected);
    }
}

#[cfg(test)]
mod mtllib_tests {
    use super::Parser;


    #[test]
    fn test_mtllib_empty() {
        let mut parser = Parser::new("mtllib       ");
        let expected: Vec<String> = vec![];
        let expected_count = Ok(0);
        let mut result = vec![];
        let result_count = parser.parse_material_library_line(&mut result);

        assert_eq!(result, expected);
        assert_eq!(result_count, expected_count);
    }

    #[test]
    fn test_mtllib1() {
        let mut parser = Parser::new("mtllib library1.mtl");
        let expected: Vec<String> = vec![String::from("library1.mtl")];
        let expected_count = Ok(1);
        let mut result = vec![];
        let result_count = parser.parse_material_library_line(&mut result);

        assert_eq!(result, expected);
        assert_eq!(result_count, expected_count);
    }

    #[test]
    fn test_mtllib2() {
        let mut parser = Parser::new("mtllib library1.mtl library2.mtl library3.mtl");
        let expected: Vec<String> = vec![
            String::from("library1.mtl"),
            String::from("library2.mtl"),
            String::from("library3.mtl"),
        ];
        let expected_count = Ok(3);
        let mut result = vec![];
        let result_count = parser.parse_material_library_line(&mut result);

        assert_eq!(result, expected);
        assert_eq!(result_count, expected_count);
    }
}


#[cfg(test)]
mod objectset_tests {
    use super::{
        Element,
        Geometry,
        Group,
        NormalVertex,
        Object,
        ObjectSet,
        ParseError,
        Parser,
        ShapeEntry,
        SmoothingGroup,
        VTNIndex,
        Vertex,
    };


    #[rustfmt::skip]
    fn test_case() -> (Result<ObjectSet, ParseError>, Result<ObjectSet, ParseError>){
        let obj_file =r"                \
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
        let vertex_set = vec![
            Vertex { x: 0.0,  y: 0.0, z: 0.0, w: 1.0 },
            Vertex { x: 0.0,  y: 0.0, z: 1.0, w: 1.0 },
            Vertex { x: 0.0,  y: 1.0, z: 0.0, w: 1.0 },
            Vertex { x: 0.0,  y: 1.0, z: 1.0, w: 1.0 },
            Vertex { x: 1.0,  y: 0.0, z: 0.0, w: 1.0 },
            Vertex { x: 1.0,  y: 0.0, z: 1.0, w: 1.0 },
            Vertex { x: 1.0,  y: 1.0, z: 0.0, w: 1.0 },
            Vertex { x: 1.0,  y: 1.0, z: 1.0, w: 1.0 },
        ];
        let texture_vertex_set = vec![];
        let element_set = vec![
            Element::Face(VTNIndex::VN(0, 1), VTNIndex::VN(6, 1), VTNIndex::VN(4, 1)),
            Element::Face(VTNIndex::VN(0, 1), VTNIndex::VN(2, 1), VTNIndex::VN(6, 1)),
            Element::Face(VTNIndex::VN(0, 5), VTNIndex::VN(3, 5), VTNIndex::VN(2, 5)),
            Element::Face(VTNIndex::VN(0, 5), VTNIndex::VN(1, 5), VTNIndex::VN(3, 5)),
            Element::Face(VTNIndex::VN(2, 2), VTNIndex::VN(7, 2), VTNIndex::VN(6, 2)),
            Element::Face(VTNIndex::VN(2, 2), VTNIndex::VN(3, 2), VTNIndex::VN(7, 2)),
            Element::Face(VTNIndex::VN(4, 4), VTNIndex::VN(6, 4), VTNIndex::VN(7, 4)),
            Element::Face(VTNIndex::VN(4, 4), VTNIndex::VN(7, 4), VTNIndex::VN(5, 4)),
            Element::Face(VTNIndex::VN(0, 3), VTNIndex::VN(4, 3), VTNIndex::VN(5, 3)),
            Element::Face(VTNIndex::VN(0, 3), VTNIndex::VN(5, 3), VTNIndex::VN(1, 3)),
            Element::Face(VTNIndex::VN(1, 0), VTNIndex::VN(5, 0), VTNIndex::VN(7, 0)),
            Element::Face(VTNIndex::VN(1, 0), VTNIndex::VN(7, 0), VTNIndex::VN(3, 0)),
        ];
        let name = String::from("object1");
        let normal_vertex_set = vec![
            NormalVertex { x:  0.0, y:  0.0, z:  1.0 },
            NormalVertex { x:  0.0, y:  0.0, z: -1.0 },
            NormalVertex { x:  0.0, y:  1.0, z:  0.0 },
            NormalVertex { x:  0.0, y: -1.0, z:  0.0 },
            NormalVertex { x:  1.0, y:  0.0, z:  0.0 },
            NormalVertex { x: -1.0, y:  0.0, z:  0.0 },
        ];
        let group_set = vec![Group(String::from("cube"))];
        let smoothing_group_set = vec![SmoothingGroup(0)];
        let shape_set = vec![
            ShapeEntry { element: 0,  groups: vec![0], smoothing_group: 0 },
            ShapeEntry { element: 1,  groups: vec![0], smoothing_group: 0 },
            ShapeEntry { element: 2,  groups: vec![0], smoothing_group: 0 },
            ShapeEntry { element: 3,  groups: vec![0], smoothing_group: 0 },
            ShapeEntry { element: 4,  groups: vec![0], smoothing_group: 0 },
            ShapeEntry { element: 5,  groups: vec![0], smoothing_group: 0 },
            ShapeEntry { element: 6,  groups: vec![0], smoothing_group: 0 },
            ShapeEntry { element: 7,  groups: vec![0], smoothing_group: 0 },
            ShapeEntry { element: 8,  groups: vec![0], smoothing_group: 0 },
            ShapeEntry { element: 9,  groups: vec![0], smoothing_group: 0 },
            ShapeEntry { element: 10, groups: vec![0], smoothing_group: 0 },
            ShapeEntry { element: 11, groups: vec![0], smoothing_group: 0 },
        ];
        let geometry_set = vec![
            Geometry { 
                material_name: None, 
                shapes: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            },
        ];
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
        let material_libraries = vec![];
        let objects = vec![object];
        let expected = ObjectSet {
            material_libraries: material_libraries,
            objects: objects,
        };
        let mut parser = Parser::new(obj_file);
        let result = parser.parse_objset();

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

        for (result, expected) in result_set.objects.iter().zip(expected_set.objects.iter()) {
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
