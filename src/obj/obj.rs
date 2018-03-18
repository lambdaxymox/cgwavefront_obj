use std::slice;
use std::ops;
use std::convert;
use obj::table::MeshTable;


#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct TextureVertex {
    pub u: f32,
    pub v: f32,
    pub w: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct NormalVertex {
    pub i: f32,
    pub j: f32,
    pub k: f32,
}

#[derive(Clone, Debug)]
enum Element {
    Point(VTNIndex),
    Line(VTNIndex, VTNIndex),
    Face(VTNIndex, VTNIndex, VTNIndex),
}

#[derive(Clone, Debug)]
struct Shape {
    element: ElementIndex,
    groups: Vec<GroupIndex>,
    smoothing_groups: Vec<SmoothingGroupIndex>,
}

type GroupName = String;
type SmoothingGroupName = String;

type ElementIndex = usize;
type VertexIndex = usize;
type TextureVertexIndex = usize;
type NormalVertexIndex = usize;
type GroupIndex = usize;
type ShapeIndex = usize;
type SmoothingGroupIndex = usize;


type VTNIndex = (VertexIndex, Option<TextureVertexIndex>, Option<NormalVertexIndex>);

type VertexSet = MeshTable<Vertex>;
type TextureVertexSet = MeshTable<TextureVertex>;
type NormalVertexSet = MeshTable<NormalVertex>;
type ElementSet = MeshTable<Element>;
type ShapeSet = MeshTable<Shape>;
type GroupSet = MeshTable<GroupName>;
type SmoothingGroupSet = MeshTable<SmoothingGroupName>;
/*
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
}


pub struct ObjectSet {
    objects: Vec<Object>,
}
*/