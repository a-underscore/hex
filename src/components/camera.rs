use crate::cid;
use hecs::component_manager::Component;
use hex_math::{Ortho, Vec2};

#[derive(Clone)]
pub struct Camera {
    radius: (Vec2, f32),
    view: Ortho,
    pub active: bool,
}

impl Camera {
    pub fn new(radius: (Vec2, f32), active: bool) -> Self {
        Self {
            radius,
            view: Self::calculate_view(&radius),
            active,
        }
    }

    pub fn radius(&self) -> (Vec2, f32) {
        self.radius
    }

    pub fn set_radius(&mut self, radius: (Vec2, f32)) {
        self.radius = radius;

        self.update_view();
    }

    pub fn view(&self) -> Ortho {
        self.view
    }

    fn update_view(&mut self) {
        self.view = Self::calculate_view(&self.radius);
    }

    fn calculate_view((v, z): &(Vec2, f32)) -> Ortho {
        Ortho::new(-v.x(), v.x(), -v.y(), v.y(), -z, *z)
    }
}

impl Component for Camera {
    fn id() -> usize {
        cid!()
    }
}
