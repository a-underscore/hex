use std::{rc::Rc, thread::LocalKey};

pub type Id = Rc<String>;

pub fn id(id: &str) -> Id {
    Rc::new(id.to_string())
}

pub fn tid(id: &'static LocalKey<Id>) -> Id {
    id.with(|c| c.clone())
}
