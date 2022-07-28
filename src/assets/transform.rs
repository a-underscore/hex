use cgmath::Vector2;
use std::{cell::RefCell, rc::Rc};

pub struct TransformData {
    pub position: Vector2<f32>,
    pub rotation: f32,
    pub scale: Vector2<f32>,
}

impl TransformData {
    pub fn new(position: Vector2<f32>, rotation: f32, scale: Vector2<f32>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            position,
            rotation,
            scale,
        }))
    }
}

pub struct Transform {
    pub data: Rc<RefCell<TransformData>>,
}

impl Transform {
    pub fn new(position: Vector2<f32>, rotation: f32, scale: Vector2<f32>) -> Rc<Self> {
        Rc::new(Self {
            data: TransformData::new(position, rotation, scale),
        })
    }
}
