# Raytracer.rs

[![Build Status](https://travis-ci.com/SkyZH/raytracer.rs.svg?branch=master)](https://travis-ci.com/SkyZH/raytracer.rs)

_The "Ray Tracing in One Weekend" implemented in Rust 2018_

[Ray Tracing in One Weekend (Ray Tracing Minibooks Book 1) on Amazon](https://www.amazon.com/Ray-Tracing-Weekend-Minibooks-Book-ebook/dp/B01B5AODD8/)

This is the new branch implemented in Rust 2018.

For previous version, refer to [Legacy](https://github.com/SkyZH/raytracer.rs/tree/legacy)

## Screenshots

## Usage

Write your scene in `main.rs`, and then run:    
```bash
RUST_LOG=info cargo run --release
```

## Run Tests and Benchmarks

```bash
cargo bench
cargo test
```

And the image will be generated at `/output/test.png`.
