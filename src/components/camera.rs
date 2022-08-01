use crate::{
    assets::Transform,
    ecs::{self, AsAny, Component},
};
use std::{any::Any, cell::RefCell, rc::Rc};

thread_local! {
    pub static CAMERA_ID: Rc<String> = ecs::id("camera");
}

pub struct CameraData {
    pub id: Rc<String>,
    pub transform: Rc<RefCell<Transform>>,
}

impl CameraData {
    fn new(id: Rc<String>, transform: Rc<RefCell<Transform>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self { id, transform }))
    }
}

#[derive(ecs::derive::Component)]
pub struct Camera {
    pub tid: Rc<String>,
    pub data: Rc<RefCell<CameraData>>,
}

impl Camera {
    pub fn new(id: Rc<String>, transform: Rc<RefCell<Transform>>) -> Rc<Self> {
        Rc::new(Self {
            tid: CAMERA_ID.with(|id| id.clone()),
            data: CameraData::new(id, transform),
        })
    }
}

impl Component for Camera {
    fn id(&self) -> Rc<String> {
        self.data.borrow().id.clone()
    }

    fn tid(&self) -> Rc<String> {
        self.tid.clone()
    }
}
