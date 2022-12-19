use crate::{
    assets::{Shader, Shape, Texture},
    cid,
    components::{Camera, Transform},
    ecs::Component,
};
use cgmath::Vector4;
use glium::{
    draw_parameters::{Blend, DepthTest},
    uniform,
    uniforms::Sampler,
    Depth, DrawParameters, Frame, Surface,
};
use std::{cell::RefCell, rc::Rc};

#[derive(Clone)]
pub struct Sprite<'a> {
    pub draw_parameters: DrawParameters<'a>,
    pub shape: Rc<RefCell<Shape>>,
    pub texture: Rc<RefCell<Texture>>,
    pub shader: Rc<RefCell<Shader>>,
    pub color: Vector4<f32>,
    pub z: f32,
    pub active: bool,
}

impl<'a> Sprite<'a> {
    pub fn new(
        draw_parameters: DrawParameters<'a>,
        shape: Rc<RefCell<Shape>>,
        texture: Rc<RefCell<Texture>>,
        shader: Rc<RefCell<Shader>>,
        color: Vector4<f32>,
        z: f32,
        active: bool,
    ) -> Self {
        Self {
            draw_parameters,
            shape,
            texture,
            shader,
            color,
            z,
            active,
        }
    }

    pub fn default(
        shape: Rc<RefCell<Shape>>,
        texture: Rc<RefCell<Texture>>,
        shader: Rc<RefCell<Shader>>,
        color: Vector4<f32>,
        z: f32,
        active: bool,
    ) -> Self {
        Self::new(
            DrawParameters {
                depth: Depth {
                    test: DepthTest::IfLess,
                    write: true,
                    ..Default::default()
                },
                blend: Blend::alpha_blending(),
                ..Default::default()
            },
            shape,
            texture,
            shader,
            color,
            z,
            active,
        )
    }

    pub fn draw(
        &self,
        target: &mut Frame,
        transform: &Transform,
        camera: &Camera,
        camera_transform: &Transform,
    ) -> anyhow::Result<()> {
        if self.active {
            let color: [f32; 4] = self.color.into();
            let transform: [[f32; 3]; 3] = transform.transform().into();
            let camera_view: [[f32; 4]; 4] = camera.view().into();
            let camera_transform: [[f32; 3]; 3] = camera_transform.transform().into();
            let texture = self.texture.try_borrow()?;
            let image = Sampler(&texture.buffer, texture.sampler_behaviour);
            let uniform = uniform! {
                z: self.z,
                transform: transform,
                camera_transform: camera_transform,
                camera_view: camera_view,
                color: color,
                image: image,
            };
            let shape = self.shape.try_borrow()?;
            let shader = self.shader.try_borrow()?;

            target.draw(
                &shape.vertices,
                &shape.indices,
                &shader.program,
                &uniform,
                &self.draw_parameters,
            )?;
        }

        Ok(())
    }
}

impl<'a> Component for Sprite<'a> {
    fn id() -> usize {
        cid!()
    }
}
