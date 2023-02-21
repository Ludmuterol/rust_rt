use serde::{Deserialize, Serialize};
use crate::vec3::Vec3;
use crate::solve_quadratic;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub pos: Vec3,
    pub dir: Vec3,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PointLight {
    pub pos: Vec3,
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
}

#[derive(Debug, Clone, Copy)]
pub struct Intersection {
    pub pos: Vec3,
    pub dist_from_ray_origin: f64,
    pub norm: Vec3,
    pub obj_color: Color,
    pub specular: i32,
}

impl Sphere {
    pub fn intersect(self, ray: Ray) -> Option<Intersection> {
        let a:f64 = ray.dir.dot(ray.dir);
        let b:f64 = 2.0 * ray.dir.dot(ray.pos - self.pos);
        let c:f64 = (ray.pos - self.pos).dot(ray.pos - self.pos) - self.radius * self.radius;
        let roots = solve_quadratic::solve_quadratic(a, b, c);
        match roots {
            (Some(x), Some(y)) => {
                if x > 0.0 && y > 0.0 {
                    if x <= y {
                        let pos = ray.pos + ray.dir * x;
                        return Some(Intersection { pos: pos, dist_from_ray_origin: ray.dir.len() * x, norm: (pos - self.pos).normalize(), obj_color: self.color, specular: self.specular})
                    }
                    let pos = ray.pos + ray.dir * y;
                    return Some(Intersection { pos: pos, dist_from_ray_origin: ray.dir.len() * y, norm: (pos - self.pos).normalize(), obj_color: self.color, specular: self.specular })
                }
                if x > 0.0 {
                    let pos = ray.pos + ray.dir * x;
                    return Some(Intersection { pos: pos, dist_from_ray_origin: ray.dir.len() * x, norm: (pos - self.pos).normalize(), obj_color: self.color, specular: self.specular })
                }
                if y > 0.0 {
                    let pos = ray.pos + ray.dir * y;
                    return Some(Intersection { pos: pos, dist_from_ray_origin: ray.dir.len() * y, norm: (pos - self.pos).normalize(), obj_color: self.color, specular: self.specular})
                }
                None
            },
            (Some(x), None) => {
                if x > 0.0 {
                    let pos = ray.pos + ray.dir * x;
                    return Some(Intersection { pos: pos, dist_from_ray_origin: ray.dir.len() * x, norm: (pos - self.pos).normalize(), obj_color: self.color, specular: self.specular })
                }
                None
            },
            _ => None,
        }
    }
}
