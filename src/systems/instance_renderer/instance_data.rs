use crate::math::{Mat4d, Vec4d};
use glium::implement_vertex;

#[derive(Copy, Clone)]
pub struct InstanceData {
    pub transform: [[f32; 4]; 4],
    pub color: [f32; 4],
}

impl InstanceData {
    pub fn new(transform: Mat4d, color: Vec4d) -> Self {
        Self {
            transform: transform.0,
            color: color.0,
        }
    }
}

implement_vertex!(InstanceData, transform, color);
use crate::{
    ecs::{component_manager::Component, Id},
    id,
    math::{Mat3, Vec2d},
};

#[derive(Clone)]
pub struct Transform {
    position: Vec2d,
    rotation: f32,
    scale: Vec2d,
    matrix: Mat3,
    pub active: bool,
}

impl Transform {
    pub fn new(position: Vec2d, rotation: f32, scale: Vec2d, active: bool) -> Self {
        Self {
            position,
            rotation,
            scale,
            matrix: Self::calculate_matrix(position, rotation, scale),
            active,
        }
    }

    pub fn position(&self) -> Vec2d {
        self.position
    }

    pub fn set_position(&mut self, position: Vec2d) {
        self.position = position;

        self.update_matrix();
    }

    pub fn rotation(&self) -> f32 {
        self.rotation
    }

    pub fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation;

        self.update_matrix();
    }

    pub fn scale(&self) -> Vec2d {
        self.scale
    }

    pub fn set_scale(&mut self, scale: Vec2d) {
        self.scale = scale;

        self.update_matrix();
    }

    pub fn matrix(&self) -> Mat3 {
        self.matrix
    }

    pub fn update_matrix(&mut self) {
        self.matrix = Self::calculate_matrix(self.position, self.rotation, self.scale);
    }

    pub fn calculate_matrix(position: Vec2d, rotation: f32, scale: Vec2d) -> Mat3 {
        Mat3::translation(position) * Mat3::rotation(rotation) * Mat3::scale(scale)
    }
}

impl Component for Transform {
    fn id() -> Id {
        id!()
    }
}
use crate::{
    ecs::{component_manager::Component, Id},
    id,
};

#[derive(Default, PartialEq, PartialOrd, Eq, Ord, Copy, Clone)]
pub struct InstanceId {
    pub id: Id,
    pub active: bool,
}

impl InstanceId {
    pub fn new(id: Id, active: bool) -> Self {
        Self { id, active }
    }
}

impl Component for InstanceId {
    fn id() -> Id {
        id!()
    }
}
pub mod projection;

pub use projection::Projection;

use crate::{
    ecs::{component_manager::Component, Id},
    id,
    math::{Vec3d, Mat4d, Vec2d},
};

#[derive(Clone)]
pub struct Camera {
    view: Mat4d,
    proj: Projection,
    pub active: bool,
}

impl Camera {
    pub fn perspective(fov: f32, aspect: f32, clip: Vec2d, active: bool) -> Self {
        let proj = Projection::Perspective(fov, aspect, clip);

        Self {
            view: proj.view(),
            proj,
            active,
        }
    }

    pub fn ortho(dims: Vec3d, active: bool) -> Self {
        let proj = Projection::Ortho(dims);

        Self {
            view: proj.view(),
            proj, 
            active,
        }
    }

    pub fn proj(&self) -> &Projection {
        &self.proj
    }

    pub fn set_proj(&mut self, proj: Projection) {
        self.proj = proj;

        self.update_view()
    }

    pub fn update_view(&mut self) {
        self.view = self.proj.view();
    }
}

impl Component for Camera {
    fn id() -> Id {
        id!()
    }
}
pub mod component_manager;
pub mod entity_manager;
pub mod ev;
pub mod scene;
pub mod system_manager;
pub mod world;

pub use component_manager::ComponentManager;
pub use entity_manager::EntityManager;
pub use ev::Ev;
pub use state::State;
pub use system_manager::SystemManager;
pub use world::World;
pub mod assets;
pub mod components;
pub mod ecs;
pub mod math;
pub mod systems;

pub use anyhow;
pub use glium;
pub use once_cell;

