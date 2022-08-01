use crate::{
    assets::{Shaders, Shape, Texture},
    components::{Transform, TRANSFORM_ID},
    ecs::{self, AsAny, Component, Entity},
    engine::Engine,
};
use cgmath::{Matrix2, Matrix3, Rad, Vector4};
use glium::{uniform, Frame, Surface};
use std::{any::Any, cell::RefCell, rc::Rc};

thread_local! {
    pub static SPRITE_ID: Rc<String> = ecs::id("sprite");
}

pub struct SpriteData {
    pub parent: Option<Rc<Entity>>,
    pub color: Vector4<f32>,
    pub shape: Rc<RefCell<Shape>>,
    pub texture: Rc<RefCell<Texture>>,
    pub shaders: Rc<RefCell<Shaders>>,
    pub z: f32,
    pub draw: bool,
}

impl SpriteData {
    fn new(
        color: Vector4<f32>,
        shape: Rc<RefCell<Shape>>,
        texture: Rc<RefCell<Texture>>,
        shaders: Rc<RefCell<Shaders>>,
        z: f32,
        draw: bool,
    ) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            parent: None,
            color,
            shape,
            texture,
            shaders,
            z,
            draw,
        }))
    }
}

#[derive(ecs::derive::Component)]
pub struct Sprite {
    pub id: Rc<String>,
    pub tid: Rc<String>,
    pub data: Rc<RefCell<SpriteData>>,
}

impl Sprite {
    pub fn new(
        id: Rc<String>,
        color: Vector4<f32>,
        shape: Rc<RefCell<Shape>>,
        texture: Rc<RefCell<Texture>>,
        shaders: Rc<RefCell<Shaders>>,
        layer: f32,
        draw: bool,
    ) -> Rc<Self> {
        Rc::new(Self {
            id,
            tid: SPRITE_ID.with(|id| id.clone()),
            data: SpriteData::new(color, shape, texture, shaders, layer, draw),
        })
    }

    pub fn draw(&self, parent: &Entity, engine: &Engine, target: &mut Frame) -> anyhow::Result<()> {
        let data = self.data.borrow();

        if data.draw {
            if let (Some(transform), Some(camera_transform)) = (
                engine
                    .scene
                    .borrow()
                    .camera
                    .get_first::<Transform>(TRANSFORM_ID.with(|id| id.clone())),
                parent.get_first::<Transform>(TRANSFORM_ID.with(|id| id.clone())),
            ) {
                let color: [f32; 4] = data.color.into();
                let translation: [[f32; 3]; 3] =
                    Matrix3::from_translation(transform.data.borrow().position).into();
                let rotation: [[f32; 2]; 2] =
                    Matrix2::from_angle(Rad(transform.data.borrow().rotation)).into();
                let scale: [[f32; 3]; 3] = Matrix3::from_nonuniform_scale(
                    transform.data.borrow().scale.x,
                    transform.data.borrow().scale.y,
                )
                .into();
                let camera_translation: [[f32; 3]; 3] =
                    Matrix3::from_translation(camera_transform.data.borrow().position).into();
                let camera_rotation: [[f32; 2]; 2] =
                    Matrix2::from_angle(Rad(camera_transform.data.borrow().rotation)).into();
                let camera_view: [[f32; 4]; 4] = cgmath::ortho(
                    -camera_transform.data.borrow().scale.x,
                    camera_transform.data.borrow().scale.x,
                    -camera_transform.data.borrow().scale.y,
                    camera_transform.data.borrow().scale.y,
                    -1.0,
                    1.0,
                )
                .into();
                let texture = data.texture.borrow();
                let uniforms = uniform! {
                    translation: translation,
                    rotation: rotation,
                    scale: scale,
                    z: data.z,
                    camera_translation: camera_translation,
                    camera_rotation: camera_rotation,
                    camera_view: camera_view,
                    color: color,
                    texture: &texture.texture,
                };

                target.draw(
                    &data.shape.borrow().vertices,
                    &data.shape.borrow().indices,
                    &data.shaders.borrow().program,
                    &uniforms,
                    &engine.draw_parameters,
                )?;
            }
        }

        Ok(())
    }
}

impl Component for Sprite {
    fn id(&self) -> Rc<String> {
        self.id.clone()
    }

    fn tid(&self) -> Rc<String> {
        self.tid.clone()
    }

    fn parent(&self) -> Option<Rc<Entity>> {
        self.data.borrow().parent.clone()
    }

    fn set_parent(&self, parent: Option<Rc<Entity>>) {
        self.data.borrow_mut().parent = parent;
    }
}
