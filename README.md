# Raytracer.rs

[![Build Status](https://travis-ci.com/skyzh/raytracer.rs.svg?branch=master)](https://travis-ci.com/skyzh/raytracer.rs)

_"Ray Tracing in One Weekend ([Amazon](https://www.amazon.com/Ray-Tracing-Weekend-Minibooks-Book-ebook/dp/B01B5AODD8/))" implemented in Rust 2018_

This is the new branch implemented in Rust 2018. For previous version, refer to [legacy](https://github.com/SkyZH/raytracer.rs/tree/legacy) branch.

## Usage

Write your specifications in `main.rs`:    
```rust
use self::renderer::utils::render_high_quality as render;
use self::scenes::simple_scene::simple_scene_perlin_noise as scene;
```    
Here you can change `render_high_quality` to `render_preview` to render faster. And you can select from examples scenes by changing `self::scenes::****::****`.    

```rust
render(hitable_list, camera, "scene.png", true)?;
```    
The third parameter indicates that with previous rendering settings, the image will be rendered to `output/scene.png`. 

The fourth parameter indicates whether to enable ambient light or not.

Finally run:    
```bash
cargo run --release
```

Note that since the latest commit, some functionalities may be broken due to new features.

## Showcases

### Cornell Box
**Cornell Box** `scenes/cornell_box.rs:cornell_box`    
**Updated: mixture pdf and reduced noise**

<img width="45%" src="https://user-images.githubusercontent.com/4198311/51380265-4922e980-1b4c-11e9-85e2-1d5430a47b22.png"> <img width="45%" src="https://user-images.githubusercontent.com/4198311/69005795-ee237400-0961-11ea-82dd-2345c9aecabd.png">

### Cover Scene

The scene from the cover of "Ray Tracing in One Weekend". It takes *~450 secs* to render.

`scenes/legacy_scene.rs:legacy_scene`

### Sphere Sea 

Generate 300 spheres (no overlap) in the space with different materials and settings.

It takes *~560 secs* to render.

`scenes/complex_scene.rs:complex_scene_2`

<img width="45%" src="https://user-images.githubusercontent.com/4198311/51119409-bcc3ae80-184d-11e9-8986-9ff48cf80e9d.png"> <img width="45%" src="https://user-images.githubusercontent.com/4198311/51087490-17e29c00-178f-11e9-88fc-996f642859d0.png">

### Textures

**Light Scene** `scenes/legacy_scene.rs:legacy_scene_light`    
**Perlin Noise** `scenes/simple_scene.rs:simple_scene_perlin_noise`    
**Checker Texture** `scenes/legacy_scene.rs:legacy_scene_texture`    

<img width="30%" src="https://user-images.githubusercontent.com/4198311/51253655-e9a4cc80-19d9-11e9-9ea4-ddec4e4208eb.png"> <img width="30%" src="https://user-images.githubusercontent.com/4198311/51223750-1b884580-197e-11e9-93c8-f4c8779d1958.png"> <img width="30%" src="https://user-images.githubusercontent.com/4198311/51226964-f9e28a80-198c-11e9-95ee-374e3598adc2.png">

## Run Tests and Benchmarks

```bash
cargo bench
cargo test
```
