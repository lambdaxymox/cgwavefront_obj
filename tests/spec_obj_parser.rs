extern crate quickcheck;
extern crate rand;
extern crate wavefront_obj;


use quickcheck::{
    Arbitrary, 
    Gen,
};
use wavefront_obj::{
    Object, 
    ObjectSet, 
    ObjectBuilder,
    Vertex,
    TextureVertex, 
    NormalVertex, 
    Element, 
    VTNIndex,
    Group, 
    SmoothingGroup, 
    ShapeEntry,
    TextObjectSetCompositor, 
    Compositor,
};
use wavefront_obj::{
    Parser, 
    ParseError,
};
use rand::{
    Rng, 
    RngCore
};
use std::marker;
use std::fmt;


/// The model of the Wavefront OBJ format parser for model based testing.
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

struct ObjectGenerator<G> {
    _marker: marker::PhantomData<G>,
}

impl<G> ObjectGenerator<G> where G: RngCore {
    fn new() -> Self { 
        Self { _marker: marker::PhantomData } 
    }

    fn gen_vertex(&self, g: &mut G, use_w: bool) -> Vertex {
        let x = g.gen::<f64>();
        let y = g.gen::<f64>();
        let z = g.gen::<f64>();
        let w = if use_w { g.gen_range(-1.0, 1.0) } else { 1.0 };

        Vertex::new(x, y, z, w)
    }

    fn gen_texture_vertex(&self, g: &mut G) -> TextureVertex {
        let u = g.gen::<f64>();
        let v = g.gen::<f64>();
        let w = g.gen::<f64>();

        TextureVertex::new(u, v, w)
    }

    fn gen_normal_vertex(&self, g: &mut G) -> NormalVertex {
        let i = g.gen::<f64>();
        let j = g.gen::<f64>();
        let k = g.gen::<f64>();

        NormalVertex::new(i, j, k)
    }

    fn gen_vertex_set(&self, g: &mut G, len: usize) -> Vec<Vertex> {
        let mut vertex_set = vec![];
        for _ in 0..len {
            let use_w = g.gen::<bool>();
            vertex_set.push(self.gen_vertex(g, use_w));
        }

        assert_eq!(vertex_set.len(), len);
        vertex_set
    }

    fn gen_texture_vertex_set(&self, g: &mut G, len: usize) -> Vec<TextureVertex> {
        let mut texture_vertex_set = vec![];
        for _ in 0..len {
            texture_vertex_set.push(self.gen_texture_vertex(g));
        }

        assert_eq!(texture_vertex_set.len(), len);
        texture_vertex_set
    }

    fn gen_normal_vertex_set(&self, g: &mut G, len: usize) -> Vec<NormalVertex> {
        let mut normal_vertex_set = vec![];
        for _ in 0..len {
            normal_vertex_set.push(self.gen_normal_vertex(g));
        }

        assert_eq!(normal_vertex_set.len(), len);
        normal_vertex_set
    }

    fn gen_slices(
        &self, 
        g: &mut G, 
        range: (usize, usize), 
        count: usize) -> Vec<(usize, usize)> {

        assert!(range.0 > 0);
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
        assert!(slices.iter().all(|slice| slice.0 > 0));
        assert!(slices.iter().all(|slice| slice.0 <= slice.1));
        slices
    }

