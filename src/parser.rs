use std::fs::File;
use std::io::prelude::*;

use serde::{Deserialize, Serialize};
use crate::objects::{Sphere, PointLight, AmbientLight};
use crate::camera::Camera;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scene {
    pub mirror_rec_depth: u32,
    pub samples_per_pixel: u32,
    pub width: u32,
    pub height: u32,
    pub camera: Camera,
    pub ambientlight: AmbientLight,
    pub spheres: Vec<Sphere>,
    pub pointlights: Vec<PointLight>,
}

pub fn parse() -> Scene{
    let mut file = File::open("scene.json").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    //dbg!(contents);
    let v: Scene = serde_json::from_str(&contents.as_str()).unwrap();
    //dbg!(v.clone());
    v
}
