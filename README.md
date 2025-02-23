# glmath

A collection of fast and convenient, graphics-oriented types. 

This includes `Vector2`, `Vector3`, `Vector4`; along with the associated square matrices `Matrix2x2`, `Matrix3x3`, `Matrix4x4`; along with a simple `Quaternion` implementation. 

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
### random_vectors

Adds some functions for which integrate well when you need randomly generated vectors and matrices 

## Compatibility

This crate is used and created with with Rust 2021 and `rustc` 1.60+.

## License

This crate — along with any subsequent additions or revisions — are all dual licensed under [MIT License](LICENSE-MIT) or [Apache License](LICENSE-APACHE) at your option.