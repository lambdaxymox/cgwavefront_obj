use std::default::Default;
use std::slice;
use std::fmt;
use std::ops;
use std::collections::BTreeMap;


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
            VTNIndex::V(v) => write!(f, "{}", v),
            VTNIndex::VT(v, vt) => write!(f, "{}/{}", v ,vt),
            VTNIndex::VN(v, vn) => write!(f, "{}//{}", v, vn),
            VTNIndex::VTN(v, vt, vn) => write!(f, "{}/{}/{}", v, vt, vn),
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
            Element::Point(vtn) => write!(f, "p  {}", vtn),
            Element::Line(vtn1, vtn2) => write!(f, "l  {}  {}", vtn1, vtn2),
            Element::Face(vtn1, vtn2, vtn3) => write!(f, "f  {}  {}  {}", vtn1, vtn2, vtn3),
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
                let vertex = self.vertex_set.get((v_index - 1) as usize)?;

                Some(VTNTriple::V(vertex))
            }
            VTNIndex::VT(v_index, vt_index) => { 
                let vertex = self.vertex_set.get((v_index - 1) as usize)?;
                let texture_vertex = self.texture_vertex_set.get((vt_index - 1) as usize)?;

                Some(VTNTriple::VT(vertex, texture_vertex))
            }
            VTNIndex::VN(v_index, vn_index) => {
                let vertex = self.vertex_set.get((v_index - 1) as usize)?;
                let normal_vertex = self.normal_vertex_set.get((vn_index - 1) as usize)?;

                Some(VTNTriple::VN(vertex, normal_vertex))
            }
            VTNIndex::VTN(v_index, vt_index, vn_index) => {
                let vertex = self.vertex_set.get((v_index - 1) as usize)?;
                let texture_vertex = self.texture_vertex_set.get((vt_index - 1) as usize)?;
                let normal_vertex = self.normal_vertex_set.get((vn_index - 1) as usize)?;

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


#[derive(Clone, PartialEq)]
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

impl fmt::Debug for ObjectSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let string = DisplayObjectSetCompositor::new().compose(self);
        write!(f, "{}", string)
    }
}

impl fmt::Display for ObjectSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let string = DisplayObjectSetCompositor::new().compose(self);
        write!(f, "{}", string)
    }
}

pub struct ObjectBuilder {
    name: Option<String>,
    vertex_set: Vec<Vertex>,
    texture_vertex_set: Option<Vec<TextureVertex>>,
    normal_vertex_set: Option<Vec<NormalVertex>>,
    group_set: Option<Vec<Group>>,
    smoothing_group_set: Option<Vec<SmoothingGroup>>,
    element_set: Vec<Element>,
    shape_set: Option<Vec<ShapeEntry>>,
}

impl ObjectBuilder {
    pub fn new(vertex_set: Vec<Vertex>, element_set: Vec<Element>) -> ObjectBuilder {
        ObjectBuilder {
            name: None,
            vertex_set: Vec::from(vertex_set),
            texture_vertex_set: None,
            normal_vertex_set: None,
            group_set: None,
            smoothing_group_set: None,
            element_set: Vec::from(element_set),
            shape_set: None,
        }
    }

    pub fn with_name(&mut self, name: String) -> &mut Self {
        self.name = Some(name);
        self
    }

    pub fn with_texture_vertex_set(&mut self, texture_vertex_set: Vec<TextureVertex>) -> &mut Self {
        self.texture_vertex_set = Some(Vec::from(texture_vertex_set));
        self
    }

    pub fn with_normal_vertex_set(&mut self, normal_vertex_set: Vec<NormalVertex>) -> &mut Self {
        self.normal_vertex_set = Some(Vec::from(normal_vertex_set));
        self
    }

    pub fn with_group_set(&mut self, group_set: Vec<Group>) -> &mut Self {
        self.group_set = Some(Vec::from(group_set));
        self
    }

