# Raytracer.rs

[![Build Status](https://travis-ci.com/SkyZH/raytracer.rs.svg?branch=master)](https://travis-ci.com/SkyZH/raytracer.rs)

_The "Ray Tracing in One Weekend" implemented in Rust 2018_

[Ray Tracing in One Weekend (Ray Tracing Minibooks Book 1) on Amazon](https://www.amazon.com/Ray-Tracing-Weekend-Minibooks-Book-ebook/dp/B01B5AODD8/)

This is the new branch implemented in Rust 2018.

For previous version, refer to [Legacy](https://github.com/SkyZH/raytracer.rs/tree/legacy)

## Screenshots

### World of Spheres 

`scenes/complex_scene.rs:complex_scene_1`

Generate 300 spheres (no overlap) in the space with different materials and settings.

It takes about *800 secs* to render.    
Parameters: `1600x1600`, anti-aliasing `256`, workers `4`.

![complex_scene_1](https://user-images.githubusercontent.com/4198311/51087070-38a7f300-1789-11e9-9b84-d96f9bb1d556.png)


## Usage

Write your scene in `main.rs`, and then run:    
```bash
RUST_LOG="raytracer=info" cargo run --release
```

## Run Tests and Benchmarks

```bash
cargo bench
cargo test
```

And the image will be generated at `/output/test.png`.
