use crate::{assets::Proj, ecs::component_manager::Component};
use cgmath::{Matrix4, Vector2, Vector3};

#[derive(Clone)]
pub struct Camera {
    proj: Proj,
    matrix: Matrix4<f32>,
    pub main: bool,
    pub active: bool,
}

impl Camera {
    pub fn new(proj: Proj, main: bool, active: bool) -> Self {
        let matrix = proj.matrix();

        Self {
            proj,
            matrix,
            main,
            active,
        }
    }

    pub fn perspective(
        fov: f32,
        aspect: f32,
        clip: Vector2<f32>,
        main: bool,
        active: bool,
    ) -> Self {
        Self::new(Proj::Perspective((fov, aspect, clip)), main, active)
    }

    pub fn ortho(dims: Vector3<f32>, main: bool, active: bool) -> Self {
        Self::new(Proj::Ortho(dims), main, active)
    }

    pub fn set_proj(&mut self, proj: Proj) {
        self.proj = proj;

        self.update_matrix()
    }

    pub fn matrix(&self) -> Matrix4<f32> {
        self.matrix
    }

    fn update_matrix(&mut self) {
        self.matrix = self.proj.matrix();
    }
}

impl Component for Camera {}
