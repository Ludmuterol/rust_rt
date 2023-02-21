use serde::{Deserialize, Serialize};
use std::ops::{Add, Sub, Mul, Div};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 { 
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 { 
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    } 
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, factor: f64) -> Vec3 {
        Vec3 { 
            x: self.x * factor,
            y: self.y * factor,
            z: self.z * factor
        }
    }
    
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, factor: f64) -> Vec3 {
        Vec3 { 
            x: self.x / factor,
            y: self.y / factor,
            z: self.z / factor
        }
    }
    
}

impl Vec3 {
    pub fn len(self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    pub fn normalize(self) -> Vec3 {
        self / self.len()
    }
    pub fn dot(self, other: Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    pub fn cross(self, other: Vec3) -> Vec3 {
        Vec3 { 
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z, 
            z: self.x * other.y - self.y * other.x
        }
    }
    pub fn dist(self, other: Vec3) -> f64 {
        (self - other).len()
    }
}
