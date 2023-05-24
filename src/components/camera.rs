use crate::{
    ecs::{component_manager::Component, Id},
    id,
    math::{Ortho, Vec2d},
};

#[derive(Clone)]
pub struct Camera {
    pub active: bool,
    dimensions: (Vec2d, f32),
    proj: Ortho,
}
impl Camera {
    pub fn new(dimensions: (Vec2d, f32), active: bool) -> Self {
        Self {
            dimensions,
            proj: Self::calculate_proj(dimensions),
            active,
        }
    }

    pub fn dimensions(&self) -> (Vec2d, f32) {
        self.dimensions
    }

    pub fn set_dimensions(&mut self, dimensions: (Vec2d, f32)) {
        self.dimensions = dimensions;

        self.update_proj();
    }

    pub fn proj(&self) -> Ortho {
        self.proj
    }

    pub fn update_proj(&mut self) {
        self.proj = Self::calculate_proj(self.dimensions);
    }

    pub fn calculate_proj((v, z): (Vec2d, f32)) -> Ortho {
        let v = v / 2.0;
        let z = z / 2.0;

        Ortho::new(-v.x(), v.x(), -v.y(), v.y(), -z, z)
    }
}

impl Component for Camera {
    fn id() -> Id {
        id!()
    }
}
