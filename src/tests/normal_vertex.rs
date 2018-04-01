use obj::object::NormalVertex;
use quickcheck;
use std::fmt;
use std::cmp;


#[derive(Clone, Debug)]
struct QcNormalVertex(NormalVertex);

impl QcNormalVertex {
    fn to_vertex(&self) -> NormalVertex { self.0 }
}

impl fmt::Display for QcNormalVertex {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "vn  {}  {}  {}", self.0.i, self.0.j, self.0.k)
    }
}

impl cmp::PartialEq<NormalVertex> for QcNormalVertex {
    fn eq(&self, other: &NormalVertex) -> bool { &self.0 == other }
}

impl<'a> cmp::PartialEq<&'a NormalVertex> for QcNormalVertex {
    fn eq(&self, other: & &NormalVertex) -> bool { &&self.0 == other }
}

impl quickcheck::Arbitrary for QcNormalVertex {
    fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
        let i = quickcheck::Arbitrary::arbitrary(g);
        let j = quickcheck::Arbitrary::arbitrary(g);
        let k = quickcheck::Arbitrary::arbitrary(g);

        QcNormalVertex(NormalVertex { i: i, j: j, k: k })
    }
}

#[derive(Clone, Debug)]
struct QcNormalVertexOracle(QcNormalVertex, String, String);

impl QcNormalVertexOracle {
    fn new(qc_texture_vertex: QcNormalVertex, 
        string: String, other_string: String) -> QcNormalVertexOracle {
        
        QcNormalVertexOracle(qc_texture_vertex, string, other_string)
    }
}

impl quickcheck::Arbitrary for QcNormalVertexOracle {
    fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
        use quickcheck::Arbitrary;

        fn spaces(n: u8) -> String {
            let mut spaces = String::new();
            for _ in 0..n {
                spaces.push(' ');
            }
            spaces
        }

        let qc_vertex: QcNormalVertex = Arbitrary::arbitrary(g);
        let string = qc_vertex.to_string();
        let spaces: [String; 4] = [
            spaces(Arbitrary::arbitrary(g)), spaces(Arbitrary::arbitrary(g)),
            spaces(Arbitrary::arbitrary(g)), spaces(Arbitrary::arbitrary(g))
        ];
        let other_string = format!("vn {} {} {} {} {} {} {} ", 
            spaces[0], qc_vertex.0.i, spaces[1], qc_vertex.0.j, 
            spaces[2], qc_vertex.0.k, spaces[3]
        );

        QcNormalVertexOracle::new(qc_vertex, string, other_string)
    }
}

mod property_tests { 
    use obj::parser::Parser;
    use quickcheck;
    use super::{QcNormalVertex, QcNormalVertexOracle};

    #[test]
    fn prop_parsing_a_texture_vertex_string_is_reversible() {
        fn property(qc_normal_vertex: QcNormalVertex) -> bool {
            let input = qc_normal_vertex.to_string();
            let mut parser = Parser::new(input.chars());
            let result = parser.parse_normal_vertex();
            let expected = Ok(qc_normal_vertex.to_vertex());

            result == expected
        }
        quickcheck::quickcheck(property as fn(QcNormalVertex) -> bool);
    }

    #[test]
    fn prop_parser_texture_vertex_should_be_invariant_to_whitespace() {
        fn property(qcnvo: QcNormalVertexOracle) -> bool {
            let result1 = Parser::new(qcnvo.1.chars()).parse_normal_vertex();
            let result2 = Parser::new(qcnvo.2.chars()).parse_normal_vertex();
            let expected = Ok(qcnvo.0.to_vertex());

            (result1 == expected) && (result2 == expected)
        }
        quickcheck::quickcheck(property as fn(QcNormalVertexOracle) -> bool);
    }
}

