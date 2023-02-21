use std::fs::File;
use std::io::prelude::*;

use serde::{Deserialize, Serialize};
use crate::vec3::Vec3;
use crate::objects::{Ray, Sphere, Intersection};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scene {
    pub spheres: Vec<Sphere>,
}

pub fn parse() -> Scene{
    let mut file = File::open("scene.json").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    //dbg!(contents);
    let v: Scene = serde_json::from_str(&contents.as_str()).unwrap();
    //dbg!(v.clone());
    //let r = Ray {pos: Vec3 {x: 0.0, y: 0.0, z: 0.0}, dir: Vec3 {x: 0.0, y: 1.0, z: 0.0}};
    //let i = v.spheres[0].intersect(r);
    //dbg!(i);
    v
}
