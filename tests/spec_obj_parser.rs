extern crate quickcheck;
extern crate wavefront;

use quickcheck::{Arbitrary, Gen};
use wavefront::obj::{
    Object, ObjectSet, ObjectBuilder,
    Vertex, TextureVertex, NormalVertex, Element, VTNIndex,
    GroupName, SmoothingGroup, ShapeEntry,
    TextObjectSetCompositor, Compositor,
    VertexSet, TextureVertexSet, NormalVertexSet, ElementSet, ShapeSet,
    GroupSet, SmoothingGroupSet,
};
use wavefront::obj::{Parser, ParseError};

use std::marker;
use std::fmt;
use std::str;
use std::collections::HashMap;


#[derive(Clone, Debug)]
struct ParserModel {
    data: ObjectSet,
}

impl ParserModel {
    fn new(data: ObjectSet) -> ParserModel {
        ParserModel { data: data }
    }

    fn parse(&self) -> Result<ObjectSet, ParseError> {
        Ok(self.data.clone())
    }
}

impl fmt::Display for ParserModel {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let string = TextObjectSetCompositor::new().compose(&self.data);
        write!(f, "{}", string)
    }
}

struct ObjectSetGen<G> { 
    _marker: marker::PhantomData<G>,
}

impl<G> ObjectSetGen<G> where G: Gen {
    fn new() -> Self { 
        ObjectSetGen { 
            _marker: marker::PhantomData 
        } 
    }

    fn gen_vertex(&self, g: &mut G, use_w: bool) -> Vertex {
        let x = Arbitrary::arbitrary(g);
        let y = Arbitrary::arbitrary(g);
        let z = Arbitrary::arbitrary(g);
        let w = if use_w { Arbitrary::arbitrary(g) } else { 1.0 };

        Vertex::new(x, y, z, w)
    }

    fn gen_texture_vertex(&self, g: &mut G) -> TextureVertex {
        let u = Arbitrary::arbitrary(g);
        let v = Arbitrary::arbitrary(g);
        let w = Arbitrary::arbitrary(g);

        TextureVertex::new(u, v, w)
    }

    fn gen_normal_vertex(&self, g: &mut G) -> NormalVertex {
        let i = Arbitrary::arbitrary(g);
        let j = Arbitrary::arbitrary(g);
        let k = Arbitrary::arbitrary(g);

        NormalVertex::new(i, j, k)
    }

    fn gen_vertex_set(&self, g: &mut G, len: usize) -> VertexSet {
        let mut vertex_set = vec![];
        for _ in 0..len {
            vertex_set.push(self.gen_vertex(g, true));
        }

        vertex_set
    }

    fn gen_texture_vertex_set(&self, g: &mut G, len: usize) -> TextureVertexSet {
        let mut texture_vertex_set = vec![];
        for _ in 0..len {
            texture_vertex_set.push(self.gen_texture_vertex(g));
        }

        texture_vertex_set
    }

    fn gen_normal_vertex_set(&self, g: &mut G, len: usize) -> NormalVertexSet {
        let mut normal_vertex_set = vec![];
        for _ in 0..len {
            normal_vertex_set.push(self.gen_normal_vertex(g));
        }
        normal_vertex_set
    }

    fn gen_slices(&self, g: &mut G, 
        range: (usize, usize), count: usize) -> Vec<(usize, usize)> {

        let mut indices = vec![g.gen_range(range.0, range.1)];
        for i in 1..count {
            let lower = indices[i - 1];
            indices.push(g.gen_range(lower, range.1));
        }

        let mut slices = vec![];
        for i in 0..count-1 {
            slices.push((indices[i], indices[i] + 1));
        }

        slices
    }

    fn gen_vtn_index(&self, g: &mut G, 
        use_vt: bool, use_vn: bool, range: (u32, u32, u32)) -> VTNIndex {

        let v = g.gen_range(1, range.0);
        if use_vt && use_vn {
            let vt = g.gen_range(1, range.1);
            let vn = g.gen_range(1, range.2);

            VTNIndex::VTN(v, vt, vn)
        } else if use_vt {
            let vt = g.gen_range(1, range.1);

            VTNIndex::VT(v, vt)
        } else if use_vn {
            let vn = g.gen_range(1, range.2);

            VTNIndex::VN(v, vn)
        } else {
            VTNIndex::V(v)
        }
    }

