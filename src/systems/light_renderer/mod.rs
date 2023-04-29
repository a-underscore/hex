use crate::{
    assets::Shader,
    components::{Camera, Light, Model, Transform},
    ecs::{system_manager::System, ComponentManager, EntityManager, Ev, Scene},
    math::{Mat4d, Vec2d},
};
use glium::{
    draw_parameters::{BackfaceCullingMode, Blend, DepthTest},
    framebuffer::SimpleFrameBuffer,
    texture::{DepthTexture2d, Texture2d},
    uniform,
    uniforms::{DepthTextureComparison, MagnifySamplerFilter, Sampler, SamplerBehavior},
    Depth, Display, DrawParameters, Surface,
};

pub struct LightRenderer<'a> {
    pub lighting_draw_parameters: DrawParameters<'a>,
    pub lighting_sampler_behavior: SamplerBehavior,
    pub lighting_shader: Shader,
    pub shadow_draw_parameters: DrawParameters<'a>,
    pub shadow_shader: Shader,
    pub shadow_sampler_behavior: SamplerBehavior,
    pub shadow_dims: (u32, u32),
    pub filter: MagnifySamplerFilter,
}

impl<'a> LightRenderer<'a> {
    pub fn new(
        display: &Display,
        lighting_sampler_behavior: SamplerBehavior,
        shadow_dims: (u32, u32),
        filter: MagnifySamplerFilter,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            lighting_draw_parameters: DrawParameters {
                depth: Depth {
                    test: DepthTest::IfLessOrEqual,
                    write: true,
                    ..Default::default()
                },
                blend: Blend::alpha_blending(),
                backface_culling: BackfaceCullingMode::CullClockwise,
                ..Default::default()
            },
            lighting_shader: Shader::new(
                display,
                include_str!("lighting/vertex.glsl"),
                include_str!("lighting/fragment.glsl"),
                None,
            )?,
            lighting_sampler_behavior,
            shadow_draw_parameters: DrawParameters {
                depth: Depth {
                    test: DepthTest::IfLessOrEqual,
                    write: true,
                    ..Default::default()
                },
                backface_culling: BackfaceCullingMode::CullCounterClockwise,
                ..Default::default()
            },
            shadow_shader: Shader::new(
                display,
                include_str!("shadow/vertex.glsl"),
                include_str!("shadow/fragment.glsl"),
                None,
            )?,
            shadow_sampler_behavior: SamplerBehavior {
                depth_texture_comparison: Some(DepthTextureComparison::LessOrEqual),
                ..lighting_sampler_behavior
            },
            filter,
            shadow_dims,
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
                        .and_then(|c| (c.active && c.main).then_some(c))?,
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
                let (surface_width, surface_height) = target.get_dimensions();
                let (shadow_width, shadow_height) = self.shadow_dims;
                let shadow_buffer =
                    DepthTexture2d::empty(&scene.display, shadow_width, shadow_height)?;
                let mut shadow_target =
                    SimpleFrameBuffer::depth_only(&scene.display, &shadow_buffer)?;

                shadow_target.clear_color(1.0, 1.0, 1.0, 1.0);
                shadow_target.clear_depth(1.0);

                for (l, lc) in em.entities.keys().cloned().filter_map(|e| {
                    Some((
                        cm.get::<Light>(e, em).and_then(|l| l.active.then_some(l))?,
                        cm.get::<Camera>(e, em)
                            .and_then(|l| l.active.then_some(l))?,
                    ))
                }) {
                    let buffer = Texture2d::empty(&scene.display, surface_width, surface_height)?;
                    let view = Mat4d::translation(l.position);

                    for (m, t) in &models {
                        let (mesh, _, _) = &*m.data;
                        let (v, i) = &*mesh.buffer;
                        let u = uniform! {
                            transform: t.matrix().0,
                            light_proj: lc.matrix().0,
                            light_transform: view.0,
                        };

                        shadow_target.draw(
                            v,
                            i.source(),
                            &self.shadow_shader.program,
                            &u,
                            &self.shadow_draw_parameters,
                        )?;
                    }

                    for (m, t) in &models {
                        target.fill(&buffer.as_surface(), self.filter);

                        let (mesh, ma, _) = &*m.data;
                        let (v, i) = &*mesh.buffer;
                        let u = uniform! {
                            transform: t.matrix().0,
                            camera_transform: ct.matrix().0,
                            camera_proj: c.matrix().0,
                            light_transform: view.0,
                            light_proj: lc.matrix().0,
                            buffer: Sampler(&buffer, self.lighting_sampler_behavior),
                            shadow_buffer: Sampler(&shadow_buffer, self.shadow_sampler_behavior),
                            camera_position: ct.position().0,
                            light_color: l.color.0,
                            light_position: l.position.0,
                            screen_dims: Vec2d::new(surface_width as f32, surface_height as f32).0,
                            light_strength: l.strength,
                            ambient_strength: ma.ambient,
                            diffuse_strength: ma.diffuse,
                            specular_strength: ma.specular,
                            reflect_strength: ma.reflect,
                            bias: ma.bias,
                        };

                        target.draw(
                            v,
                            i.source(),
                            &self.lighting_shader.program,
                            &u,
                            &self.lighting_draw_parameters,
                        )?;
                    }
                }
            }
        }

        Ok(())
    }
}
