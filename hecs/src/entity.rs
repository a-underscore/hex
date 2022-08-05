use crate::{self as ecs, derive::AsAny, AsAny, Component, Id, Parent};
use glium::glutin::event::Event;
use std::{any::Any, cell::RefCell, collections::HashMap, rc::Rc, time::Duration};

thread_local! {
    pub static ENTITY_ID: Id = ecs::id("entity");
}

pub struct EntityData {
    components: HashMap<(Id, Id), Rc<dyn Component>>,
}

impl EntityData {
    fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            components: HashMap::new(),
        }))
    }

    pub fn components<'a>(&'a self) -> &'a HashMap<(Id, Id), Rc<dyn Component>> {
        &self.components
    }
}

#[derive(AsAny)]
pub struct Entity {
    id: Id,
    tid: Id,
    parent: Rc<RefCell<Parent>>,
    pub data: Rc<RefCell<EntityData>>,
}

impl Entity {
    pub fn new(id: Id) -> Rc<Self> {
        Rc::new(Self {
            id,
            tid: ecs::tid(&ENTITY_ID),
            parent: Rc::new(RefCell::new(None)),
            data: EntityData::new(),
        })
    }

    pub fn add<C>(self: &Rc<Self>, component: &Rc<C>)
    where
        C: Component,
    {
        self.data
            .borrow_mut()
            .components
            .insert(
                (component.id(), component.tid()),
                component.clone() as Rc<dyn Component>,
            )
            .and_then(|c| {
                c.set_parent(None);

                Some(())
            });

        component.set_parent(Some(self.clone()));
    }

    pub fn get<C>(&self, id: &Id, tid: &Id) -> Option<Rc<C>>
    where
        C: Component,
    {
        self.data
            .borrow()
            .components
            .get(&(id.clone(), tid.clone()))
            .and_then(|c| c.clone().as_any().downcast::<C>().ok())
    }

    pub fn get_all<C>(&self, tid: &Id) -> Vec<Rc<C>>
    where
        C: Component,
    {
        self.data
            .borrow()
            .components
            .values()
            .filter_map(|c| {
                if *c.tid() == **tid {
                    c.clone().as_any().downcast::<C>().ok()
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn get_first<C>(&self, tid: &Id) -> Option<Rc<C>>
    where
        C: Component,
    {
        self.data
            .borrow()
            .components
            .values()
            .cloned()
            .find(|c| *c.tid() == **tid)
            .and_then(|c| c.clone().as_any().downcast::<C>().ok())
    }

    pub fn remove(&self, id: &Id, tid: &Id) {
        self.data
            .borrow_mut()
            .components
            .remove(&(id.clone(), tid.clone()))
            .and_then(|c| {
                c.set_parent(None);

                Some(())
            });
    }

    pub fn remove_all(&self, tid: &Id) {
        let mut data = self.data.borrow_mut();

        data.components = data
            .components
            .iter()
            .filter_map(|(k, c)| {
                if *c.tid() == **tid {
                    c.set_parent(None);

                    None
                } else {
                    Some((k.clone(), c.clone()))
                }
            })
            .collect();
    }

    pub fn remove_first(&self, tid: &Id) {
        let mut data = self.data.borrow_mut();

        data.components
            .iter()
            .find_map(|(k, c)| {
                if *c.tid() == **tid {
                    Some((k.clone(), c.clone()))
                } else {
                    None
                }
            })
            .and_then(|(k, c)| {
                c.set_parent(None);

                data.components.remove(&k);

                Some(())
            });
    }

    pub fn remove_struct<C>(&self, component: &Rc<C>)
    where
        C: Component,
    {
        self.remove(&component.id(), &component.tid());
    }
}

impl Component for Entity {
    fn id(&self) -> Id {
        self.id.clone()
    }

    fn tid(&self) -> Id {
        self.tid.clone()
    }

    fn get_parent(&self) -> Parent {
        self.parent.borrow().clone()
    }

    fn set_parent(&self, parent: Parent) {
        *self.parent.borrow_mut() = parent;
    }

    fn on_init(self: Rc<Self>, _parent: Parent) {
        for component in self.data.borrow().components.values().cloned() {
            component.on_init(Some(self.clone()));
        }
    }

    fn on_update(self: Rc<Self>, _parent: Parent, event: &Event<()>, delta: Duration) {
        for component in self.data.borrow().components.values().cloned() {
            component.on_update(Some(self.clone()), event, delta);
        }
    }
}
