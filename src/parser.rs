use std::fs::File;
use std::io::prelude::*;

use serde::{Deserialize, Serialize};
use crate::vec3::Vec3;

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    pos: Vec3,
    dir: Vec3,
    phones: Vec<String>,
}

pub fn parse() {
    let mut file = File::open("scene.json").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    //dbg!(contents);
    let v: Person = serde_json::from_str(&contents.as_str()).unwrap();
    let x = v.pos - v.dir;
    dbg!(x);
    dbg!(v.pos * 3.0);
}
