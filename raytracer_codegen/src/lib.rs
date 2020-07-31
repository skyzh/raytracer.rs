extern crate proc_macro;
use proc_macro2::TokenStream;
use quote::quote;
use rand::Rng;

#[derive(Copy, Clone, Debug)]
struct Vec3(f32, f32, f32);

#[derive(Clone, Copy, Debug)]
struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}

fn overlap(positions: &Vec<(Vec3, f32)>, pos: Vec3, size: f32, offset: f32) -> bool {
    !positions.iter().all(|(that_pos, that_size)| {
        (that_pos.0 - pos.0).powi(2) + (that_pos.1 - pos.1).powi(2) + (that_pos.2 - pos.2).powi(2)
            > (that_size + size + offset).powf(2.0) as f32
    })
}

#[derive(Copy, Clone, Debug)]
struct Sphere {
    pub pos: Vec3,
    pub size: f32,
    pub color: Vec3,
}

fn generate_spheres() -> Vec<Sphere> {
    let mut rng = rand::thread_rng();
    let mut positions: Vec<(Vec3, f32)> = vec![];
    let mut spheres = vec![];
    let total = 500;
    for _ in 0..total {
        let (pos, size) = loop {
            let size = rng.gen_range(0.05, 0.2);
            let pos = Vec3(
                rng.gen_range(-5.0, 5.0),
                size - 0.5,
                rng.gen_range(-5.0, 5.0),
            );
            if !overlap(&positions, pos, size, 0.0) {
                positions.push((pos, size));
                break (pos, size);
            }
        };

        let color = Vec3(
            rng.gen_range(0.3, 0.6),
            rng.gen_range(0.3, 0.6),
            rng.gen_range(0.3, 0.6),
        );

        let sphere = Sphere { pos, size, color };
        spheres.push(sphere);
    }
    spheres
}

#[proc_macro]
pub fn make_spheres(_item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut tokens = vec![];

    let spheres = generate_spheres();
    for sphere in spheres.iter() {
        let sphere_src = sphere_code(sphere);
        let glass_src = glass_code(sphere);
        tokens.push(quote! {
            sphere_objects.push(#sphere_src);
            sphere_objects.push(#glass_src);
        });
    }
    let x: TokenStream = tokens.into_iter().collect();
    proc_macro::TokenStream::from(quote! {
        fn make_spheres_bvh() -> Box<dyn Hitable> {
            let mut sphere_objects: Vec<Box<dyn Hitable>> = Vec::new();
            #x
            let bvh_node = BVHNode::new(HitableList {
                hitables: sphere_objects,
            });
            bvh_node
        }
    })
}

fn sphere_code(sphere: &Sphere) -> TokenStream {
    let Vec3(x, y, z) = sphere.pos;
    let Vec3(r, g, b) = sphere.color;
    let size = sphere.size * 0.9;

    quote! {
        box Sphere {
            center: Vec3::new(#x, #y, #z),
            radius: #size,
            material: DiffuseLight::new(
                ConstantTexture::new(
                    Vec3::new(#r, #g, #b)
                )
            )
        }
    }
}

fn glass_code(sphere: &Sphere) -> TokenStream {
    let Vec3(x, y, z) = sphere.pos;
    let size = sphere.size;

    quote! {
        box Sphere {
            center: Vec3::new(#x, #y, #z),
            radius: #size,
            material: Dielectric { ref_idx: 1.5 },
        }
    }
}

fn construct_bvh(mut spheres: Vec<Sphere>) -> TokenStream {
    match spheres.len() {
        0 => unreachable!(),
        1 => {
            let sphere = sphere_code(spheres.first().unwrap());
            let glass = glass_code(spheres.first().unwrap());
            quote! { box BVHNodeStatic::construct(#sphere, #glass) }
        }
        _ => {
            let axis = rand::thread_rng().gen_range(0, 3);
            spheres.sort_by(|a, b| match axis {
                0 => a.pos.0.partial_cmp(&b.pos.0).unwrap(),
                1 => a.pos.1.partial_cmp(&b.pos.1).unwrap(),
                2 => a.pos.2.partial_cmp(&b.pos.2).unwrap(),
                _ => unreachable!(),
            });
            let mut a = spheres;
            let b = a.split_off(a.len() / 2);
            let left = construct_bvh(a);
            let right = construct_bvh(b);
            quote! { box BVHNodeStatic::construct(#left, #right) }
        }
    }
}

#[proc_macro]
pub fn make_spheres_static(_item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let spheres = generate_spheres();
    let x: TokenStream = construct_bvh(spheres);
    proc_macro::TokenStream::from(quote! {
        fn make_spheres_static_bvh() -> Box<dyn Hitable> {
            let mut sphere_objects: Vec<Box<dyn Hitable>> = Vec::new();
            #x
        }
    })
}
