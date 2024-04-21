use std::hint::black_box;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use glmath::{
    vector::{generate_rand_vector3, Vector3},
    Element,
};
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
fn generate_input_data_vector3_pair(len: usize) -> Vec<(Vector3<f32>, Vector3<f32>)> {
    let input: Vec<(Vector3<f32>, Vector3<f32>)> = (0..len)
        .into_iter()
        .map(|_| (generate_rand_vector3(), generate_rand_vector3()))
        .collect();
    input
}
fn generate_input_data_vector3(len: usize) -> Vec<Vector3<f32>> {
    let input: Vec<Vector3<f32>> = (0..len)
        .into_iter()
        .map(|_| generate_rand_vector3())
        .collect();
    input
}

static mut RNG_GEN: Lazy<rand::rngs::ThreadRng> = Lazy::new(|| rand::rngs::ThreadRng::default());

fn generate_input_data_scalar(len: usize) -> Vec<f32> {
    let input: Vec<f32> = (0..len)
        .into_iter()
        .map(|_| unsafe { RNG_GEN.gen() })
        .collect();
    input
}

struct VectorScalarPair<T: Element>(pub Vec<Vector3<T>>, pub Vec<T>);

pub fn standard_vector3(c: &mut Criterion) {
    let input_vector_pair = generate_input_data_vector3_pair(500);
    let input_vector_single = generate_input_data_vector3(200);
    let input_vector_scalar_pair = VectorScalarPair(
        generate_input_data_vector3(200),
        generate_input_data_scalar(200),
    );

    // input_vector_pair
    Bench_with_input! {
        c,"add","random float [0,1)",&input_vector_pair,myinput
        {
            for cx in 0..100 {
                let (ca,cb) = myinput[cx];
                black_box(ca.add(black_box(cb)));
            }
        }
    }

    Bench_with_input! {
        c,"sub","random float [0,1)",&input_vector_pair,myinput
        {
            for cx in 100..200 {
                let (ca,cb) = myinput[cx];
                black_box(ca.sub(black_box(cb)));
            }
        }
    }

    Bench_with_input! {
        c,"mul_inner","random float [0,1)",&input_vector_pair,myinput
        {
            for cx in 200..300 {
                let (ca,cb) = myinput[cx];
                black_box(ca.mul_inner(black_box(cb)));
            }
        }
    }

    Bench_with_input! {
        c,"dist","random float [0,1)",&input_vector_pair,myinput
        {
            for cx in 300..400 {
                let (ca,cb) = myinput[cx];
                black_box(ca.dist(black_box(cb)));
            }
        }
    }

    Bench_with_input! {
        c,"angle","random float [0,1)",&input_vector_pair,myinput
        {
            for cx in 400..500 {
                let (ca,cb) = myinput[cx];
                black_box(ca.angle(black_box(cb)));
            }
        }
    }

    // input_vector_scalar_pair
    Bench_with_input! {
        c,"mul_scalar","random float [0,1)",&input_vector_scalar_pair,myinput
        {
            for cx in 0..100 {
                let (ca,cb) = (myinput.0[cx],myinput.1[cx]);
                black_box(ca.mul_scalar(black_box(cb)));
            }
        }
    }

    Bench_with_input! {
        c,"div_scalar","random float [0,1)",&input_vector_scalar_pair,myinput
        {
            for cx in 100..200 {
                let (ca,cb) = (myinput.0[cx],myinput.1[cx]);
                black_box(ca.div_scalar(black_box(cb)));
            }
        }
    }

    // input_vector_single
    Bench_with_input! {
        c,"len","random float [0,1)",&input_vector_single,myinput
        {
            for cx in 0..100 {
                let ca = myinput[cx];
                black_box(ca.len());
            }
        }
    }

    Bench_with_input! {
        c,"norm","random float [0,1)",&input_vector_single,myinput
        {
            for cx in 100..200 {
                let ca = myinput[cx];
                black_box(ca.norm());
            }
        }
    }
}

criterion_group!(benches, standard_vector3);
criterion_main!(benches);
