use serde::{Deserialize, Serialize};
use crate::vec3::Vec3;
use crate::solve_quadratic;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub pos: Vec3,
    pub dir: Vec3,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Camera {
    pub pos: Vec3,
    pub dir: Vec3,
    pub up: Vec3,
    pub hfov: f64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PointLight {
    pub pos: Vec3,
    pub intensity: f64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct AmbientLight {
    pub intensity: f64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Sphere {
    pub pos: Vec3,
    pub radius: f64,
    pub color: Color,
    pub specular: i32,
    pub reflective: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct Intersection {
    pub pos: Vec3,
    pub dist_from_ray_origin: f64,
    pub norm: Vec3,
    pub obj_color: Color,
    pub specular: i32,
    pub reflective: f64,
}

impl Sphere {
    pub fn intersect(self, ray: Ray) -> Option<Intersection> {
        let a:f64 = ray.dir.dot(ray.dir);
        let b:f64 = 2.0 * ray.dir.dot(ray.pos - self.pos);
        let c:f64 = (ray.pos - self.pos).dot(ray.pos - self.pos) - self.radius * self.radius;
        let roots = solve_quadratic::solve_quadratic(a, b, c);
        let mut inter:Intersection = Intersection { pos: ray.pos, dist_from_ray_origin: f64::MAX, norm: ray.pos, obj_color: self.color, specular: self.specular, reflective: self.reflective};
        match roots {
            (Some(x), Some(y)) => {
                if x > 0.0 && y > 0.0 {
                    if x <= y {
                        inter.pos = ray.pos + ray.dir * x;
                        inter.dist_from_ray_origin = ray.dir.len() * x;
                        inter.norm = (inter.pos - self.pos).normalize();
                        return Some(inter)
                    }
                    inter.pos = ray.pos + ray.dir * y;
                    inter.dist_from_ray_origin = ray.dir.len() * y;
                    inter.norm = (inter.pos - self.pos).normalize();
                    return Some(inter)
                }
                if x > 0.0 {
                    inter.pos = ray.pos + ray.dir * x;
                    inter.dist_from_ray_origin = ray.dir.len() * x;
                    inter.norm = (inter.pos - self.pos).normalize();
                    return Some(inter)
                }
                if y > 0.0 {
                    inter.pos = ray.pos + ray.dir * y;
                    inter.dist_from_ray_origin = ray.dir.len() * y;
                    inter.norm = (inter.pos - self.pos).normalize();
                    return Some(inter)
                }
                None
            },
            (Some(x), None) => {
                if x > 0.0 {
                    inter.pos = ray.pos + ray.dir * x;
                    inter.dist_from_ray_origin = ray.dir.len() * x;
                    inter.norm = (inter.pos - self.pos).normalize();
                    return Some(inter)
                }
                None
            },
            _ => None,
        }
    }
}
