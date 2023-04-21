use crate::{
    ecs::{component_manager::Component, Id},
    id,
    math::{Ortho, Vec2d},
};

#[derive(Clone)]
pub struct Camera2d {
    dimensions: (Vec2d, f32),
    view: Ortho,
    pub active: bool,
}

impl Camera2d {
    pub fn new(dimensions: (Vec2d, f32), active: bool) -> Self {
        Self {
            dimensions,
            view: Self::calculate_view(dimensions),
            active,
        }
    }

    pub fn dimensions(&self) -> (Vec2d, f32) {
        self.dimensions
    }

    pub fn set_dimensions(&mut self, dimensions: (Vec2d, f32)) {
        self.dimensions = dimensions;

        self.update_view();
    }

    pub fn view(&self) -> Ortho {
        self.view
    }

    pub fn update_view(&mut self) {
        self.view = Self::calculate_view(self.dimensions);
    }

    pub fn calculate_view((v, z): (Vec2d, f32)) -> Ortho {
        let v = v / 2.0;
        let z = z / 2.0;

        Ortho::new(-v.x(), v.x(), -v.y(), v.y(), -z, z)
    }
}

impl Component for Camera2d {
    fn id() -> Id {
        id!()
    }
}
