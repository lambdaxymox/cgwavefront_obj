extern crate wavefront;


use wavefront::obj::Parser;

use std::fs::File;
use std::io::{BufReader, Read};


const SAMPLE_DATA: &str = "sample_data/teapot.obj";


fn main() {
    let file = File::open(SAMPLE_DATA).expect("File not found.");
    let mut reader = BufReader::new(file);
    let mut string = String::new();
    reader.read_to_string(&mut string).unwrap();

    let mut parser = Parser::new(string.chars());
    let object_set = parser.parse().expect(&format!("Failed to parse {}.", SAMPLE_DATA));
    println!("{}", object_set);
}