#[macro_export]
macro_rules! id {
    () => {{
        use $crate::{
            ecs::{id, Id},
            once_cell::sync::Lazy,
        };

        static ID: Lazy<Id> = Lazy::new(|| id());

        *ID
    }};
}
use crate::{
    assets::Shader,
    components::{Camera, Model, Transform},
    ecs::{system_manager::System, ComponentManager, EntityManager, Ev, Scene},
};
use glium::{uniform, uniforms::Sampler, Display, Surface};

pub struct Renderer {
    pub texture_shader: Shader,
    pub color_shader: Shader,
}

impl Renderer {
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

impl<'a> System<'a> for Renderer {
    fn update(
        &mut self,
        event: &mut Ev,
        _: &mut Scene,
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
                                cm.get::<Model>(e, em).and_then(|m| m.active.then_some(m))?,
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

                for (m, t) in models {
                    let (v, i) = &*m.mesh.buffer;

                    match &m.texture {
                        Some(texture) => {
                            let (uv, buffer) = &*texture.buffer;
                            let u = uniform! {
                                transform: t.matrix().0,
                                camera_transform: ct.matrix().0,
                                camera_view: c.view().0,
                                color: m.color.0,
                                tex: Sampler(buffer, texture.sampler_behaviour),
                            };

                            target.draw(
                                (v, uv),
                                i.source(),
                                &self.texture_shader.program,
                                &u,
                                &m.draw_parameters,
                            )?;
                        }
                        None => {
                            let u = uniform! {
                                transform: t.matrix().0,
                                camera_transform: ct.matrix().0,
                                camera_view: c.view().0,
                                color: m.color.0,
                            };

                            target.draw(
                                v,
                                i.source(),
                                &self.color_shader.program,
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
pub mod mat3;
pub mod ortho;
pub mod vec2d;

pub use mat3::Mat3;
pub use ortho::Ortho;
pub use vec2d::Vec2d;
#version 330

in vec3 position;
in vec3 normal;
in vec2 uv;

out vec3 v_pos;
out vec3 v_normal;
out vec2 tex_pos;

uniform mat4 transform;
uniform mat4 camera_transform;
uniform mat4 camera_view;

void main(void) {
	mat4 view = transform * inverse(camera_transform);
        vec4 pos = vec4(position, 1.0) * view;
	vec4 normal = vec4(normal, 1.0) * view;

        gl_Position = pos * camera_view;

	v_pos = vec3(pos);
	v_normal = vec3(normal);
	tex_pos = uv;
}
use super::{ev::Control, Ev, SystemManager, World};
use glium::{
    glutin::{
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
    },
    Display, Surface,
};

#[derive(Clone)]
pub struct State {
    pub display: Display,
    pub bg: [f32; 4],
}

impl State {
    pub fn new(display: Display, bg: [f32; 4]) -> Self {
        Self { display, bg }
    }

    pub fn init(
        mut self,
        event_loop: EventLoop<()>,
        mut world: World<'static>,
        mut system_manager: SystemManager<'static>,
    ) -> anyhow::Result<()> {
        system_manager.init(&mut self, &mut world)?;

        event_loop.run(move |event, _, control_flow| {
            if let Err(e) = self.update(
                Control::new(event),
                control_flow,
                &mut world,
                &mut system_manager,
            ) {
                eprintln!("{}", e);
            }
        })
    }

    pub fn update(
        &mut self,
        mut control: Control,
        flow: &mut ControlFlow,
        world: &mut World,
        system_manager: &mut SystemManager,
    ) -> anyhow::Result<()> {
        system_manager.update(&mut Ev::Event(&mut control), self, world)?;

        if let Event::MainEventsCleared = &control.event {
            let mut target = self.display.draw();

            target.clear_color_and_depth(
                {
                    let [r, g, b, a] = self.bg;

                    (r, g, b, a)
                },
                1.0,
            );

            system_manager.update(&mut Ev::Draw((&mut control, &mut target)), self, world)?;

            target.finish()?;
        }

        if let Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } = &control.event
        {
            *flow = ControlFlow::Exit;
        } else if let Some(control_flow) = control.flow {
            *flow = control_flow;
        } else {
            *flow = ControlFlow::Poll;
        }

        Ok(())
    }
}
#version 330

in vec3 position;
in vec3 normal;

out vec3 v_pos;
out vec3 v_normal;

uniform mat4 transform;
uniform mat4 camera_transform;
uniform mat4 camera_view;

void main(void) {
	mat4 view = transform * inverse(camera_transform);
        vec4 pos = vec4(position, 1.0) * view;
	vec4 normal = vec4(normal, 1.0) * view;

        gl_Position = pos * camera_view;

	v_pos = vec3(pos);
	v_normal = vec3(normal);
}
use crate::math::{Vec2d, Vec3d, Mat4d};

#[derive(Clone)]
pub enum Projection {
    Perspective(f32, f32, Vec2d),
    Ortho(Vec3d),
}

impl Projection {
    pub fn view(&self) -> Mat4d {
        match self {     
            Self::Perspective(fov, aspect, clip) => {
                Mat4d::perspective(*fov, *aspect, clip.x(), clip.y())
            },
            Self::Ortho(dims) => {
                let dims = *dims / 2.0;

                Mat4d::ortho(-dims.x(), dims.x(), -dims.y(), dims.y(), -dims.z(), dims.z())
            }
        }
    }
}
pub mod instance_renderer;
pub mod renderer;

pub use instance_renderer::InstanceRenderer;
pub use renderer::Renderer;
use super::Vec2d;
use std::ops::{Div, DivAssign, Mul, MulAssign};

#[derive(Default, PartialEq, PartialOrd, Copy, Clone)]
pub struct Mat3(pub [[f32; 3]; 3]);

impl Mat3 {
    pub fn new(x: [f32; 3], y: [f32; 3], z: [f32; 3]) -> Self {
        Self([x, y, z])
    }

    pub fn rotation(rotation: f32) -> Self {
        let (sin, cos) = rotation.sin_cos();

        Self([[cos, -sin, 0.0], [sin, cos, 0.0], [0.0, 0.0, 1.0]])
    }

    pub fn scale(scale: Vec2d) -> Self {
        Self([
            [scale.x(), 0.0, 0.0],
            [0.0, scale.y(), 0.0],
            [0.0, 0.0, 1.0],
        ])
    }

    pub fn translation(translation: Vec2d) -> Self {
        Self([
            [1.0, 0.0, translation.x()],
            [0.0, 1.0, translation.y()],
            [0.0, 0.0, 1.0],
        ])
    }

    pub fn determinant(&self) -> f32 {
        self.0[0][0] * (self.0[1][1] * self.0[2][2] - self.0[2][1] * self.0[1][2])
            - self.0[1][0] * (self.0[0][1] * self.0[2][2] - self.0[2][1] * self.0[0][2])
            + self.0[2][0] * (self.0[0][1] * self.0[1][2] - self.0[1][1] * self.0[0][2])
    }

    pub fn adjacent(&self) -> Self {
        Self::new(
            [
                (self.0[1][1] * self.0[2][2] - self.0[1][2] * self.0[2][1]),
                (self.0[1][2] * self.0[2][0] - self.0[1][0] * self.0[2][2]),
                (self.0[1][0] * self.0[2][1] - self.0[1][1] * self.0[2][0]),
            ],
            [
                (self.0[2][1] * self.0[0][2] - self.0[2][2] * self.0[0][1]),
                (self.0[2][2] * self.0[0][0] - self.0[2][0] * self.0[0][2]),
                (self.0[2][0] * self.0[0][1] - self.0[2][1] * self.0[0][0]),
            ],
            [
                (self.0[0][1] * self.0[1][2] - self.0[0][2] * self.0[1][1]),
                (self.0[0][2] * self.0[1][0] - self.0[0][0] * self.0[1][2]),
                (self.0[0][0] * self.0[1][1] - self.0[0][1] * self.0[1][0]),
            ],
        )
    }

    pub fn inverse(&self) -> Self {
        self.adjacent() / self.determinant()
    }

    pub fn identity() -> Self {
        Self([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]])
    }
}

impl Mul<f32> for Mat3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self([
            [self.0[0][0] * rhs, self.0[0][1] * rhs, self.0[0][2] * rhs],
            [self.0[1][0] * rhs, self.0[1][1] * rhs, self.0[1][2] * rhs],
            [self.0[2][0] * rhs, self.0[2][1] * rhs, self.0[2][2] * rhs],
        ])
    }
}

impl MulAssign<f32> for Mat3 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = Self::mul(*self, rhs);
    }
}

impl Div<f32> for Mat3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        Self::mul(self, 1.0 / rhs)
    }
}

