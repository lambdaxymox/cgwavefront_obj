extern crate wavefront_obj;

use std::fs::File;
use std::io;
use std::io::Read;
use wavefront_obj::obj;

const SAMPLE_DATA: &str = "sample_data/teapot.obj";


fn main() -> io::Result<()> {
    let mut file = File::open(SAMPLE_DATA).expect("File not found.");
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    let object_set = obj::parse(buffer);

    assert!(object_set.is_ok());
    Ok(())
}
