use crate::{self as ecs, AsAny, Component};
use std::{any::Any, cell::RefCell, rc::Rc, time::Duration};

thread_local! {
    pub static ENTITY_ID: Rc<String> = ecs::id("entity");
}

#[derive(hecs_derive::Component)]
pub struct Entity {
    id: Rc<String>,
    tid: Rc<String>,
    parent: Rc<RefCell<Option<Rc<Entity>>>>,
    components: Rc<RefCell<Vec<Rc<dyn Component>>>>,
}

impl Entity {
    pub fn new(id: Rc<String>) -> Rc<Self> {
        Rc::new(Self {
            id,
            tid: ecs::tid(&ENTITY_ID),
            parent: Rc::new(RefCell::new(None)),
            components: Rc::new(RefCell::new(Vec::new())),
        })
    }

    pub fn add<C>(self: Rc<Self>, component: Rc<C>)
    where
        C: Component,
    {
        component.set_parent(Some(self.clone()));

        self.components.borrow_mut().push(component.clone());
    }

    pub fn get<C>(&self, id: Rc<String>, tid: Rc<String>) -> Option<Rc<C>>
    where
        C: Component,
    {
        self.components
            .borrow()
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
        self.components
            .borrow()
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
        self.components
            .borrow()
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
        let mut components = self.components.borrow_mut();

        *components = components
            .iter()
            .filter_map(|c| {
                if *c.id() == *component.id() && *c.tid() == *component.tid() {
                    c.set_parent(None);

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
        self.id.clone()
    }

    fn tid(&self) -> Rc<String> {
        self.tid.clone()
    }

    fn init(self: Rc<Self>, _parent: Option<Rc<Self>>) {
        for component in self.components.borrow().iter().cloned() {
            component.init(Some(self.clone()));
        }
    }

    fn update(self: Rc<Self>, _parent: Option<Rc<Self>>, delta: Duration) {
        for component in self.components.borrow().iter().cloned() {
            component.update(Some(self.clone()), delta);
        }
    }

    fn parent(&self) -> Option<Rc<Entity>> {
        self.parent.borrow().clone()
    }

    fn set_parent(&self, parent: Option<Rc<Entity>>) {
        *self.parent.borrow_mut() = parent;
    }
}
