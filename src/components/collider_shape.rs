use crate::{
    components::{Transform, TRANSFORM_ID},
    ecs::{self, derive::AsAny, AsAny, Component, Entity, Id, Parent},
};
use cgmath::{Vector2, Vector3, Zero};
use collider::{
    geom::{v2, Shape},
    Collider, HbProfile,
};
use glium::glutin::event::Event;
use std::{any::Any, cell::RefCell, rc::Rc, time::Duration};

thread_local! {
    pub static COLLIDER_SHAPE_ID: Id = ecs::id("sprite");
}

pub struct ColliderShapeData<P>
where
    P: HbProfile,
{
    pub profile: P,
    pub collider: Rc<RefCell<Collider<P>>>,
    pub shape: Shape,
    pub velocity: Vector2<f32>,
}

impl<P> ColliderShapeData<P>
where
    P: HbProfile,
{
    pub fn new(
        profile: P,
        collider: Rc<RefCell<Collider<P>>>,
        shape: Shape,
        velocity: Vector2<f32>,
    ) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            profile,
            collider,
            shape,
            velocity,
        }))
    }
}

#[derive(AsAny)]
pub struct ColliderShape<C>
where
    C: HbProfile + 'static,
{
    id: Id,
    tid: Id,
    parent: Parent,
    pub data: Rc<RefCell<ColliderShapeData<C>>>,
}

impl<P> ColliderShape<P>
where
    P: HbProfile,
{
    pub fn new(id: Id, data: Rc<RefCell<ColliderShapeData<P>>>) -> Rc<Self> {
        Rc::new(Self {
            id,
            tid: ecs::tid(&COLLIDER_SHAPE_ID),
            parent: Rc::new(RefCell::new(None)),
            data,
        })
    }
}

impl<P> Component for ColliderShape<P>
where
    P: HbProfile,
{
    fn id(&self) -> Id {
        self.id.clone()
    }

    fn tid(&self) -> Id {
        self.tid.clone()
    }

    fn parent(&self) -> Parent {
        self.parent.clone()
    }

    fn on_update(self: Rc<Self>, parent: Option<Rc<Entity>>, _event: &Event<()>, _delta: Duration) {
        if let Some(transform) =
            parent.and_then(|p| p.get_first::<Transform>(&ecs::tid(&TRANSFORM_ID)))
        {
            let data = self.data.borrow();
            let position = transform.transform() * Vector3::zero();
            let hitbox = data
                .shape
                .place(v2(position.x as f64, position.y as f64))
                .moving(v2(data.velocity.x as f64, data.velocity.y as f64));

            data.collider.borrow_mut().add_hitbox(data.profile, hitbox);
        }
    }
}
