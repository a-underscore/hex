use crate::{
    assets::{shaders::Uniforms, texture::TextureBuffer, Shaders, Shape},
    components::{Camera, Transform},
    ecs::{self, Component, Id},
};
use cgmath::Vector4;
use glium::{
    draw_parameters::{Blend, DepthTest},
    uniform,
    uniforms::UniformBuffer,
    Depth, Display, DrawParameters, Frame, Surface,
};
use std::{cell::RefCell, rc::Rc};

pub struct Sprite<'a> {
    pub color: Vector4<f32>,
    pub shape: Rc<RefCell<Shape>>,
    pub texture: Rc<RefCell<dyn TextureBuffer>>,
    pub shaders: Rc<RefCell<Shaders>>,
    pub draw_parameters: Rc<RefCell<DrawParameters<'a>>>,
    pub z: f32,
    pub active: bool,
}

impl<'a> Sprite<'a> {
    thread_local! {
        pub static ID: Id = ecs::id("sprite");
    }

    pub fn new(
        color: Vector4<f32>,
        shape: Rc<RefCell<Shape>>,
        texture: Rc<RefCell<dyn TextureBuffer>>,
        shaders: Rc<RefCell<Shaders>>,
        draw_parameters: Rc<RefCell<DrawParameters<'a>>>,
        z: f32,
        active: bool,
    ) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            color,
            shape,
            texture,
            shaders,
            draw_parameters,
            z,
            active,
        }))
    }

    pub fn new_default(
        color: Vector4<f32>,
        shape: Rc<RefCell<Shape>>,
        texture: Rc<RefCell<dyn TextureBuffer>>,
        shaders: Rc<RefCell<Shaders>>,
        z: f32,
        active: bool,
    ) -> Rc<RefCell<Self>> {
        Self::new(
            color,
            shape,
            texture,
            shaders,
            Rc::new(RefCell::new(DrawParameters {
                depth: Depth {
                    test: DepthTest::IfLess,
                    write: true,
                    ..Default::default()
                },
                blend: Blend::alpha_blending(),
                ..Default::default()
            })),
            z,
            active,
        )
    }

    pub fn draw(
        &self,
        display: &Display,
        target: &mut Frame,
        transform: &Transform,
        camera: &Camera,
        camera_transform: &Transform,
    ) -> anyhow::Result<()> {
        if self.active {
            let color: [f32; 4] = self.color.into();
            let transform: [[f32; 3]; 3] = transform.get_transform().into();
            let camera_view: [[f32; 4]; 4] = camera.get_view().into();
            let camera_transform: [[f32; 3]; 3] = camera_transform.get_transform().into();
            let mut texture = self.texture.try_borrow_mut()?;
            let texture = texture.unit()?;
            let buffer = UniformBuffer::new(display, Uniforms { image: texture })?;
            let uniforms = uniform! {
                    z: self.z,
                    transform: transform,
                    camera_transform: camera_transform,
                    camera_view: camera_view,
                    color: color,
                Uniforms:
                &*buffer
            };
            let shape = self.shape.try_borrow()?;
            let shaders = self.shaders.try_borrow()?;
            let draw_parameters = self.draw_parameters.try_borrow()?;

            target.draw(
                &shape.vertices,
                &shape.indices,
                &shaders.program,
                &uniforms,
                &draw_parameters,
            )?;
        }

        Ok(())
    }
}

impl Component for Sprite<'static> {
    fn get_id() -> Id {
        ecs::tid(&Self::ID)
    }
}
