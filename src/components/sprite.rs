use crate::{
    assets::{Shaders, Shape, Texture, Transform},
    ecs::{self, AsAny, Component},
    engine::Engine,
};
use cgmath::{Matrix2, Matrix3, Rad, Vector4};
use glium::{uniform, Frame, Surface};
use std::{any::Any, cell::RefCell, rc::Rc};

pub const SPRITE_ID: &str = "sprite";

pub struct SpriteData {
    pub id: Rc<String>,
    pub color: Vector4<f32>,
    pub shape: Rc<RefCell<Shape>>,
    pub texture: Rc<RefCell<Texture>>,
    pub shaders: Rc<RefCell<Shaders>>,
    pub transform: Rc<RefCell<Transform>>,
    pub z: f32,
}

impl SpriteData {
    fn new(
        id: Rc<String>,
        color: Vector4<f32>,
        shape: Rc<RefCell<Shape>>,
        texture: Rc<RefCell<Texture>>,
        shaders: Rc<RefCell<Shaders>>,
        transform: Rc<RefCell<Transform>>,
        z: f32,
    ) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            id,
            color,
            shape,
            texture,
            shaders,
            transform,
            z,
        }))
    }
}

#[derive(ecs::derive::Component)]
pub struct Sprite {
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
        transform: Rc<RefCell<Transform>>,
        layer: f32,
    ) -> Rc<Self> {
        Rc::new(Self {
            tid: ecs::id(SPRITE_ID),
            data: SpriteData::new(id, color, shape, texture, shaders, transform, layer),
        })
    }

    pub fn draw(&self, engine: &Engine, target: &mut Frame) -> anyhow::Result<()> {
        let data = self.data.borrow();
        let scene = engine.scene.borrow();
        let camera = scene.camera.data.borrow();
        let color: [f32; 4] = data.color.into();
        let translation: [[f32; 3]; 3] =
            Matrix3::from_translation(data.transform.borrow().position).into();
        let rotation: [[f32; 2]; 2] =
            Matrix2::from_angle(Rad(data.transform.borrow().rotation)).into();
        let scale: [[f32; 3]; 3] = Matrix3::from_nonuniform_scale(
            data.transform.borrow().scale.x,
            data.transform.borrow().scale.y,
        )
        .into();
        let camera_translation: [[f32; 3]; 3] =
            Matrix3::from_translation(camera.transform.borrow().position).into();
        let camera_rotation: [[f32; 2]; 2] =
            Matrix2::from_angle(Rad(camera.transform.borrow().rotation)).into();
        let camera_view: [[f32; 4]; 4] = cgmath::ortho(
            -camera.transform.borrow().scale.x,
            camera.transform.borrow().scale.x,
            -camera.transform.borrow().scale.y,
            camera.transform.borrow().scale.y,
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

        Ok(())
    }
}

impl Component for Sprite {
    fn id(&self) -> Rc<String> {
        self.data.borrow().id.clone()
    }

    fn tid(&self) -> Rc<String> {
        self.tid.clone()
    }
}
