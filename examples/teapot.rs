extern crate wavefront;

use wavefront::obj;
use std::fs::File;

const SAMPLE_DATA: &str = "sample_data/teapot.obj";


fn main() {
    let file = File::open(SAMPLE_DATA).expect("File not found.");
    let object_set = obj::parse(file).expect(&format!("Failed to parse {}.", SAMPLE_DATA));
    println!("{}", object_set);
}

