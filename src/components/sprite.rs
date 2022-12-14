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

pub struct Sprite<'a> {
    pub color: Vector4<f32>,
    pub shape: Shape,
    pub texture: Texture,
    pub shaders: Shaders,
    pub draw_parameters: DrawParameters<'a>,
    pub z: f32,
    pub active: bool,
}

impl<'a> Sprite<'a> {
    pub fn new(
        color: Vector4<f32>,
        shape: Shape,
        texture: Texture,
        shaders: Shaders,
        draw_parameters: DrawParameters<'a>,
        z: f32,
        active: bool,
    ) -> Self {
        Self {
            color,
            shape,
            texture,
            shaders,
            draw_parameters,
            z,
            active,
        }
    }

    pub fn new_default(
        color: Vector4<f32>,
        shape: Shape,
        texture: Texture,
        shaders: Shaders,
        z: f32,
        active: bool,
    ) -> Self {
        Self::new(
            color,
            shape,
            texture,
            shaders,
            DrawParameters {
                depth: Depth {
                    test: DepthTest::IfLess,
                    write: true,
                    ..Default::default()
                },
                blend: Blend::alpha_blending(),
                ..Default::default()
            },
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
