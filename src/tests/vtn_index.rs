use obj::object::VTNIndex;
use obj::parser::ParseError;
use quickcheck;
use std::fmt;
use std::cmp;


#[derive(Clone, Debug)]
struct VTNIndexParserModel(VTNIndex, String);

impl VTNIndexParserModel {
    fn new(vtn_index: VTNIndex, string: String) -> VTNIndexParserModel {
        VTNIndexParserModel(vtn_index, string)
    }

    fn parse(&self) -> Result<VTNIndex, ParseError> { 
        Ok(self.0) 
    }
}

impl quickcheck::Arbitrary for VTNIndexParserModel {
    fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
        use quickcheck::Arbitrary;

        let vtn_index_type = g.gen_range(0, 4);
        let vtn_index = match vtn_index_type {
            0 => VTNIndex::V(Arbitrary::arbitrary(g)),
            1 => VTNIndex::VT(Arbitrary::arbitrary(g), Arbitrary::arbitrary(g)),
            2 => VTNIndex::VN(Arbitrary::arbitrary(g), Arbitrary::arbitrary(g)),
            _ => VTNIndex::VTN(
                Arbitrary::arbitrary(g), Arbitrary::arbitrary(g), Arbitrary::arbitrary(g)
            ),
        };

        let string = match vtn_index {
            VTNIndex::V(v) => format!("{}", v),
            VTNIndex::VT(v, tv) => format!("{}/{}", v, tv),
            VTNIndex::VN(v, nv) => format!("{}//{}", v, nv),
            VTNIndex::VTN(v, tv, nv) => format!("{}/{}/{}", v, tv, nv),
        };

        VTNIndexParserModel::new(vtn_index, string)
    }
}


mod property_tests {
    use obj::parser::Parser;
    use quickcheck;
    use super::VTNIndexParserModel;


    #[test]
    fn prop_parser_vertex_encode_decode_inverses() {
        fn property(vtn_model: VTNIndexParserModel) -> bool {
            let result = Parser::new(vtn_model.1.chars()).parse_vtn_index();
            let expected = vtn_model.parse();

            result == expected
        }
        quickcheck::quickcheck(property as fn(VTNIndexParserModel) -> bool);
    }
}