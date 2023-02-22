use std::f64::consts::PI;

use crate::objects::{Ray, Intersection, Color};
use crate::parser::Scene;
use crate::vec3::Vec3;

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
    let reflected_ray = Ray {pos: inter.pos + reflected_ray_dir * 0.001, dir: reflected_ray_dir};
    let reflected_color = cast_ray(scene, reflected_ray, limit - 1);
    let r2 = (local_color.r as f64 * (1.0 - inter.reflective) + reflected_color.r as f64 * inter.reflective) as u8;
    let g2 = (local_color.g as f64 * (1.0 - inter.reflective) + reflected_color.g as f64 * inter.reflective) as u8;
    let b2 = (local_color.b as f64 * (1.0 - inter.reflective) + reflected_color.b as f64 * inter.reflective) as u8;
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

pub fn render (width: u32, height: u32, scene: &Scene) -> Vec<u8>{
    let mut vec = vec![0u8; (width * height * 3).try_into().unwrap()];
    let angle_rad = scene.camera.hfov * PI / 180.0;
    let viewport_width = (angle_rad / 2.0).tan() * 2.0;
    let viewport_height = (viewport_width / width as f64) * height as f64;
    let w = scene.camera.dir * -1.0;
    let u = scene.camera.up.cross(w).normalize();
    let v = w.cross(u).normalize();
    let origin = scene.camera.pos;
    let horizontal = u * viewport_width * -1.0;
    let vertical = v * viewport_height;
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w;

    for i in 0..height {
        for j in 0..width {
            
            let r = Ray {pos: origin, dir: (lower_left_corner + vertical * (i as f64 / height as f64) + horizontal * (j as f64 / width as f64) - origin).normalize()};
            let offset:usize = j as usize * 3 + i as usize * width as usize * 3;
            let color = cast_ray(scene, r, scene.mirror_rec_depth);
            vec[offset + 0] = color.r;
            vec[offset + 1] = color.g;
            vec[offset + 2] = color.b;
        }
    }
    vec
}
