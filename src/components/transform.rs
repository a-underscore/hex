use crate::ecs::{self, derive::AsAny, AsAny, Component, Id, Parent};
use cgmath::{Matrix2, Matrix3, Rad, Vector2};
use glium::glutin::event::Event;
use std::{any::Any, cell::RefCell, rc::Rc, time::Duration};

thread_local! {
    pub static TRANSFORM_ID: Id = ecs::id("transform");
}

pub struct TransformData {
    transform: Matrix3<f32>,
    pub position: Vector2<f32>,
    pub rotation: f32,
    pub scale: Vector2<f32>,
}

impl TransformData {
    pub fn new(position: Vector2<f32>, rotation: f32, scale: Vector2<f32>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            transform: Self::calculate_transform(position, rotation, scale),
            position,
            rotation,
            scale,
        }))
    }

    pub fn update_transform(&mut self) {
        self.transform = Self::calculate_transform(self.position, self.rotation, self.scale);
    }

    fn calculate_transform(
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

    pub fn get_transform(&self) -> Matrix3<f32> {
        self.data.borrow().transform
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

    fn on_update(self: Rc<Self>, parent: Parent, _event: &Event<()>, _delta: Duration) {
        let mut data = self.data.borrow_mut();

        data.update_transform();

        if let Some(transform) = parent
            .and_then(|p| p.get_parent())
            .and_then(|p| p.get_first::<Transform>(&ecs::tid(&TRANSFORM_ID)))
        {
            data.transform = data.transform * transform.data.borrow().transform;
        }
    }
}
