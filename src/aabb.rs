use std::mem::swap;

use crate::{vec3::Vec3, objects::Ray};

pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}

impl AABB {
    pub fn hit(self, r: Ray) -> bool {
        let mi = self.min - r.pos;
        let ma = self.max - r.pos;
        {
            let inv_d = 1.0 / r.dir.x;
            let mut t0 = mi.x * inv_d;
            let mut t1 = ma.x * inv_d;
            if inv_d < 0.0 {
                swap(&mut t0, &mut t1);
            }
            if t1 <= t0 {
                return false;
            }
        }
        {
            let inv_d = 1.0 / r.dir.y;
            let mut t0 = mi.y * inv_d;
            let mut t1 = ma.y * inv_d;
            if inv_d < 0.0 {
                swap(&mut t0, &mut t1);
            }
            if t1 <= t0 {
                return false;
            }
        }
        {
            let inv_d = 1.0 / r.dir.z;
            let mut t0 = mi.z * inv_d;
            let mut t1 = ma.z * inv_d;
            if inv_d < 0.0 {
                swap(&mut t0, &mut t1);
            }
            if t1 <= t0 {
                return false;
            }
        }
        return true;   
    }
}

pub fn surrounding_box(box1: AABB, box2: AABB) -> AABB {
    AABB { 
        min: Vec3 { x: box1.min.x.min(box2.min.x), y: box1.min.y.min(box2.min.y), z: box1.min.z.min(box2.min.z)},
        max: Vec3 { x: box1.max.x.max(box2.max.x), y: box1.max.y.max(box2.max.y), z: box1.max.z.max(box2.max.z)}
    }
}
