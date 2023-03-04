use crate::{cid, ecs::component_manager::Component};
use cgmath::{Matrix4, Vector3};

#[derive(Clone)]
pub struct Camera {
    dimensions: Vector3<f32>,
    view: Matrix4<f32>,
    pub active: bool,
}

impl Camera {
    pub fn new(dimensions: Vector3<f32>, active: bool) -> Self {
        Self {
            dimensions,
            view: Self::calculate_view(dimensions),
            active,
        }
    }

    pub fn dimensions(&self) -> Vector3<f32> {
        self.dimensions
    }

    pub fn set_dimensions(&mut self, dimensions: Vector3<f32>) {
        self.dimensions = dimensions;

        self.update_view();
    }

    pub fn view(&self) -> Matrix4<f32> {
        self.view
    }

    fn update_view(&mut self) {
        self.view = Self::calculate_view(self.dimensions);
    }

    fn calculate_view(dimensions: Vector3<f32>) -> Matrix4<f32> {
        let dimensions = dimensions / 2.0;

        cgmath::ortho(
            -dimensions.x,
            dimensions.x,
            -dimensions.y,
            dimensions.y,
            -dimensions.z,
            dimensions.z,
        )
    }
}

impl Component for Camera {
    fn id() -> usize {
        cid!()
    }
}
