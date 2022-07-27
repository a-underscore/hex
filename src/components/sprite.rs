use crate::{
    assets::{Shaders, Shape, Texture},
    ecs::{self, AsAny, Component},
};
use cgmath::Vector4;
use glium::{uniform, DrawParameters, Frame, Surface};
use std::{any::Any, cell::RefCell, rc::Rc};

pub const SPRITE_ID: &str = "sprite";

pub struct SpriteData {
    pub color: Vector4<f32>,
    pub shape: Rc<Shape>,
    pub texture: Rc<Texture>,
    pub shaders: Rc<Shaders>,
}

impl SpriteData {
    fn new(
        color: Vector4<f32>,
        shape: Rc<Shape>,
        texture: Rc<Texture>,
        shaders: Rc<Shaders>,
    ) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            color,
            shape,
            texture,
            shaders,
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
        shape: Rc<Shape>,
        texture: Rc<Texture>,
        shaders: Rc<Shaders>,
    ) -> Rc<Self> {
        Rc::new(Self {
            id,
            tid: ecs::id(SPRITE_ID),
            data: SpriteData::new(color, shape, texture, shaders),
        })
    }

    pub fn draw(
        self: Rc<Self>,
        target: &mut Frame,
        draw_params: &DrawParameters,
    ) -> anyhow::Result<()> {
        let data = self.data.borrow();
        let color: [f32; 4] = data.color.into();
        let uniforms = uniform! {
            color: color,
            texture: &data.texture.texture,
        };

        target.draw(
            &data.shape.vertices,
            &data.shape.indices,
            &data.shaders.program,
            &uniforms,
            draw_params,
        )?;

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
}
