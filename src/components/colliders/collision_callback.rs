use crate::ecs::{Component, Entity};
use cgmath::Vector2;

pub trait CollisionCallback: 'static + Component {
    fn callback(&mut self, parent: &mut Entity, other: &mut Entity, points: &[Vector2<f32>]);
}
