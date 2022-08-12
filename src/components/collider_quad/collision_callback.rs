use crate::ecs::Component;

pub trait CollisionCallback: 'static + Component {
    fn callback(&mut self, other: &mut dyn Component);
}
