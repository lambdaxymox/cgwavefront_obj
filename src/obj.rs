use crate::lexer::{
    Tokenizer,
    Lexer,
};
use std::error;
use std::fmt;
use std::default::{
    Default,
};
use std::collections::{
    BTreeMap,
};
use std::slice;
use std::ops;



/// Parse a wavefront object file from a string.
pub fn parse<T: AsRef<str>>(input: T) -> Result<ObjectSet, ParseError> {
    Parser::new(input.as_ref()).parse()
}


#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vertex {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Vertex {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Vertex {
        Vertex { 
            x: x, 
            y: y, 
            z: z, 
            w: w 
        }
    }
}

impl fmt::Display for Vertex {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "v  {}  {}  {}  {}", self.x, self.y, self.z, self.w)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TextureVertex {
    pub u: f64,
    pub v: f64,
    pub w: f64,
}

impl TextureVertex {
    pub fn new(u: f64, v: f64, w: f64) -> TextureVertex {
        TextureVertex { 
            u: u, 
            v: v, 
            w: w 
        }
    }
}

impl fmt::Display for TextureVertex {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "vt  {}  {}  {}", self.u, self.v, self.w)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NormalVertex {
    pub i: f64,
    pub j: f64,
    pub k: f64,
}

impl NormalVertex {
    pub fn new(i: f64, j: f64, k: f64) -> NormalVertex {
        NormalVertex { 
            i: i, 
            j: j, 
            k: k 
        }
    }
}

impl fmt::Display for NormalVertex {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "vn  {}  {}  {}", self.i, self.j, self.k)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum VTNIndex { 
    V(VertexIndex),
    VT(VertexIndex, TextureVertexIndex), 
    VN(VertexIndex, NormalVertexIndex),
    VTN(VertexIndex, TextureVertexIndex, NormalVertexIndex),
}

impl VTNIndex {
    pub fn has_same_type_as(&self, other: &VTNIndex) -> bool {
        match (self, other) {
            (&VTNIndex::V(_),   &VTNIndex::V(_)) |
            (&VTNIndex::VT(_,_),  &VTNIndex::VT(_,_)) | 
            (&VTNIndex::VN(_,_),  &VTNIndex::VN(_,_)) | 
            (&VTNIndex::VTN(_,_,_), &VTNIndex::VTN(_,_,_)) => true,
            _ => false,
        }
    }
}

impl fmt::Display for VTNIndex {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            VTNIndex::V(v) => write!(f, "{}", v + 1),
            VTNIndex::VT(v, vt) => write!(f, "{}/{}", v + 1 ,vt + 1),
            VTNIndex::VN(v, vn) => write!(f, "{}//{}", v + 1, vn + 1),
            VTNIndex::VTN(v, vt, vn) => write!(f, "{}/{}/{}", v + 1, vt + 1, vn + 1),
        }
    }
}

type ElementIndex = usize;
type VertexIndex = usize;
type TextureVertexIndex = usize;
type NormalVertexIndex = usize;
type GroupIndex = usize;
type ShapeIndex = usize;
type SmoothingGroupIndex = usize;

#[derive(Clone, Debug, PartialEq)]
pub enum Element {
    Point(VTNIndex),
    Line(VTNIndex, VTNIndex),
    Face(VTNIndex, VTNIndex, VTNIndex),
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            Element::Point(vtn) => {
                write!(f, "p  {}", vtn)
            },
            Element::Line(vtn1, vtn2) => {
                write!(f, "l  {}  {}", vtn1, vtn2)
            },
            Element::Face(vtn1, vtn2, vtn3) => {
                write!(f, "f  {}  {}  {}", vtn1, vtn2, vtn3)
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Group(String);

impl Group {
    pub fn new(name: &str) -> Group { 
        Group(String::from(name)) 
    }
}

impl fmt::Display for Group {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.0)
    }
}