    fn gen_element_set(
        &self, g: &mut G, element_count: u32,
        v_count: u32, vt_count: u32, vn_count: u32) -> ElementSet {
        
        let mut element_set = vec![];
        for _ in 0..element_count {
            let vtn_index1 = self.gen_vtn_index(g, true, true, (v_count, vt_count, vn_count));
            let vtn_index2 = self.gen_vtn_index(g, true, true, (v_count, vt_count, vn_count));
            let vtn_index3 = self.gen_vtn_index(g, true, true, (v_count, vt_count, vn_count));

            element_set.push(Element::Face(vtn_index1, vtn_index2, vtn_index3));
        }

        element_set

    }

    fn gen_group_set(&self, g: &mut G, use_default: bool, count: usize) -> GroupSet {
        let mut group_set = vec![];
        if use_default && count == 1 {
            group_set.push(Default::default());
            return group_set;
        }

        for i in 0..count {
            let group_i = GroupName::new(&format!("Group{}", i));
            group_set.push(group_i);
        }

        group_set
    }

    fn gen_smoothing_group_set(&self, g: &mut G, count: usize) -> SmoothingGroupSet {
        let mut smoothing_group_set = vec![];
        for i in 0..count {
            let smoothing_group_i = SmoothingGroup::new(i as u32);
            smoothing_group_set.push(smoothing_group_i);
        }

        smoothing_group_set
    }

    fn gen_shape_set(&self, 
        elements: &ElementSet, 
        group_slices: &[(usize, usize)], group_set: &[u32],
        smoothing_group_slices: &[(usize, usize)], smoothing_group_set: &[u32]
    ) -> ShapeSet {
        
        let mut shape_set = vec![];
        for i in 1..(group_slices.len() + 1) {
            for j in group_slices[i].0..group_slices[i].1 {
                let shape_entry = ShapeEntry::new(j as u32, &group_set[i..(i+1)], &vec![]);
                shape_set.push(shape_entry);
            }
        }

        for i in 0..smoothing_group_slices.len() {
            for j in smoothing_group_slices[i].0..smoothing_group_slices[i].1 {
                shape_set[j].smoothing_groups = vec![smoothing_group_set[i].clone()];
            }
        }

        shape_set
    }

    fn gen_object(&self, g: &mut G) -> Object {
        let len = g.gen_range(1, 100000);
        let vertex_set = self.gen_vertex_set(g, len);
        let texture_vertex_set = self.gen_texture_vertex_set(g, len);
        let normal_vertex_set = self.gen_normal_vertex_set(g, len);

        let element_count = g.gen_range(len, 2*len);
        let element_set = self.gen_element_set(
            g,  element_count as u32,
            vertex_set.len() as u32, texture_vertex_set.len() as u32, 
            normal_vertex_set.len() as u32,
        );

        let use_g_default: bool = Arbitrary::arbitrary(g);
        let group_count = if use_g_default { 1 } else { g.gen_range(2, 6) };
        let group_slices = self.gen_slices(g, (0, element_set.len()), group_count);
        let group_set = self.gen_group_set(g, use_g_default, group_count);

        let smoothing_group_count = g.gen_range(1, 6);
        let smoothing_group_slices = self.gen_slices(
            g, (0, element_set.len()), smoothing_group_count
        );
        let smoothing_group_set = self.gen_smoothing_group_set(g, smoothing_group_count);

        let shape_set = self.gen_shape_set(
            &element_set, 
            &group_slices,
            &(0..(group_set.len() as u32)).collect::<Vec<u32>>(), 
            &smoothing_group_slices, 
            &(0..(smoothing_group_set.len() as u32)).collect::<Vec<u32>>(), 
        );

        let mut builder = ObjectBuilder::new(vertex_set, element_set);
        builder
            .with_texture_vertex_set(texture_vertex_set)
            .with_normal_vertex_set(normal_vertex_set)
            .with_group_set(group_set)
            .with_smoothing_group_set(smoothing_group_set)
            .with_shape_set(shape_set);
        
        builder.build()
    }

