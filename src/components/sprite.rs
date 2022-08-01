use crate::{
    assets::{Shaders, Shape, Texture},
    components::{Transform, TRANSFORM_ID},
    ecs::{self, AsAny, Component, Entity},
    engine::Engine,
};
use cgmath::Vector4;
use glium::{uniform, Frame, Surface};
use std::{any::Any, cell::RefCell, rc::Rc};

thread_local! {
    pub static SPRITE_ID: Rc<String> = ecs::id("sprite");
}

pub struct SpriteData {
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
    pub parent: Rc<RefCell<Option<Rc<Entity>>>>,
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
            parent: Rc::new(RefCell::new(None)),
            data: SpriteData::new(color, shape, texture, shaders, layer, draw),
        })
    }

    pub fn draw(&self, parent: &Entity, engine: &Engine, target: &mut Frame) -> anyhow::Result<()> {
        let data = self.data.borrow();

        if data.draw {
            let camera = engine.scene.borrow().camera.clone();

            if let (Some(transform), Some(camera_transform)) = (
                parent.get_first::<Transform>(TRANSFORM_ID.with(|id| id.clone())),
                camera.parent().and_then(|parent| {
                    parent.get_first::<Transform>(TRANSFORM_ID.with(|id| id.clone()))
                }),
            ) {
                let color: [f32; 4] = data.color.into();
                let transform: [[f32; 3]; 3] = transform.get_global_transform().into();
                let camera_view: [[f32; 4]; 4] = camera.view().into();
                let camera_transform: [[f32; 3]; 3] =
                    camera_transform.get_global_transform().into();
                let texture = data.texture.borrow();
                let uniforms = uniform! {
                    z: data.z,
                    transform: transform,
                    camera_transform: camera_transform,
                    camera_view: camera_view,
                    color: color,
                    texture: &texture.texture,
                };
                let shape = data.shape.borrow();
                let shaders = data.shaders.borrow();

                target.draw(
                    &shape.vertices,
                    &shape.indices,
                    &shaders.program,
                    &uniforms,
                    &engine.draw_parameters.borrow(),
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
        self.parent.borrow().clone()
    }

    fn set_parent(&self, parent: Option<Rc<Entity>>) {
        *self.parent.borrow_mut() = parent;
    }
}
