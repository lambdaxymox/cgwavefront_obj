use obj::table::ObjectTable;
use std::default::Default;


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

#[derive(Clone, Debug, PartialEq)]
pub struct Shape {
    element: Element,
    groups: Vec<GroupName>,
    smoothing_groups: Vec<SmoothingGroupName>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GroupName(String);

impl GroupName {
    pub fn new(name: &str) -> GroupName { GroupName(String::from(name)) }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SmoothingGroupName(u32);

impl SmoothingGroupName {
    pub fn new(name: u32) -> SmoothingGroupName { 
        SmoothingGroupName(name)
    }
}

type ElementIndex = u32;
type VertexIndex = u32;
type TextureVertexIndex = u32;
type NormalVertexIndex = u32;
type GroupIndex = u32;
type ShapeIndex = u32;
type SmoothingGroupIndex = u32;

pub type VertexSet = ObjectTable<Vertex>;
pub type TextureVertexSet = ObjectTable<TextureVertex>;
pub type NormalVertexSet = ObjectTable<NormalVertex>;
pub type ElementSet = ObjectTable<Element>;
pub type ShapeSet = ObjectTable<ShapeEntry>;
pub type GroupSet = ObjectTable<GroupName>;
pub type SmoothingGroupSet = ObjectTable<SmoothingGroupName>;

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
    name: String,
    vertex_set: VertexSet,
    texture_vertex_set: TextureVertexSet,
    normal_vertex_set: NormalVertexSet,
    group_set: GroupSet,
    smoothing_group_set: SmoothingGroupSet,
    element_set: ElementSet,
    shape_set: ShapeSet,
}

impl Object {
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

impl ObjectQuery<SmoothingGroupIndex, SmoothingGroupName> for Object {
    fn query(&self, key: SmoothingGroupIndex) -> Option<SmoothingGroupName> {
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
            vertex_set: ObjectTable::from(vertex_set),
            texture_vertex_set: None,
            normal_vertex_set: None,
            group_set: None,
            smoothing_group_set: None,
            element_set: ObjectTable::from(element_set),
            shape_set: None,
        }
    }

    pub fn with_name(&mut self, name: String) -> &mut Self {
        self.name = Some(name);
        self
    }

    pub fn with_texture_vertex_set(&mut self, texture_vertex_set: Vec<TextureVertex>) -> &mut Self {
        self.texture_vertex_set = Some(ObjectTable::from(texture_vertex_set));
        self
    }

    pub fn with_normal_vertex_set(&mut self, normal_vertex_set: Vec<NormalVertex>) -> &mut Self {
        self.normal_vertex_set = Some(ObjectTable::from(normal_vertex_set));
        self
    }

    pub fn with_group_set(&mut self, group_set: Vec<GroupName>) -> &mut Self {
        self.group_set = Some(ObjectTable::from(group_set));
        self
    }

    pub fn with_smoothing_group_set(&mut self, smoothing_group_set: Vec<SmoothingGroupName>) -> &mut Self {
        self.smoothing_group_set = Some(ObjectTable::from(smoothing_group_set));
        self
    }

    pub fn with_shape_set(&mut self, shape_set: Vec<ShapeEntry>) -> &mut Self {
        self.shape_set = Some(ObjectTable::from(shape_set));
        self
    }

    pub fn build(self) -> Object {
        Object {
            name: self.name.unwrap_or(String::from("")),
            vertex_set: self.vertex_set,
            texture_vertex_set: self.texture_vertex_set.unwrap_or(Default::default()),
            normal_vertex_set: self.normal_vertex_set.unwrap_or(Default::default()),
            group_set: self.group_set.unwrap_or(Default::default()),
            smoothing_group_set: self.smoothing_group_set.unwrap_or(Default::default()),
            element_set: self.element_set,
            shape_set: self.shape_set.unwrap_or(Default::default()),
        }
    }
}

