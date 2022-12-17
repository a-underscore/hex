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
pub struct Sprite<'a, 'b, 'c, 'd> {
    pub draw_parameters: DrawParameters<'a>,
    pub shape: &'b Shape,
    pub texture: &'c Texture,
    pub shaders: &'d Shader,
    pub color: Vector4<f32>,
    pub z: f32,
    pub active: bool,
}

impl<'a, 'b, 'c, 'd> Sprite<'a, 'b, 'c, 'd> {
    pub fn new(
        draw_parameters: DrawParameters<'a>,
        shape: &'b Shape,
        texture: &'c Texture,
        shaders: &'d Shader,
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
        shape: &'b Shape,
        texture: &'c Texture,
        shaders: &'d Shader,
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

impl<'a, 'b, 'c, 'd> Component for Sprite<'a, 'b, 'c, 'd> {
    fn id() -> usize {
        cid!()
    }
}
