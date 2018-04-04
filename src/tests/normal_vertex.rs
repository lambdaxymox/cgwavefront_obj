use obj::object::NormalVertex;
use obj::parser::ParseError;
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
struct QcNormalVertexModel(QcNormalVertex, String);

impl QcNormalVertexModel {
    fn new(qc_normal_vertex: QcNormalVertex, string: String) -> QcNormalVertexModel {
        QcNormalVertexModel(qc_normal_vertex, string)
    }

    fn parse(&self) -> Result<NormalVertex, ParseError> {
        Ok(self.0.to_vertex())
    }
}

impl quickcheck::Arbitrary for QcNormalVertexModel {
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
        let spaces: [String; 5] = [
            spaces(Arbitrary::arbitrary(g)), spaces(Arbitrary::arbitrary(g)),
            spaces(Arbitrary::arbitrary(g)), spaces(Arbitrary::arbitrary(g)),
            spaces(Arbitrary::arbitrary(g))
        ];
        let string = format!("{}vn {} {} {} {} {} {} {} ", 
            spaces[0], spaces[1], qc_vertex.0.i, spaces[2], qc_vertex.0.j, 
            spaces[3], qc_vertex.0.k, spaces[4]
        );

        QcNormalVertexModel::new(qc_vertex, string)
    }
}

#[cfg(test)]
mod property_tests { 
    use obj::parser::Parser;
    use quickcheck;
    use super::QcNormalVertexModel;

    #[test]
    fn prop_parsing_a_texture_vertex_string_is_reversible() {
        fn property(qcnvm: QcNormalVertexModel) -> bool {
            let input = qcnvm.0.to_string();
            let result = Parser::new(input.chars()).parse_normal_vertex();
            let expected = qcnvm.parse();

            result == expected
        }
        quickcheck::quickcheck(property as fn(QcNormalVertexModel) -> bool);
    }

    #[test]
    fn prop_parser_texture_vertex_should_be_invariant_to_whitespace() {
        fn property(qcnvm: QcNormalVertexModel) -> bool {
            let result = Parser::new(qcnvm.1.chars()).parse_normal_vertex();
            let expected = qcnvm.parse();

            result == expected
        }
        quickcheck::quickcheck(property as fn(QcNormalVertexModel) -> bool);
    }
}

