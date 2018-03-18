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

type VertexIndex = u32;
struct VertexSet(Vec<Vertex>);

type TextureIndex = u32;
struct TextureVertexSet(Vec<TextureVertex>);

type NormalVertexIndex = u32;
struct NormalVertexSet(Vec<NormalVertex>);

type GroupName = String;
type GroupIndex = u32;
struct GroupSet(Vec<GroupName>);

type ShapeIndex = u32;
struct ShapeSet(Vec<Shape>);

type ElementIndex = u32;
struct ElementSet(Vec<Element>);

type SmoothingGroupName = String;
type SmoothingGroupIndex = u32;
struct SmoothingGroupSet(Vec<SmoothingGroupName>);


enum Element {
    Point(VertexIndex),
    Line(VertexIndex, VertexIndex),
    Face(VertexIndex, VertexIndex, VertexIndex),
}

struct Shape {
    element: ElementIndex,
    groups: Vec<GroupIndex>,
    smoothing_groups: Vec<SmoothingGroupIndex>,
}

pub struct Object {
    name: String,
    vertex_set: Vec<Vertex>,
    texture_vertex_set: Vec<TextureVertex>,
    normal_vertex_set: Vec<NormalVertex>,
    group_set: Vec<GroupName>,
    smoothing_group_set: Vec<SmoothingGroupName>,
    element_set: Vec<Element>,
    shape_set: Vec<Shape>,
}

pub struct ObjectSet {
    objects: Vec<Object>,
}