impl Default for Group {
    fn default() -> Group { 
        Group::new("default") 
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct SmoothingGroup(usize);

impl SmoothingGroup {
    #[inline]
    pub const fn new(name: usize) -> SmoothingGroup { 
        SmoothingGroup(name)
    }

    #[inline]
    pub fn as_usize(&self) -> usize { 
        self.0 
    }
}

impl Default for SmoothingGroup {
    fn default() -> SmoothingGroup { 
        SmoothingGroup::new(0) 
    }
}

impl fmt::Display for SmoothingGroup {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        if self.0 == 0 {
            write!(f, "off")
        } else {
            write!(f, "{}", self.0)
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ShapeEntry {
    pub element: ElementIndex,
    pub groups: Vec<GroupIndex>,
    pub smoothing_group: SmoothingGroupIndex,
}

impl ShapeEntry {
    pub fn new(
        element: ElementIndex, 
        groups: &[GroupIndex], 
        smoothing_group: SmoothingGroupIndex) -> ShapeEntry {

        ShapeEntry {
            element: element,
            groups: Vec::from(groups),
            smoothing_group: smoothing_group,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Shape {
    element: Element,
    groups: Vec<Group>,
    smoothing_groups: Vec<SmoothingGroup>,
}

#[derive(Clone, Debug)]
pub enum VTNTriple<'a> {
    V(&'a Vertex),
    VT(&'a Vertex, &'a TextureVertex), 
    VN(&'a Vertex, &'a NormalVertex),
    VTN(&'a Vertex, &'a TextureVertex, &'a NormalVertex),
}

#[derive(Clone, PartialEq)]
pub struct Object {
    pub name: String,
    pub vertex_set: Vec<Vertex>,
    pub texture_vertex_set: Vec<TextureVertex>,
    pub normal_vertex_set: Vec<NormalVertex>,
    pub group_set: Vec<Group>,
    pub smoothing_group_set: Vec<SmoothingGroup>,
    pub element_set: Vec<Element>,
    pub shape_set: Vec<ShapeEntry>,
}

impl Object {
    pub fn new(
        name: String,
        vertex_set: Vec<Vertex>, 
        texture_vertex_set: Vec<TextureVertex>, 
        normal_vertex_set: Vec<NormalVertex>,
        group_set: Vec<Group>, 
        smoothing_group_set: Vec<SmoothingGroup>, 
        element_set: Vec<Element>,
        shape_set: Vec<ShapeEntry>) -> Object {
        
        Object {
            name: name,
            vertex_set: vertex_set,
            texture_vertex_set: texture_vertex_set,
            normal_vertex_set: normal_vertex_set,
            group_set: group_set,
            smoothing_group_set: smoothing_group_set,
            element_set: element_set,
            shape_set: shape_set,
        }
    }

    pub fn name(&self) -> &str { 
        &self.name
    }

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

impl fmt::Debug for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let string = DisplayObjectCompositor::new().compose(self);
        write!(f, "{}", string)
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let string = DisplayObjectCompositor::new().compose(self);
        write!(f, "{}", string)
    }
}

impl Default for Object {
    fn default() -> Object {
        Object::new(
            String::from(""),   
            Default::default(), 
            Default::default(), 
            Default::default(), 
            Default::default(), 
            Default::default(), 
            Default::default(),
            Default::default()
        )
    }
}


#[derive(Clone, Debug, PartialEq)]
pub struct ObjectSet {
    objects: Vec<Object>,
}

impl ObjectSet {
    pub fn new(objects: Vec<Object>) -> ObjectSet {
        ObjectSet {
            objects: objects,
        }    
    }

    pub fn iter(&self) -> ObjectSetIter {
        ObjectSetIter {
            inner: self.objects.iter(),
        }
    }

    pub fn len(&self) -> usize { 
        self.objects.len()
    }
}

pub struct ObjectSetIter<'a> {
    inner: slice::Iter<'a, Object>,   
}

impl<'a> Iterator for ObjectSetIter<'a> {
    type Item = &'a Object;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl ops::Index<usize> for ObjectSet {
    type Output = Object;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.objects[index]
    }
}

impl fmt::Display for ObjectSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let string = DisplayObjectSetCompositor::new().compose(self);
        write!(f, "{}", string)
    }
}


struct DisplayObjectCompositor { }

impl DisplayObjectCompositor {
    fn new() -> Self { Self {} }

    fn compose_set<T: fmt::Display>(&self, set: &[T], name: &str) -> String {
        let mut string = String::from(format!("    {} set:\n", name));
        if set.is_empty() {
            string += &format!("        data: []\n");
        } else {
            let length = set.len();
            string += &format!("        data: [({}) ... ({})]\n", set[0], set[length-1]);
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
        string += &format!("}}\n");

        string       
    }
}


#[derive(Clone, Debug, PartialEq)]
enum GroupingStatement {
    G(Vec<Group>),
    S(SmoothingGroup),
}

/// The compositor instructions to pass to an object compositor. 
///
/// The instructions describe to the compositor where to place grouping 
/// statements in a Wavefront OBJ file derived from an object set.
#[derive(Clone, Debug, PartialEq)]
struct CompositorInstructions {
    instructions: BTreeMap<(usize, usize), Vec<GroupingStatement>>,
}

impl CompositorInstructions {
    fn new(instructions: BTreeMap<(usize, usize), Vec<GroupingStatement>>) -> Self {
        Self { instructions: instructions }
    }

    /// Find any missing groups and smoothing groups that contain no elements.
    ///
    /// We can find the missing groups in the object set because per the 
    /// specification, grouping statements are indexed in monotone increasing 
    /// order. So any gaps in the grouping indices in the index buffer indicates 
    /// which groups are missing, and we can fill there in when generating a 
    /// *.obj file from an object set.
    fn generate_missing_groups(object: &Object) -> BTreeMap<(usize, usize), Vec<GroupingStatement>> {
        let mut missing_groups = BTreeMap::new();
        
        // It is possible that there are missing groups that appear before the 
        // first interval of elements in the shape set. We must calculate
        // these first for otherwise we would miss them when filling in the table.
        let initial_group = object.shape_set[0].groups[0];
        let initial_smoothing_group = object.shape_set[0].smoothing_group;
        let mut current_statements = vec![];
        for group_index in 1..initial_group {
            current_statements.push(GroupingStatement::G(vec![
                object.group_set[group_index - 1].clone()
            ]));
        }

        for smoothing_group_index in 1..initial_smoothing_group {
            current_statements.push(GroupingStatement::S(
                object.smoothing_group_set[smoothing_group_index - 1]
            ));
        }

        // In order to fill in the missing groups and smoothing groups, we need
        // to know which groups and smoothing groups are occupied in the object. 
        // After that, we can determine which groups are missing and fill them in.
        let mut current_entry = &object.shape_set[0];
        let mut min_element_index = 0;
        let mut max_element_index = 0;
        for shape_entry in object.shape_set.iter() {
            if shape_entry.groups != current_entry.groups || 
                shape_entry.smoothing_group != current_entry.smoothing_group {
                // We have crossed a group or smoothing group boundary. Here we 
                // have found the end of the interval of elements with the same 
                // groups and smoothing groups.
                missing_groups.insert((min_element_index, max_element_index), current_statements);

                // Which groups and smoothing groups are missing? There is 
                // ambiguity in ordering any possible missing groups and 
                // smoothing groups. We choose to disambiguate this by finding
                // the missing groups and dropping them in first, followed by 
                // dropping in the missing smoothing groups before proceeding 
                // with the groups and smoothing groups that have elements.
                current_statements = vec![];

                // Are the groups different?
                if shape_entry.groups != current_entry.groups {
                    // Derive the missing groups from the gap between shape_entry 
                    // and current_entry.
                    let gap_start = 1 + current_entry.groups[current_entry.groups.len() - 1];
                    let gap_end = shape_entry.groups[0];
                    for group_index in gap_start..gap_end {
                        current_statements.push(GroupingStatement::G(vec![
                            object.group_set[group_index - 1].clone()
                        ]));
                    }
                }

                // Are the smoothing groups different?
                if shape_entry.smoothing_group != current_entry.smoothing_group {
                    // Derive the missing smoothing groups from the gap between 
                    // shape_entry and current_entry.
                    let gap_start = 1 + current_entry.smoothing_group;
                    let gap_end = shape_entry.smoothing_group;
                    for smoothing_group_index in gap_start..gap_end {
                        current_statements.push(GroupingStatement::S(
                            object.smoothing_group_set[smoothing_group_index - 1]
                        ));
                    }
                }

                // Place the missing groups into the table.
                //missing_groups.insert((min_element_index, max_element_index), statements);

                // Continue with the next interval.
                current_entry = shape_entry;
                min_element_index = max_element_index;
            } 
            
            // We have processed this group element.
            max_element_index += 1;
        }

        // Process the last existing interval.
        missing_groups.insert((min_element_index, max_element_index), current_statements);

        // The last interval of empty groups and smoothing groups
        // lies off the end of the element list.
        min_element_index = max_element_index;

        // It is possible that there are missing groups that appear after the 
        // final groups and smoothing groups in the shape set. We must calculate
        // these last one for otherwise they would get lost when passing the 
        // text to the parser.
        let final_shape_entry = &object.shape_set[min_element_index - 1];
        let final_group = final_shape_entry.groups[final_shape_entry.groups.len() - 1];
        let final_smoothing_group = final_shape_entry.smoothing_group;
        let mut final_statements = vec![];
        for group_index in (final_group + 1)..((object.group_set.len() + 1)) {
            final_statements.push(GroupingStatement::G(vec![
                object.group_set[group_index - 1].clone()
            ]));
        }

        for smoothing_group_index 
            in (final_smoothing_group + 1)..((object.smoothing_group_set.len() + 1)) {
            
            final_statements.push(GroupingStatement::S(
                object.smoothing_group_set[smoothing_group_index - 1]
            ));
        }

        missing_groups.insert((min_element_index, min_element_index), final_statements);

        missing_groups
    }

    /// Place the grouping statements for the groups that contain elements.
    fn generate_found_groups(object: &Object) -> BTreeMap<(usize, usize), Vec<GroupingStatement>> {
        let mut found_groups = BTreeMap::new();
        let mut min_element_index = 0;
        let mut max_element_index = 0;
        let mut current_statements = vec![];
        let mut current_entry = &object.shape_set[0];

        let new_groups = current_entry.groups.iter().fold(vec![], |mut acc, group_index| {
            acc.push(object.group_set[group_index - 1].clone());
            acc
        });
        current_statements.push(GroupingStatement::G(new_groups));
        current_statements.push(GroupingStatement::S(
            object.smoothing_group_set[current_entry.smoothing_group - 1])
        );

        for shape_entry in object.shape_set.iter() {
            if shape_entry.groups != current_entry.groups || 
                shape_entry.smoothing_group != current_entry.smoothing_group {

                found_groups.insert((min_element_index, max_element_index), current_statements);

                current_statements = vec![];

                // Are the groups different?
                if shape_entry.groups != current_entry.groups {
                    let new_groups = shape_entry.groups.iter().fold(vec![], |mut acc, group_index| {
                        acc.push(object.group_set[group_index - 1].clone());
                        acc
                    });
                    current_statements.push(GroupingStatement::G(new_groups));
                }
                
                // Are the smoothing groups different?
                if shape_entry.smoothing_group != current_entry.smoothing_group {
                    current_statements.push(GroupingStatement::S(
                        object.smoothing_group_set[shape_entry.smoothing_group - 1])
                    );
                }

                current_entry = shape_entry;
                min_element_index = max_element_index;
            }

            // We have processed this group element.
            max_element_index += 1;
        }
        
        found_groups.insert((min_element_index, max_element_index), current_statements);

        // The last interval of empty groups and smoothing groups
        // lies off the end of the element list. In the case of the found
        // groups, there are none off the end of the element list.
        min_element_index = max_element_index;

        let final_statements = vec![];
        found_groups.insert((min_element_index, min_element_index), final_statements);

        found_groups
    }

    /// Generate the grouping statements for an object set.
    fn generate(object: &Object) -> CompositorInstructions {
        let missing_groups = Self::generate_missing_groups(object);
        let found_groups = Self::generate_found_groups(object);
        
        debug_assert!(missing_groups.len() == found_groups.len());
        debug_assert!(
            missing_groups.keys().zip(found_groups.keys()).all(
                |(mg_key, fg_key)| { mg_key == fg_key }
            )
        );

        // The missing groups appear in the gaps between groups of elements. 
        // In order to fill in these groups correctly, we place the missing
        // group statements for each interval, followed by the grouping 
        // statements for the corresponding interval of elements.
        let mut instructions: BTreeMap<(usize, usize), Vec<GroupingStatement>> = BTreeMap::new();
        for interval in missing_groups.keys() {
            let mut statements = missing_groups[interval].clone();
            statements.append(&mut found_groups[interval].clone());
            instructions.insert(*interval, statements);
        }

        debug_assert!(instructions.len() == missing_groups.len());
        debug_assert!(missing_groups.len() == found_groups.len());
        debug_assert!(
            instructions.keys().zip(missing_groups.keys().zip(found_groups.keys())).all(
                |(instr_key, (mg_key, fg_key))| { instr_key == mg_key && mg_key == fg_key }
            )
        );
        Self::new(instructions)
    }
}

/// A `TextObjectCompositor` generates a Wavefront OBJ text block from an 
/// object. One can use it to automatically generate OBJ files.
struct TextObjectCompositor {}

impl TextObjectCompositor {
    fn new() -> TextObjectCompositor { 
        TextObjectCompositor {} 
    }

    fn compose_object_name(&self, object: &Object) -> String {
        match object.name.as_ref() {
            "" => String::from(""),
            _  => format!("o  {}\n", object.name),
        }     
    }

    fn compose_groups(&self, groups: &[Group]) -> String {
        let string = groups.iter().fold(
            String::from("g  "), |acc, group| {
                acc + &format!("{}  ", group)
            }
        );
        format!("{}\n", string)
    }

    fn compose_smoothing_group(&self, smoothing_group: SmoothingGroup) -> String {
        format!("s  {}\n", smoothing_group)
    }

    fn compose_vertex_set(&self, object: &Object) -> String {
        object.vertex_set.iter().fold(String::new(), |acc, v| {
            match v.w == 1.0 {
                true  => acc + &format!("v  {}  {}  {}\n", v.x, v.y, v.z),
                false => acc + &format!("v  {}  {}  {}  {}\n", v.x, v.y, v.z, v.w),
            }
        })
    }

    fn compose_texture_vertex_set(&self, object: &Object) -> String {
        object.texture_vertex_set.iter().fold(String::new(), |acc, vt| {
            acc + &format!("vt  {}  {}  {} \n", vt.u, vt.v, vt.w)
        })
    }

    fn compose_normal_vertex_set(&self, object: &Object) -> String {
        object.normal_vertex_set.iter().fold(String::new(), |acc, vn| {
            acc + &format!("vn  {}  {}  {} \n", vn.i, vn.j, vn.k)
        })        
    }

    fn compose_elements(&self, object: &Object, interval: (usize, usize)) -> String {
        let string = (interval.0..interval.1).fold(String::new(), |acc, i| {
            acc + &format!("{}\n", object.element_set[i])
        });
        format!("{}", string)
    }

    fn get_group_instructions(&self, object: &Object) -> BTreeMap<(usize, usize), Vec<GroupingStatement>> {
        let instructions = CompositorInstructions::generate(object);
        instructions.instructions
    }

    fn compose_instructions(&self, instructions: &[GroupingStatement]) -> String {
        instructions.iter().fold(String::new(), |acc, statement| {
            match *statement {
                GroupingStatement::G(ref groups)      => acc + &self.compose_groups(&groups),
                GroupingStatement::S(smoothing_group) => acc + &self.compose_smoothing_group(smoothing_group)
            }
        })
    }

    fn compose_comment<T>(&self, set: &[T], unit_singular: &str, unit_plural: &str) -> String {
        if set.len() == 1 { 
            format!("# {} {}\n", set.len(), unit_singular)
        } else {
            format!("# {} {}\n", set.len(), unit_plural)
        }
    }

    fn compose(&self, object: &Object) -> String {
        let mut string = String::new();

        string += &self.compose_object_name(object);
        string += &self.compose_vertex_set(object);
        string += &self.compose_comment(&object.vertex_set, "vertex", "vertices");
        string += &format!("\n");

        string += &self.compose_texture_vertex_set(object);
        string += &self.compose_comment(
            &object.texture_vertex_set, "texture vertex", "texture vertices"
        );
        string += &format!("\n");

        string += &self.compose_normal_vertex_set(object);
        string += &self.compose_comment(&object.normal_vertex_set, "normal vertex", "normal vertices");
        string += &format!("\n");

        let group_instructions = self.get_group_instructions(object);
        for (interval, instructions) in group_instructions.iter() {
            string += &self.compose_instructions(&instructions);
            string += &self.compose_elements(object, *interval);
            if interval.1 - interval.0 == 1 {
                string += &format!("# {} element\n\n", (interval.1 - interval.0));
            } else {
                string += &format!("# {} elements\n\n", (interval.1 - interval.0));
            }
        }

        string
    }
}

/// The `DisplayObjectCompositor` type is the default compositor
/// for presenting object set information to the end user.
pub struct DisplayObjectSetCompositor { }

impl DisplayObjectSetCompositor {
    pub fn new() -> Self { 
        Self {} 
    }

    pub fn compose(&self, object_set: &ObjectSet) -> String {
        let compositor = DisplayObjectCompositor::new();
        let mut string = String::from("ObjectSet {\n");
        
        for object in object_set.iter() {
            string += &compositor.compose(&object);
            string += &"\n";
        }

        string += &"}\n";
        string
    }
}

/// A `TextObjectSetCompositor` generates a Wavefront OBJ file from an ObjectSet.
/// One can use it to automatically generate OBJ files from object sets.
pub struct TextObjectSetCompositor { }

impl TextObjectSetCompositor {
    pub fn new() -> Self { 
        Self {}
    }

    pub fn compose(&self, object_set: &ObjectSet) -> String {
        let compositor = TextObjectCompositor::new();
        
        let mut string = String::new();
        for (i, object_i) in object_set.iter().enumerate().map(|(i, obj)| (i + 1, obj)) {
            string += &format!("# ### BEGIN Object {}\n", i);
            string += &compositor.compose(&object_i);
            string += &format!("# ### END Object {}\n", i);
            string += &"\n";
        }

        string
    }
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
    ExpectedVTNIndexButGot(String),
    EveryFaceElementMustHaveAtLeastThreeVertices,
    EveryVTNIndexMustHaveTheSameFormForAGivenElement,
    InvalidElementDeclaration(String),
    ElementMustBeAPointLineOrFace,
    SmoothingGroupNameMustBeOffOrInteger(String),
    SmoothingGroupDeclarationHasNoName,
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ErrorKind::*;
        match &self {
            &EndOfFile => {
                write!(f, 
                    "Prematurely reached the end of the file in the process of 
                     getting the next token."
                )
            }
            &ExpectedStatementButGot(expected, got) => {
                write!(f, "Parse Error: Expected `{}` but got `{}` instead.", expected, got)
            }
            &ExpectedFloatButGot(got) => {
                write!(f, "Expected a floating point number but got `{}` instead.", got)
            }
            &ExpectedIntegerButGot(got) => {
                write!(f, "Expected an integer but got `{}` instead.", got)
            }
            &ExpectedVertexIndexButGot(got) => {
                write!(f, "Expected a vertex index but got `{}` instead.", got)
            }
            &ExpectedTextureIndexButGot(got) => {
                write!(f, "Expected a texture vertex index but got `{}` instead.", got)
            }
            &ExpectedNormalIndexButGot(got) => {
                write!(f, "Expected a normal vertex index but got `{}` instead.", got)
            }
            &ExpectedVertexNormalIndexButGot(got) => {
                write!(f, "Expected a `vertex//normal` index but got `{}` instead.", got)
            }
            &ExpectedVertexTextureIndexButGot(got) => {
                write!(f, "Expected a `vertex/texture` index but got `{}` instead.", got)
            }
            &ExpectedVTNIndexButGot(got) => {
                write!(f, "Expected a `vertex/texture/normal` index but got `{}` instead.", got)
            }
            &EveryFaceElementMustHaveAtLeastThreeVertices => {
                write!(f, 
                    "A face primitive must have at least three vertices.
                     At minimum, a triangle requires three indices."
                )
            }
            &EveryVTNIndexMustHaveTheSameFormForAGivenElement => {
                write!(f, 
                    "Every index describing the vertex data for a face must have the same form.
                     For example, if the element is a face, and the geometry data wants to provide
                     vertex and texture data to an application, each VTN index must be of the form
                     `vertex/texture`."
                )
            }
            &InvalidElementDeclaration(got) => {
                write!(f, "The parser encountered an unsupported or invalid element declaration `{}`.", got)
            }
            &ElementMustBeAPointLineOrFace => {
                write!(f, "An element must be declared as either a point (`p`), line (`l`), or face `f`.")
            }
            &SmoothingGroupNameMustBeOffOrInteger(got) => {
                write!(f, 
                    "A smoothing group name must either be `off`, which denotes that an
                     object has no smoothing groups, or an integer. The parser got `{}` instead.",
                    got
                )
            }
            &SmoothingGroupDeclarationHasNoName => {
                write!(f, "Got a smoothing group declaration without a smoothing group name.")
            }
        }
    }
}

/// An error that is returned from parsing an invalid *.obj file, or
/// another kind of error.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParseError {
    line_number: usize,
    kind: ErrorKind,
}

impl ParseError {
    /// Generate a new parse error.
    fn new(line_number: usize, kind: ErrorKind) -> ParseError {
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
fn error<T>(line_number: usize, kind: ErrorKind) -> Result<T, ParseError> {
    Err(ParseError::new(line_number, kind))
}


/// A Wavefront OBJ file parser.
pub struct Parser<'a> {
    line_number: usize,
    lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Parser<'a> {
        Parser {
            line_number: 1,
            lexer: Lexer::new(Tokenizer::new(input)),
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

    fn next_string(&mut self) -> Result<&'a str, ParseError> {
        match self.next() {
            Some(st) => Ok(st),
            None => error(self.line_number, ErrorKind::EndOfFile)
        }
    }

    fn expect_tag(&mut self, tag: &str) -> Result<(), ParseError> {
        match self.next() {
            None => error(self.line_number, ErrorKind::EndOfFile),
            Some(st) if st != tag => error(
                self.line_number, 
                ErrorKind::ExpectedStatementButGot(tag.into(), st.into())
            ),
            _ => Ok(())
        }
    }

    fn parse_f64(&mut self) -> Result<f64, ParseError> {
        let st = self.next_string()?;
        match st.parse::<f64>() {
            Ok(val) => Ok(val),
            Err(_) => error(
                self.line_number, 
                ErrorKind::ExpectedFloatButGot(st.into())
            ),
        }
    }

    fn parse_usize(&mut self) -> Result<usize, ParseError> {
        let st = self.next_string()?;
        match st.parse::<usize>() {
            Ok(val) => Ok(val),
            Err(_) => error(
                self.line_number, 
                ErrorKind::ExpectedIntegerButGot(st.into())
            ),
        }
    }

    fn try_once<P, T>(&mut self, parser: P) -> Option<T> where P: FnOnce(&str) -> Option<T> {
        match self.peek() {
            Some(st) => parser(&st).map(|got| { self.advance(); got }),
            None => None,
        }
    }

    fn parse_vertex(&mut self) -> Result<Vertex, ParseError> {
        self.expect_tag("v")?;
 
        let x = self.parse_f64()?;
        let y = self.parse_f64()?;
        let z = self.parse_f64()?;
        let mw = self.try_once(|st| st.parse::<f64>().ok());
        let w = mw.unwrap_or(1_f64);

        Ok(Vertex { x: x, y: y, z: z, w: w })
    }

    fn parse_texture_vertex(&mut self) -> Result<TextureVertex, ParseError> {
        self.expect_tag("vt")?;

        let u = self.parse_f64()?;
        let mv = self.try_once(|st| st.parse::<f64>().ok());
        let v = mv.unwrap_or(0_f64);
        let mw = self.try_once(|st| st.parse::<f64>().ok());
        let w = mw.unwrap_or(0_f64);

        Ok(TextureVertex { u: u, v: v, w: w })
    }

    fn parse_normal_vertex(&mut self) -> Result<NormalVertex, ParseError> {
        self.expect_tag("vn")?;

        let i = self.parse_f64()?;
        let j = self.parse_f64()?;
        let k = self.parse_f64()?;

        Ok(NormalVertex { i: i, j: j, k: k })
    }

    fn skip_zero_or_more_newlines(&mut self) {
        while let Some("\n") = self.peek() {
            self.advance();
        }
    }

    fn skip_one_or_more_newlines(&mut self) -> Result<(), ParseError> {
        self.expect_tag("\n")?;
        self.skip_zero_or_more_newlines();
        Ok(())
    }

    fn parse_object_name(&mut self) -> Result<&'a str, ParseError> {
        match self.peek() {
            Some("o") => {
                self.expect_tag("o")?;
                let object_name = self.next_string();
                self.skip_one_or_more_newlines()?;
                
                object_name
            }
            _ => Ok("")
        }
    }

    fn parse_vtn_index(&mut self) -> Result<VTNIndex, ParseError> {
        let process_split = |split: &str| -> Result<Option<usize>, ParseError> {
            if split.len() > 0 {
                let index = split.parse::<usize>();
                Ok(index.ok())
            } else {
                Ok(None)
            }
        };
    
        let st = self.next_string()?;
        let mut splits_iter = st.split('/');
        let split1 = splits_iter
            .next()
            .and_then(|s| process_split(&s).transpose())
            .transpose()?;
        let split2 = splits_iter
            .next()
            .and_then(|s| process_split(&s).transpose())
            .transpose()?;
        let split3 = splits_iter
            .next()
            .and_then(|s| process_split(&s).transpose())
            .transpose()?;
    
        if split1.is_none() || splits_iter.next().is_some() {
            return error(
                self.line_number, 
                ErrorKind::ExpectedVTNIndexButGot(st.into())
            );
        }
        
        match (split1, split2, split3) {
            (Some(v), None, None) => Ok(VTNIndex::V(v - 1)),
            (Some(v), None, Some(n)) => Ok(VTNIndex::VN(v - 1, n - 1)),
            (Some(v), Some(t), None) => Ok(VTNIndex::VT(v - 1, t - 1)),
            (Some(v), Some(t), Some(n)) => Ok(VTNIndex::VTN(v - 1, t - 1, n - 1)),
            _ => return error(
                self.line_number, 
                ErrorKind::ExpectedVTNIndexButGot(st.into())
            ),
        }
    }

    fn parse_vtn_indices(&mut self, vtn_indices: &mut Vec<VTNIndex>) -> Result<usize, ParseError> {
        let mut indices_parsed = 0;
        while let Ok(vtn_index) = self.parse_vtn_index() {
            vtn_indices.push(vtn_index);
            indices_parsed += 1;
        }

        Ok(indices_parsed)
    }

    fn parse_point(&mut self, elements: &mut Vec<Element>) -> Result<usize, ParseError> {
        self.expect_tag("p")?;

        let v_index = self.parse_usize()?;
        elements.push(Element::Point(VTNIndex::V(v_index - 1)));
        let mut elements_parsed = 1;
        loop {
            match self.next() {
                Some(st) if st != "\n" => match st.parse::<usize>() {
                    Ok(v_index) => { 
                        elements.push(Element::Point(VTNIndex::V(v_index - 1)));
                        elements_parsed += 1;
                    }
                    Err(_) => {
                        return error(
                            self.line_number,
                            ErrorKind::ExpectedIntegerButGot(st.into())
                        )
                    }
                }
                _ => break,
            }
        }

        Ok(elements_parsed)
    }

    fn parse_line(&mut self, elements: &mut Vec<Element>) -> Result<usize, ParseError> {
        self.expect_tag("l")?;

        let mut vtn_indices = vec![];
        vtn_indices.push(self.parse_vtn_index()?);
        vtn_indices.push(self.parse_vtn_index()?);
        self.parse_vtn_indices(&mut vtn_indices)?;

        // Verify that each VTN index has the same type and if of a valid form.
        for i in 1..vtn_indices.len() {
            if !vtn_indices[i].has_same_type_as(&vtn_indices[0]) {
                return error(
                    self.line_number, 
                    ErrorKind::EveryVTNIndexMustHaveTheSameFormForAGivenElement
                );
            }
        }

        // Now that we have verified the indices, build the line elements.
        for i in 0..vtn_indices.len()-1 {
            elements.push(Element::Line(vtn_indices[i], vtn_indices[i + 1]));
        }

        Ok(vtn_indices.len() - 1)
    }

    fn parse_face(&mut self, elements: &mut Vec<Element>) -> Result<usize, ParseError> {
        self.expect_tag("f")?;
        
        let mut vtn_indices = vec![];
        self.parse_vtn_indices(&mut vtn_indices)?;

        // Check that there are enough vtn indices.
        if vtn_indices.len() < 3 {
            return error(
                self.line_number, 
                ErrorKind::EveryFaceElementMustHaveAtLeastThreeVertices
            );
        }

        // Verify that each VTN index has the same type and if of a valid form.
        for i in 1..vtn_indices.len() {
            if !vtn_indices[i].has_same_type_as(&vtn_indices[0]) {
                return error(
                    self.line_number, 
                    ErrorKind::EveryVTNIndexMustHaveTheSameFormForAGivenElement
                );
            }
        }

        // Triangulate the polygon with a triangle fan. Note that the OBJ 
        // specification assumes that polygons are coplanar, and consequently 
        // the parser does not check this. It is up to the model creator to 
        // ensure this.
        let vertex0 = vtn_indices[0];
        for i in 0..vtn_indices.len()-2 {
            elements.push(Element::Face(vertex0, vtn_indices[i+1], vtn_indices[i+2]));
        }

        Ok(vtn_indices.len() - 2)
    }

    fn parse_elements(&mut self, elements: &mut Vec<Element>) -> Result<usize, ParseError> {  
        match self.peek() {
            Some("p") => self.parse_point(elements),
            Some("l") => self.parse_line(elements),
            Some("f") => self.parse_face(elements),
            _ => error(self.line_number, ErrorKind::ElementMustBeAPointLineOrFace),
        }
    }

    fn parse_groups(&mut self, groups: &mut Vec<Group>) -> Result<usize, ParseError> {
        self.expect_tag("g")?;
        let mut groups_parsed = 0;
        loop {
            match self.next() {
                Some(name) if name != "\n" => {
                    groups.push(Group::new(name));
                    groups_parsed += 1;
                }
                _ => break,
            }
        }

        Ok(groups_parsed)
    }

    fn parse_smoothing_group(
        &mut self, 
        smoothing_groups: &mut Vec<SmoothingGroup>) -> Result<usize, ParseError> {

        self.expect_tag("s")?;
        if let Some(name) = self.next() {
            if name == "off" {
                smoothing_groups.push(SmoothingGroup::new(0));
            } else if let Ok(number) = name.parse::<usize>() {
                smoothing_groups.push(SmoothingGroup::new(number));
            } else {
                return error(
                    self.line_number, 
                    ErrorKind::SmoothingGroupNameMustBeOffOrInteger(name.into())
                );
            }
        } else {
            return error(
                self.line_number, 
                ErrorKind::SmoothingGroupDeclarationHasNoName
            );
        }

        Ok(1)
    }

    fn parse_shape_entries(&self,
        shape_entry_table: &mut Vec<ShapeEntry>,
        elements: &[Element],
        group_entry_table: &[((usize, usize), (usize, usize))],
        smoothing_group_entry_table: &[((usize, usize), usize)]) {

        for &((min_element_index, max_element_index), 
              (min_group_index, max_group_index)) in group_entry_table { 
            
            let groups: Vec<usize> = (min_group_index..max_group_index).collect();
            for i in min_element_index..max_element_index {
                shape_entry_table.push(ShapeEntry::new(i, &groups, 1));
            }
        }
        debug_assert!(shape_entry_table.len() == elements.len());

        for &((min_element_index, max_element_index), 
               smoothing_group_index) in smoothing_group_entry_table {
 
            for i in min_element_index..max_element_index {
                shape_entry_table[i].smoothing_group = smoothing_group_index;
            }
        }
        debug_assert!(shape_entry_table.len() == elements.len());
    }

    fn parse_object(&mut self,
        min_vertex_index:  &mut usize,  
        max_vertex_index:  &mut usize,
        min_texture_index: &mut usize,  
        max_texture_index: &mut usize,
        min_normal_index:  &mut usize,  
        max_normal_index:  &mut usize) -> Result<Object, ParseError> {
        
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
                    // Fetch the new groups.
                    let amount_parsed = self.parse_groups(&mut groups)?;
                    max_group_index += amount_parsed;
                }
                Some("g") => {
                    // Save the shape entry ranges for the current group.
                    group_entry_table.push((
                        (min_element_group_index, max_element_group_index), 
                        (min_group_index, max_group_index)
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
                        (min_element_smoothing_group_index, max_element_smoothing_group_index),
                        smoothing_group_index
                    ));

                    self.parse_smoothing_group(&mut smoothing_groups)?;
                    smoothing_group_index += 1;
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
                        min_group_index = 0;
                        max_group_index = 1;
                    }

                    if smoothing_groups.is_empty() {
                        smoothing_groups.push(Default::default());
                        smoothing_group_index = 0;
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
                    return error(
                        self.line_number, 
                        ErrorKind::InvalidElementDeclaration(other_st.into())
                    );
                }
            }
        }

