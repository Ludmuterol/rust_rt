use serde::{Deserialize, Serialize};
use crate::{vec3::Vec3, parser::Scene, objects::Ray};
use std::f64::consts::PI;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Camera {
    pub pos: Vec3,
    pub dir: Vec3,
    pub up: Vec3,
    pub hfov: f64,
}

impl Camera {
    pub fn get_viewport_size(self, scene: &Scene) -> (f64, f64) {
        let angle_rad = self.hfov * PI / 180.0;
        let viewport_width = (angle_rad / 2.0).tan() * 2.0;
        let viewport_height = (viewport_width / scene.width as f64) * scene.height as f64;
        (viewport_width, viewport_height)
    }

    pub fn get_hor_ver_vec(self, scene: &Scene) -> (Vec3, Vec3) {
        let (_, u, v) = self.get_wuv();
        let (viewport_width, viewport_height) = self.get_viewport_size(scene);
        let horizontal = u * viewport_width * -1.0;
        let vertical = v * viewport_height;
        (horizontal, vertical)
    }

    pub fn get_lower_left_corner(self, scene: &Scene) -> Vec3 {
        let (w, ..) = self.get_wuv();
        let (horizontal, vertical) = self.get_hor_ver_vec(scene);
        self.pos - horizontal / 2.0 - vertical / 2.0 - w
    }
    pub fn get_wuv(self) -> (Vec3, Vec3, Vec3) {
        let w = self.dir * -1.0;
        let u = self.up.cross(w).normalize();
        let v = w.cross(u).normalize();
        (w, u, v)
    }
    pub fn get_ray(self, scene: &Scene, c_v: f64, c_h: f64) -> Ray {

        let (horizontal, vertical) = self.get_hor_ver_vec(scene);
        Ray {
                pos: scene.camera.pos, 
                dir: (
                    scene.camera.get_lower_left_corner(scene) + 
                    vertical * (c_h / scene.height as f64) + 
                    horizontal * (c_v / scene.width as f64) - 
                    scene.camera.pos
                ).normalize()
            }
    }
}
