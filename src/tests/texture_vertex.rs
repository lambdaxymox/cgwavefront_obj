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
struct TextureVertexParserModel(QcTextureVertex, String);

impl TextureVertexParserModel {
    fn new(qc_texture_vertex: QcTextureVertex, string: String) -> TextureVertexParserModel {
        TextureVertexParserModel(qc_texture_vertex, string)
    }

    fn parse(&self) -> Result<TextureVertex, ParseError> {
        Ok(self.0.to_vertex())
    }
}

impl quickcheck::Arbitrary for TextureVertexParserModel {
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

        TextureVertexParserModel::new(qc_vertex, string)
    }
}

#[cfg(test)]
mod property_tests { 
    use obj::parser::Parser;
    use quickcheck;
    use super::TextureVertexParserModel;


    #[test]
    fn prop_parsing_a_texture_vertex_string_is_reversible() {
        fn property(tvpm: TextureVertexParserModel) -> bool {
            let input = tvpm.0.to_string();
            let result = Parser::new(input.chars()).parse_texture_vertex();
            let expected = tvpm.parse();

            result == expected
        }
        quickcheck::quickcheck(property as fn(TextureVertexParserModel) -> bool);
    }

    #[test]
    fn prop_parser_vertex_encode_decode_inverses() {
        fn property(qctvm: TextureVertexParserModel) -> bool {
            let result = Parser::new(qctvm.1.chars()).parse_texture_vertex();
            let expected = qctvm.parse();

            result == expected
        }
        quickcheck::quickcheck(property as fn(TextureVertexParserModel) -> bool);
    }
}

