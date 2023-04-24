pub mod instance_data;
pub mod instance_id;

pub use instance_data::InstanceData;
pub use instance_id::InstanceId;

use crate::{
    assets::Shader,
    components::{Camera, Model, Transform},
    ecs::{system_manager::System, ComponentManager, EntityManager, Ev, Scene},
};
use glium::{uniform, uniforms::Sampler, Display, Surface, VertexBuffer};
use std::collections::BTreeMap;

pub struct InstanceRenderer {
    pub texture_shader: Shader,
    pub color_shader: Shader,
}

impl InstanceRenderer {
    pub fn new(display: &Display) -> anyhow::Result<Self> {
        Ok(Self {
            texture_shader: Shader::new(
                display,
                include_str!("vertex/texture_vertex.glsl"),
                include_str!("fragment/texture_fragment.glsl"),
                None,
            )?,
            color_shader: Shader::new(
                display,
                include_str!("vertex/color_vertex.glsl"),
                include_str!("fragment/color_fragment.glsl"),
                None,
            )?,
        })
    }
}

impl<'a> System<'a> for InstanceRenderer {
    fn update(
        &mut self,
        event: &mut Ev,
        scene: &mut Scene,
        (em, cm): (&mut EntityManager, &mut ComponentManager),
    ) -> anyhow::Result<()> {
        if let Ev::Draw((_, target)) = event {
            if let Some((c, ct)) = em.entities.keys().cloned().find_map(|e| {
                Some((
                    cm.get::<Camera>(e, em)
                        .and_then(|c| c.active.then_some(c))?,
                    cm.get::<Transform>(e, em)
                        .and_then(|t| t.active.then_some(t))?,
                ))
            }) {
                let models = {
                    let mut models: Vec<_> = em
                        .entities
                        .keys()
                        .cloned()
                        .filter_map(|e| {
                            Some((
                                *cm.get::<InstanceId>(e, em)
                                    .and_then(|s| s.active.then_some(s))?,
                                (
                                    cm.get::<Model>(e, em).and_then(|s| s.active.then_some(s))?,
                                    cm.get::<Transform>(e, em)
                                        .and_then(|t| t.active.then_some(t))?,
                                ),
                            ))
                        })
                        .fold(BTreeMap::new(), |mut acc, (id, d @ (ref m, ref t))| {
                            let entry = acc.entry(id).or_insert(Vec::new());

                            entry.push((InstanceData::new(t.matrix(), m.color), d));

                            acc
                        })
                        .into_values()
                        .filter_map(|d| {
                            Some((
                                d.clone().into_iter().min_by(|(_, (_, t1)), (_, (_, t2))| {
                                    (ct.position() - t1.position())
                                        .magnitude()
                                        .total_cmp(&(ct.position() - t2.position()).magnitude())
                                })?,
                                d,
                            ))
                        })
                        .collect();

                    models.sort_by(|((_, (_, t1)), _), ((_, (_, t2)), _)| {
                        (ct.position() - t1.position())
                            .magnitude()
                            .total_cmp(&(ct.position() - t2.position()).magnitude())
                    });

                    models
                };

                for ((_, (m, _)), i) in models {
                    let i: Vec<_> = i.into_iter().map(|(i, _)| i).collect();
                    let instance_buffer = VertexBuffer::dynamic(&scene.display, &i)?;
                    let (v, i) = &*m.mesh.buffer;

                    match &m.texture {
                        Some(texture) => {
                            let (uv, buffer) = &*texture.buffer;
                            let u = uniform! {
                                camera_transform: ct.matrix().0,
                                camera_view: c.view().0,
                                tex: Sampler(buffer, texture.sampler_behaviour),
                            };

                            target.draw(
                                (
                                    v,
                                    uv,
                                    instance_buffer
                                        .per_instance()
                                        .map_err(|e| anyhow::Error::msg(format!("{e:?}")))?,
                                ),
                                i.source(),
                                &self.texture_shader.program,
                                &u,
                                &m.draw_parameters,
                            )?;
                        }
                        None => {
                            let u = uniform! {
                                camera_transform: ct.matrix().0,
                               camera_view: c.view().0,
                            };

                            target.draw(
                                (
                                    v,
                                    instance_buffer
                                        .per_instance()
                                        .map_err(|e| anyhow::Error::msg(format!("{e:?}")))?,
                                ),
                                i.source(),
                                &self.texture_shader.program,
                                &u,
                                &m.draw_parameters,
                            )?;
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
