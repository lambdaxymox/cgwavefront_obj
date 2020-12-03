use criterion::{
    black_box, 
    criterion_group, 
    criterion_main, 
    Criterion
};
use wavefront_obj::obj;
use std::fs::{
    File,
};
use std::io::{
    Read,
};


const SAMPLE_DATA: &str = "assets/teapot.obj";


fn benchmark(c: &mut Criterion) {
    c.bench_function("parse teapot.obj", |b| b.iter(|| {
        let mut file = File::open(SAMPLE_DATA).expect("File not found.");
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();
        obj::parse(black_box(buffer)).unwrap()
    }));
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
