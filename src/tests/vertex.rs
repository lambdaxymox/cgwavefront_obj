use obj::object::Vertex;
use obj::parser::ParseError;
use quickcheck;
use std::fmt;
use std::cmp;


#[derive(Clone, Debug)]
struct QcVertex3(Vertex);

impl QcVertex3 {
    fn to_vertex(&self) -> Vertex { self.0 }
}

impl fmt::Display for QcVertex3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "v  {}  {}  {}", self.0.x, self.0.y, self.0.z)
    }
}

impl cmp::PartialEq<Vertex> for QcVertex3 {
    fn eq(&self, other: &Vertex) -> bool { &self.0 == other }
}

impl<'a> cmp::PartialEq<&'a Vertex> for QcVertex3 {
    fn eq(&self, other: & &Vertex) -> bool { &&self.0 == other }
}

impl quickcheck::Arbitrary for QcVertex3 {
    fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
        let x = quickcheck::Arbitrary::arbitrary(g);
        let y = quickcheck::Arbitrary::arbitrary(g);
        let z = quickcheck::Arbitrary::arbitrary(g);

        QcVertex3(Vertex { x: x, y: y, z: z, w: 1.0 })
    }
}

#[derive(Clone, Debug)]
struct QcVertex4(Vertex);

impl QcVertex4 {
    fn to_vertex(&self) -> Vertex { self.0 }
}

impl fmt::Display for QcVertex4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "v  {}  {}  {}  {}", self.0.x, self.0.y, self.0.z, self.0.w)
    }
}

impl cmp::PartialEq<Vertex> for QcVertex4 {
    fn eq(&self, other: &Vertex) -> bool { &self.0 == other }
}

impl<'a> cmp::PartialEq<&'a Vertex> for QcVertex4 {
    fn eq(&self, other: & &Vertex) -> bool { &&self.0 == other }
}

impl quickcheck::Arbitrary for QcVertex4 {
    fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
        let x = quickcheck::Arbitrary::arbitrary(g);
        let y = quickcheck::Arbitrary::arbitrary(g);
        let z = quickcheck::Arbitrary::arbitrary(g);
        let w = quickcheck::Arbitrary::arbitrary(g);

        QcVertex4(Vertex { x: x, y: y, z: z, w: w })
    }
}

impl From<QcVertex3> for QcVertex4 {
    fn from(qc_vertex: QcVertex3) -> QcVertex4 {
        QcVertex4(qc_vertex.0)
    }
}

#[derive(Clone, Debug)]
struct QcVertexModel(QcVertex4, String);

impl QcVertexModel {
    fn new(qc_vertex: QcVertex4, string: String) -> QcVertexModel {
        QcVertexModel(qc_vertex, string)
    }

    fn parse_vertex(&self) -> Result<Vertex, ParseError> {
        Ok(self.0.to_vertex())
    }
}

impl quickcheck::Arbitrary for QcVertexModel {
    fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
        use quickcheck::Arbitrary;

        fn spaces(n: u8) -> String {
            let mut spaces = String::new();
            for _ in 0..n {
                spaces.push(' ');
            }
            spaces
        }

        let qc_vertex: QcVertex4 = Arbitrary::arbitrary(g);
        let spaces: [String; 6] = [
            spaces(Arbitrary::arbitrary(g)), spaces(Arbitrary::arbitrary(g)),
            spaces(Arbitrary::arbitrary(g)), spaces(Arbitrary::arbitrary(g)), 
            spaces(Arbitrary::arbitrary(g)), spaces(Arbitrary::arbitrary(g)),
        ];
        let string = format!("{}v {} {} {} {} {} {} {} {} {} ", 
            spaces[0], spaces[1], qc_vertex.0.x, spaces[2], qc_vertex.0.y, 
            spaces[3], qc_vertex.0.z, spaces[4], qc_vertex.0.w,
            spaces[5], 
        );

        QcVertexModel::new(qc_vertex, string)
    }
}

mod property_tests { 
    use obj::parser::Parser;
    use quickcheck;
    use super::{QcVertex3, QcVertex4, QcVertexModel};


    #[test]
    fn prop_parsing_a_vertex_string_should_yield_the_same_vertex() {
        fn property(qc_vertex: QcVertex4) -> bool {
            let input = qc_vertex.to_string();
            let result = Parser::new(input.chars()).parse_vertex();
            let expected = Ok(qc_vertex.to_vertex());

            result == expected
        }
        quickcheck::quickcheck(property as fn(QcVertex4) -> bool);
    }

    #[test]
    fn prop_parsing_a_vertex_with_three_coordinates_should_have_default_w_1() {
        fn property(qc_vertex: QcVertex3) -> bool {
            let input = qc_vertex.to_string();
            let result = Parser::new(input.chars()).parse_vertex();
            let expected = Ok(qc_vertex.to_vertex());

            result == expected
        }
        quickcheck::quickcheck(property as fn(QcVertex3) -> bool);
    }

    #[test]
    fn prop_a_three_vertex_and_a_four_vertex_with_w_1_identical() {
        fn property(qc_vertex: QcVertex3) -> bool {
            let input = qc_vertex.to_string();
            let result = Parser::new(input.chars()).parse_vertex();
            let expected = Ok(QcVertex4::from(qc_vertex).0);

            result == expected
        }
        quickcheck::quickcheck(property as fn(QcVertex3) -> bool);
    }

    #[test]
    fn prop_parser_vertex_should_be_invariant_to_whitespace() {
        fn property(qcvm: QcVertexModel) -> bool {
            let result = Parser::new(qcvm.1.chars()).parse_vertex();
            let expected = qcvm.parse_vertex();

            result == expected
        }
        quickcheck::quickcheck(property as fn(QcVertexModel) -> bool);
    }
}