    pub fn with_smoothing_group_set(&mut self, smoothing_group_set: Vec<SmoothingGroup>) -> &mut Self {
        self.smoothing_group_set = Some(Vec::from(smoothing_group_set));
        self
    }

    pub fn with_shape_set(&mut self, shape_set: Vec<ShapeEntry>) -> &mut Self {
        self.shape_set = Some(Vec::from(shape_set));
        self
    }

    pub fn build(self) -> Object {
        Object {
            name: self.name.unwrap_or(String::from("")),
            vertex_set: self.vertex_set,
            texture_vertex_set: self.texture_vertex_set.unwrap_or(Default::default()),
            normal_vertex_set: self.normal_vertex_set.unwrap_or(Default::default()),
            group_set: self.group_set.unwrap_or(Default::default()),
            smoothing_group_set: self.smoothing_group_set.unwrap_or(Vec::from(vec![Default::default()])),
            element_set: self.element_set,
            shape_set: self.shape_set.unwrap_or(Default::default()),
        }
    }
}


trait ObjectCompositor {
    fn compose(&self, object: &Object) -> String;
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

impl ObjectCompositor for DisplayObjectCompositor {
    fn compose(&self, object: &Object) -> String {
        self.compose(object)
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
    instructions: BTreeMap<(u32, u32), Vec<GroupingStatement>>,
}

impl CompositorInstructions {
    fn new(instructions: BTreeMap<(u32, u32), Vec<GroupingStatement>>) -> Self {
        Self { instructions: instructions }
    }

    /// Find any missing groups and smoothing groups that contain no elements.
    ///
    /// We can find the missing groups in the object set because per the 
    /// specification, grouping statements are indexed in monotone increasing 
    /// order. So any gaps in the grouping indices in the index buffer indicates 
    /// which groups are missing, and we can fill there in when generating a 
    /// *.obj file from an object set.
    fn generate_missing_groups(object: &Object) -> BTreeMap<(u32, u32), Vec<GroupingStatement>> {
        let mut missing_groups = BTreeMap::new();
        
        // It is possible that there are missing groups that appear before the 
        // first interval of elements in the shape set. We must calculate
        // these first for otherwise we would miss them when filling in the table.
        let initial_group = object.shape_set[0].groups[0];
        let initial_smoothing_group = object.shape_set[0].smoothing_group;
        let mut current_statements = vec![];
        for group_index in 1..initial_group {
            current_statements.push(GroupingStatement::G(vec![
                object.group_set[(group_index - 1) as usize].clone()
            ]));
        }

        for smoothing_group_index in 1..initial_smoothing_group {
            current_statements.push(GroupingStatement::S(
                object.smoothing_group_set[(smoothing_group_index - 1) as usize]
            ));
        }

        // In order to fill in the missing groups and smoothing groups, we need
        // to know which groups and smoothing groups are occupied in the object. 
        // After that, we can determine which groups are missing and fill them in.
        let mut current_entry = &object.shape_set[0];
        let mut min_element_index = 1;
        let mut max_element_index = 1;
        for shape_entry in object.shape_set.iter() {
            if shape_entry.groups != current_entry.groups || 
                shape_entry.smoothing_group != current_entry.smoothing_group {
                // We have crossed a group or smoothing group boundary. Here we 
                // have found the end of the interval of elements with the same 
                // groups and smoothing groups.
                missing_groups.insert((min_element_index as u32, max_element_index as u32), current_statements);

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
                            object.group_set[(group_index - 1) as usize].clone()
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
                            object.smoothing_group_set[(smoothing_group_index - 1) as usize]
                        ));
                    }
                }

                // Place the missing groups into the table.
                //missing_groups.insert((min_element_index as u32, max_element_index as u32), statements);

                // Continue with the next interval.
                current_entry = shape_entry;
                min_element_index = max_element_index;
            } 
            
