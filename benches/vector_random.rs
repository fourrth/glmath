use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};

use glmath::vector;

pub fn random_vector4(c: &mut Criterion) {
    let mut group = c.benchmark_group("Random Vector4's");
    // compare two functions
    group.bench_function("generate_rand_vector4", |b| {
        b.iter(|| {
            //
            black_box(vector::generate_rand_vector4::<f32>())
        });
    });
}
criterion_group!(benches, random_vector4);
criterion_main!(benches);
