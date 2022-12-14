use crate::{
    components::{Camera, Sprite, Transform},
    ecs::{self, Component, Manager, Entities, System, World},
};
use cgmath::Vector4;
use glium::{glutin::event::Event, Display, Surface};

pub struct DrawingSystem {
    pub display: Display,
    pub bg: Vector4<f32>,
}

impl DrawingSystem {
    pub fn new(display: Display, bg: Vector4<f32>) -> Self {
        Self { display, bg }
    }
}

impl System for DrawingSystem {
    fn update(&mut self, entities: &mut Manager, event: &Event<()>) -> anyhow::Result<()> {
        if let Event::MainEventsCleared = event {}

        Ok(())
    }
}
