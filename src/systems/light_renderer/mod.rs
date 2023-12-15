use crate::{
    assets::{Proj, Shader},
    components::{Camera, Light, Model, Transform},
    ecs::{system_manager::System, ComponentManager, Context, EntityManager, Ev},
};
use cgmath::{prelude::*, Matrix4, Point3, Vector3};
use glium::{
    draw_parameters::{BackfaceCullingMode, Blend, DepthTest},
    framebuffer::SimpleFrameBuffer,
    texture::{CubeLayer, DepthCubemap, Texture2d},
    uniform,
    uniforms::{DepthTextureComparison, MagnifySamplerFilter, Sampler, SamplerBehavior},
    Depth, Display, DrawParameters, Surface,
};

pub const LAYERS: &[(CubeLayer, Vector3<f32>, Vector3<f32>)] = &[
    (
        CubeLayer::PositiveX,
        Vector3::new(1.0, 0.0, 0.0),
        Vector3::new(0.0, -1.0, 0.0),
    ),
    (
        CubeLayer::NegativeX,
        Vector3::new(-1.0, 0.0, 0.0),
        Vector3::new(0.0, -1.0, 0.0),
    ),
    (
        CubeLayer::PositiveY,
        Vector3::new(0.0, 1.0, 0.0),
        Vector3::new(0.0, 0.0, -1.0),
    ),
    (
        CubeLayer::NegativeY,
        Vector3::new(0.0, -1.0, 0.0),
        Vector3::new(0.0, 0.0, 1.0),
    ),
    (
        CubeLayer::PositiveZ,
        Vector3::new(0.0, 0.0, 1.0),
        Vector3::new(0.0, -1.0, 0.0),
    ),
    (
        CubeLayer::NegativeZ,
        Vector3::new(0.0, 0.0, -1.0),
        Vector3::new(0.0, -1.0, 0.0),
    ),
];

pub struct LightRenderer {
    pub lighting_draw_parameters: DrawParameters<'static>,
    pub lighting_shader: Shader,
    pub lighting_sampler_behavior: SamplerBehavior,
    pub shadow_draw_parameters: DrawParameters<'static>,
    pub shadow_shader: Shader,
    pub shadow_sampler_behavior: SamplerBehavior,
    pub shadow_dimension: u32,
    pub filter: MagnifySamplerFilter,
    pub proj: Proj,
}

impl LightRenderer {
    pub fn new(
        display: &Display,
        lighting_sampler_behavior: SamplerBehavior,
        shadow_dimension: u32,
        filter: MagnifySamplerFilter,
        proj: Proj,
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
            shadow_dimension,
            proj,
        })
    }
}

impl System for LightRenderer {
    fn update(
        &mut self,
        event: &mut Ev,
        scene: &mut Context,
        (em, cm): (&mut EntityManager, &mut ComponentManager),
    ) -> anyhow::Result<()> {
        if let Ev::Draw((_, target)) = event {
            if let Some((c, ct)) = em.entities().find_map(|e| {
                Some((
                    cm.get::<Camera>(e).and_then(|c| c.active.then_some(c))?,
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
                let (surface_width, surface_height) = target.get_dimensions();
                let shadow_buffer = DepthCubemap::empty(&scene.display, self.shadow_dimension)?;
                let camera_proj: [[f32; 4]; 4] = c.matrix().into();
                let camera_transform: [[f32; 4]; 4] = ct.matrix().into();
                let camera_position: [f32; 3] = ct.position().into();
                let light_proj: [[f32; 4]; 4] = self.proj.matrix().into();

                for (l, lt) in em.entities().filter_map(|e| {
                    Some((
                        cm.get::<Light>(e).and_then(|l| l.active.then_some(l))?,
                        cm.get::<Transform>(e).and_then(|t| t.active.then_some(t))?,
                    ))
                }) {
                    let light_color: [f32; 3] = l.color.into();
                    let light_position: [f32; 3] = lt.position().into();

                    for (layer, t, u) in LAYERS {
                        let mut shadow_target = SimpleFrameBuffer::depth_only(
                            &scene.display,
                            shadow_buffer.main_level().image(*layer),
                        )?;
                        let light_transform: [[f32; 4]; 4] = Matrix4::look_at_rh(
                            Point3::from_vec(lt.position()),
                            Point3::from_vec(lt.position() + t),
                            *u,
                        )
                        .into();

                        for (m, t) in &models {
                            let (mesh, _, _) = &*m.data;
                            let (v, i) = &*mesh.buffer;
                            let transform: [[f32; 4]; 4] = t.matrix().into();
                            let u = uniform! {
                                transform: transform,
                                light_transform: light_transform,
                                light_proj: light_proj,
                            };

                            shadow_target.draw(
                                v,
                                i.source(),
                                &self.shadow_shader.program,
                                &u,
                                &self.shadow_draw_parameters,
                            )?;
                        }
                    }

                    for (m, t) in &models {
                        let buffer =
                            Texture2d::empty(&scene.display, surface_width, surface_height)?;

                        target.fill(&buffer.as_surface(), self.filter);

                        let (mesh, ma, _) = &*m.data;
                        let (v, i) = &*mesh.buffer;
                        let transform: [[f32; 4]; 4] = t.matrix().into();
                        let u = uniform! {
                            transform: transform,
                            camera_transform: camera_transform,
                            camera_proj: camera_proj,
                            buffer: Sampler(&buffer, self.lighting_sampler_behavior),
                            shadow_buffer: Sampler(&shadow_buffer, self.shadow_sampler_behavior),
                            camera_position: camera_position,
                            light_color: light_color,
                            light_position: light_position,
                            screen_dims: [surface_width as f32, surface_height as f32],
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
