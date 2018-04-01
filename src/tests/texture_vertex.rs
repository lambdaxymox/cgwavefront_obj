use obj::object::TextureVertex;
use quickcheck;
use std::fmt;
use std::cmp;


#[derive(Clone, Debug)]
struct QcTextureVertex(TextureVertex);

impl QcTextureVertex {
    fn to_texture_vertex(&self) -> TextureVertex { self.0 }
}

impl fmt::Display for QcTextureVertex {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "vt  {}  {}  {}", self.0.u, self.0.v, self.0.w)
    }
}

impl cmp::PartialEq<TextureVertex> for QcTextureVertex {
    fn eq(&self, other: &TextureVertex) -> bool { &self.0 == other }
}

impl<'a> cmp::PartialEq<&'a TextureVertex> for QcTextureVertex {
    fn eq(&self, other: & &TextureVertex) -> bool { &&self.0 == other }
}

impl quickcheck::Arbitrary for QcTextureVertex {
    fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
        let u = quickcheck::Arbitrary::arbitrary(g);
        let v = quickcheck::Arbitrary::arbitrary(g);
        let w = quickcheck::Arbitrary::arbitrary(g);

        QcTextureVertex(TextureVertex { u: u, v: v, w: w })
    }
}

#[derive(Clone, Debug)]
struct QcTextureVertexOracle(QcTextureVertex, String, String);

impl QcTextureVertexOracle {
    fn new(qc_texture_vertex: QcTextureVertex, 
        string: String, other_string: String) -> QcTextureVertexOracle {
        
        QcTextureVertexOracle(qc_texture_vertex, string, other_string)
    }
}

impl quickcheck::Arbitrary for QcTextureVertexOracle {
    fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
        use quickcheck::Arbitrary;

        fn spaces(n: u8) -> String {
            let mut spaces = String::new();
            for _ in 0..n {
                spaces.push(' ');
            }
            spaces
        }

        let qc_vertex: QcTextureVertex = Arbitrary::arbitrary(g);
        let string = qc_vertex.to_string();
        let spaces: [String; 4] = [
            spaces(Arbitrary::arbitrary(g)), spaces(Arbitrary::arbitrary(g)),
            spaces(Arbitrary::arbitrary(g)), spaces(Arbitrary::arbitrary(g))
        ];
        let other_string = format!("vt {} {} {} {} {} {} {} ", 
            spaces[0], qc_vertex.0.u, spaces[1], qc_vertex.0.v, 
            spaces[2], qc_vertex.0.w, spaces[3]
        );

        QcTextureVertexOracle::new(qc_vertex, string, other_string)
    }
}

mod property_tests { 
    use obj::parser::Parser;
    use quickcheck;
    use super::{QcTextureVertex, QcTextureVertexOracle};

    #[test]
    fn prop_parsing_a_texture_vertex_string_is_reversible() {
        fn property(qc_texture_vertex: QcTextureVertex) -> bool {
            let input = qc_texture_vertex.to_string();
            let mut parser = Parser::new(input.chars());
            let result = parser.parse_texture_vertex();
            let expected = Ok(qc_texture_vertex.to_texture_vertex());

            result == expected
        }
        quickcheck::quickcheck(property as fn(QcTextureVertex) -> bool);
    }

    #[test]
    fn prop_parser_texture_vertex_should_be_invariant_to_whitespace() {
        fn property(qctvo: QcTextureVertexOracle) -> bool {
            let result1 = Parser::new(qctvo.1.chars()).parse_texture_vertex();
            let result2 = Parser::new(qctvo.2.chars()).parse_texture_vertex();
            let expected = Ok(qctvo.0.to_texture_vertex());

            (result1 == expected) && (result2 == expected)
        }
        quickcheck::quickcheck(property as fn(QcTextureVertexOracle) -> bool);
    }
}

