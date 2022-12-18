use crate::cid;
use crate::ecs::Component;
use cgmath::{Vector3, Matrix4};
use once_cell::sync::Lazy;

#[derive(Clone)]
pub struct Camera {
    dims: Vector3<f32>,
    view: Matrix4<f32>,
    pub active: bool,
}

impl Camera {
    pub fn new(
        dims: Vector3<f32>,
        active: bool,
    ) -> Self {
        Self {
            dims,
            view: Self::calculate_view(dims),
            active,
        }
    }

    pub fn dims(&self) -> Vector3<f32> {
        self.dims
    }

    pub fn set_dims(&mut self, dims: Vector3<f32>) {
        self.dims = dims;

        self.update_view();
    }

    pub fn view(&self) -> Matrix4<f32> {
        self.view
    }

    fn update_view(&mut self) {
        self.view = Self::calculate_view(
            self.dims
        );
    }

    fn calculate_view(
        dims: Vector3<f32>,
    ) -> Matrix4<f32> {
        let dims = dims / 2.0;

        cgmath::ortho(-dims.x, dims.x, -dims.y, dims.y, -dims.z, dims.z)
    }
}

impl Component for Camera {
    fn id() -> usize {
        static ID: Lazy<usize> = Lazy::new(|| cid!());

        *ID
    }
}
