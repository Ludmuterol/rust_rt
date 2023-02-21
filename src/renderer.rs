use crate::objects::{self, Ray, Intersection, Color};
use crate::parser::Scene;
use crate::vec3::Vec3;

pub fn render (width: u32, height: u32, scene: &Scene) -> Vec<u8>{
    let mut vec = vec![0u8; (width * height * 3).try_into().unwrap()];

    let pos = Vec3 {x: 0.0, y: 0.0, z: 0.0};
    for i in 0..height {
        for j in 0..width {
            let r = Ray {pos: pos, dir: Vec3 { x: j as f64 / width as f64 - 0.5, y: i as f64 / height as f64 - 0.5, z: 1.0 }.normalize()};
            let mut closest_intersect:Intersection = Intersection { pos: pos, dist_from_ray_origin: f64::MAX, norm: pos, obj_color: Color { r: 0, g: 0, b: 0 }};
            for sp in &scene.spheres {
                let i = sp.intersect(r);
                match i {
                    Some(x) => {
                        if x.dist_from_ray_origin < closest_intersect.dist_from_ray_origin {
                            closest_intersect = x;
                        }
                    },
                    _ => ()
                }
            }
            let offset:usize = j as usize * 3 + i as usize * width as usize * 3;
            if closest_intersect.dist_from_ray_origin == f64::MAX {
                vec[offset + 0] = 0;
                vec[offset + 1] = 0;
                vec[offset + 2] = 0;
            }
            else {
                vec[offset + 0] = closest_intersect.obj_color.r;
                vec[offset + 1] = closest_intersect.obj_color.g;
                vec[offset + 2] = closest_intersect.obj_color.b;
            }
        }
    }
    vec
}
