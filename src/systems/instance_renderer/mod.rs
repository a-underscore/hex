pub mod instance_data;

pub use instance_data::InstanceData;

use crate::{
    assets::Shader,
    components::{Camera, Model, Transform},
    ecs::{system_manager::System, ComponentManager, Context, EntityManager, Ev},
};
use cgmath::{prelude::*, Matrix4};
use glium::{
    draw_parameters::{BackfaceCullingMode, Blend, DepthTest},
    uniform,
    uniforms::Sampler,
    Depth, Display, DrawParameters, Surface, VertexBuffer,
};
use std::{collections::HashMap, rc::Rc};

pub struct InstanceRenderer {
    pub draw_parameters: DrawParameters<'static>,
    pub texture_shader: Shader,
    pub color_shader: Shader,
}

impl InstanceRenderer {
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

impl System for InstanceRenderer {
    fn update(
        &mut self,
        event: &mut Ev,
        scene: &mut Context,
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
                            Some(Rc::new((
                                cm.get::<Model>(e).and_then(|s| s.active.then_some(s))?,
                                cm.get::<Transform>(e).and_then(|t| t.active.then_some(t))?,
                            )))
                        })
                        .fold(HashMap::new(), |mut acc, d| {
                            let (i, t) = &*d;
                            let entry = acc.entry(Rc::as_ptr(&i.data)).or_insert(Vec::new());
                            let (_, m, _) = &*i.data;

                            entry.push((i.data.clone(), InstanceData::new(t.matrix(), m.color), d));

                            acc
                        })
                        .into_values()
                        .map(Rc::new)
                        .filter_map(|d| {
                            Some((
                                d.iter()
                                    .min_by(|(_, _, r1), (_, _, r2)| {
                                        let (_, t1) = &**r1;
                                        let (_, t2) = &**r2;

                                        (ct.position() - t1.position())
                                            .magnitude()
                                            .total_cmp(&(ct.position() - t2.position()).magnitude())
                                    })
                                    .cloned()?,
                                d,
                            ))
                        })
                        .collect();

                    models.sort_by(|((_, _, r1), _), ((_, _, r2), _)| {
                        let (_, t1) = &**r1;
                        let (_, t2) = &**r2;

                        (ct.position() - t1.position())
                            .magnitude()
                            .total_cmp(&(ct.position() - t2.position()).magnitude())
                    });

                    models
                };

                for ((instance, _, _), instances) in models {
                    let (m, _, t) = &*instance;
                    let (v, i) = &*m.buffer;
                    let instance_buffer = {
                        let i: Vec<_> = instances.iter().map(|(_, i, _)| *i).collect();

                        VertexBuffer::dynamic(&scene.display, &i)?
                    };
                    let ib = instance_buffer
                        .per_instance()
                        .map_err(|e| anyhow::Error::msg(format!("{e:?}")))?;

                    match t {
                        Some(texture) => {
                            let (uv, buffer) = &*texture.buffer;
                            let camera_transform: [[f32; 4]; 4] = ct.matrix().into();
                            let camera_proj: [[f32; 4]; 4] = c.matrix().into();
                            let u = uniform! {
                                camera_transform: camera_transform,
                                camera_proj: camera_proj,
                                buffer: Sampler(buffer, texture.sampler_behaviour),
                            };

                            target.draw(
                                (v, ib, uv),
                                i.source(),
                                &self.texture_shader.program,
                                &u,
                                &self.draw_parameters,
                            )?;
                        }
                        None => {
                            let u = uniform! {
                                camera_transform: <Matrix4<f32> as Into<[[f32; 4] ;4]>>::into(ct.matrix()),
                                camera_proj: <Matrix4<f32> as Into<[[f32; 4]; 4]>>::into(c.matrix()),
                            };

                            target.draw(
                                (v, ib),
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
