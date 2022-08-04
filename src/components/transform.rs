use crate::ecs::{self, derive::Component, AsAny, Component, Entity};
use cgmath::{Matrix2, Matrix3, Rad, Vector2};
use glium::glutin::event::Event;
use std::{any::Any, cell::RefCell, rc::Rc, time::Duration};

thread_local! {
    pub static TRANSFORM_ID: Rc<String> = ecs::id("transform");
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
            transform: calculate_transform(position, rotation, scale),
            position,
            rotation,
            scale,
        }))
    }

    pub fn update_transform(&mut self) {
        self.transform = calculate_transform(self.position, self.rotation, self.scale);
    }
}

#[derive(Component)]
pub struct Transform {
    id: Rc<String>,
    tid: Rc<String>,
    parent: Rc<RefCell<Option<Rc<Entity>>>>,
    pub data: Rc<RefCell<TransformData>>,
}

impl Transform {
    pub fn new(id: Rc<String>, data: Rc<RefCell<TransformData>>) -> Rc<Self> {
        Rc::new(Self {
            id,
            tid: ecs::tid(&TRANSFORM_ID),
            parent: Rc::new(RefCell::new(None)),
            data,
        })
    }

    pub fn transform(&self) -> Matrix3<f32> {
        self.data.borrow().transform
    }
}

impl Component for Transform {
    fn id(&self) -> Rc<String> {
        self.id.clone()
    }

    fn tid(&self) -> Rc<String> {
        self.tid.clone()
    }

    fn parent(&self) -> Option<Rc<Entity>> {
        self.parent.borrow().clone()
    }

    fn set_parent(&self, parent: Option<Rc<Entity>>) {
        *self.parent.borrow_mut() = parent;
    }

    fn update(self: Rc<Self>, parent: Option<Rc<Entity>>, _event: &Event<()>, _delta: Duration) {
        let mut data = self.data.borrow_mut();

        match parent
            .and_then(|p| p.parent())
            .and_then(|p| p.get_first::<Transform>(&ecs::tid(&TRANSFORM_ID)))
        {
            Some(transform) => {
                data.transform = calculate_transform(data.position, data.rotation, data.scale)
                    * transform.data.borrow().transform;
            }
            None => data.update_transform(),
        }
    }
}

fn calculate_transform(position: Vector2<f32>, rotation: f32, scale: Vector2<f32>) -> Matrix3<f32> {
    Matrix3::from_translation(position)
        * Matrix3::from(Matrix2::from_angle(Rad(rotation)))
        * Matrix3::from_nonuniform_scale(scale.x, scale.y)
}
