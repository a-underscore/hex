use crate::{
    assets::Shader,
    components::{Camera, Sprite, Transform},
    ecs::{ev::Control, system_manager::System, Ev, World},
};
use glium::{glutin::event::Event, index::NoIndices, uniform, uniforms::Sampler, Display, Surface};

pub static VERTEX_SRC: &str = include_str!("vertex.glsl");
pub static FRAGMENT_SRC: &str = include_str!("fragment.glsl");

pub struct Renderer {
    pub shader: Shader,
}

impl Renderer {
    pub fn new(display: &Display) -> anyhow::Result<Self> {
        Ok(Self {
            shader: Shader::new(display, VERTEX_SRC, FRAGMENT_SRC, None)?,
        })
    }
}

impl<'a> System<'a> for Renderer {
    fn update(&mut self, event: &mut Ev, world: &mut World<'a>) -> anyhow::Result<()> {
        if let Ev::Draw((
            Control {
                event: Event::MainEventsCleared,
                flow: _,
            },
            target,
        )) = event
        {
            if let Some((c, ct)) = world.em.entities.keys().cloned().find_map(|e| {
                Some((
                    world
                        .cm
                        .get::<Camera>(e, &world.em)
                        .and_then(|c| c.active.then_some(c))?,
                    world
                        .cm
                        .get::<Transform>(e, &world.em)
                        .and_then(|t| t.active.then_some(t))?,
                ))
            }) {
                let sprites = {
                    let mut sprites: Vec<_> = world
                        .em
                        .entities
                        .keys()
                        .cloned()
                        .filter_map(|e| {
                            Some((
                                world
                                    .cm
                                    .get::<Sprite>(e, &world.em)
                                    .and_then(|s| s.active.then_some(s))?,
                                world
                                    .cm
                                    .get::<Transform>(e, &world.em)
                                    .and_then(|t| t.active.then_some(t))?,
                            ))
                        })
                        .collect();

                    sprites.sort_by(|(s1, _), (s2, _)| s1.z.total_cmp(&s2.z));

                    sprites
                };

                let camera_view: [[f32; 4]; 4] = c.view().into();
                let camera_transform: [[f32; 3]; 3] = ct.matrix().into();

                for (s, t) in sprites {
                    let color: [f32; 4] = s.color.into();
                    let transform: [[f32; 3]; 3] = t.matrix().into();
                    let uniform = uniform! {
                        z: s.z,
                        transform: transform,
                        camera_transform: camera_transform,
                        camera_view: camera_view,
                        color: color,
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
