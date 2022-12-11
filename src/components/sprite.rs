use crate::{
    assets::{Shaders, Shape, Texture},
    components::{Camera, Transform},
    ecs::{self, Component, Id, Type},
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
    pub color: Vector4<f32>,
    pub shape: Type<Shape>,
    pub texture: Type<Texture>,
    pub shaders: Type<Shaders>,
    pub draw_parameters: Type<DrawParameters<'a>>,
    pub z: f32,
    pub active: bool,
}

impl<'a> Sprite<'a> {
    pub fn new(
        color: Vector4<f32>,
        shape: Type<Shape>,
        texture: Type<Texture>,
        shaders: Type<Shaders>,
        draw_parameters: Type<DrawParameters<'a>>,
        z: f32,
        active: bool,
    ) -> Type<Self> {
        ecs::new(Self {
            color,
            shape,
            texture,
            shaders,
            draw_parameters,
            z,
            active,
        })
    }

    pub fn new_default(
        color: Vector4<f32>,
        shape: Type<Shape>,
        texture: Type<Texture>,
        shaders: Type<Shaders>,
        z: f32,
        active: bool,
    ) -> Type<Self> {
        Self::new(
            color,
            shape,
            texture,
            shaders,
            ecs::new(DrawParameters {
                depth: Depth {
                    test: DepthTest::IfLess,
                    write: true,
                    ..Default::default()
                },
                blend: Blend::alpha_blending(),
                ..Default::default()
            }),
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
            let shaders = self.shaders.try_borrow()?;
            let draw_parameters = self.draw_parameters.try_borrow()?;

            target.draw(
                &shape.vertices,
                &shape.indices,
                &shaders.program,
                &uniform,
                &draw_parameters,
            )?;
        }

        Ok(())
    }
}

impl<'a> Component for Sprite<'a> {
    fn id() -> Id {
        ecs::id("sprite")
    }
}
