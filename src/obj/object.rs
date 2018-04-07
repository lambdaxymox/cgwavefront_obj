use std::default::Default;
use std::slice;
use std::fmt;
use std::ops;


#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TextureVertex {
    pub u: f32,
    pub v: f32,
    pub w: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NormalVertex {
    pub i: f32,
    pub j: f32,
    pub k: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Element {
    Point(VTNIndex),
    Line(VTNIndex, VTNIndex),
    Face(VTNIndex, VTNIndex, VTNIndex),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ShapeEntry {
    pub element: ElementIndex,
    pub groups: Vec<GroupIndex>,
    pub smoothing_groups: Vec<SmoothingGroupIndex>,
}

impl ShapeEntry {
    pub fn new(
        element: ElementIndex, 
        groups: &[GroupIndex], smoothing_groups: &[SmoothingGroupIndex]
    ) -> ShapeEntry {
        ShapeEntry {
            element: element,
            groups: Vec::from(groups),
            smoothing_groups: Vec::from(smoothing_groups),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Shape {
    element: Element,
    groups: Vec<GroupName>,
    smoothing_groups: Vec<SmoothingGroup>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GroupName(String);

impl fmt::Display for GroupName {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.0)
    }
}

impl GroupName {
    pub fn new(name: &str) -> GroupName { GroupName(String::from(name)) }
}

impl Default for GroupName {
    fn default() -> GroupName { GroupName::new("default") }
}

#[derive(Clone, Debug, PartialEq, Eq)]
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

type ElementIndex = u32;
type VertexIndex = u32;
type TextureVertexIndex = u32;
type NormalVertexIndex = u32;
type GroupIndex = u32;
type ShapeIndex = u32;
type SmoothingGroupIndex = u32;

pub type VertexSet = Vec<Vertex>;
pub type TextureVertexSet = Vec<TextureVertex>;
pub type NormalVertexSet = Vec<NormalVertex>;
pub type ElementSet = Vec<Element>;
pub type ShapeSet = Vec<ShapeEntry>;
pub type GroupSet = Vec<GroupName>;
pub type SmoothingGroupSet = Vec<SmoothingGroup>;

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
    pub vertex_set: VertexSet,
    pub texture_vertex_set: TextureVertexSet,
    pub normal_vertex_set: NormalVertexSet,
    pub group_set: GroupSet,
    pub smoothing_group_set: SmoothingGroupSet,
    pub element_set: ElementSet,
    pub shape_set: ShapeSet,
}

impl Object {
    pub fn new() -> Object {
        Object {
            name: String::from(""),
            vertex_set: Default::default(),
            texture_vertex_set: Default::default(),
            normal_vertex_set: Default::default(),
            group_set: Default::default(),
            smoothing_group_set: Default::default(),
            element_set: Default::default(),
            shape_set: Default::default(),
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

    fn get_shape(&self, entry: &ShapeEntry) -> Option<&Shape> {
        unimplemented!();
    }
}

impl fmt::Debug for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let mut string = String::from("Object {\n");
    
        string.push_str(&format!("    name: {:?}\n", self.name));
        if self.vertex_set.is_empty() {
            string.push_str(&format!("    vertex set: []\n"));
        } else {
            let length = self.vertex_set.len();
            string.push_str(&format!("    vertex set: [{:?} ... {:?}]\n", 
                self.vertex_set[0], self.vertex_set[length - 1]
            ));
        }
        string.push_str(&format!(
            "    vertex set length: {:?}\n", self.vertex_set.len()
        ));
        
        if self.texture_vertex_set.is_empty() {
            string.push_str(&format!("    texture vertex set: []\n"));
        } else {
            let length = self.texture_vertex_set.len();
            string.push_str(&format!("    texture vertex set: [{:?} ... {:?}]\n", 
                self.texture_vertex_set[0], self.texture_vertex_set[length - 1]
            ));
        }
        string.push_str(&format!(
            "    texture vertex set length: {:?}\n", 
            self.texture_vertex_set.len()
        ));

        if self.normal_vertex_set.is_empty() {
            string.push_str(&format!("    normal vertex set: []\n"));
        } else {
            let length = self.normal_vertex_set.len();
            string.push_str(&format!("    normal vertex set: [{:?} ... {:?}]\n", 
                self.normal_vertex_set[0], self.normal_vertex_set[length - 1]
            ));
        }
        string.push_str(&format!(
            "    normal vertex set length: {:?}\n", 
            self.normal_vertex_set.len()
        ));

        if self.group_set.is_empty() {
            string.push_str(&format!("    group set: []\n"));
        } else {
            let length = self.group_set.len();
            string.push_str(&format!("    group set: [{:?} ... {:?}]\n", 
                self.group_set[0], self.group_set[length - 1]
            ));
        }
        string.push_str(&format!(
            "    group set length: {:?}\n", 
            self.group_set.len()
        ));

        if self.smoothing_group_set.is_empty() {
            string.push_str(&format!("    smoothing group set: []\n"));
        } else {
            let length = self.smoothing_group_set.len();
            string.push_str(&format!("    smoothing group set: [{:?} ... {:?}]\n", 
                self.smoothing_group_set[0], self.smoothing_group_set[length - 1]
            ));
        }
        string.push_str(&format!(
            "    smoothing group set length: {:?}\n", 
            self.smoothing_group_set.len()
        ));

        if self.element_set.is_empty() {
            string.push_str(&format!("    smoothing group set: []\n"));
        } else {
            let length = self.element_set.len();
            string.push_str(&format!("    element set: [{:?} ... {:?}]\n", 
                self.element_set[0], self.element_set[length - 1]
            ));
        }
        string.push_str(&format!(
            "    element set length: {:?}\n", self.element_set.len()
        ));

        string.push_str(&format!("}}\n"));
        write!(f, "{:?}", string)
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

impl ObjectQuery<GroupIndex, GroupName> for Object {
    fn query(&self, key: GroupIndex) -> Option<GroupName> {
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

    pub fn with_group_set(&mut self, group_set: Vec<GroupName>) -> &mut Self {
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

