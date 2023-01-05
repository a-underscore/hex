use crate::{
    assets::Shader,
    components::{Camera, Sprite, Transform},
    ecs::{
        system_manager::{Ev, System},
        world::World,
    },
};
use glium::{uniform, uniforms::Sampler, Display, Surface};

pub struct Renderer {
    pub shader: Shader,
}

impl Renderer {
    pub fn new(display: &Display) -> anyhow::Result<Self> {
        Ok(Self {
            shader: Shader::new(display)?,
        })
    }
}

impl<'a> System<'a> for Renderer {
    fn update(&mut self, event: &mut Ev, world: &mut World) -> anyhow::Result<()> {
        if let Ev::Draw((_, target)) = event {
            if let Some((c, ct)) = world.entity_manager.entities.keys().cloned().find_map(|e| {
                world
                    .component_manager
                    .get::<Camera>(e, &world.entity_manager)
                    .and_then(|c| {
                        Some((
                            c.active.then_some(c)?,
                            world
                                .component_manager
                                .get::<Transform>(e, &world.entity_manager)?,
                        ))
                    })
            }) {
                let mut sprites: Vec<_> = world
                    .entity_manager
                    .entities
                    .keys()
                    .cloned()
                    .filter_map(|e| {
                        world
                            .component_manager
                            .get::<Sprite>(e, &world.entity_manager)
                            .and_then(|s| {
                                Some((
                                    s.active.then_some(s)?,
                                    world
                                        .component_manager
                                        .get::<Transform>(e, &world.entity_manager)?,
                                ))
                            })
                    })
                    .collect();

                sprites.sort_by(|(s1, _), (s2, _)| s1.z.total_cmp(&s2.z));

                for (s, t) in sprites {
                    let color: [f32; 4] = s.color.into();
                    let transform: [[f32; 3]; 3] = t.matrix().into();
                    let camera_view: [[f32; 4]; 4] = c.view().into();
                    let camera_transform: [[f32; 3]; 3] = ct.matrix().into();
                    let uniform = uniform! {
                        z: s.z,
                        transform: transform,
                        camera_transform: camera_transform,
                        camera_view: camera_view,
                        color: color,
                        image: Sampler(&*s.texture.buffer, s.texture.sampler_behaviour),
                    };

                    target.draw(
                        &*s.shape.vertices,
                        &*s.shape.indices,
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
