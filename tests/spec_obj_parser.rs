#[cfg(test)]
#[macro_use]
extern crate quickcheck;
extern crate wavefront;

use quickcheck::Arbitrary;
use wavefront::obj::{Vertex};
use std::fmt;
use std::cmp;


#[derive(Clone, Debug)]
struct QcVertex {
    inner: Vertex,
    display_w: bool,
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

