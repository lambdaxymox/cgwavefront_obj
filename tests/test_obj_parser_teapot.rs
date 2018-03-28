extern crate wavefront;

use wavefront::obj::ObjectSet;
use wavefront::obj::Parser;

use std::slice;
use std::fs::File;
use std::io::Read;


const SAMPLE_DATA: &str = "sample_data/teapot.obj";


struct Test {
    data: String,
    expected: ObjectSet,
}

struct TestSet { 
    data: Vec<Test>,
}

impl TestSet {
    fn iter(&self) -> TestSetIter {
        TestSetIter {
            inner: self.data.iter(),
        }
    }
}

struct TestSetIter<'a> {
    inner: slice::Iter<'a, Test>,
}

impl<'a> Iterator for TestSetIter<'a> {
    type Item = &'a Test;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

fn test_cases(file_path: &str) -> TestSet {
    let mut file = File::open(file_path).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data);

    TestSet {
        data: vec![
            Test {
                data: data,
                expected: ObjectSet::new(vec![])
            },
        ],
    }
}

#[test]
fn test_parse_object_set1() {
    let tests = test_cases(SAMPLE_DATA);
    
    for test in tests.iter() {
        let mut parser = Parser::new(&test.data);
        let result = parser.parse().unwrap();

        assert_eq!(result, test.expected);
    }
}

#[test]
fn test_parse_object_set1_tokenwise() {
    let tests = test_cases(SAMPLE_DATA);

    for test in tests.iter() {
        let mut parser = Parser::new(&test.data);
        let result_set = parser.parse().unwrap();
        for (result, expected) in result_set.iter().zip(test.expected.iter()) {
            assert_eq!(result, expected);
        }
    }
}

