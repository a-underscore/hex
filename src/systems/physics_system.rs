use crate::{
    components::{ColliderObject, Transform, COLLIDER_OBJECT_ID, TRANSFORM_ID},
    ecs::{self, Id, System, World},
};
use cgmath::Vector2;
use glium::glutin::event::Event;
use rapier2d::prelude::*;
use std::{
    cell::{Ref, RefCell},
    rc::Rc,
    time::Duration,
};

thread_local! {
    pub static PHYSICS_SYSTEM_ID: Id = ecs::id("physics_system");
}

pub struct PhysicsSystem {
    pub gravity: Vector2<f32>,
    pub integration_parameters: IntegrationParameters,
    pub physics_pipeline: PhysicsPipeline,
    pub island_manager: IslandManager,
    pub broad_phase: BroadPhase,
    pub narrow_phase: NarrowPhase,
    pub rigid_body_set: RigidBodySet,
    pub collider_set: ColliderSet,
    pub impulse_joint_set: ImpulseJointSet,
    pub multibody_joint_set: MultibodyJointSet,
    pub ccd_solver: CCDSolver,
}

impl PhysicsSystem {
    pub fn new(gravity: Vector2<f32>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            gravity,
            integration_parameters: IntegrationParameters::default(),
            physics_pipeline: PhysicsPipeline::new(),
            island_manager: IslandManager::new(),
            broad_phase: BroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            rigid_body_set: RigidBodySet::new(),
            collider_set: ColliderSet::new(),

            impulse_joint_set: ImpulseJointSet::new(),
            multibody_joint_set: MultibodyJointSet::new(),
            ccd_solver: CCDSolver::new(),
        }))
    }
}

impl PhysicsSystem {
    fn update_colliders(&mut self, world: &World, _delta: Duration) {
        for e in world.get_entities().values() {
            if let Some((c, t)) = e
                .borrow()
                .get_all(&[&ecs::tid(&COLLIDER_OBJECT_ID), &ecs::tid(&TRANSFORM_ID)])
                .and_then(|c| match c.as_slice() {
                    [c, t] => Some((c.clone(), t.clone())),

                    _ => None,
                })
            {
                if let (Some(c), Some(t)) = (
                    Ref::filter_map(c.borrow(), |c| {
                        c.as_any_ref().downcast_ref::<ColliderObject>()
                    })
                    .ok(),
                    Ref::filter_map(t.borrow(), |t| t.as_any_ref().downcast_ref::<Transform>())
                        .ok(),
                ) {
                    c.update(&t);
                }
            }
        }

        self.physics_pipeline.step(
            &vector![self.gravity.x, self.gravity.y],
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.rigid_body_set,
            &mut self.collider_set,
            &mut self.impulse_joint_set,
            &mut self.multibody_joint_set,
            &mut self.ccd_solver,
            &(),
            &(),
        );
    }
}

impl System for PhysicsSystem {
    fn id(&self) -> Id {
        ecs::tid(&PHYSICS_SYSTEM_ID)
    }

    fn on_update(&mut self, world: &mut World, _event: &Event<()>, delta: Duration) {
        self.update_colliders(world, delta);
    }
}
