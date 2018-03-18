#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct NormalVertex {
    pub i: f32,
    pub j: f32,
    pub k: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct TextureVertex {
    pub u: f32,
    pub v: f32,
    pub w: f32,
}

type GroupName = String;
type SmoothingGroupName = String;

type VertexIndex = u32;
type TextureIndex = u32;
type NormalIndex = u32;
type GroupNameIndex = u32;
type ShapeIndex = u32;
type ElementIndex = u32;
type SmoothingGroupIndex = u32;

struct VertexSet(Vec<Vertex>);
struct TextureVertexSet(Vec<TextureVertex>);
struct NormalVertexSet(Vec<NormalVertex>);
struct GroupSet(Vec<GroupName>);
struct ElementSet(Vec<Element>);
struct SmoothingGroupSet(Vec<SmoothingGroupName>);
struct ShapeSet(Vec<Shape>);

enum Element {
    Point(VertexIndex),
    Line(VertexIndex, VertexIndex),
    Face(VertexIndex, VertexIndex, VertexIndex),
}

struct Shape {
    element: Element,
    groups: Vec<GroupNameIndex>,
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
