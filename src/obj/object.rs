use std::default::Default;
use std::slice;


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
}

impl Default for SmoothingGroup {
    fn default() -> SmoothingGroup { SmoothingGroup::new(0) }
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
                let vertex = match self.vertex_set.get(v_index as usize) {
                    Some(val) => val,
                    None => return None,
                };

                Some(VTNTriple::V(vertex))
            },
            VTNIndex::VT(v_index, vt_index) => { 
                let vertex = match self.vertex_set.get(v_index as usize) {
                    Some(val) => val,
                    None => return None,
                };
                let texture_vertex = match self.texture_vertex_set.get(vt_index as usize) {
                    Some(val) => val,
                    None => return None,
                };

                Some(VTNTriple::VT(vertex, texture_vertex))
            },
            VTNIndex::VN(v_index, vn_index) => {
                let vertex = match self.vertex_set.get(v_index as usize) {
                    Some(val) => val,
                    None => return None,
                };
                let normal_vertex = match self.normal_vertex_set.get(vn_index as usize) {
                    Some(val) => val,
                    None => return None,
                };

                Some(VTNTriple::VN(vertex, normal_vertex))
            },
            VTNIndex::VTN(v_index, vt_index, vn_index) => {
                let vertex = match self.vertex_set.get(v_index as usize) {
                    Some(val) => val,
                    None => return None,
                };
                let texture_vertex = match self.texture_vertex_set.get(vt_index as usize) {
                    Some(val) => val,
                    None => return None,
                };
                let normal_vertex = match self.normal_vertex_set.get(vn_index as usize) {
                    Some(val) => val,
                    None => return None,
                };

                Some(VTNTriple::VTN(vertex, texture_vertex, normal_vertex))
            },
        }
    }

    fn get_shape(&self, entry: &ShapeEntry) -> Option<&Shape> {
        unimplemented!();
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

