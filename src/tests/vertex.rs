use obj::object::Vertex;
use obj::parser::ParseError;
use quickcheck;
use std::fmt;
use std::cmp;


#[derive(Clone, Debug)]
enum QcVertex {
    Vertex3(Vertex),
    Vertex4(Vertex),
}

impl QcVertex {
    fn to_vertex(&self) -> Vertex { 
        match *self {
            QcVertex::Vertex3(v) => v.clone(),
            QcVertex::Vertex4(v) => v.clone(),
        }
    }
}

impl fmt::Display for QcVertex {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            QcVertex::Vertex3(v) => {
                write!(f, "v  {}  {}  {}", v.x, v.y, v.z)
            }
            QcVertex::Vertex4(v) => {
                write!(f, "v  {}  {}  {}  {}", v.x, v.y, v.z, v.w)
            }
        }
    }
}

#[derive(Clone, Debug)]
struct VertexParserModel(QcVertex, String);

impl VertexParserModel {
    fn new(qc_vertex: QcVertex, string: String) -> VertexParserModel {
        VertexParserModel(qc_vertex, string)
    }

    fn parse(&self) -> Result<Vertex, ParseError> {
        Ok(self.0.to_vertex())
    }
}

impl quickcheck::Arbitrary for VertexParserModel {
    fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
        use quickcheck::Arbitrary;

        fn spaces(n: u8) -> String {
            let mut spaces = String::new();
            for _ in 0..n {
                spaces.push(' ');
            }
            spaces
        }

        let x = quickcheck::Arbitrary::arbitrary(g);
        let y = quickcheck::Arbitrary::arbitrary(g);
        let z = quickcheck::Arbitrary::arbitrary(g);

        let use_w = Arbitrary::arbitrary(g);
        if use_w {
            let w = Arbitrary::arbitrary(g);
            let v = Vertex { x: x, y: y, z: z, w: w };
            let qc_vertex = QcVertex::Vertex4(v);

            let spaces: [String; 6] = [
                spaces(Arbitrary::arbitrary(g)), spaces(Arbitrary::arbitrary(g)),
                spaces(Arbitrary::arbitrary(g)), spaces(Arbitrary::arbitrary(g)), 
                spaces(Arbitrary::arbitrary(g)), spaces(Arbitrary::arbitrary(g)),
            ];
            let string = format!("{}v {} {} {} {} {} {} {} {} {}", 
                spaces[0], spaces[1], v.x, spaces[2], v.y, 
                spaces[3], v.z, spaces[4], v.w, spaces[5],
            );

            VertexParserModel::new(qc_vertex, string)
        } else {
            let w = 1.0;
            let v = Vertex { x: x, y: y, z: z, w: w };
            let qc_vertex = QcVertex::Vertex3(v);

            let spaces: [String; 5] = [
                spaces(Arbitrary::arbitrary(g)), spaces(Arbitrary::arbitrary(g)),
                spaces(Arbitrary::arbitrary(g)), spaces(Arbitrary::arbitrary(g)), 
                spaces(Arbitrary::arbitrary(g))
            ];
            let string = format!("{}v {} {} {} {} {} {} {}", 
                spaces[0], spaces[1], v.x, spaces[2], v.y, 
                spaces[3], v.z, spaces[4]
            );

            VertexParserModel::new(qc_vertex, string)
        }
    }
}

#[cfg(test)]
mod property_tests { 
    use obj::parser::Parser;
    use quickcheck;
    use super::{QcVertex, VertexParserModel};


    #[test]
    fn prop_parsing_a_vertex_string_should_yield_the_same_vertex() {
        fn property(vpm: VertexParserModel) -> bool {
            let input = vpm.0.to_string();
            let result = Parser::new(input.chars()).parse_vertex();
            let expected = vpm.parse();

            result == expected
        }
        quickcheck::quickcheck(property as fn(VertexParserModel) -> bool);
    }

    #[test]
    fn prop_parser_vertex_encode_decode_inverses() {
        fn property(vpm: VertexParserModel) -> bool {
            let result = Parser::new(vpm.1.chars()).parse_vertex();
            let expected = vpm.parse();

            result == expected
        }
        quickcheck::quickcheck(property as fn(VertexParserModel) -> bool);
    }
}