impl DivAssign<f32> for Mat3 {
    fn div_assign(&mut self, rhs: f32) {
        *self = Self::div(*self, rhs);
    }
}

impl Mul for Mat3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self([
            [
                self.0[0][0] * rhs.0[0][0]
                    + self.0[0][1] * rhs.0[1][0]
                    + self.0[0][2] * rhs.0[2][0],
                self.0[0][0] * rhs.0[0][1]
                    + self.0[0][1] * rhs.0[1][1]
                    + self.0[0][2] * rhs.0[2][1],
                self.0[0][0] * rhs.0[0][2]
                    + self.0[0][1] * rhs.0[1][2]
                    + self.0[0][2] * rhs.0[2][2],
            ],
            [
                self.0[1][0] * rhs.0[0][0]
                    + self.0[1][1] * rhs.0[1][0]
                    + self.0[1][2] * rhs.0[2][0],
                self.0[1][0] * rhs.0[0][1]
                    + self.0[1][1] * rhs.0[1][1]
                    + self.0[1][2] * rhs.0[2][1],
                self.0[1][0] * rhs.0[0][2]
                    + self.0[1][1] * rhs.0[1][2]
                    + self.0[1][2] * rhs.0[2][2],
            ],
            [
                self.0[2][0] * rhs.0[0][0]
                    + self.0[2][1] * rhs.0[1][0]
                    + self.0[2][2] * rhs.0[2][0],
                self.0[2][0] * rhs.0[0][1]
                    + self.0[2][1] * rhs.0[1][1]
                    + self.0[2][2] * rhs.0[2][1],
                self.0[2][0] * rhs.0[0][2]
                    + self.0[2][1] * rhs.0[1][2]
                    + self.0[2][2] * rhs.0[2][2],
            ],
        ])
    }
}

