use obj::table::ObjectTable;


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

#[derive(Clone, Debug)]
struct ShapeEntry {
    element: ElementIndex,
    groups: Vec<GroupIndex>,
    smoothing_groups: Vec<SmoothingGroupIndex>,
}

#[derive(Clone, Debug)]
struct Shape {
    element: Element,
    groups: Vec<GroupName>,
    smoothing_groups: Vec<SmoothingGroupName>,
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct GroupName(String);

#[derive(Clone, Eq, PartialEq, Debug)]
struct SmoothingGroupName(String);

type ElementIndex = usize;
type VertexIndex = usize;
type TextureVertexIndex = usize;
type NormalVertexIndex = usize;
type GroupIndex = usize;
type ShapeIndex = usize;
type SmoothingGroupIndex = usize;

type VertexSet = ObjectTable<Vertex>;
type TextureVertexSet = ObjectTable<TextureVertex>;
type NormalVertexSet = ObjectTable<NormalVertex>;
type ElementSet = ObjectTable<Element>;
type ShapeSet = ObjectTable<ShapeEntry>;
type GroupSet = ObjectTable<GroupName>;
type SmoothingGroupSet = ObjectTable<SmoothingGroupName>;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VTNIndex(VertexIndex, Option<TextureVertexIndex>, Option<NormalVertexIndex>);

impl VTNIndex {
    pub fn new(
        vertex_index: VertexIndex, 
        tv_index: Option<TextureVertexIndex>, nv_index: Option<NormalVertexIndex>) -> VTNIndex {

        VTNIndex(vertex_index, tv_index, nv_index)
    }
}

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

    fn get_vtn_triple(&self, index: VTNIndex) -> Option<(&Vertex, Option<&TextureVertex>, Option<&NormalVertex>)> {
        let vertex = match self.vertex_set.get(index.0) {
            Some(v) => v,
            None => { return None; }
        };
        let texture_vertex = match index.1 {
            Some(tv_idx) => {
                match self.texture_vertex_set.get(tv_idx) {
                    Some(tv) => Some(tv),
                    None => { return None; }
                } 
            }
            None => None
        };
        let normal_vertex = match index.2 {
            Some(nv_idx) => {
                match self.normal_vertex_set.get(nv_idx) {
                    Some(nv) => Some(nv),
                    None => { return None; }
                }
            }
            None => None
        };

        Some((vertex, texture_vertex, normal_vertex))
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
         self.vertex_set.get(key).map(|&x| x)
    }
}

impl ObjectQuery<TextureVertexIndex, TextureVertex> for Object {
    fn query(&self, key: TextureVertexIndex) -> Option<TextureVertex> {
         self.texture_vertex_set.get(key).map(|&x| x)
    }
}

impl ObjectQuery<NormalVertexIndex, NormalVertex> for Object {
    fn query(&self, key: NormalVertexIndex) -> Option<NormalVertex> {
         self.normal_vertex_set.get(key).map(|&x| x)
    }
}

impl ObjectQuery<ElementIndex, Element> for Object {
    fn query(&self, key: ElementIndex) -> Option<Element> {
         self.element_set.get(key).map(|x| x.clone())
    }
}

impl ObjectQuery<GroupIndex, GroupName> for Object {
    fn query(&self, key: GroupIndex) -> Option<GroupName> {
        self.group_set.get(key).map(|x| x.clone())
    }
}

impl ObjectQuery<SmoothingGroupIndex, SmoothingGroupName> for Object {
    fn query(&self, key: SmoothingGroupIndex) -> Option<SmoothingGroupName> {
        self.smoothing_group_set.get(key).map(|x| x.clone())
    }
}

impl ObjectQuery<ShapeIndex, ShapeEntry> for Object {
    fn query(&self, key: ShapeIndex) -> Option<ShapeEntry> {
        self.shape_set.get(key).map(|x| x.clone())
    }
}

pub struct ObjectSet {
    objects: Vec<Object>,
}

impl ObjectSet {
    fn new() -> ObjectSet {
        ObjectSet {
            objects: Vec::new(),
        }    
    }
}

