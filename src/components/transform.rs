use crate::ecs::{self, derive::AsAny, AsAny, Component, Id, Parent};
use cgmath::{Matrix2, Matrix3, Rad, Vector2};
use std::{any::Any, cell::RefCell, rc::Rc};

thread_local! {
    pub static TRANSFORM_ID: Id = ecs::id("transform");
}

pub struct TransformData {
    position: Vector2<f32>,
    rotation: f32,
    scale: Vector2<f32>,
    transform: Matrix3<f32>,
}

impl TransformData {
    pub fn new(position: Vector2<f32>, rotation: f32, scale: Vector2<f32>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            position,
            rotation,
            scale,
            transform: Self::calculate_transform(position, rotation, scale),
        }))
    }

    pub fn get_position(&self) -> Vector2<f32> {
        self.position
    }

    pub fn set_position(&mut self, position: Vector2<f32>) {
        self.position = position;

        self.update_transform();
    }

    pub fn get_rotation(&self) -> f32 {
        self.rotation
    }

    pub fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation;

        self.update_transform();
    }

    pub fn get_scale(&self) -> Vector2<f32> {
        self.scale
    }

    pub fn set_scale(&mut self, scale: Vector2<f32>) {
        self.scale = scale;
    }

    pub fn update_transform(&mut self) {
        self.transform = Self::calculate_transform(self.position, self.rotation, self.scale);
    }

    pub fn get_transform(&self) -> Matrix3<f32> {
        self.transform
    }

    pub fn calculate_transform(
        position: Vector2<f32>,
        rotation: f32,
        scale: Vector2<f32>,
    ) -> Matrix3<f32> {
        Matrix3::from_translation(position)
            * Matrix3::from(Matrix2::from_angle(Rad(rotation)))
            * Matrix3::from_nonuniform_scale(scale.x, scale.y)
    }
}

#[derive(AsAny)]
pub struct Transform {
    id: Id,
    tid: Id,
    parent: Rc<RefCell<Parent>>,
    pub data: Rc<RefCell<TransformData>>,
}

impl Transform {
    pub fn new(id: Id, data: Rc<RefCell<TransformData>>) -> Rc<Self> {
        Rc::new(Self {
            id,
            tid: ecs::tid(&TRANSFORM_ID),
            parent: Rc::new(RefCell::new(None)),
            data,
        })
    }
}

impl Component for Transform {
    fn id(&self) -> Id {
        self.id.clone()
    }

    fn tid(&self) -> Id {
        self.tid.clone()
    }

    fn get_parent(&self) -> Parent {
        self.parent.borrow().clone()
    }

    fn set_parent(&self, parent: Parent) {
        *self.parent.borrow_mut() = parent;
    }
}
