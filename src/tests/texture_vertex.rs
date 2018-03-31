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


mod property_tests { 
    use obj::parser::Parser;
    use quickcheck;
    use super::QcTextureVertex;

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
}

