#[macro_use]
extern crate criterion;

use criterion::Criterion;
use raytracer::{
    renderer::{utils::render_to_file, BasicRenderer, Renderer, ThreadedRenderer},
    scenes::simple_scene::{simple_scene_1, simple_scene_2},
};
use std::sync::Arc;

fn render_simple_scene_threaded_1() {
    let (hitable_list, camera) = simple_scene_1();
    let renderer = ThreadedRenderer {
        hitable_list: Arc::new(hitable_list),
        camera: Arc::new(camera),
        size: (400, 200),
        anti_aliasing: 50,
        block_count: (4, 2),
        workers: 2,
        ambient_light: true,
    };
    renderer.render();
}

fn render_simple_scene_basic_1() {
    let (hitable_list, camera) = simple_scene_1();
    let renderer = BasicRenderer {
        hitable_list: &hitable_list,
        camera: &camera,
        size: (400, 200),
        anti_aliasing: 50,
        crop_region: ((0, 0), (400, 200)),
        ambient_light: true,
    };
    renderer.render();
}

fn render_simple_scene_threaded_2() {
    let (hitable_list, camera) = simple_scene_2();
    let renderer = ThreadedRenderer {
        hitable_list: Arc::new(hitable_list),
        camera: Arc::new(camera),
        size: (400, 200),
        anti_aliasing: 50,
        block_count: (4, 2),
        workers: 2,
        ambient_light: true,
    };
    renderer.render();
}

fn render_simple_scene_basic_2() {
    let (hitable_list, camera) = simple_scene_2();
    let renderer = BasicRenderer {
        hitable_list: &hitable_list,
        camera: &camera,
        size: (400, 200),
        anti_aliasing: 50,
        crop_region: ((0, 0), (400, 200)),
        ambient_light: true,
    };
    renderer.render();
}

fn simple_scene_benchmark(c: &mut Criterion) {
    c.bench_function("render simple scene 1 with threaded renderer", |b| {
        b.iter(|| render_simple_scene_threaded_1())
    });
    c.bench_function("render simple scene 1 with basic renderer", |b| {
        b.iter(|| render_simple_scene_basic_1())
    });
    c.bench_function("render simple scene 2 with threaded renderer", |b| {
        b.iter(|| render_simple_scene_threaded_2())
    });
    c.bench_function("render simple scene 2 with basic renderer", |b| {
        b.iter(|| render_simple_scene_basic_2())
    });
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(5);
    targets = simple_scene_benchmark
);
criterion_main!(benches);