        // At the end of file, collect any remaining shapes.
        // Fill in the shape entries for the current group.
        let mut shape_entries = vec![];
        self.parse_shape_entries(
            &mut shape_entries, 
            &elements, 
            &group_entry_table, 
            &smoothing_group_entry_table
        );

        *min_vertex_index  += vertices.len();
        *max_vertex_index  += vertices.len();
        *min_texture_index += texture_vertices.len();
        *max_texture_index += texture_vertices.len();
        *min_normal_index  += normal_vertices.len();
        *max_normal_index  += normal_vertices.len();

        Ok(Object {
            name: object_name.into(),
            vertex_set: vertices,
            texture_vertex_set: texture_vertices,
            normal_vertex_set: normal_vertices,
            group_set: groups,
            smoothing_group_set: smoothing_groups,
            element_set: elements,
            shape_set: shape_entries,
        })
    }

    fn parse_objects(&mut self) -> Result<Vec<Object>, ParseError> {
        let mut result = Vec::new();

        let mut min_vertex_index = 0;
        let mut max_vertex_index = 0;
        let mut min_tex_index    = 0;
        let mut max_tex_index    = 0;
        let mut min_normal_index = 0;
        let mut max_normal_index = 0;

        self.skip_zero_or_more_newlines();
        while let Some(_) = self.peek() {
            result.push(self.parse_object(
                &mut min_vertex_index, 
                &mut max_vertex_index,
                &mut min_tex_index,    
                &mut max_tex_index,
                &mut min_normal_index, 
                &mut max_normal_index
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
mod compositor_tests {
    use super::*;
    use std::iter::{
        FromIterator
    };


    struct Test {
        object: Object,
        expected_missing_groups: BTreeMap<(usize, usize), Vec<GroupingStatement>>, 
        expected_found_groups: BTreeMap<(usize, usize), Vec<GroupingStatement>>,
        expected: CompositorInstructions,
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
                    /* 
                    #### Original object file text.
                    o  Object1
                    v  -36.84435  -31.289864  -23.619797  -8.21862 
                    # 1 vertex

                    vt  -44.275238  28.583176  -23.780418
                    # 1 texture vertex

                    vn  93.94331  -61.460472  -32.00753 
                    # 1 normal vertex

                    g  Group0
                    g  Group1
                    # ### Equivalently,
                    # ### g Group2,
                    # ### s  0
                    s  0
                    g  Group2
                    s  1
                    g  Group3
                    f 1/1/1 1/1/1 1/1/1
                    # 1 element

                    g  Group4
                    s  2 
                    # ### End Object 1
                    */
                    object: Object::new(
                        String::from("Object1"),
                        vec![Vertex::new(-36.84435, -31.289864, -23.619797, -8.21862)],
                        vec![TextureVertex::new(-44.275238, 28.583176, -23.780418)],
                        vec![NormalVertex::new(93.94331, -61.460472, -32.00753)],
                        vec![
                            Group::new("Group0"), Group::new("Group1"), 
                            Group::new("Group2"), Group::new("Group3"), Group::new("Group4")
                        ],
                        vec![SmoothingGroup::new(0), SmoothingGroup::new(1), SmoothingGroup::new(2)],
                        vec![Element::Face(VTNIndex::VTN(1, 1, 1), VTNIndex::VTN(1, 1, 1), VTNIndex::VTN(1, 1, 1))], 
                        vec![ShapeEntry::new(1, &vec![4], 2)],
                    ),
                    expected_missing_groups: BTreeMap::from(FromIterator::from_iter(
                        vec![
                            ((0, 1), vec![
                                GroupingStatement::G(vec![Group::new("Group0")]),
                                GroupingStatement::G(vec![Group::new("Group1")]),
                                GroupingStatement::G(vec![Group::new("Group2")]),
                                GroupingStatement::S(SmoothingGroup::new(0)),
                            ]),
                            ((1, 1), vec![
                                GroupingStatement::G(vec![Group::new("Group4")]),
                                GroupingStatement::S(SmoothingGroup::new(2)),
                            ]),
                        ]
                    )),
                    expected_found_groups: BTreeMap::from(FromIterator::from_iter(
                        vec![
                            ((0, 1), vec![
                                GroupingStatement::G(vec![Group::new("Group3")]),
                                GroupingStatement::S(SmoothingGroup::new(1)),
                            ]),
                            ((1, 1), vec![]),

                        ]
                    )),
                    expected: CompositorInstructions::new(FromIterator::from_iter(
                        vec![
                            ((0, 1), vec![
                                GroupingStatement::G(vec![Group::new("Group0")]),
                                GroupingStatement::G(vec![Group::new("Group1")]),
                                GroupingStatement::G(vec![Group::new("Group2")]),
                                GroupingStatement::S(SmoothingGroup::new(0)),
                                GroupingStatement::G(vec![Group::new("Group3")]),
                                GroupingStatement::S(SmoothingGroup::new(1))
                            ]),
                            ((1, 1), vec![
                                GroupingStatement::G(vec![Group::new("Group4")]),
                                GroupingStatement::S(SmoothingGroup::new(2))
                            ]),
                        ]    
                    )),
                },
            ]
        }
    }

    fn test_cases2() -> TestSet {
        TestSet { 
            data: vec![ 
                Test {
                    /* 
                    #### BEGIN Object 1
                    o  Object1 
                    v  -81.75473  49.89659  50.217773  -0.21859932 
                    v  58.582382  40.18698  20.389153  -0.8563268 
                    v  20.67199  -32.264946  -43.075634  -0.8146236 
                    v  -0.51555634  -61.86371  -63.40442  -0.816622 
                    v  64.52879  5.6848984  82.95958  -0.8919699 
                    v  -22.82035  26.620651  98.339966  0.47607088 
                    v  74.063614  -72.82653  16.68911  0.57268834 
                    v  36.223984  40.50911  -46.372032  -0.27578998 
                    # 8 vertices

                    vt  -31.56221  -66.285965  85.67 
                    vt  -94.91446  -32.6334  -76.25124 
                    vt  74.14935  -93.767525  -95.665504 
                    vt  58.248764  -77.56836  -90.145615 
                    vt  -23.581291  -45.771004  -2.3966064 
                    vt  47.556717  -94.74621  -95.27831 
                    vt  -40.32562  -28.224586  -69.58597 
                    vt  18.032005  41.304443  83.784836 
                    # 8 texture vertices

                    vn  37.12401  65.5159  -67.49673 
                    vn  -27.513626  68.86371  -40.72206 
                    vn  -2.038643  -48.640347  65.63937 
                    vn  93.694565  63.53665  52.100876 
                    vn  40.664124  55.000015  -45.83249 
                    vn  30.624634  31.461197  -93.17193 
                    vn  25.595596  30.777481  79.21614 
                    vn  -36.078453  1.8164139  21.209381 
                    # 8 normal vertices

                    # 0 elements

                    g  Group0
                    s  off
                    f  7/1/4  2/7/1  4/1/7
                    # 1 element

                    s  off
                    f  7/5/7  5/8/5  2/7/4
                    f  8/2/7  8/1/1  7/8/4
                    # 2 elements

                    g  Group1
                    g  Group2
                    f  8/5/2  7/1/1  5/6/3
                    f  4/7/8  7/7/7  6/8/5
                    f  5/8/2  6/7/2  3/2/5
                    f  6/3/6  3/5/4  6/5/1
                    # 4 elements

                    g  Group3  
                    s  1 
                    f  2/5/1  4/6/5  8/1/6
                    # 1 element

                    # 0 elements

                    #### END Object 1
                    */
                    object: Object::new(
                        String::from("Object1"),
                        vec![
                            Vertex::new(-81.75473,    49.89659,   50.217773, -0.21859932),
                            Vertex::new( 58.582382,   40.18698,   20.389153, -0.8563268 ),
                            Vertex::new( 20.67199,   -32.264946, -43.075634, -0.8146236 ),
                            Vertex::new(-0.51555634, -61.86371,  -63.40442,  -0.816622  ),
                            Vertex::new( 64.52879,    5.6848984,  82.95958,  -0.8919699 ),
                            Vertex::new(-22.82035,    26.620651,  98.339966,  0.47607088),
                            Vertex::new( 74.063614,  -72.82653,   16.68911,   0.57268834),
                            Vertex::new( 36.223984,   40.50911,  -46.372032, -0.27578998),
                        ],
                        vec![
                            TextureVertex::new(-31.56221,  -66.285965,  85.67    ),
                            TextureVertex::new(-94.91446,  -32.6334,   -76.25124 ),
                            TextureVertex::new( 74.14935,  -93.767525, -95.665504),
                            TextureVertex::new( 58.248764, -77.56836,  -90.145615),
                            TextureVertex::new(-23.581291, -45.771004, -2.3966064), 
                            TextureVertex::new( 47.556717, -94.74621,  -95.27831 ),
                            TextureVertex::new(-40.32562,  -28.224586, -69.58597 ), 
                            TextureVertex::new( 18.032005,  41.304443,  83.784836),
                        ],
                        vec![
                            NormalVertex::new( 37.12401,   65.5159,   -67.49673 ),
                            NormalVertex::new(-27.513626,  68.86371,  -40.72206 ),
                            NormalVertex::new(-2.038643,  -48.640347,  65.63937 ),
                            NormalVertex::new( 93.694565,  63.53665,   52.100876),
                            NormalVertex::new( 40.664124,  55.000015, -45.83249 ),
                            NormalVertex::new( 30.624634,  31.461197, -93.17193 ),
                            NormalVertex::new( 25.595596,  30.777481,  79.21614 ),
                            NormalVertex::new(-36.078453,  1.8164139,  21.209381),
                        ],
                        vec![
                            Group::new("Group0"), Group::new("Group1"), 
                            Group::new("Group2"), Group::new("Group3"),
                        ],
                        vec![
                            SmoothingGroup::new(0), SmoothingGroup::new(1),
                        ],
                        vec![
                            Element::Face(VTNIndex::VTN(7, 1, 4), VTNIndex::VTN(2, 7, 1), VTNIndex::VTN(4, 1, 7)),
                            Element::Face(VTNIndex::VTN(7, 5, 7), VTNIndex::VTN(5, 8, 5), VTNIndex::VTN(2, 7, 4)),
                            Element::Face(VTNIndex::VTN(8, 2, 7), VTNIndex::VTN(8, 1, 1), VTNIndex::VTN(7, 8, 4)),
                            Element::Face(VTNIndex::VTN(8, 5, 2), VTNIndex::VTN(7, 1, 1), VTNIndex::VTN(5, 6, 3)),
                            Element::Face(VTNIndex::VTN(4, 7, 8), VTNIndex::VTN(7, 7, 7), VTNIndex::VTN(6, 8, 5)),
                            Element::Face(VTNIndex::VTN(5, 8, 2), VTNIndex::VTN(6, 7, 2), VTNIndex::VTN(3, 2, 5)),
                            Element::Face(VTNIndex::VTN(6, 3, 6), VTNIndex::VTN(3, 5, 4), VTNIndex::VTN(6, 5, 1)),
                            Element::Face(VTNIndex::VTN(2, 5, 1), VTNIndex::VTN(4, 6, 5), VTNIndex::VTN(8, 1, 6)),
                        ], 
                        vec![
                            ShapeEntry::new(1, &vec![1], 1), ShapeEntry::new(2, &vec![1], 1),
                            ShapeEntry::new(3, &vec![1], 1), ShapeEntry::new(4, &vec![3], 1),
                            ShapeEntry::new(5, &vec![3], 1), ShapeEntry::new(6, &vec![3], 1),
                            ShapeEntry::new(7, &vec![3], 1), ShapeEntry::new(8, &vec![4], 2),
                        ],
                    ),
                    expected_missing_groups: BTreeMap::from(FromIterator::from_iter(
                        vec![
                            ((0, 3), vec![]),
                            ((3, 7), vec![
                                GroupingStatement::G(vec![Group::new("Group1")]),
                            ]),
                            ((7, 8), vec![]),
                            ((8, 8), vec![]),
                        ]                        
                    )),
                    expected_found_groups: BTreeMap::from(FromIterator::from_iter(
                        vec![
                            ((0, 3), vec![
                                GroupingStatement::G(vec![Group::new("Group0")]),
                                GroupingStatement::S(SmoothingGroup::new(0)),
                            ]),
                            ((3, 7), vec![
                                GroupingStatement::G(vec![Group::new("Group2")]),
                            ]),
                            ((7, 8), vec![
                                GroupingStatement::G(vec![Group::new("Group3")]),
                                GroupingStatement::S(SmoothingGroup::new(1)),
                            ]),
                            ((8, 8), vec![]),
                        ]
                    )),
                    expected: CompositorInstructions::new(FromIterator::from_iter(
                        vec![
                            ((0, 3), vec![
                                GroupingStatement::G(vec![Group::new("Group0")]),
                                GroupingStatement::S(SmoothingGroup::new(0)),
                            ]),
                            ((3, 7), vec![
                                GroupingStatement::G(vec![Group::new("Group1")]),
                                GroupingStatement::G(vec![Group::new("Group2")]),
                            ]),
                            ((7, 8), vec![
                                GroupingStatement::G(vec![Group::new("Group3")]),
                                GroupingStatement::S(SmoothingGroup::new(1)),
                            ]),
                            ((8, 8), vec![]),
                        ]
                    )),
                },
            ]
        }
    }

    #[test]
    fn test_compositor_instructions() {
        let tests = test_cases();

        for test in tests.iter() {
            let result_missing_groups = CompositorInstructions::generate_missing_groups(&test.object);
            let result_found_groups = CompositorInstructions::generate_found_groups(&test.object);
            let result = CompositorInstructions::generate(&test.object);

            assert_eq!(result_missing_groups, test.expected_missing_groups);
            assert_eq!(result_found_groups, test.expected_found_groups);
            assert_eq!(result, test.expected);
        }
    }

    #[test]
    fn test_compositor_instructions_with_s_zero_declared() {
        let tests = test_cases2();

        for test in tests.iter() {
            let result_missing_groups = CompositorInstructions::generate_missing_groups(&test.object);
            let result_found_groups = CompositorInstructions::generate_found_groups(&test.object);
            let result = CompositorInstructions::generate(&test.object);

            assert_eq!(result_missing_groups, test.expected_missing_groups);
            assert_eq!(result_found_groups, test.expected_found_groups);
            assert_eq!(result, test.expected);
        }
    }

}


#[cfg(test)]
mod primitive_tests {
    #[test]
    fn test_parse_f64() {
        let mut parser = super::Parser::new("-1.929448");
        assert_eq!(parser.parse_f64(), Ok(-1.929448));
    }

    #[test]
    fn test_parse_usize() {
        let mut parser = super::Parser::new("    763   ");
        assert_eq!(parser.parse_usize(), Ok(763));
    }
}

#[cfg(test)]
mod vertex_tests {
    use crate::obj::{
        Vertex,
    };


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
    use crate::obj::{
        TextureVertex,
    };


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
    use crate::obj::{
        NormalVertex,
    };


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
    use crate::obj::{
        VTNIndex,
    };


    #[test]
    fn test_parse_vtn_index1() {
        let mut parser = super::Parser::new("1291");
        let expected = VTNIndex::V(1290);
        let result = parser.parse_vtn_index();
        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn test_parse_vtn_index2() {
        let mut parser = super::Parser::new("1291/1315");
        let expected = VTNIndex::VT(1290, 1314);
        let result = parser.parse_vtn_index();
        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn test_parse_vtn_index3() {
        let mut parser = super::Parser::new("1291/1315/1314");
        let expected = VTNIndex::VTN(1290, 1314, 1313);
        let result = parser.parse_vtn_index();
        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn test_parse_vtn_index4() {
        let mut parser = super::Parser::new("1291//1315");
        let expected = VTNIndex::VN(1290, 1314);
        let result = parser.parse_vtn_index();
        assert_eq!(result, Ok(expected));
    }

}

#[cfg(test)]
mod element_tests {
    use crate::obj::{
        Element, 
        VTNIndex,
    };


    #[test]
    fn test_parse_point1() {
        let mut parser = super::Parser::new("p 1 2 3 4 \n");
        let mut result = vec![];
        let expected = vec![
            Element::Point(VTNIndex::V(0)), Element::Point(VTNIndex::V(1)),
            Element::Point(VTNIndex::V(2)), Element::Point(VTNIndex::V(3)),
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
            Element::Line(VTNIndex::V(296), VTNIndex::V(37)), 
            Element::Line(VTNIndex::V(37),  VTNIndex::V(117)),
            Element::Line(VTNIndex::V(117), VTNIndex::V(107)),
        ];
        assert!(parser.parse_elements(&mut result).is_ok());
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_line2() {
        let mut parser = super::Parser::new("l 297/38 118/108 \n");
        let mut result = vec![];
        let expected = vec![
            Element::Line(VTNIndex::VT(296, 37), VTNIndex::VT(117, 107)),
        ];
        assert!(parser.parse_elements(&mut result).is_ok());
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_line3() {
        let mut parser = super::Parser::new("l 297/38 118/108 324/398 \n");
        let mut result = vec![];
        let expected = vec![
            Element::Line(VTNIndex::VT(296, 37), VTNIndex::VT(117, 107)),
            Element::Line(VTNIndex::VT(117, 107), VTNIndex::VT(323, 397)),
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
            Element::Face(VTNIndex::V(296), VTNIndex::V(117), VTNIndex::V(107)),
        ];
        assert!(parser.parse_elements(&mut result).is_ok());
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_face2() {
        let mut parser = super::Parser::new("f 297 118 108 324\n");
        let mut result = vec![];
        let expected = vec![
            Element::Face(VTNIndex::V(296), VTNIndex::V(117), VTNIndex::V(107)),
            Element::Face(VTNIndex::V(296), VTNIndex::V(107), VTNIndex::V(323)),
        ];
        assert!(parser.parse_elements(&mut result).is_ok());
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_face3() {
        let mut parser = super::Parser::new("f 297 118 108 324 398 \n");
        let mut result = vec![];
        let expected = vec![
            Element::Face(VTNIndex::V(296), VTNIndex::V(117), VTNIndex::V(107)),
            Element::Face(VTNIndex::V(296), VTNIndex::V(107), VTNIndex::V(323)),
            Element::Face(VTNIndex::V(296), VTNIndex::V(323), VTNIndex::V(397)),
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
            Element::Face(VTNIndex::VN(34183, 34183), VTNIndex::VN(34087, 34087), VTNIndex::VN(34078, 34078)),
            Element::Face(VTNIndex::VN(34183, 34183), VTNIndex::VN(34078, 34078), VTNIndex::VN(34083, 34083)),
            Element::Face(VTNIndex::VN(34183, 34183), VTNIndex::VN(34083, 34083), VTNIndex::VN(34090, 34090)),
            Element::Face(VTNIndex::VN(34183, 34183), VTNIndex::VN(34090, 34090), VTNIndex::VN(34075, 34075)),
        ];
        parser.parse_elements(&mut result).unwrap();
        assert_eq!(result, expected);
    }
}

#[cfg(test)]
mod group_tests {
    use crate::obj::{
        Group,
    };


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
    use crate::obj::{
        SmoothingGroup
    };


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
    use crate::obj::{
        ObjectSet, 
        Object,
        Vertex, 
        NormalVertex, 
        Element, 
        VTNIndex, 
        Group, 
        SmoothingGroup, 
        ShapeEntry,
    };


    fn test_case() -> (Result<ObjectSet, super::ParseError>, Result<ObjectSet, super::ParseError>){
        let obj_file =r"                 \
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
            NormalVertex { i:  0.0, j:  0.0, k:  1.0 },
            NormalVertex { i:  0.0, j:  0.0, k: -1.0 },
            NormalVertex { i:  0.0, j:  1.0, k:  0.0 },
            NormalVertex { i:  0.0, j: -1.0, k:  0.0 },
            NormalVertex { i:  1.0, j:  0.0, k:  0.0 },
            NormalVertex { i: -1.0, j:  0.0, k:  0.0 },
        ];
        let group_set = vec![Group::new("cube")];
        let smoothing_group_set = vec![SmoothingGroup::new(0)];
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
        let object = Object {
            name: name,
            vertex_set: vertex_set,
            texture_vertex_set: texture_vertex_set,
            normal_vertex_set: normal_vertex_set,
            group_set: group_set,
            smoothing_group_set: smoothing_group_set,
            element_set: element_set,
            shape_set: shape_set,
        };
        let expected = ObjectSet::new(vec![object]);
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

