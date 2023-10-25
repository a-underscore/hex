use crate::{
    assets::Shader,
    components::{Camera, Model, Transform},
    ecs::{system_manager::System, ComponentManager, Context, EntityManager, Ev},
};
use cgmath::prelude::*;
use glium::{
    draw_parameters::{BackfaceCullingMode, Blend, DepthTest},
    uniform,
    uniforms::Sampler,
    Depth, Display, DrawParameters, Surface,
};

pub struct Renderer {
    pub draw_parameters: DrawParameters<'static>,
    pub texture_shader: Shader,
    pub color_shader: Shader,
}

impl Renderer {
    pub fn new(display: &Display) -> anyhow::Result<Self> {
        Ok(Self {
            draw_parameters: DrawParameters {
                depth: Depth {
                    test: DepthTest::IfLessOrEqual,
                    write: true,
                    ..Default::default()
                },
                blend: Blend::alpha_blending(),
                backface_culling: BackfaceCullingMode::CullClockwise,
                ..Default::default()
            },
            texture_shader: Shader::new(
                display,
                include_str!("texture/vertex.glsl"),
                include_str!("texture/fragment.glsl"),
                None,
            )?,
            color_shader: Shader::new(
                display,
                include_str!("color/vertex.glsl"),
                include_str!("color/fragment.glsl"),
                None,
            )?,
        })
    }
}

impl System for Renderer {
    fn update(
        &mut self,
        event: &mut Ev,
        _: &mut Context,
        (em, cm): (&mut EntityManager, &mut ComponentManager),
    ) -> anyhow::Result<()> {
        if let Ev::Draw((_, target)) = event {
            if let Some((c, ct)) = em.entities().find_map(|e| {
                Some((
                    cm.get::<Camera>(e)
                        .and_then(|c| (c.active && c.main).then_some(c))?,
                    cm.get::<Transform>(e).and_then(|t| t.active.then_some(t))?,
                ))
            }) {
                let models = {
                    let mut models: Vec<_> = em
                        .entities()
                        .filter_map(|e| {
                            Some((
                                cm.get::<Model>(e).and_then(|s| s.active.then_some(s))?,
                                cm.get::<Transform>(e).and_then(|t| t.active.then_some(t))?,
                            ))
                        })
                        .collect();

                    models.sort_by(|(_, t1), (_, t2)| {
                        (ct.position() - t1.position())
                            .magnitude()
                            .total_cmp(&(ct.position() - t2.position()).magnitude())
                    });

                    models
                };

                for (m, t) in models {
                    let (mesh, ma, texture) = &*m.data;
                    let (v, i) = &*mesh.buffer;
                    let transform: [[f32; 4]; 4] = t.matrix().into();
                    let camera_transform: [[f32; 4]; 4] = ct.matrix().into();
                    let camera_proj: [[f32; 4]; 4] = c.matrix().into();
                    let color: [f32; 4] = ma.color.into();

                    match texture {
                        Some(texture) => {
                            let (uv, buffer) = &*texture.buffer;
                            let u = uniform! {
                                transform: transform,
                                camera_transform: camera_transform,
                                camera_proj: camera_proj,
                                buffer: Sampler(buffer, texture.sampler_behaviour),
                                color: color,
                            };

                            target.draw(
                                (v, uv),
                                i.source(),
                                &self.texture_shader.program,
                                &u,
                                &self.draw_parameters,
                            )?;
                        }
                        None => {
                            let u = uniform! {
                                transform: transform,
                                camera_transform: camera_transform,
                                camera_proj: camera_proj,
                                color: color,
                            };

                            target.draw(
                                v,
                                i.source(),
                                &self.color_shader.program,
                                &u,
                                &self.draw_parameters,
                            )?;
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
