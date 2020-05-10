use criterion::{
    black_box, 
    criterion_group, 
    criterion_main, 
    Criterion
};
use wavefront_obj as obj;

const SAMPLE_DATA: &str = "assets/symphysis.obj";


fn benchmark(c: &mut Criterion) {
    c.bench_function("parse symphysis.obj", |b| b.iter(|| {
        obj::parse_file(black_box(SAMPLE_DATA)).unwrap()
    }));
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
