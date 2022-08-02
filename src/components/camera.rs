use crate::ecs::{self, derive::Component, AsAny, Component, Entity};
use cgmath::Matrix4;
use std::{any::Any, cell::RefCell, rc::Rc};

thread_local! {
    pub static CAMERA_ID: Rc<String> = ecs::id("camera");
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

#[derive(Component)]
pub struct Camera {
    id: Rc<String>,
    tid: Rc<String>,
    parent: Rc<RefCell<Option<Rc<Entity>>>>,
    pub data: Rc<RefCell<CameraData>>,
}

impl Camera {
    pub fn new(
        id: Rc<String>,
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        near: f32,
        far: f32,
    ) -> Rc<Self> {
        Rc::new(Self {
            id,
            tid: ecs::tid(&CAMERA_ID),
            parent: Rc::new(RefCell::new(None)),
            data: CameraData::new(left, right, bottom, top, near, far),
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