            // We have processed this group element.
            max_element_index += 1;
        }

        // Process the last existing interval.
        missing_groups.insert((min_element_index as u32, max_element_index as u32), current_statements);

        // The last interval of empty groups and smoothing groups
        // lies off the end of the element list.
        min_element_index = max_element_index;

        // It is possible that there are missing groups that appear after the 
        // final groups and smoothing groups in the shape set. We must calculate
        // these last one for otherwise they would get lost when passing the 
        // text to the parser.
        let final_shape_entry = &object.shape_set[(min_element_index - 1) - 1];
        let final_group = final_shape_entry.groups[final_shape_entry.groups.len() - 1];
        let final_smoothing_group = final_shape_entry.smoothing_group;
        let mut final_statements = vec![];
        for group_index in (final_group + 1)..((object.group_set.len() + 1)) {
            final_statements.push(GroupingStatement::G(vec![
                object.group_set[(group_index - 1) as usize].clone()
            ]));
        }

        for smoothing_group_index 
            in (final_smoothing_group + 1)..((object.smoothing_group_set.len() + 1)) {
            
            final_statements.push(GroupingStatement::S(
                object.smoothing_group_set[(smoothing_group_index - 1) as usize]
            ));
        }

        missing_groups.insert((min_element_index as u32, min_element_index as u32), final_statements);

        missing_groups
    }

    /// Place the grouping statements for the groups that contain elements.
    fn generate_found_groups(object: &Object) -> BTreeMap<(u32, u32), Vec<GroupingStatement>> {
        let mut found_groups = BTreeMap::new();
        let mut min_element_index = 1;
        let mut max_element_index = 1;
        let mut current_statements = vec![];
        let mut current_entry = &object.shape_set[0];

        let new_groups = current_entry.groups.iter().fold(vec![], |mut acc, group_index| {
            acc.push(object.group_set[(group_index - 1) as usize].clone());
            acc
        });
        current_statements.push(GroupingStatement::G(new_groups));
        current_statements.push(GroupingStatement::S(
            object.smoothing_group_set[(current_entry.smoothing_group - 1) as usize])
        );

        for shape_entry in object.shape_set.iter() {
            if shape_entry.groups != current_entry.groups || 
                shape_entry.smoothing_group != current_entry.smoothing_group {

                found_groups.insert((min_element_index, max_element_index), current_statements);

                current_statements = vec![];

                // Are the groups different?
                if shape_entry.groups != current_entry.groups {
                    let new_groups = shape_entry.groups.iter().fold(vec![], |mut acc, group_index| {
                        acc.push(object.group_set[(group_index - 1) as usize].clone());
                        acc
                    });
                    current_statements.push(GroupingStatement::G(new_groups));
                }
                
                // Are the smoothing groups different?
                if shape_entry.smoothing_group != current_entry.smoothing_group {
                    current_statements.push(GroupingStatement::S(
                        object.smoothing_group_set[(shape_entry.smoothing_group - 1) as usize])
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
        let mut instructions: BTreeMap<(u32, u32), Vec<GroupingStatement>> = BTreeMap::new();
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

/// A `TextObjectCompositor` generates a Wavefront OBJ text block from an object.
/// One can use it to automatically generate OBJ files.
struct TextObjectCompositor { }

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

    fn compose_elements(&self, object: &Object, interval: (u32, u32)) -> String {
        let string = (interval.0..interval.1).fold(String::new(), |acc, i| {
            acc + &format!("{}\n", object.element_set[(i - 1) as usize])
        });
        format!("{}", string)
    }

    fn get_group_instructions(&self, object: &Object) -> BTreeMap<(u32, u32), Vec<GroupingStatement>> {
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

impl ObjectCompositor for TextObjectCompositor {
    fn compose(&self, object: &Object) -> String {
        self.compose(object)
    }
}

pub trait Compositor {
    fn compose(&self, object_set: &ObjectSet) -> String;
}

/// The `DisplayObjectCompositor` type is the default compositor
/// for presenting object set information to the end user.
pub struct DisplayObjectSetCompositor { }

impl DisplayObjectSetCompositor {
    pub fn new() -> Self { Self {} }
}

impl Compositor for DisplayObjectSetCompositor {
    fn compose(&self, object_set: &ObjectSet) -> String {
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
    pub fn new() -> Self { Self {} }
}

impl Compositor for TextObjectSetCompositor {
    fn compose(&self, object_set: &ObjectSet) -> String {
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


#[cfg(test)]
mod compositor_tests {
    use super::*;
    use std::iter::{
        FromIterator
    };


    struct Test {
        object: Object,
        expected_missing_groups: BTreeMap<(u32, u32), Vec<GroupingStatement>>, 
        expected_found_groups: BTreeMap<(u32, u32), Vec<GroupingStatement>>,
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
                            ((1, 2), vec![
                                GroupingStatement::G(vec![Group::new("Group0")]),
                                GroupingStatement::G(vec![Group::new("Group1")]),
                                GroupingStatement::G(vec![Group::new("Group2")]),
                                GroupingStatement::S(SmoothingGroup::new(0)),
                            ]),
                            ((2, 2), vec![
                                GroupingStatement::G(vec![Group::new("Group4")]),
                                GroupingStatement::S(SmoothingGroup::new(2)),
                            ]),
                        ]
                    )),
                    expected_found_groups: BTreeMap::from(FromIterator::from_iter(
                        vec![
                            ((1, 2), vec![
                                GroupingStatement::G(vec![Group::new("Group3")]),
                                GroupingStatement::S(SmoothingGroup::new(1)),
                            ]),
                            ((2, 2), vec![]),

                        ]
                    )),
                    expected: CompositorInstructions::new(FromIterator::from_iter(
                        vec![
                            ((1, 2), vec![
                                GroupingStatement::G(vec![Group::new("Group0")]),
                                GroupingStatement::G(vec![Group::new("Group1")]),
                                GroupingStatement::G(vec![Group::new("Group2")]),
                                GroupingStatement::S(SmoothingGroup::new(0)),
                                GroupingStatement::G(vec![Group::new("Group3")]),
                                GroupingStatement::S(SmoothingGroup::new(1))
                            ]),
                            ((2, 2), vec![
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
                            ((1, 4), vec![]),
                            ((4, 8), vec![
                                GroupingStatement::G(vec![Group::new("Group1")]),
                            ]),
                            ((8, 9), vec![]),
                            ((9, 9), vec![]),
                        ]                        
                    )),
                    expected_found_groups: BTreeMap::from(FromIterator::from_iter(
                        vec![
                            ((1, 4), vec![
                                GroupingStatement::G(vec![Group::new("Group0")]),
                                GroupingStatement::S(SmoothingGroup::new(0)),
                            ]),
                            ((4, 8), vec![
                                GroupingStatement::G(vec![Group::new("Group2")]),
                            ]),
                            ((8, 9), vec![
                                GroupingStatement::G(vec![Group::new("Group3")]),
                                GroupingStatement::S(SmoothingGroup::new(1)),
                            ]),
                            ((9, 9), vec![]),
                        ]
                    )),
                    expected: CompositorInstructions::new(FromIterator::from_iter(
                        vec![
                            ((1, 4), vec![
                                GroupingStatement::G(vec![Group::new("Group0")]),
                                GroupingStatement::S(SmoothingGroup::new(0)),
                            ]),
                            ((4, 8), vec![
                                GroupingStatement::G(vec![Group::new("Group1")]),
                                GroupingStatement::G(vec![Group::new("Group2")]),
                            ]),
                            ((8, 9), vec![
                                GroupingStatement::G(vec![Group::new("Group3")]),
                                GroupingStatement::S(SmoothingGroup::new(1)),
                            ]),
                            ((9, 9), vec![]),
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

