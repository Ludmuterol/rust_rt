use serde::{Deserialize, Serialize};
use crate::vec3::Vec3;
use crate::solve_quadratic;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Ray {
    pos: Vec3,
    dir: Vec3,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Sphere {
    pos: Vec3,
    radius: f64,
}

pub struct Intersection {
    pos: Vec3,
    dist_from_ray_origin: f64,
    norm: Vec3,
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
                        return Some(Intersection { pos: pos, dist_from_ray_origin: ray.dir.len() * x, norm: (pos - self.pos).normalize() })
                    }
                    let pos = ray.pos + ray.dir * y;
                    return Some(Intersection { pos: pos, dist_from_ray_origin: ray.dir.len() * y, norm: (pos - self.pos).normalize() })
                }
                if x > 0.0 {
                    let pos = ray.pos + ray.dir * x;
                    return Some(Intersection { pos: pos, dist_from_ray_origin: ray.dir.len() * x, norm: (pos - self.pos).normalize() })
                }
                if y > 0.0 {
                    let pos = ray.pos + ray.dir * y;
                    return Some(Intersection { pos: pos, dist_from_ray_origin: ray.dir.len() * y, norm: (pos - self.pos).normalize() })
                }
                None
            },
            (Some(x), None) => {
                if x > 0.0 {
                    let pos = ray.pos + ray.dir * x;
                    return Some(Intersection { pos: pos, dist_from_ray_origin: ray.dir.len() * x, norm: (pos - self.pos).normalize() })
                }
                None
            },
            _ => None,
        }
    }
}
