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
    pub shape: Rc<Shape>,
    pub texture: Rc<Texture>,
    pub shaders: Rc<Shaders>,
    pub transform: Rc<Transform>,
    pub z: f32,
}

impl SpriteData {
    fn new(
        id: Rc<String>,
        color: Vector4<f32>,
        shape: Rc<Shape>,
        texture: Rc<Texture>,
        shaders: Rc<Shaders>,
        transform: Rc<Transform>,
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
        shape: Rc<Shape>,
        texture: Rc<Texture>,
        shaders: Rc<Shaders>,
        transform: Rc<Transform>,
        layer: f32,
    ) -> Rc<Self> {
        Rc::new(Self {
            tid: ecs::id(SPRITE_ID),
            data: SpriteData::new(id, color, shape, texture, shaders, transform, layer),
        })
    }

    pub fn draw(&self, engine: &Engine, target: &mut Frame) -> anyhow::Result<()> {
        let data = self.data.borrow();
        let transform_data = data.transform.data.borrow();
        let scene_data = engine.scene.data.borrow();
        let camera_data = scene_data.camera.data.borrow();
        let camera_transform_data = camera_data.transform.data.borrow();
        let color: [f32; 4] = data.color.into();
        let translation: [[f32; 3]; 3] = Matrix3::from_translation(transform_data.position).into();
        let rotation: [[f32; 2]; 2] = Matrix2::from_angle(Rad(transform_data.rotation)).into();
        let scale: [[f32; 3]; 3] =
            Matrix3::from_nonuniform_scale(transform_data.scale.x, transform_data.scale.y).into();
        let camera_translation: [[f32; 3]; 3] =
            Matrix3::from_translation(camera_transform_data.position).into();
        let camera_rotation: [[f32; 2]; 2] =
            Matrix2::from_angle(Rad(camera_transform_data.rotation)).into();
        let camera_view: [[f32; 4]; 4] = cgmath::ortho(
            -camera_transform_data.scale.x,
            camera_transform_data.scale.x,
            -camera_transform_data.scale.y,
            camera_transform_data.scale.y,
            -1.0,
            1.0,
        )
        .into();
        let uniforms = uniform! {
            translation: translation,
            rotation: rotation,
            scale: scale,
            z: data.z,
            camera_translation: camera_translation,
            camera_rotation: camera_rotation,
            camera_view: camera_view,
            color: color,
            texture: &data.texture.texture,
        };

        target.draw(
            &data.shape.vertices,
            &data.shape.indices,
            &data.shaders.program,
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
