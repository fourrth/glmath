use std::hint::black_box;

use criterion::BenchmarkId;
use criterion::{criterion_group, criterion_main, Criterion};
use glmath::scalar::lerp;
use once_cell::sync::Lazy;
use rand::Rng;

macro_rules! Bench_with_input {
    {$Criterion:ident,$function_name:expr, $parameter:expr,$input:expr,$var_name:ident$($doer:tt)+} => {
        $Criterion.bench_with_input(
            BenchmarkId::new($function_name, $parameter),
            $input,
            |b, $var_name| {
                b.iter(|| {
                    $($doer)+
                })
            },
        );
    };
}

static mut RNG_GEN: Lazy<rand::rngs::ThreadRng> = Lazy::new(|| rand::rngs::ThreadRng::default());

fn generate_input_data_scalar(len: usize) -> Vec<f32> {
    let input: Vec<f32> = (0..len)
        .into_iter()
        .map(|_| unsafe { RNG_GEN.gen() })
        .collect();
    input
}
pub fn scalar(c: &mut Criterion) {
    let input_scalar_triple = (
        generate_input_data_scalar(100),
        generate_input_data_scalar(100),
        generate_input_data_scalar(100),
    );
    Bench_with_input! {
        c,"scalar","random float [0,1)",&input_scalar_triple,myinput
        {
            for cx in 0..100 {
                // what is bigger shouldn't matter
                // also t values > 1 don't really matter either
                black_box(lerp(black_box(myinput.0[cx]), black_box(myinput.1[cx]), black_box(myinput.2[cx])));
            }
        }
    }
}
criterion_group!(benches, scalar);
criterion_main!(benches);
