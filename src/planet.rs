use crate::{Color, Fragment, Uniforms};
use fastnoise_lite::FastNoiseLite; // For FastNoiseLite type
use nalgebra_glm::Vec3; // For Vec3 type // Replace `some_crate` with the actual crate name where Fragment, Uniforms, and Color are defined

// Your existing code
type PlanetShaderFn = fn(&Fragment, &Uniforms) -> Color;

pub struct Planet {
    pub translation: Vec3,
    pub rotation: Vec3,
    pub scale: f32,
    pub obj_path: String,
    pub shader: PlanetShaderFn,
    pub noise: FastNoiseLite,
}

impl Planet {
    fn new(
        translation: Vec3,
        rotation: Vec3,
        scale: f32,
        obj_path: &str,
        shader: PlanetShaderFn,
        noise: FastNoiseLite,
    ) -> Self {
        Planet {
            translation,
            rotation,
            scale,
            obj_path: obj_path.to_string(),
            shader,
            noise,
        }
    }
}
