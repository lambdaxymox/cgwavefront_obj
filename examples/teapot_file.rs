extern crate wavefront_obj;

use wavefront_obj as obj;

const SAMPLE_DATA: &str = "sample_data/teapot.obj";


fn main() {
    let object_set = obj::parse_file(SAMPLE_DATA);

    assert!(object_set.is_ok());
}
