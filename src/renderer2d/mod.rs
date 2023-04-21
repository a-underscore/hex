use crate::{
    assets::Shader,
    components::{Camera2d, Sprite, Transform2d},
    ecs::{ev::Control, system_manager::System, ComponentManager, EntityManager, Ev, Scene},
};
use glium::{glutin::event::Event, index::NoIndices, uniform, uniforms::Sampler, Display, Surface};

pub struct Renderer2d {
    pub shader: Shader,
}

impl Renderer2d {
    pub fn new(display: &Display) -> anyhow::Result<Self> {
        Ok(Self {
            shader: Shader::new(
                display,
                include_str!("vertex.glsl"),
                include_str!("fragment.glsl"),
                None,
            )?,
        })
    }
}

impl<'a> System<'a> for Renderer2d {
    fn update(
        &mut self,
        event: &mut Ev,
        _: &mut Scene,
        (em, cm): (&mut EntityManager, &mut ComponentManager),
    ) -> anyhow::Result<()> {
        if let Ev::Draw((
            Control {
                event: Event::MainEventsCleared,
                flow: _,
            },
            target,
        )) = event
        {
            if let Some((c, ct)) = em.entities.keys().cloned().find_map(|e| {
                Some((
                    cm.get::<Camera2d>(e, em)
                        .and_then(|c| c.active.then_some(c))?,
                    cm.get::<Transform2d>(e, em)
                        .and_then(|t| t.active.then_some(t))?,
                ))
            }) {
                let sprites = {
                    let mut sprites: Vec<_> = em
                        .entities
                        .keys()
                        .cloned()
                        .filter_map(|e| {
                            Some((
                                cm.get::<Sprite>(e, em)
                                    .and_then(|s| s.active.then_some(s))?,
                                cm.get::<Transform2d>(e, em)
                                    .and_then(|t| t.active.then_some(t))?,
                            ))
                        })
                        .collect();

                    sprites.sort_by(|(s1, _), (s2, _)| s1.z.total_cmp(&s2.z));

                    sprites
                };

                for (s, t) in sprites {
                    let uniform = uniform! {
                        z: s.z,
                        transform: t.matrix().0,
                        camera_transform: ct.matrix().0,
                        camera_view: c.view().0,
                        color: s.color,
                        tex: Sampler(&*s.texture.buffer, s.texture.sampler_behaviour),
                    };

                    target.draw(
                        &*s.shape.vertices,
                        NoIndices(s.shape.format),
                        &self.shader.program,
                        &uniform,
                        &s.draw_parameters,
                    )?;
                }
            }
        }

        Ok(())
    }
}
