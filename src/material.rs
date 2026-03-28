use serde::Deserialize;
use crate::vec3::Vec3;

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum Material {
    Matte { 
        color: Vec3
    },
    Mirror,
    Glass { 
        index_of_refraction: f64 // 1.0 is air, 1.5 is glass, 2.4 is diamond
    },
}