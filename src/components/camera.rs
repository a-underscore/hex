use crate::ecs::{self, derive::AsAny, AsAny, Component, Id, Parent};
use cgmath::Matrix4;
use std::{any::Any, cell::RefCell, rc::Rc};

thread_local! {
    pub static CAMERA_ID: Id = ecs::id("camera");
}

pub struct CameraData {
    pub left: f32,
    pub right: f32,
    pub bottom: f32,
    pub top: f32,
    pub near: f32,
    pub far: f32,
}

impl CameraData {
    pub fn new(
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        near: f32,
        far: f32,
    ) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            left,
            right,
            bottom,
            top,
            near,
            far,
        }))
    }
}

#[derive(AsAny)]
pub struct Camera {
    id: Id,
    tid: Id,
    parent: Parent,
    pub data: Rc<RefCell<CameraData>>,
}

impl Camera {
    pub fn new(id: Id, data: Rc<RefCell<CameraData>>) -> Rc<Self> {
        Rc::new(Self {
            id,
            tid: ecs::tid(&CAMERA_ID),
            parent: Rc::new(RefCell::new(None)),
            data,
        })
    }

    pub fn view(&self) -> Matrix4<f32> {
        let data = self.data.borrow();

        cgmath::ortho(
            data.left,
            data.right,
            data.bottom,
            data.top,
            data.near,
            data.far,
        )
    }
}

impl Component for Camera {
    fn id(&self) -> Id {
        self.id.clone()
    }

    fn tid(&self) -> Id {
        self.tid.clone()
    }

    fn parent(&self) -> Parent {
        self.parent.clone()
    }
}
