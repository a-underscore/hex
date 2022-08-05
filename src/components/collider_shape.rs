use crate::{
    components::{Transform, TRANSFORM_ID},
    ecs::{self, derive::AsAny, AsAny, Component, Id, Parent},
};
use cgmath::Vector2;
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
    added: bool,
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
            added: false,
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
    parent: Rc<RefCell<Parent>>,
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

    fn get_parent(&self) -> Parent {
        self.parent.borrow().clone()
    }

    fn set_parent(&self, parent: Parent) {
        *self.parent.borrow_mut() = parent;
    }

    fn on_update(self: Rc<Self>, parent: Parent, _event: &Event<()>, _delta: Duration) {
        if let Some(transform) =
            parent.and_then(|p| p.get_first::<Transform>(&ecs::tid(&TRANSFORM_ID)))
        {
            let mut data = self.data.borrow_mut();
            let collider = data.collider.clone();
            let collider = collider.borrow();
            let transform_data = transform.data.clone();
            let mut transform_data = transform_data.borrow_mut();

            if data.added {
                let hitbox = collider.get_hitbox(data.profile.id());

                data.velocity = Vector2::new(hitbox.vel.value.x as f32, hitbox.vel.value.y as f32);
                transform_data.position =
                    Vector2::new(hitbox.value.pos.x as f32, hitbox.value.pos.y as f32);
            } else {
                let hitbox = data
                    .shape
                    .place(v2(
                        transform_data.position.x as f64,
                        transform_data.position.y as f64,
                    ))
                    .moving(v2(data.velocity.x as f64, data.velocity.y as f64));

                data.collider.borrow_mut().add_hitbox(data.profile, hitbox);
            }
        }
    }
}
