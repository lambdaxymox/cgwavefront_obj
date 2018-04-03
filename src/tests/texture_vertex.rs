use obj::object::TextureVertex;
use obj::parser::ParseError;
use quickcheck;
use std::fmt;
use std::cmp;


#[derive(Clone, Debug)]
struct QcTextureVertex(TextureVertex);

impl QcTextureVertex {
    fn to_vertex(&self) -> TextureVertex { self.0 }
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
struct QcTextureVertexModel(QcTextureVertex, String);

impl QcTextureVertexModel {
    fn new(qc_texture_vertex: QcTextureVertex, string: String) -> QcTextureVertexModel {
        QcTextureVertexModel(qc_texture_vertex, string)
    }

    fn parse(&self) -> Result<TextureVertex, ParseError> {
        Ok(self.0.to_vertex())
    }
}

impl quickcheck::Arbitrary for QcTextureVertexModel {
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
        let spaces: [String; 5] = [
            spaces(Arbitrary::arbitrary(g)), spaces(Arbitrary::arbitrary(g)),
            spaces(Arbitrary::arbitrary(g)), spaces(Arbitrary::arbitrary(g)),
            spaces(Arbitrary::arbitrary(g))
        ];
        let string = format!("{}vt {} {} {} {} {} {} {} ", 
            spaces[0], spaces[1], qc_vertex.0.u, spaces[2], qc_vertex.0.v, 
            spaces[3], qc_vertex.0.w, spaces[4]
        );

        QcTextureVertexModel::new(qc_vertex, string)
    }
}

mod property_tests { 
    use obj::parser::Parser;
    use quickcheck;
    use super::{QcTextureVertex, QcTextureVertexModel};


    #[test]
    fn prop_parsing_a_texture_vertex_string_is_reversible() {
        fn property(qc_texture_vertex: QcTextureVertex) -> bool {
            let input = qc_texture_vertex.to_string();
            let mut parser = Parser::new(input.chars());
            let result = parser.parse_texture_vertex();
            let expected = Ok(qc_texture_vertex.to_vertex());

            result == expected
        }
        quickcheck::quickcheck(property as fn(QcTextureVertex) -> bool);
    }

    #[test]
    fn prop_parser_texture_vertex_should_be_invariant_to_whitespace() {
        fn property(qctvm: QcTextureVertexModel) -> bool {
            let result = Parser::new(qctvm.1.chars()).parse_texture_vertex();
            let expected = qctvm.parse();

            result == expected
        }
        quickcheck::quickcheck(property as fn(QcTextureVertexModel) -> bool);
    }
}

