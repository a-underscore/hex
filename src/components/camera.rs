use crate::cid;
use hecs::component_manager::Component;
use hex_math::{Ortho, Vec2};

#[derive(Clone)]
pub struct Camera {
    dimensions: (Vec2, f32),
    view: Ortho,
    pub active: bool,
}

impl Camera {
    pub fn new(dimensions: (Vec2, f32), active: bool) -> Self {
        Self {
            dimensions,
            view: Self::calculate_view(dimensions),
            active,
        }
    }

    pub fn dimensions(&self) -> (Vec2, f32) {
        self.dimensions
    }

    pub fn set_dimensions(&mut self, dimensions: (Vec2, f32)) {
        self.dimensions = dimensions;

        self.update_view();
    }

    pub fn view(&self) -> Ortho {
        self.view
    }

    fn update_view(&mut self) {
        self.view = Self::calculate_view(self.dimensions);
    }

    fn calculate_view((v, z): (Vec2, f32)) -> Ortho {
        Ortho::new(-v.x(), v.x(), -v.y(), v.y(), -z, z)
    }
}

impl Component for Camera {
    fn id() -> usize {
        cid!()
    }
}
