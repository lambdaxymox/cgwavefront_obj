use std::default::Default;
use std::slice;
use std::fmt;
use std::ops;
use std::collections::BTreeMap;


#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vertex {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vertex {
        Vertex { x: x, y: y, z: z, w: w }
    }
}

impl fmt::Display for Vertex {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "v  {}  {}  {}  {}", self.x, self.y, self.z, self.w)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TextureVertex {
    pub u: f32,
    pub v: f32,
    pub w: f32,
}

impl TextureVertex {
    pub fn new(u: f32, v: f32, w: f32) -> TextureVertex {
        TextureVertex { u: u, v: v, w: w }
    }
}

impl fmt::Display for TextureVertex {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "vt  {}  {}  {}", self.u, self.v, self.w)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NormalVertex {
    pub i: f32,
    pub j: f32,
    pub k: f32,
}

impl NormalVertex {
    pub fn new(i: f32, j: f32, k: f32) -> NormalVertex {
        NormalVertex { i: i, j: j, k: k }
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

type ElementIndex = u32;
type VertexIndex = u32;
type TextureVertexIndex = u32;
type NormalVertexIndex = u32;
type GroupIndex = u32;
type ShapeIndex = u32;
type SmoothingGroupIndex = u32;

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
    pub fn new(name: &str) -> Group { Group(String::from(name)) }
}

impl fmt::Display for Group {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.0)
    }
}

impl Default for Group {
    fn default() -> Group { Group::new("default") }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct SmoothingGroup(u32);

impl SmoothingGroup {
    pub fn new(name: u32) -> SmoothingGroup { 
        SmoothingGroup(name)
    }