    fn gen_vtn_index(
        &self, 
        g: &mut G, 
        use_vt: bool, 
        use_vn: bool, 
        v_count: usize, 
        vt_count: usize, 
        vn_count: usize) -> VTNIndex {

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
        &self, 
        g: &mut G, 
        element_count: usize, 
        v_count: usize, 
        vt_count: usize, 
        vn_count: usize) -> Vec<Element> {

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

    fn gen_group_set(&self, use_default: bool, count: usize) -> Vec<Group> {
        assert!(count > 0);

        let mut group_set = vec![];
        if use_default && (count == 1) {
            group_set.push(Default::default());
            return group_set;
        }

        assert!(count > 1);

        for i in 0..count {
            let group_i = Group::new(&format!("Group{}", i));
            group_set.push(group_i);
        }

        assert_eq!(group_set.len(), count);
        group_set
    }

    fn gen_smoothing_group_set(&self, count: usize) -> Vec<SmoothingGroup> {
        assert!(count > 0);

        let mut smoothing_group_set = vec![];
        for i in 0..count {
            let smoothing_group_i = SmoothingGroup::new(i);
            smoothing_group_set.push(smoothing_group_i);
        }

        assert_eq!(smoothing_group_set.len(), count);
        smoothing_group_set
    }

    fn gen_shape_set(&self, 
        element_set: &[Element], 
        group_slices: &[(usize, usize)], 
        group_set: &[usize],
        smoothing_group_slices: &[(usize, usize)], 
        smoothing_group_set: &[usize]) -> Vec<ShapeEntry> {
        
        assert!(group_slices.len() > 0);
        assert!(group_slices.len() == group_set.len());
        assert!(group_set.len() > 0);
        assert!(group_set.iter().all(|&index| index > 0));
        assert!(smoothing_group_slices.len() > 0);
        assert!(smoothing_group_slices.len() == smoothing_group_set.len());
        assert!(smoothing_group_set.len() > 0);
        assert!(smoothing_group_set.iter().all(|&index| index > 0));

        let mut shape_set = vec![];
        for i in 1..(group_slices.len() + 1) {
            for j in group_slices[i - 1].0..group_slices[i - 1].1 {
                let shape_entry = ShapeEntry::new(j, &group_set[(i - 1)..i], 1);
                shape_set.push(shape_entry);
            }
        }

        // The group slices should contain the entire range of elements
        // in the element set, and no more.
        assert!(shape_set.iter().all(|shape_entry| shape_entry.element > 0));
        assert_eq!(shape_set.len(), element_set.len());

        for i in 1..(smoothing_group_slices.len() + 1) {
            for j in smoothing_group_slices[i - 1].0..smoothing_group_slices[i - 1].1 {
                shape_set[j - 1].smoothing_group = smoothing_group_set[i - 1];
            }
        }

        // The smoothing group iteration should not change the length
        // of the shape set.
        assert_eq!(shape_set.len(), element_set.len());
        // Wavefront OBJ files are one-indexed in elements, vertices, groups, etc.
        assert!(shape_set.iter().all(|shape_entry| shape_entry.element > 0));
        assert!(shape_set.iter().all(|shape_entry| shape_entry.groups.iter().all(|&group_index| group_index > 0)));
        assert!(shape_set.iter().all(|shape_entry| shape_entry.smoothing_group > 0));
        shape_set
    }

    fn gen_group_count(&self, g: &mut G, use_default: bool) -> usize {
        if use_default { 
            1 
        } else { 
            g.gen_range(2, 10)
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
            g,  element_count,
            vertex_set.len(), texture_vertex_set.len(), 
            normal_vertex_set.len(),
        );

        let use_g_default: bool = g.gen::<bool>();
        let group_count = self.gen_group_count(g, use_g_default);
        let group_slices = self.gen_slices(g, (1, element_set.len() + 1), group_count);
        let group_set = self.gen_group_set(use_g_default, group_count);

        let smoothing_group_count = g.gen_range(1, 10);
        let smoothing_group_slices = self.gen_slices(
            g, (1, element_set.len() + 1), smoothing_group_count
        );
        let smoothing_group_set = self.gen_smoothing_group_set(smoothing_group_count);

        let shape_set = self.gen_shape_set(
            &element_set, 
            &group_slices,
            &(1..((group_set.len() + 1))).collect::<Vec<usize>>(), 
            &smoothing_group_slices, 
            &(1..((smoothing_group_set.len() + 1))).collect::<Vec<usize>>(), 
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

    fn generate(&self, g: &mut G, index: usize) -> Object {
        self.gen_object(g, index)
    }
}

struct ObjectSetGen<G> { 
    _marker: marker::PhantomData<G>,
}

impl<G> ObjectSetGen<G> where G: RngCore {
    fn new() -> Self { 
        Self { _marker: marker::PhantomData } 
    }

    fn generate(&self, g: &mut G) -> ObjectSet {
        // We want one-object sets to appear frequently since that is the most
        // commonly encountered case in the wild.
        let object_gen = ObjectGenerator::new();
        let one_obj = g.gen::<bool>();
        let object_count = if one_obj { 1 } else { g.gen_range(2, 6) };

        let mut objects = vec![];
        for index in 1..(object_count + 1) {
            let object = object_gen.generate(g, index);
            objects.push(object);
        }

        assert_eq!(objects.len(), object_count);
        ObjectSet::new(objects)
    }
}

impl Arbitrary for ParserModel {
    fn arbitrary<G: Gen>(_g: &mut G) -> Self {
        let mut rng = rand::thread_rng();
        ParserModel::new(ObjectSetGen::new().generate(&mut rng))
    }
}

/// The testing oracle defines an environment against which to assess the 
/// correctness of the parser. The oracle consists of a parser model and an 
/// object text. Given a generated object set, the model produces an equivalent 
/// object text string and passes it to the parser. The parser satisfies the 
/// model if the parsed object set matches the one produced by the model.
#[derive(Clone, Debug)]
struct Oracle { 
    model: ParserModel,
    text: String,
}

impl Oracle {
    fn new(model: ParserModel, text: String) -> Oracle {
        Oracle { model: model, text: text }
    }

    fn actual(&self) -> Parser {
        let input = &self.text;
        Parser::new(input)
    }

    fn model(&self) -> &ParserModel {
        &self.model
    }
}

impl Arbitrary for Oracle  {
    fn arbitrary<G: Gen>(g: &mut G) -> Oracle {
        let model: ParserModel = Arbitrary::arbitrary(g);
        let text = TextObjectSetCompositor::new().compose(&model.data);
        Oracle::new(model, text)
    }
}


/// The parser should correctly parse object statements.
#[test]
fn prop_parse_object_set_should_parse_object_names() {
    fn property(oracle: Oracle) -> bool {
        let result_set = oracle.actual().parse().unwrap();
        let expected_set = oracle.model().parse().unwrap();

        result_set.iter().zip(expected_set.iter()).all(|(result, expected)| {
            assert_eq!(
                result.name, expected.name, "{}",
                format!("\nOBJECT FILE GENERATED: \n\n{}\n", oracle.model())
            );
            result.name == expected.name
        })
    }
    quickcheck::quickcheck(property as fn(Oracle) -> bool);
}

/// The parser should correctly parse vertex statements.
#[test]
fn prop_parse_object_set_should_parse_vertices() {
    fn property(oracle: Oracle) -> bool {
        let result_set = oracle.actual().parse().unwrap();
        let expected_set = oracle.model().parse().unwrap();

        result_set.iter().zip(expected_set.iter()).all(|(result, expected)| {
            assert_eq!(
                result.vertex_set, expected.vertex_set, "{}",
                format!("\nOBJECT FILE GENERATED: \n\n{}\n", oracle.model())
            );
            result.vertex_set == expected.vertex_set
        })
    }
    quickcheck::quickcheck(property as fn(Oracle) -> bool);
}

/// The parser should correctly parse texture vertex statements.
#[test]
fn prop_parse_object_set_should_parse_texture_vertices() {
    fn property(oracle: Oracle) -> bool {
        let result_set = oracle.actual().parse().unwrap();
        let expected_set = oracle.model().parse().unwrap();

        result_set.iter().zip(expected_set.iter()).all(|(result, expected)| {
            assert_eq!(
                result.texture_vertex_set, expected.texture_vertex_set, "{}",
                format!("\nOBJECT FILE GENERATED: \n\n{}\n", oracle.model())
            );
            result.texture_vertex_set == expected.texture_vertex_set
        })
    }
    quickcheck::quickcheck(property as fn(Oracle) -> bool);
}

/// The parser should correctly parse normal vertex statements.
#[test]
fn prop_parse_object_set_should_parse_normal_vertices() {
    fn property(oracle: Oracle) -> bool {
        let result_set = oracle.actual().parse().unwrap();
        let expected_set = oracle.model().parse().unwrap();

        result_set.iter().zip(expected_set.iter()).all(|(result, expected)| {
            assert_eq!(
                result.normal_vertex_set, expected.normal_vertex_set, "{}",
                format!("\nOBJECT FILE GENERATED: \n\n{}\n", oracle.model())
            );
            result.normal_vertex_set == expected.normal_vertex_set
        })
    }
    quickcheck::quickcheck(property as fn(Oracle) -> bool);
}

/// The parser should correctly parse group statements.
#[test]
fn prop_parse_object_set_should_parse_groups() {
    fn property(oracle: Oracle) -> bool {
        let result_set = oracle.actual().parse().unwrap();
        let expected_set = oracle.model().parse().unwrap();

        result_set.iter().zip(expected_set.iter()).all(|(result, expected)| {
            assert_eq!(
                result.group_set, expected.group_set, "{}",
                format!("\nOBJECT FILE GENERATED: \n\n{}\n", oracle.model())
            );
            result.group_set == expected.group_set
        })
    }
    quickcheck::quickcheck(property as fn(Oracle) -> bool);
}

/// The parser should correctly parse smoothing group statements.
#[test]
fn prop_parse_object_set_should_parse_smoothing_groups() {
    fn property(oracle: Oracle) -> bool {
        let result_set = oracle.actual().parse().unwrap();
        let expected_set = oracle.model().parse().unwrap();

        result_set.iter().zip(expected_set.iter()).all(|(result, expected)| {
            assert_eq!(
                result.smoothing_group_set, expected.smoothing_group_set, "{}",
                format!("\nOBJECT FILE GENERATED: \n\n{}\n", oracle.model())
            );
            result.smoothing_group_set == expected.smoothing_group_set
        })
    }
    quickcheck::quickcheck(property as fn(Oracle) -> bool);
}

/// The parser should correctly parse elements from the file.
#[test]
fn prop_parse_object_set_should_parse_elements() {
    fn property(oracle: Oracle) -> bool {
        let result_set = oracle.actual().parse().unwrap();
        let expected_set = oracle.model().parse().unwrap();

        result_set.iter().zip(expected_set.iter()).all(|(result, expected)| {
            assert_eq!(
                result.element_set, expected.element_set, "{}",
                format!("\nOBJECT FILE GENERATED: \n\n{}\n", oracle.model())
            );
            result.element_set == expected.element_set
        })
    }
    quickcheck::quickcheck(property as fn(Oracle) -> bool);
}

/// The parser should correctly parse shape entries.
#[test]
fn prop_parse_object_set_should_parse_shape_entries() {
    fn property(oracle: Oracle) -> bool {
        let result_set = oracle.actual().parse().unwrap();
        let expected_set = oracle.model().parse().unwrap();

        result_set.iter().zip(expected_set.iter()).all(|(result, expected)| {
            assert_eq!(
                result.shape_set, expected.shape_set, "{}",
                format!("\nOBJECT FILE GENERATED: \n\n{}\n", oracle.model())
            );
            result.shape_set == expected.shape_set
        })
    }
    quickcheck::quickcheck(property as fn(Oracle) -> bool);
}

/// The parser should correctly parse a valid wavefront obj file.
#[test]
fn prop_parser_correctly_parses_valid_obj_files() {
    fn property(oracle: Oracle) -> bool {
        let result = oracle.actual().parse();
        let expected = oracle.model().parse();

        assert_eq!(
            result, expected, "{}",
            format!("\nOBJECT FILE GENERATED: \n\n{}\n", oracle.model())
        );
        result == expected
    }
    quickcheck::quickcheck(property as fn(Oracle) -> bool);
}

/// Wavefront OBJ files implicitly index the elements in a file in monotone
/// increasing order. The shape set element indices should appear in the same order
/// in the set as the elements appear in the original file.
#[test]
fn prop_every_shape_set_should_be_monotone_increasing() {
    fn is_monotone(set: &[ShapeEntry]) -> bool {
        set[0..set.len()-1].iter().zip(set[1..set.len()].iter()).all(
            |(shape_entry, next_shape_entry)| { shape_entry.element <= next_shape_entry.element }
        )
    }

    fn property(oracle: Oracle) -> bool {
        let result_set = oracle.actual().parse().unwrap();

        result_set.iter().all(|result| is_monotone(&result.shape_set))
    }
    quickcheck::quickcheck(property as fn(Oracle) -> bool);
}

/// Every element belongs to a group. If no grouping statements are specified,
/// the elements default to a value of `default` for the group and `off`, or `0`
/// for the smoothing group.
#[test]
fn prop_every_element_belongs_to_a_group() {
    fn property(oracle: Oracle) -> bool {
        let result_set = oracle.actual().parse().unwrap();

        result_set.iter().all(|result| {
            result.shape_set.iter().all(|shape_entry| { 
                !shape_entry.groups.is_empty()
            })
        })
    }
    quickcheck::quickcheck(property as fn(Oracle) -> bool);
}

/// Every shape entry's smoothing group exists in the object file. It is possible
/// for groups and smoothing groups to be empty, but they cannot be bogus.
#[test]
fn prop_every_smoothing_group_exists() {
    fn property(oracle: Oracle) -> bool {
        let result_set = oracle.actual().parse().unwrap();

        result_set.iter().all(|result| {
            result.shape_set.iter().all(|shape_entry| {
                result.smoothing_group_set.get((shape_entry.smoothing_group - 1) as usize).is_some()
            })
        })
    }
    quickcheck::quickcheck(property as fn(Oracle) -> bool);
}

/// Every element belongs to a smoothing group. If no grouping statements are specified,
/// the elements default to a value of `default` for the group and `off`, or `0`
/// for the smoothing group.
#[test]
fn prop_every_smoothing_group_has_at_least_one_element() {
    fn property(oracle: Oracle) -> bool {
        let result_set = oracle.actual().parse().unwrap();

        result_set.iter().all(|result| !result.smoothing_group_set.is_empty())
    }
    quickcheck::quickcheck(property as fn(Oracle) -> bool);
}

/// Every element belongs to a group. If no grouping statements are specified,
/// the elements default to a value of `default` for the group and `off`, or `0`
/// for the smoothing group.
#[test]
fn prop_every_group_has_at_least_one_element() {
    fn property(oracle: Oracle) -> bool {
        let result_set = oracle.actual().parse().unwrap();

        result_set.iter().all(|result| !result.group_set.is_empty())
    }
    quickcheck::quickcheck(property as fn(Oracle) -> bool);
}

/// Every element belongs to a group. If no group entry is specified in
/// a wavefront obj file, a default value of `default` is used for groups, and
/// a default value of `off`, or `0` is used for smoothing groups.
#[test]
fn prop_every_shape_entry_element_exists() {
    fn property(oracle: Oracle) -> bool {
        let result_set = oracle.actual().parse().unwrap();

        result_set.iter().all(|result| { 
            result.shape_set.iter().all(|shape_entry| { 
                result.element_set.get((shape_entry.element - 1) as usize).is_some()
            })
        })
    }
    quickcheck::quickcheck(property as fn(Oracle) -> bool);
}

/// Every element in a Wavefront OBJ file is implicitly indexed
/// starting from 1.
#[test]
fn prop_every_shape_entry_element_index_is_nonzero() {
    fn property(oracle: Oracle) -> bool {
        let result_set = oracle.actual().parse().unwrap();

        result_set.iter().all(|result| {
            result.shape_set.iter().all(|shape_entry| {
                shape_entry.smoothing_group > 0
            })
        })
    }
    quickcheck::quickcheck(property as fn(Oracle) -> bool);   
}

/// Every vertex, element, and grouping statement is implicitly indexed
/// in a Wavefront OBJ file starting from 1 rather than 0. Every index
/// in a shape entry should be nonzero.
#[test]
fn prop_every_shape_entry_group_index_is_nonzero() {
    fn property(oracle: Oracle) -> bool {
        let result_set = oracle.actual().parse().unwrap();

        result_set.iter().all(|result| {
            result.shape_set.iter().all(|shape_entry| {
                shape_entry.groups.iter().all(|&group_index| group_index > 0)
            })
        })
    }
    quickcheck::quickcheck(property as fn(Oracle) -> bool);   
}

/// A Wavefront OBJ file implicitly indexes every vertex, element, and grouping
/// statement in monotone increasing order from the beginning of the file to
/// to the end of the file. Also, every index is indexed beginning at 1 rather than 0.
/// The shape entries should reflect this. 
#[test]
fn prop_every_shape_entry_smoothing_group_index_is_nonzero() {
    fn property(oracle: Oracle) -> bool {
        let result_set = oracle.actual().parse().unwrap();

        result_set.iter().all(|result| {
            result.shape_set.iter().all(|shape_entry| {
                shape_entry.smoothing_group > 0
            })
        })
    }
    quickcheck::quickcheck(property as fn(Oracle) -> bool);   
}

