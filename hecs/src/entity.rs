use crate::{self as ecs, AsAny, Component};
use std::{any::Any, cell::RefCell, rc::Rc};

pub const ENTITY_ID: &str = "entity";

pub struct EntityData {
    pub id: Rc<String>,
    pub tid: Rc<String>,
    components: Vec<Rc<dyn Component>>,
}

impl EntityData {
    fn new(id: Rc<String>, tid: Rc<String>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            id,
            tid,
            components: Vec::new(),
        }))
    }
}

#[derive(hecs_derive::Component)]
pub struct Entity {
    pub data: Rc<RefCell<EntityData>>,
}

impl Entity {
    pub fn new(id: Rc<String>) -> Rc<Self> {
        Rc::new(Self {
            data: EntityData::new(id, ecs::id(ENTITY_ID)),
        })
    }

    pub fn add<C>(&self, component: Rc<C>)
    where
        C: Component,
    {
        self.data.borrow_mut().components.push(component.clone());
    }

    pub fn get<C>(&self, id: Rc<String>, tid: Rc<String>) -> Option<Rc<C>>
    where
        C: Component,
    {
        self.data
            .borrow()
            .components
            .iter()
            .filter_map(|c| {
                if *c.id() == *id && c.tid() == tid {
                    Some(c)
                } else {
                    None
                }
            })
            .next()
            .and_then(|c| c.clone().as_any().downcast::<C>().ok())
    }

    pub fn get_first<C>(&self, tid: Rc<String>) -> Option<Rc<C>>
    where
        C: Component,
    {
        self.data
            .borrow()
            .components
            .iter()
            .find_map(|c| {
                if *c.tid() == *tid {
                    c.clone().as_any().downcast::<C>().ok()
                } else {
                    None
                }
            })
            .and_then(|c| Some(c.clone()))
    }

    pub fn get_all<C>(&self, tid: Rc<String>) -> Vec<Rc<C>>
    where
        C: Component,
    {
        self.data
            .borrow()
            .components
            .iter()
            .filter_map(|c| {
                if *c.tid() == *tid {
                    c.clone().as_any().downcast::<C>().ok()
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn remove<C>(&self, component: Rc<C>)
    where
        C: Component + ?Sized,
    {
        let mut data = self.data.borrow_mut();

        data.components = data
            .components
            .iter()
            .filter_map(|c| {
                if *c.id() == *component.id() && *c.tid() == *component.tid() {
                    c.on_remove(Some(self));

                    None
                } else {
                    Some(c.clone())
                }
            })
            .collect();
    }
}

impl Component for Entity {
    fn id(&self) -> Rc<String> {
        self.data.borrow().id.clone()
    }

    fn tid(&self) -> Rc<String> {
        self.data.borrow().tid.clone()
    }

    fn on_init(&self, _owner: Option<&Self>) {
        let components = { self.data.borrow().components.clone() };

        for component in components {
            component.on_init(Some(self));
        }
    }

    fn on_update(&self, _owner: Option<&Self>) {
        let components = { self.data.borrow().components.clone() };

        for component in components {
            component.on_update(Some(self));
        }
    }

    fn on_remove(&self, _owner: Option<&Self>) {
        let components = { self.data.borrow().components.clone() };

        for component in components {
            self.clone().remove(component);
        }
    }
}