    fn generate(&self, g: &mut G) -> ObjectSet {
        // We want one object sets to appear frequently since that is the most
        // commonly encountered case in the wild.
        let one_obj: bool = Arbitrary::arbitrary(g);
        let object_count = if one_obj { 1 } else { g.gen_range(2, 20) };

        let mut objects = vec![];
        for _ in 0..object_count {  
            let object = self.gen_object(g);
            objects.push(object);
        }

        ObjectSet::new(objects)
    }
}

impl Arbitrary for ParserModel {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        ParserModel::new(ObjectSetGen::new().generate(g))
    }
}

#[derive(Clone, Debug)]
struct Machine { 
    model: ParserModel, 
    text: String,
}

impl Machine {
    fn new(model: ParserModel, text: String) -> Machine {
        Machine { model: model, text: text }
    }

    fn actual(&self) -> Parser<str::Chars> {
        let input = self.text.chars();
        Parser::new(input)
    }

    fn model(&self) -> &ParserModel {
        &self.model
    }
}

impl Arbitrary for Machine {
    fn arbitrary<G: Gen>(g: &mut G) -> Machine {
        let model: ParserModel = Arbitrary::arbitrary(g);
        let text = TextObjectSetCompositor::new().compose(&model.data);
        Machine::new(model, text)
    }
}


#[test]
fn prop_parser_correctly_parses_valid_obj_files() {
    fn property(machine: Machine) -> bool {
        let result = machine.actual().parse();
        let expected = machine.model().parse();

        result == expected
    }
    quickcheck::quickcheck(property as fn(Machine) -> bool);
}
/*
#[test]
fn prop_parse_object_set_should_parse_objects() {
    fn property(machine: Machine) -> bool {
        let result = machine.actual().parse();
        let expected = machine.model().parse();

        unimplemented!();
    }
    quickcheck::quickcheck(property as fn(Machine) -> bool);
}

#[test]
fn prop_parse_object_set_should_parse_object_names() {
    fn property(machine: Machine) -> bool {
        let result = machine.actual().parse();
        let expected = machine.model().parse();

        unimplemented!();
    }
    quickcheck::quickcheck(property as fn(Machine) -> bool);
}

#[test]
fn prop_parse_object_set_should_parse_vertices() {
    fn property(machine: Machine) -> bool {
        let result = machine.actual().parse();
        let expected = machine.model().parse();

        unimplemented!();
    }
    quickcheck::quickcheck(property as fn(Machine) -> bool);
}

#[test]
fn prop_parse_object_set_should_parse_texture_vertices() {
    fn property(machine: Machine) -> bool {
        let result = machine.actual().parse();
        let expected = machine.model().parse();

        unimplemented!();
    }
    quickcheck::quickcheck(property as fn(Machine) -> bool);
}

#[test]
fn prop_parse_object_set_should_parse_normal_vertices() {
    fn property(machine: Machine) -> bool {
        let result = machine.actual().parse();
        let expected = machine.model().parse();

        unimplemented!();
    }
    quickcheck::quickcheck(property as fn(Machine) -> bool);
}

#[test]
fn prop_parse_object_set_should_parse_groups() {
    fn property(machine: Machine) -> bool {
        let result = machine.actual().parse();
        let expected = machine.model().parse();

        unimplemented!();
    }
    quickcheck::quickcheck(property as fn(Machine) -> bool);
}

#[test]
fn prop_parse_object_set_should_parse_smoothing_groups() {
    fn property(machine: Machine) -> bool {
        let result = machine.actual().parse();
        let expected = machine.model().parse();

        unimplemented!();
    }
    quickcheck::quickcheck(property as fn(Machine) -> bool);
}

#[test]
fn prop_parse_object_set_should_parse_elements() {
    fn property(machine: Machine) -> bool {
        let result = machine.actual().parse();
        let expected = machine.model().parse();

        unimplemented!();
    }
    quickcheck::quickcheck(property as fn(Machine) -> bool);
}

#[test]
fn prop_parse_object_set_should_parse_shape_entries() {
    fn property(machine: Machine) -> bool {
        let result = machine.actual().parse();
        let expected = machine.model().parse();

        unimplemented!();
    }
    quickcheck::quickcheck(property as fn(Machine) -> bool);
}
*/
