# glmath

A collection of fast and convenient, graphics-oriented types.

This includes `Vector2`, `Vector3`, `Vector4`; along with the associated square matrices `Matrix2x2`, `Matrix3x3`, `Matrix4x4`; along with a simple `Quaternion` implementation. All of the content in this crate has been added by an *as-needed* basis. My rust, opengl crate [winter](https://github.com/fourrth/winter.git) uses this library for all of it's vector manipulation

## Usage

First, clone the repository,
```
git clone https://github.com/fourrth/glmath.git
```

and then add this to your `Cargo.toml`

```
[dependencies]
glmath = {version = "1.0", path = "path/to/cloned/crate"}
```

## Features

For using features, simply edit `Cargo.toml` to:

```
[dependencies]
glmath = {version = "1.0", path = "path/to/cloned/crate", features = ["feature1, feature2"]}
```

### quaternions

On by default: adds quaternions type and functions

### random_vectors

Adds some functions for which integrate well when you need randomly generated vectors and matrices 

## Testing

This project does have unit tests for each module. To run the unit tests, run:
```
cargo test
```

## Benchmarks

This library does have benchmarks through the `criterion` crate. To run the benchmarks, run:
```
cargo bench --features="required_feature" --bench=feature_to_benchmark
```

See command output for more detail on which benchmarks are available and what features they require.

For more info on criterion, see it's [documentation](https://bheisler.github.io/criterion.rs/book/index.html).

## Compatibility

This crate is used and created with  Rust 2021 and `rustc` 1.60+.

## License

This crate — along with any subsequent additions or revisions — are all dual licensed under [MIT License](LICENSE-MIT) or [Apache License](LICENSE-APACHE) at your option.