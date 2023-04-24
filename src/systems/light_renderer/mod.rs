use crate::{
    assets::Shader,
    components::{Camera, Light, Model, Transform},
    ecs::{system_manager::System, ComponentManager, EntityManager, Ev, Scene},
};
use glium::{
    draw_parameters::{BackfaceCullingMode, Blend, DepthTest},
    texture::Texture2d,
    uniform,
    uniforms::{MagnifySamplerFilter, Sampler},
    Depth, Display, DrawParameters, Surface,
};

pub struct LightRenderer<'a> {
    pub draw_parameters: DrawParameters<'a>,
    pub filter: MagnifySamplerFilter,
    pub shader: Shader,
}

impl<'a> LightRenderer<'a> {
    pub fn new(display: &Display, filter: MagnifySamplerFilter) -> anyhow::Result<Self> {
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
            filter,
            shader: Shader::new(
                display,
                include_str!("vertex.glsl"),
                include_str!("fragment.glsl"),
                None,
            )?,
        })
    }
}

impl<'a> System<'a> for LightRenderer<'a> {
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
                                cm.get::<Model>(e, em).and_then(|s| s.active.then_some(s))?,
                                cm.get::<Transform>(e, em)
                                    .and_then(|t| t.active.then_some(t))?,
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

                for l in em
                    .entities
                    .keys()
                    .cloned()
                    .filter_map(|e| cm.get::<Light>(e, em).and_then(|l| l.active.then_some(l)))
                {
                    for (m, t) in &models {
                        let (mesh, _) = &*m.data;
                        let (v, i) = &*mesh.buffer;
                        let u = uniform! {
                            transform: t.matrix().0,
                            camera_transform: ct.matrix().0,
                            camera_view: c.view().0,
                            color: m.color.0,
                            light_color: l.color.0,
                            specular: l.specular,
                            diffuse: l.diffuse,
                            ambient: l.ambient,
                        };

                        target.draw(
                            v,
                            i.source(),
                            &self.shader.program,
                            &u,
                            &self.draw_parameters,
                        )?;
                    }
                }
            }
        }

        Ok(())
    }
}
