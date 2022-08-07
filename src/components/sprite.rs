use crate::{
    assets::{Shaders, Shape, Texture},
    components::{Camera, Transform},
    ecs::{self, Component, Id},
    engine::Engine,
};
use cgmath::Vector4;
use glium::{uniform, Frame, Surface};
use std::{cell::RefCell, rc::Rc};

thread_local! {
    pub static SPRITE_ID: Id = ecs::id("sprite");
}

pub struct Sprite {
    pub color: Vector4<f32>,
    pub shape: Rc<RefCell<Shape>>,
    pub texture: Rc<RefCell<Texture>>,
    pub shaders: Rc<RefCell<Shaders>>,
    pub z: f32,
    pub draw: bool,
}

impl Sprite {
    pub fn new(
        color: Vector4<f32>,
        shape: Rc<RefCell<Shape>>,
        texture: Rc<RefCell<Texture>>,
        shaders: Rc<RefCell<Shaders>>,
        z: f32,
        draw: bool,
    ) -> Rc<RefCell<Box<Self>>> {
        Rc::new(RefCell::new(Box::new(Self {
            color,
            shape,
            texture,
            shaders,
            z,
            draw,
        })))
    }

    pub fn draw(
        &self,
        transform: &Transform,
        camera: &Camera,
        camera_transform: &Transform,
        engine: &Engine,
        target: &mut Frame,
    ) -> anyhow::Result<()> {
        if self.draw {
            let color: [f32; 4] = self.color.into();
            let transform: [[f32; 3]; 3] = transform.get_transform().into();
            let camera_view: [[f32; 4]; 4] = camera.view().into();
            let camera_transform: [[f32; 3]; 3] = camera_transform.get_transform().into();
            let texture = self.texture.borrow();
            let uniforms = uniform! {
                z: self.z,
                transform: transform,
                camera_transform: camera_transform,
                camera_view: camera_view,
                color: color,
                texture: &texture.texture,
            };
            let shape = self.shape.borrow();
            let shaders = self.shaders.borrow();

            target.draw(
                &shape.vertices,
                &shape.indices,
                &shaders.program,
                &uniforms,
                &engine.draw_parameters.borrow(),
            )?;
        }

        Ok(())
    }
}

impl Component for Sprite {
    fn id(&self) -> Id {
        ecs::tid(&SPRITE_ID)
    }
}
