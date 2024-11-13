use nalgebra_glm::Vec3;
pub struct PlanetTrail {
    pub positions: Vec<Vec3>,
    pub max_length: usize,
}

impl PlanetTrail {
    pub fn new(max_length: usize) -> Self {
        PlanetTrail {
            positions: Vec::with_capacity(max_length),
            max_length,
        }
    }

    pub fn add_position(&mut self, position: Vec3) {
        if self.positions.len() >= self.max_length {
            self.positions.remove(0); // Elimina la posición más antigua
        }
        self.positions.push(position);
    }
}
