extern crate quickcheck;
extern crate wavefront;

use quickcheck::{Arbitrary, Gen};
use wavefront::obj::{
    Object, ObjectSet, ObjectBuilder,
    Vertex, TextureVertex, NormalVertex, Element, VTNIndex,
    Group, SmoothingGroup, ShapeEntry,
    TextObjectSetCompositor, Compositor,
    VertexSet, TextureVertexSet, NormalVertexSet, ElementSet, ShapeSet,
    GroupSet, SmoothingGroupSet,
};
use wavefront::obj::{Parser, ParseError};

use std::marker;
use std::fmt;
use std::str;


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

        assert_eq!(vertex_set.len(), len);
        vertex_set
    }

    fn gen_texture_vertex_set(&self, g: &mut G, len: usize) -> TextureVertexSet {
        let mut texture_vertex_set = vec![];
        for _ in 0..len {
            texture_vertex_set.push(self.gen_texture_vertex(g));
        }

        assert_eq!(texture_vertex_set.len(), len);
        texture_vertex_set
    }

    fn gen_normal_vertex_set(&self, g: &mut G, len: usize) -> NormalVertexSet {
        let mut normal_vertex_set = vec![];
        for _ in 0..len {
            normal_vertex_set.push(self.gen_normal_vertex(g));
        }

        assert_eq!(normal_vertex_set.len(), len);
        normal_vertex_set
    }

    fn gen_slices(&self, g: &mut G, 
        range: (usize, usize), count: usize) -> Vec<(usize, usize)> {

        assert!(range.0 < range.1);

        let mut indices = vec![range.0];
        for i in 0..(count - 1) {
            let lower = indices[i];
            indices.push(g.gen_range(lower, range.1));
        }
        indices.push(range.1);

        let mut slices = vec![];
        for i in 0..count {
            slices.push((indices[i], indices[i + 1]));
        }

        assert_eq!(slices.len(), count);
        slices
    }

    fn gen_vtn_index(&self, g: &mut G, 
        use_vt: bool, use_vn: bool, 
        v_count: u32, vt_count: u32, vn_count: u32) -> VTNIndex {

        let v = g.gen_range(1, v_count + 1);
        if use_vt && use_vn {
            let vt = g.gen_range(1, vt_count + 1);
            let vn = g.gen_range(1, vn_count + 1);

            VTNIndex::VTN(v, vt, vn)
        } else if use_vt {
            let vt = g.gen_range(1, vt_count + 1);

            VTNIndex::VT(v, vt)
        } else if use_vn {
            let vn = g.gen_range(1, vn_count + 1);

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
            let vtn_index1 = self.gen_vtn_index(g, true, true, v_count, vt_count, vn_count);
            let vtn_index2 = self.gen_vtn_index(g, true, true, v_count, vt_count, vn_count);
            let vtn_index3 = self.gen_vtn_index(g, true, true, v_count, vt_count, vn_count);

            element_set.push(Element::Face(vtn_index1, vtn_index2, vtn_index3));
        }

        assert_eq!(element_set.len(), element_count as usize);
        element_set
    }

    fn gen_group_set(&self, use_default: bool, count: usize) -> GroupSet {
        assert!(count > 0);

        let mut group_set = vec![];
        if use_default && (count == 1) {
            group_set.push(Default::default());
            return group_set;
        }

        for i in 0..count {
            let group_i = Group::new(&format!("Group{}", i));
            group_set.push(group_i);
        }

        assert!(count > 1);
        assert_eq!(group_set.len(), count);
        group_set
    }

    fn gen_smoothing_group_set(&self, count: usize) -> SmoothingGroupSet {
        assert!(count > 0);

        let mut smoothing_group_set = vec![];
        for i in 0..count {
            let smoothing_group_i = SmoothingGroup::new(i as u32);
            smoothing_group_set.push(smoothing_group_i);
        }

        assert_eq!(smoothing_group_set.len(), count);
        smoothing_group_set
    }

    fn gen_shape_set(&self, 
        element_set: &ElementSet, 
        group_slices: &[(usize, usize)], group_set: &[u32],
        smoothing_group_slices: &[(usize, usize)], smoothing_group_set: &[u32]) -> ShapeSet {
        
        assert!(group_slices.len() > 0);
        assert!(group_set.len() > 0);
        assert_eq!(group_slices.len(), group_set.len());
        assert!(smoothing_group_slices.len() > 0);
        assert!(smoothing_group_set.len() > 0);
        assert_eq!(smoothing_group_slices.len(), smoothing_group_set.len());

        let mut shape_set = vec![];
        for i in 0..group_slices.len() {
            for j in group_slices[i].0..group_slices[i].1 {
                let shape_entry = ShapeEntry::new(j as u32, &group_set[i..(i+1)], 1);
                shape_set.push(shape_entry);
            }
        }

        // The group slices should contain the entire range of elements
        // in the element set, and no more.
        assert_eq!(shape_set.len(), element_set.len());

        for i in 0..smoothing_group_slices.len() {
            for j in smoothing_group_slices[i].0..smoothing_group_slices[i].1 {
                shape_set[j].smoothing_group = smoothing_group_set[i];
            }
        }

        // The smoothing group iteration should not change the length
        // of the shape set.
        assert_eq!(shape_set.len(), element_set.len());
        shape_set
    }

    fn gen_group_count(&self, g: &mut G, use_default: bool) -> usize {
        if use_default { 
            1 
        } else { 
            g.gen_range(2, 6)
        }
    }

    fn gen_object_name(&self, index: usize) -> String {
        format!("Object{}", index)
    }

    fn gen_object(&self, g: &mut G, index: usize) -> Object {
        let object_name = self.gen_object_name(index);
        let len = g.gen_range(1, 10);
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
        let group_count = self.gen_group_count(g, use_g_default);
        let group_slices = self.gen_slices(g, (0, element_set.len()), group_count);
        let group_set = self.gen_group_set(use_g_default, group_count);

        let smoothing_group_count = g.gen_range(1, 6);
        let smoothing_group_slices = self.gen_slices(
            g, (0, element_set.len()), smoothing_group_count
        );
        let smoothing_group_set = self.gen_smoothing_group_set(smoothing_group_count);

        let shape_set = self.gen_shape_set(
            &element_set, 
            &group_slices,
            &(0..(group_set.len() as u32)).collect::<Vec<u32>>(), 
            &smoothing_group_slices, 
            &(0..(smoothing_group_set.len() as u32)).collect::<Vec<u32>>(), 
        );

        let mut builder = ObjectBuilder::new(vertex_set, element_set);
        builder
            .with_name(object_name)
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
        let object_count = if one_obj { 1 } else { g.gen_range(2, 10) };

        let mut objects = vec![];
        for index in 0..object_count {
            let object = self.gen_object(g, index);
            objects.push(object);
        }

        assert_eq!(objects.len(), object_count);
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
/*
type PropertyFunc = Box<Fn(&Machine) -> bool>;

struct Property {
    description: String,
    prop: PropertyFunc,
}

impl Property {
    fn new(description: String, prop: PropertyFunc) -> Property {
        Property { description: description, prop: prop }
    }

    fn description(&self) -> &str { 
        &self.description 
    }

    fn property(&self, machine: &Machine) -> bool { 
        (self.prop)(machine)
    }
}

struct Spec {
    props: Vec<Property>,
}

impl Spec {
    fn new(props: Vec<Property>) -> Spec {
        Spec { props: props }
    }

    fn satisfied_by(&self, machine: &Machine) -> bool {
        for prop in self.props {

        }

        true
    }
}

macro_rules! describe {
    ($desc:expr, $prop:expr) => {
        Property::new($desc, $prop)
    }
}

macro_rules! spec {
    ($ ( $prop:expr ), *) => {

    }
}

#[test]
fn prop_parser_satisfies_specification() {
    fn property(spec: Spec, machine: Machine) -> bool {
        spec.satisfied_by(machine)
    }
    quickcheck::quickcheck(property as fn(Machine) -> bool);
}
*/
#[test]
fn prop_parse_object_set_should_parse_object_names() {
    fn property(machine: Machine) -> bool {
        let result_set = machine.actual().parse().unwrap();
        let expected_set = machine.model().parse().unwrap();

        result_set.iter().zip(expected_set.iter()).all(|(result, expected)| {
            result.name == expected.name
        })
    }
    quickcheck::quickcheck(property as fn(Machine) -> bool);
}

#[test]
fn prop_parse_object_set_should_parse_vertices() {
    fn property(machine: Machine) -> bool {
        let result_set = machine.actual().parse().unwrap();
        let expected_set = machine.model().parse().unwrap();

        result_set.iter().zip(expected_set.iter()).all(|(result, expected)| {
            result.vertex_set == expected.vertex_set
        })
    }
    quickcheck::quickcheck(property as fn(Machine) -> bool);
}

#[test]
fn prop_parse_object_set_should_parse_texture_vertices() {
    fn property(machine: Machine) -> bool {
        let result_set = machine.actual().parse().unwrap();
        let expected_set = machine.model().parse().unwrap();

        result_set.iter().zip(expected_set.iter()).all(|(result, expected)| {
            result.texture_vertex_set == expected.texture_vertex_set
        })
    }
    quickcheck::quickcheck(property as fn(Machine) -> bool);
}

#[test]
fn prop_parse_object_set_should_parse_normal_vertices() {
    fn property(machine: Machine) -> bool {
        let result_set = machine.actual().parse().unwrap();
        let expected_set = machine.model().parse().unwrap();

        result_set.iter().zip(expected_set.iter()).all(|(result, expected)| {
            result.normal_vertex_set == expected.normal_vertex_set
        })
    }
    quickcheck::quickcheck(property as fn(Machine) -> bool);
}

#[test]
fn prop_parse_object_set_should_parse_groups() {
    fn property(machine: Machine) -> bool {
        let result_set = machine.actual().parse().unwrap();
        let expected_set = machine.model().parse().unwrap();

        result_set.iter().zip(expected_set.iter()).all(|(result, expected)| {
            eprintln!("{:?}\n", result.group_set);
            eprintln!("{:?}\n", expected.group_set);
            result.group_set == expected.group_set
        })
    }
    quickcheck::quickcheck(property as fn(Machine) -> bool);
}

#[test]
fn prop_parse_object_set_should_parse_smoothing_groups() {
    fn property(machine: Machine) -> bool {
        let result_set = machine.actual().parse().unwrap();
        let expected_set = machine.model().parse().unwrap();

        result_set.iter().zip(expected_set.iter()).all(|(result, expected)| {
            result.smoothing_group_set == expected.smoothing_group_set
        })
    }
    quickcheck::quickcheck(property as fn(Machine) -> bool);
}
/*
#[test]
fn prop_parse_object_set_should_parse_elements() {
    fn property(machine: Machine) -> bool {
        let result_set = machine.actual().parse().unwrap();
        let expected_set = machine.model().parse().unwrap();

        result_set.iter().zip(expected_set.iter()).all(|(result, expected)| {
            result.element_set == expected.element_set
        })
    }
    quickcheck::quickcheck(property as fn(Machine) -> bool);
}

#[test]
fn prop_parse_object_set_should_parse_shape_entries() {
    fn property(machine: Machine) -> bool {
        let result_set = machine.actual().parse().unwrap();
        let expected_set = machine.model().parse().unwrap();

        result_set.iter().zip(expected_set.iter()).all(|(result, expected)| {
            result.shape_set == expected.shape_set
        })
    }
    quickcheck::quickcheck(property as fn(Machine) -> bool);
}
*/
/*
#[test]
fn prop_parser_correctly_parses_valid_obj_files() {
    fn property(machine: Machine) -> bool {
        let result = machine.actual().parse();
        let expected = machine.model().parse();

        result == expected
    }
    quickcheck::quickcheck(property as fn(Machine) -> bool);
}
*/