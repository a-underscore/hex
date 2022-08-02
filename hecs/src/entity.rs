use crate::{self as ecs, AsAny, Component};
use std::{any::Any, cell::RefCell, collections::HashMap, rc::Rc, time::Duration};

thread_local! {
    pub static ENTITY_ID: Rc<String> = ecs::id("entity");
}

#[derive(hecs_derive::Component)]
pub struct Entity {
    id: Rc<String>,
    tid: Rc<String>,
    parent: Rc<RefCell<Option<Rc<Entity>>>>,
    components: Rc<RefCell<HashMap<(Rc<String>, Rc<String>), Rc<dyn Component>>>>,
}

impl Entity {
    pub fn new(id: Rc<String>) -> Rc<Self> {
        Rc::new(Self {
            id,
            tid: ecs::tid(&ENTITY_ID),
            parent: Rc::new(RefCell::new(None)),
            components: Rc::new(RefCell::new(HashMap::new())),
        })
    }

    pub fn add<C>(self: Rc<Self>, component: Rc<C>)
    where
        C: Component,
    {
        self.components
            .borrow_mut()
            .insert(
                (component.id(), component.tid()),
                component.clone() as Rc<dyn Component>,
            )
            .and_then(|c| Some(c.set_parent(None)));

        component.set_parent(Some(self.clone()));
    }

    pub fn get<C>(&self, id: Rc<String>, tid: Rc<String>) -> Option<Rc<C>>
    where
        C: Component,
    {
        self.components
            .borrow()
            .get(&(id, tid))
            .and_then(|c| c.clone().as_any().downcast::<C>().ok())
    }

    pub fn get_first<C>(&self, tid: Rc<String>) -> Option<Rc<C>>
    where
        C: Component,
    {
        self.get_all::<C>(tid).first().and_then(|c| Some(c.clone()))
    }

    pub fn get_all<C>(&self, tid: Rc<String>) -> Vec<Rc<C>>
    where
        C: Component,
    {
        self.components
            .borrow()
            .iter()
            .filter_map(|(_, c)| {
                if *c.tid() == *tid {
                    c.clone().as_any().downcast::<C>().ok()
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn remove<C>(&self, id: Rc<String>, tid: Rc<String>)
    where
        C: Component + ?Sized,
    {
        self.components
            .borrow_mut()
            .remove(&(id, tid))
            .and_then(|c| Some(c.set_parent(None)));
    }

    pub fn remove_struct<C>(&self, component: Rc<C>)
    where
        C: Component + ?Sized,
    {
        self.remove::<C>(component.id(), component.tid());
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
        for component in self.components.borrow().values().cloned() {
            component.init(Some(self.clone()));
        }
    }

    fn update(self: Rc<Self>, _parent: Option<Rc<Self>>, delta: Duration) {
        for component in self.components.borrow().values().cloned() {
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