    pub fn as_int(&self) -> u32 { self.0 }
}

impl Default for SmoothingGroup {
    fn default() -> SmoothingGroup { SmoothingGroup::new(0) }
}

impl fmt::Display for SmoothingGroup {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.0)
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

pub type VertexSet = Vec<Vertex>;
pub type TextureVertexSet = Vec<TextureVertex>;
pub type NormalVertexSet = Vec<NormalVertex>;
pub type ElementSet = Vec<Element>;
pub type ShapeSet = Vec<ShapeEntry>;
pub type GroupSet = Vec<Group>;
pub type SmoothingGroupSet = Vec<SmoothingGroup>;



#[derive(Clone, Debug)]
pub enum VTNTriple<'a> {
    V(&'a Vertex),
    VT(&'a Vertex, &'a TextureVertex), 
    VN(&'a Vertex, &'a NormalVertex),
    VTN(&'a Vertex, &'a TextureVertex, &'a NormalVertex),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Object {
    pub name: String,
    pub vertex_set: VertexSet,
    pub texture_vertex_set: TextureVertexSet,
    pub normal_vertex_set: NormalVertexSet,
    pub group_set: GroupSet,
    pub smoothing_group_set: SmoothingGroupSet,
    pub element_set: ElementSet,
    pub shape_set: ShapeSet,
}

impl Object {
    pub fn new(
        name: String,
        vertex_set: VertexSet, texture_vertex_set: TextureVertexSet, normal_vertex_set: NormalVertexSet,
        group_set: GroupSet, smoothing_group_set: SmoothingGroupSet, element_set: ElementSet,
        shape_set: ShapeSet) -> Object {
        
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

    fn get_vtn_triple(&self, index: VTNIndex) -> Option<VTNTriple> {
        match index {
            VTNIndex::V(v_index) => {
                let vertex = self.vertex_set.get(v_index as usize)?;

                Some(VTNTriple::V(vertex))
            }
            VTNIndex::VT(v_index, vt_index) => { 
                let vertex = self.vertex_set.get(v_index as usize)?;
                let texture_vertex = self.texture_vertex_set.get(vt_index as usize)?;

                Some(VTNTriple::VT(vertex, texture_vertex))
            }
            VTNIndex::VN(v_index, vn_index) => {
                let vertex = self.vertex_set.get(v_index as usize)?;
                let normal_vertex = self.normal_vertex_set.get(vn_index as usize)?;

                Some(VTNTriple::VN(vertex, normal_vertex))
            }
            VTNIndex::VTN(v_index, vt_index, vn_index) => {
                let vertex = self.vertex_set.get(v_index as usize)?;
                let texture_vertex = self.texture_vertex_set.get(vt_index as usize)?;
                let normal_vertex = self.normal_vertex_set.get(vn_index as usize)?;

                Some(VTNTriple::VTN(vertex, texture_vertex, normal_vertex))
            }
        }
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
            Default::default(), Default::default(), Default::default(), 
            Default::default(), Default::default(), Default::default(),
            Default::default()
        )
    }
}

trait ObjectQuery<K, V> {
    fn query(&self, key: K) -> Option<V>;
}

impl ObjectQuery<VertexIndex, Vertex> for Object {
    fn query(&self, key: VertexIndex) -> Option<Vertex> {
        self.vertex_set.get(key as usize).map(|&x| x)
    }
}

impl ObjectQuery<TextureVertexIndex, TextureVertex> for Object {
    fn query(&self, key: TextureVertexIndex) -> Option<TextureVertex> {
        self.texture_vertex_set.get(key as usize).map(|&x| x)
    }
}

impl ObjectQuery<NormalVertexIndex, NormalVertex> for Object {
    fn query(&self, key: NormalVertexIndex) -> Option<NormalVertex> {
        self.normal_vertex_set.get(key as usize).map(|&x| x)
    }
}

impl ObjectQuery<ElementIndex, Element> for Object {
    fn query(&self, key: ElementIndex) -> Option<Element> {
        self.element_set.get(key as usize).map(|x| x.clone())
    }
}

impl ObjectQuery<GroupIndex, Group> for Object {
    fn query(&self, key: GroupIndex) -> Option<Group> {
        self.group_set.get(key as usize).map(|x| x.clone())
    }
}

impl ObjectQuery<SmoothingGroupIndex, SmoothingGroup> for Object {
    fn query(&self, key: SmoothingGroupIndex) -> Option<SmoothingGroup> {
        self.smoothing_group_set.get(key as usize).map(|x| x.clone())
    }
}

impl ObjectQuery<ShapeIndex, ShapeEntry> for Object {
    fn query(&self, key: ShapeIndex) -> Option<ShapeEntry> {
        self.shape_set.get(key as usize).map(|x| x.clone())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ObjectSet {
    objects: Vec<Object>,
}

impl ObjectSet {
    pub fn new(vec: Vec<Object>) -> ObjectSet {
        ObjectSet {
            objects: vec,
        }    
    }

    pub fn iter(&self) -> ObjectSetIter {
        ObjectSetIter {
            inner: self.objects.iter(),
        }
    }

    pub fn len(&self) -> usize { self.objects.len() }
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

pub struct ObjectBuilder {
    name: Option<String>,
    vertex_set: VertexSet,
    texture_vertex_set: Option<TextureVertexSet>,
    normal_vertex_set: Option<NormalVertexSet>,
    group_set: Option<GroupSet>,
    smoothing_group_set: Option<SmoothingGroupSet>,
    element_set: ElementSet,
    shape_set: Option<ShapeSet>,
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

    fn compose(&self, object: &Object) -> String {
        let mut string = String::from("Object {\n");
    
        macro_rules! compose_set {
            ($set:expr, $out:ident, $name:expr) => {
                $out += &format!("    {} set:\n", $name);
                if $set.is_empty() {
                    $out += &format!("        data: []\n");
                } else {
                    let length = $set.len();
                    $out += &format!("        data: [({}) ... ({})]\n", $set[0], $set[length-1]);
                }

                $out += &format!("        length: {}\n", $set.len());            
            }
        };

        string += &format!("    name: {}\n", object.name);

        compose_set!(object.vertex_set, string, "vertex");
        compose_set!(object.texture_vertex_set, string, "texture vertex");
        compose_set!(object.normal_vertex_set, string, "normal vertex");
        compose_set!(object.group_set, string, "group");
        compose_set!(object.smoothing_group_set, string, "smoothing group");
        compose_set!(object.element_set, string, "element");

        string += &format!("}}\n");

        string       
    }
}

impl ObjectCompositor for DisplayObjectCompositor {
    fn compose(&self, object: &Object) -> String {
        self.compose(object)
    } 
}

#[derive(Clone, Debug)]
enum GroupingStatement {
    G(Vec<Group>),
    S(SmoothingGroup),
}

#[derive(Clone, Debug)]
struct CompositorInstructions {
    instructions: BTreeMap<(u32, u32), Vec<GroupingStatement>>,
}

impl CompositorInstructions {
    fn new(instructions: BTreeMap<(u32, u32), Vec<GroupingStatement>>) -> Self {
        Self { instructions: instructions }
    }

    fn generate(object: &Object) -> CompositorInstructions {
        // Initialize the groups and smoothing groups.
        let mut current_entry = &object.shape_set[0];
        let mut min_element_index = 1;
        let mut intervals = vec![];
        // Find the intervals.
        for (max_element_index, shape_entry) in object.shape_set.iter().enumerate() {
            if shape_entry.groups != current_entry.groups || 
                shape_entry.smoothing_group != current_entry.smoothing_group {

                current_entry = shape_entry;
                intervals.push((min_element_index as u32, max_element_index as u32));
                min_element_index = max_element_index;
            }
        }

        // Precalculate the gaps.
        let mut instructions: BTreeMap<(u32, u32), Vec<GroupingStatement>> = BTreeMap::new();
        for i in 0..(intervals.len() - 1) {
            let mut statements = vec![];

            let gap_start_groups = &object.shape_set[intervals[i].1 as usize].groups;
            let gap_end_groups = &object.shape_set[intervals[i + 1].0 as usize].groups;
            if gap_start_groups != gap_end_groups {
                // The groups change across the partition boundary.
                let gap_start = *gap_start_groups.last().unwrap();
                let gap_end = *gap_end_groups.first().unwrap();
                for group_index in gap_start..gap_end {
                    statements.push(GroupingStatement::G(
                        vec![object.group_set[(group_index - 1) as usize].clone()]
                    ));
                }
            }

            let gap_start_sg = object.shape_set[intervals[i].1 as usize].smoothing_group;
            let gap_end_sg = object.shape_set[intervals[i + 1].0 as usize].smoothing_group;
            if gap_start_sg != gap_end_sg {
                // The smoothing groups change across the partition boundary.
                let gap_start = gap_start_sg;
                let gap_end = gap_end_sg;
                for sg_index in gap_start..gap_end {
                    statements.push(
                        GroupingStatement::S(object.smoothing_group_set[(sg_index - 1) as usize])
                    );
                }
            }

            instructions.insert(intervals[i], statements);
        }

        // Calculate the occupied groups and smoothing groups.

        Self::new(instructions)
    }
}

struct TextObjectCompositor { }

impl TextObjectCompositor {
    fn new() -> TextObjectCompositor { 
        TextObjectCompositor {} 
    }

    fn compose_object_name(&self, object: &Object) -> String {
        match object.name.as_ref() {
            "" => String::from(""),
            _  => format!("o  {} \n", object.name),
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
        format!("s  {} \n", smoothing_group)
    }

    fn compose_vertex_set(&self, object: &Object) -> String {
        object.vertex_set.iter().fold(String::new(), |acc, v| {
            match v.w == 1.0 {
                true  => acc + &format!("v  {}  {}  {} \n", v.x, v.y, v.z),
                false => acc + &format!("v  {}  {}  {}  {} \n", v.x, v.y, v.z, v.w),
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
            acc + &format!("{}\n", object.element_set[i as usize])
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

    fn compose(&self, object: &Object) -> String {
        let mut string = String::new();

        string += &self.compose_object_name(object);
        string += &self.compose_vertex_set(object);
        string += &format!("# {} vertices\n", object.vertex_set.len());
        string += &format!("\n");
        string += &self.compose_texture_vertex_set(object);
        string += &format!("# {} texture vertices\n", object.texture_vertex_set.len());
        string += &format!("\n");
        string += &self.compose_normal_vertex_set(object);
        string += &format!("# {} normal vertices\n", object.normal_vertex_set.len());
        string += &format!("\n");

        let group_instructions = self.get_group_instructions(object);
        for (interval, instructions) in group_instructions.iter() {
            string += &self.compose_instructions(&instructions);
            string += &self.compose_elements(object, *interval);
            string += &format!("# {} elements\n\n", (interval.1 - interval.0));
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

pub struct TextObjectSetCompositor { }

impl TextObjectSetCompositor {
    pub fn new() -> Self { Self {} }
}

impl Compositor for TextObjectSetCompositor {
    fn compose(&self, object_set: &ObjectSet) -> String {
        let compositor = TextObjectCompositor::new();
        
        let mut string = String::new();
        for (i, object_i) in object_set.iter().enumerate() {
            string += &format!("#### BEGIN Object {}\n", i);
            string += &compositor.compose(&object_i);
            string += &format!("#### END Object {}\n", i);
            string += &"\n";
        }
        eprintln!("TEXT GENERATED: \n\n{}\n\n", string);
        string
    }
}

