use crate::{
    assets::{Shaders, Shape, Texture},
    components::{Camera, Transform},
    ecs::Component,
    id,
};
use cgmath::Vector4;
use glium::{
    draw_parameters::{Blend, DepthTest},
    uniform,
    uniforms::Sampler,
    Depth, DrawParameters, Frame, Surface,
};

#[derive(Clone)]
pub struct Sprite<'a> {
    pub draw_parameters: DrawParameters<'a>,
    pub shape: &'a Shape,
    pub texture: &'a Texture,
    pub shaders: &'a Shaders,
    pub color: Vector4<f32>,
    pub z: f32,
    pub active: bool,
}

impl<'a> Sprite<'a> {
    pub fn new(
        draw_parameters: DrawParameters<'a>,
        shape: &'a Shape,
        texture: &'a Texture,
        shaders: &'a Shaders,
        color: Vector4<f32>,
        z: f32,
        active: bool,
    ) -> Self {
        Self {
            draw_parameters,
            shape,
            texture,
            shaders,
            color,
            z,
            active,
        }
    }

    pub fn new_default(
        shape: &'a Shape,
        texture: &'a Texture,
        shaders: &'a Shaders,
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
            shaders,
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
            let image = Sampler(&self.texture.buffer, self.texture.sampler_behaviour);
            let uniform = uniform! {
                z: self.z,
                transform: transform,
                camera_transform: camera_transform,
                camera_view: camera_view,
                color: color,
                image: image,
            };

            target.draw(
                &self.shape.vertices,
                &self.shape.indices,
                &self.shaders.program,
                &uniform,
                &self.draw_parameters,
            )?;
        }

        Ok(())
    }
}

impl<'a> Component for Sprite<'a> {
    fn id() -> usize {
        id!()
    }
}
