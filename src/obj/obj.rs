use std::slice;
use std::ops;
use std::convert;


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


macro_rules! set_impl {
    ($set_item:ty, $set_type:ident, $set_iter:ident, $set_idx:ident) => {
        type $set_idx = u32;

        struct $set_type(Vec<$set_item>);

        impl $set_type {
            fn new() -> $set_type {
                $set_type(Vec::new())
            }

            fn iter(&self) -> $set_iter {
                $set_iter {
                    inner: self.0.iter(),
                }
            }

            fn get(&self, index: $set_idx) -> Option<&$set_item> {
                self.0.get(index as usize)
            }

            fn as_slice(&self) -> &[$set_item] {
                self.0.as_slice()
            }
        }

        struct $set_iter<'a> {
            inner: slice::Iter<'a, $set_item>,
        }

        impl<'a> Iterator for $set_iter<'a> {
            type Item = &'a $set_item;

            #[inline]
            fn next(&mut self) -> Option<Self::Item> {
                self.inner.next()
            }
        }

        impl ops::Index<$set_idx> for $set_type {
            type Output = $set_item;

            #[inline]
            fn index(&self, index: $set_idx) -> &Self::Output {
                &self.0[index as usize]
            }
        }

        impl convert::AsRef<[$set_item]> for $set_type {
            fn as_ref(&self) -> &[$set_item] {
                self.0.as_ref()
            }
        }
    }
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

set_impl!(Vertex, VertexSet, VertexSetIter, VertexIndex);
set_impl!(TextureVertex, TextureVertexSet, TextureVertexSetIter, TextureVertexIndex);
set_impl!(NormalVertex, NormalVertexSet, NormalVertexSetIter, NormalVertexIndex);
set_impl!(GroupName, GroupSet, GroupSetIter, GroupIndex);
set_impl!(Shape, ShapeSet, ShapeSetIter, ShapeIndex);
set_impl!(Element, ElementSet, ElementSetIter, ElementIndex);
set_impl!(SmoothingGroupName, SmoothingGroupSet, SmoothingGroupSetIter, SmoothingGroupIndex);

type VTNIndex = (VertexIndex, Option<TextureVertexIndex>, Option<NormalVertexIndex>);


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
