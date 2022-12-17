use crate::cid;
use crate::ecs::Component;
use cgmath::Matrix4;
use once_cell::sync::Lazy;

#[derive(Clone)]
pub struct Camera {
    left: f32,
    right: f32,
    bottom: f32,
    top: f32,
    near: f32,
    far: f32,
    view: Matrix4<f32>,
    pub active: bool,
}

impl Camera {
    pub fn new(
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        near: f32,
        far: f32,
        active: bool,
    ) -> Self {
        Self {
            left,
            right,
            bottom,
            top,
            near,
            far,
            view: Self::calculate_view(left, right, bottom, top, near, far),
            active,
        }
    }

    pub fn left(&self) -> f32 {
        self.left
    }

    pub fn set_left(&mut self, left: f32) {
        self.left = left;

        self.update_view();
    }

    pub fn right(&self) -> f32 {
        self.right
    }

    pub fn set_right(&mut self, right: f32) {
        self.right = right;

        self.update_view();
    }

    pub fn bottom(&self) -> f32 {
        self.bottom
    }

    pub fn set_bottom(&mut self, bottom: f32) {
        self.bottom = bottom;

        self.update_view();
    }

    pub fn top(&self) -> f32 {
        self.top
    }

    pub fn set_top(&mut self, top: f32) {
        self.top = top;

        self.update_view();
    }

    pub fn near(&self) -> f32 {
        self.near
    }

    pub fn set_near(&mut self, near: f32) {
        self.near = near;

        self.update_view();
    }

    pub fn far(&self) -> f32 {
        self.far
    }

    pub fn set_far(&mut self, far: f32) {
        self.far = far;

        self.update_view();
    }

    pub fn view(&self) -> Matrix4<f32> {
        self.view
    }

    fn update_view(&mut self) {
        self.view = Self::calculate_view(
            self.left,
            self.right,
            self.bottom,
            self.top,
            self.near,
            self.far,
        );
    }

    fn calculate_view(
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        near: f32,
        far: f32,
    ) -> Matrix4<f32> {
        cgmath::ortho(left, right, bottom, top, near, far)
    }
}

impl Component for Camera {
    fn id() -> usize {
        static ID: Lazy<usize> = Lazy::new(|| cid!());

        *ID
    }
}