impl MulAssign for Mat3 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = Self::mul(*self, rhs);
    }
}

impl Mul<(Vec2d, f32)> for Mat3 {
    type Output = (Vec2d, f32);

    fn mul(self, (rhs, z): (Vec2d, f32)) -> (Vec2d, f32) {
        (
            Vec2d::new(
                self.0[0][0] * rhs.x() + self.0[0][1] * rhs.y() + self.0[0][2] * z,
                self.0[1][0] * rhs.x() + self.0[1][1] * rhs.y() + self.0[1][2] * z,
            ),
            self.0[2][0] * rhs.x() + self.0[2][1] * rhs.y() + self.0[2][2] * z,
        )
    }
}
use crate::{
    ecs::{component_manager::Component, Id},
    id,
    math::{Mat4d, Vec2d, Vec3d},
};

#[derive(Clone)]
pub struct Camera2d {
    dimensions: Vec3d,
    view: Mat4d,
    pub active: bool,
}

impl Camera2d {
    pub fn new(dimensions: Vec3d, active: bool) -> Self {
        Self {
            dimensions,
            view: Self::calculate_view(dimensions),
            active,
        }
    }

    pub fn dimensions(&self) -> Vec3d {
        self.dimensions
    }

    pub fn set_dimensions(&mut self, dimensions: Vec3d) {
        self.dimensions = dimensions;

        self.update_view();
    }

    pub fn view(&self) -> Mat4d {
        self.view
    }

    pub fn update_view(&mut self) {
        self.view = Self::calculate_view(self.dimensions);
    }

    pub fn calculate_view(view: Vec3d) -> Mat4d {
        let view = view / 2.0;

        Mat4d::perspective(-view.x(), view.x(), -view.y(), view.y(), -view.z(), view.z())
    }
}

impl Component for Camera2d {
    fn id() -> Id {
        id!()
    }
}

