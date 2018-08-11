extern crate image;

mod tracer;
use tracer::Vec3;
use tracer::Ray;
use tracer::Sphere;
use tracer::World;
use tracer::Hitable;
use tracer::Camera;

fn color(ray: &Ray, world: &World) -> Vec3 {
    match world.hit(&ray, 0.0, std::f64::MAX) {
        Some(t) => {
            let n = t.normal;
            Vec3::new(n.x + 1.0, n.y + 1.0, n.z + 1.0) * 0.5
        },
        None => {
            let unit = ray.destination.unit();
            let t: f64 = (unit.y + 1.0) * 0.5;
            Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
        }
    }
}

fn main() {
    let width = 800;
    let height = 400;
    let mut imgbuf = image::RgbaImage::new(width, height);

    let antialiasing = 4;
    let camera = Camera {
        origin: Vec3::new(0.0, 0.0, 0.0),
        lower_left_corner: Vec3::new(-2.0, -1.0, -1.0),
        horizontal: Vec3::new(4.0, 0.0, 0.0),
        vertical: Vec3::new(0.0, 2.0, 0.0)
    };

    let mut world_items: Vec<Box<Hitable>> = Vec::new();
    world_items.push(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world_items.push(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));
    let world = World::new(world_items);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let u: f64 = x as f64 / width as f64;
        let v: f64 = 1.0 - y as f64 / height as f64;
        let ray = camera.get_ray(u, v);
        let color_f = color(&ray, &world);
        *pixel = color_f.rgba()
    }
    imgbuf.save("output/test.png").unwrap()
}
