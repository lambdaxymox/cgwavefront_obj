use obj::object::Vertex;
use quickcheck::Arbitrary;
use quickcheck;
use std::fmt;
use std::cmp;


#[derive(Clone, Debug)]
struct QcVertex {
    inner: Vertex,
    display_w: bool,
}

impl QcVertex {
    fn to_vertex(&self) -> Vertex {
        self.inner
    }

    fn to_qc_vertex3(&self) -> QcVertex {
        let mut qc_vertex = self.clone();
        qc_vertex.display_w = false;
        qc_vertex.inner.w = 1.0;

        qc_vertex
    }
}

impl fmt::Display for QcVertex {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        if self.display_w {
            write!(f, "v  {}  {}  {}  {}", 
                self.inner.x, self.inner.y, self.inner.z, self.inner.w
            )
        } else {
            write!(f, "v  {}  {}  {}", self.inner.x, self.inner.y, self.inner.z)
        }
    }
}

impl cmp::PartialEq<Vertex> for QcVertex {
    fn eq(&self, other: &Vertex) -> bool {
        &self.inner == other
    }
}

impl<'a> cmp::PartialEq<&'a Vertex> for QcVertex {
    fn eq(&self, other: & &Vertex) -> bool {
        &&self.inner == other
    }
}

impl Arbitrary for QcVertex {
    fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
        let display_w = Arbitrary::arbitrary(g);
        let w = if display_w { Arbitrary::arbitrary(g) } else { 1.0 };
        let x = Arbitrary::arbitrary(g);
        let y = Arbitrary::arbitrary(g);
        let z = Arbitrary::arbitrary(g);

        QcVertex { inner: Vertex { x: x, y: y, z: z, w: w }, display_w: display_w }
    }
}

mod property_tests { 
    use obj::parser::Parser;
    use quickcheck;
    use super::QcVertex;

    #[test]
    fn prop_parsing_a_vertex_with_three_coordinates_should_have_default_w() {
        fn property(qc_vertex: QcVertex) -> bool {
            let input = qc_vertex.to_qc_vertex3().to_string();
            let mut parser = Parser::new(input.chars());
            let result = parser.parse_vertex();
            let expected = Ok(qc_vertex.to_vertex());

            result == expected
        }
        quickcheck::quickcheck(property as fn(QcVertex) -> bool);
    }

    #[test]
    fn prop_parsing_a_vertex_string_should_yield_the_same_vertex() {
        fn property(qc_vertex: QcVertex) -> bool {
            let input = qc_vertex.to_string();
            let mut parser = Parser::new(input.chars());
            let result = parser.parse_vertex();
            let expected = Ok(qc_vertex.to_vertex());

            result == expected
        }
        quickcheck::quickcheck(property as fn(QcVertex) -> bool);
    }

    #[test]
    fn prop_a_three_vertex_and_a_four_vertex_with_w_1_identical() {
        fn property(qc_vertex: QcVertex) -> bool {
            let input = qc_vertex.to_string();
            let mut parser = Parser::new(input.chars());
            let result = parser.parse_vertex();
            let expected = Ok(qc_vertex.to_vertex());

            result == expected
        }
        quickcheck::quickcheck(property as fn(QcVertex) -> bool);
    }

}

