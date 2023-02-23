use crate::objects::{Ray, Intersection, Color};
use crate::parser::Scene;
use crate::vec3::random_in_unit_sphere;
use rand::prelude::*;
use rayon::prelude::*;

fn calc_light (inter: Intersection, scene: &Scene, ray: Ray, limit: u32) -> Color {
    let mut intensity = scene.ambientlight.intensity;
    for l in &scene.pointlights {
        let mut factor = 0.0;
        let i_to_l = l.pos - inter.pos;
        
        let shadow_ray = Ray {pos: inter.pos + i_to_l * 0.001, dir: i_to_l};
        let shadow_intersect = ray_intersect(scene, shadow_ray);
        match shadow_intersect {
            None => {
                let tmp = inter.norm.dot(i_to_l);
                if tmp >= 0.0 {
                    factor += tmp / (inter.norm.len() * i_to_l.len());
                    let reflection = inter.norm * 2.0 * tmp - i_to_l;
                    let i_to_rayorigin = ray.pos - inter.pos;
                    let tmp2 = reflection.dot(i_to_rayorigin);
                    if tmp2 >= 0.0 && inter.specular != -1 {
                        factor += (tmp2 / (reflection.len() * i_to_rayorigin.len())).powi(inter.specular);
                    }
                    //if factor > 1.0 {
                    //    factor = 1.0;
                    //}
                    intensity += l.intensity * factor;
                }
            },
            _ => ()
        }
    }
    let r = (inter.obj_color.r as f64 * intensity) as u8;
    let g = (inter.obj_color.g as f64 * intensity) as u8;
    let b = (inter.obj_color.b as f64 * intensity) as u8;
    let local_color = Color { r: r, g: g, b: b };
    if limit <= 0 || inter.reflective <= 0.0 {
        return local_color;
    }
    let reflected_ray_dir = inter.norm * 2.0 * inter.norm.dot(ray.dir * -1.0) - (ray.dir * -1.0);
    let mut r2:u32 = 0;
    let mut g2:u32 = 0;
    let mut b2:u32 = 0;
    for _ in 0..scene.samples_per_pixel {
        let reflected_ray = Ray {pos: inter.pos + reflected_ray_dir * 0.001, dir: (reflected_ray_dir * 1.2 + random_in_unit_sphere() * 0.2).normalize()};
        let reflected_color = cast_ray(scene, reflected_ray, limit - 1);
        r2 += reflected_color.r as u32;
        g2 += reflected_color.g as u32;
        b2 += reflected_color.b as u32;
    }
    let r2 = (local_color.r as f64 * (1.0 - inter.reflective) + (r2 as f64 / scene.samples_per_pixel as f64) * inter.reflective) as u8;
    let g2 = (local_color.g as f64 * (1.0 - inter.reflective) + (g2 as f64 / scene.samples_per_pixel as f64) * inter.reflective) as u8;
    let b2 = (local_color.b as f64 * (1.0 - inter.reflective) + (b2 as f64 / scene.samples_per_pixel as f64) * inter.reflective) as u8;
    return Color { r: r2, g: g2, b: b2 };
}

fn ray_intersect (scene: &Scene, ray: Ray) -> Option<Intersection>{
    let mut closest_intersect:Intersection = Intersection { pos: ray.pos, dist_from_ray_origin: f64::MAX, norm: ray.pos, obj_color: Color { r: 0, g: 0, b: 0 }, specular: -1, reflective: -1.0};
    for sp in &scene.spheres {
        let i = sp.intersect(ray);
        match i {
            Some(x) => {
                if x.dist_from_ray_origin < closest_intersect.dist_from_ray_origin {
                    closest_intersect = x;
                }
            },
            _ => ()
        }
    }
    if closest_intersect.dist_from_ray_origin == f64::MAX {
        return None
    }
    Some(closest_intersect)
}

fn cast_ray (scene: &Scene, ray: Ray, limit: u32) -> Color{
    let inter = ray_intersect(scene, ray);
    match inter {
        Some(x) => {
            calc_light(x, scene, ray, limit)
        },
        None => {
            Color {r: 0, g: 0, b: 0}
        }
    }
}

pub fn render (scene: &Scene) -> Vec<u8>{
    let mut vec = vec![0u8; (scene.width * scene.height * 3).try_into().unwrap()];
    let bands: Vec<(usize, &mut [u8])> = vec.chunks_mut(scene.width as usize * 3).enumerate().collect();
    bands.into_par_iter().for_each(|(i, band)| {
        let mut rng = thread_rng();
        for (j, col) in band.chunks_mut(3).enumerate() {
            let mut r: u32 = 0;
            let mut g: u32 = 0;
            let mut b: u32 = 0;
            for _ in 0..scene.samples_per_pixel {
                let mut x = j as f64;
                let mut y = i as f64;
                if scene.samples_per_pixel > 1 {
                    x += rng.gen::<f64>();
                    y += rng.gen::<f64>();
                }
                let ray = scene.camera.get_ray(scene, x, y);
                let tmp_col = cast_ray(scene, ray, scene.mirror_rec_depth);
                r += tmp_col.r as u32;
                g += tmp_col.g as u32;
                b += tmp_col.b as u32;
            }
            col[0] = (r / scene.samples_per_pixel) as u8;
            col[1] = (g / scene.samples_per_pixel) as u8;
            col[2] = (b / scene.samples_per_pixel) as u8;
        }
    });
    vec
}
