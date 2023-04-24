use crate::{
    assets::Shader,
    components::{Camera, Transform},
    ecs::{system_manager::System, ComponentManager, EntityManager, Ev, Scene},
};
use glium::{
    draw_parameters::{Blend, DepthTest},
    texture::Texture2d,
    uniforms::MagnifySamplerFilter,
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
            if let Some((_, _)) = em.entities.keys().cloned().find_map(|e| {
                Some((
                    cm.get::<Camera>(e, em)
                        .and_then(|c| c.active.then_some(c))?,
                    cm.get::<Transform>(e, em)
                        .and_then(|t| t.active.then_some(t))?,
                ))
            }) {
                let (x, y) = target.get_dimensions();
                let texture = Texture2d::empty(&scene.display, x, y)?;

                // For every light
                texture.as_surface().fill(*target, self.filter);

                // For every model
                // target.draw();
                unimplemented!()
            }
        }

        Ok(())
    }
}
