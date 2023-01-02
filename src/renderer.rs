use crate::{
    assets::Shader,
    components::{Camera, Sprite, Transform},
    ecs::{
        component_manager::ComponentManager,
        entity_manager::EntityManager,
        system_manager::{Ev, System},
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
    fn update(
        &mut self,
        _: &Display,
        event: &mut Ev,
        entity_manager: &mut EntityManager,
        component_manager: &mut ComponentManager,
    ) -> anyhow::Result<()> {
        if let Ev::Draw((_, target)) = event {
            if let Some((c, ct)) = entity_manager.entities.keys().find_map(|e| {
                component_manager
                    .get::<Camera>(*e, entity_manager)
                    .and_then(|c| {
                        Some((
                            c.active.then_some(c)?,
                            component_manager.get::<Transform>(*e, entity_manager)?,
                        ))
                    })
            }) {
                for e in entity_manager.entities.keys() {
                    if let Some((s, t)) = component_manager
                        .get::<Sprite>(*e, entity_manager)
                        .and_then(|s| {
                            Some((
                                s.active.then_some(s)?,
                                component_manager.get::<Transform>(*e, entity_manager)?,
                            ))
                        })
                    {
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
        }

        Ok(())
    }
}
