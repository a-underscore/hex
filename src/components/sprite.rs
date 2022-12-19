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

#[derive(Clone)]
pub struct Sprite<'a> {
    pub draw_parameters: DrawParameters<'a>,
    pub shape: Shape,
    pub texture: Texture,
    pub shader: Shader,
    pub color: Vector4<f32>,
    pub z: f32,
    pub active: bool,
}

impl<'a> Sprite<'a> {
    pub fn new(
        draw_parameters: DrawParameters<'a>,
        shape: Shape,
        texture: Texture,
        shader: Shader,
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
        shape: Shape,
        texture: Texture,
        shader: Shader,
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
            let image = Sampler(
                &*self.texture.buffer,
                self.texture.sampler_behaviour,
            );
            let uniform = uniform! {
                z: self.z,
                transform: transform,
                camera_transform: camera_transform,
                camera_view: camera_view,
                color: color,
                image: image,
            };

            target.draw(
                &*self.shape.vertices,
                &*self.shape.indices,
                &self.shader.program,
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
