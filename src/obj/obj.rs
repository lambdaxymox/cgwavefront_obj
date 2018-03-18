
pub struct Mesh {
    name: String,
    vertices: Vec<Vertex>,
    texture_vertices: Vec<TextureVertex>,
    normals: Vec<NormalVector>,
}

#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct NormalVector {
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

type VIndex = usize;

enum Element {
    Point(VIndex),
    Line(VIndex, VIndex),
    Face(VIndex, VIndex, VIndex),
}

type GroupName = String;

struct Shape {
    element: Element,
    groups: Vec<GroupName>,
    smoothing_groups: Vec<u32>,
}