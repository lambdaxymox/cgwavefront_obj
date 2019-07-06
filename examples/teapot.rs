extern crate wavefront_obj;

use wavefront_obj as obj;
use std::fs::File;

const SAMPLE_DATA: &str = "sample_data/teapot.obj";


fn main() {
    let file = File::open(SAMPLE_DATA).expect("File not found.");
    let object_set = obj::parse(file);

    assert!(object_set.is_ok());
}

