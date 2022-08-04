use crate::{self as ecs, AsAny, Component};
use glium::glutin::event::Event;
use std::{any::Any, cell::RefCell, collections::HashMap, rc::Rc, time::Duration};

thread_local! {
    pub static ENTITY_ID: Rc<String> = ecs::id("entity");
}

pub struct EntityData {
    components: HashMap<(Rc<String>, Rc<String>), Rc<dyn Component>>,
}

impl EntityData {
    fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            components: HashMap::new(),
        }))
    }

    pub fn components<'a>(&'a self) -> &'a HashMap<(Rc<String>, Rc<String>), Rc<dyn Component>> {
        &self.components
    }
}

#[derive(hecs_derive::Component)]
pub struct Entity {
    id: Rc<String>,
    tid: Rc<String>,
    parent: Rc<RefCell<Option<Rc<Entity>>>>,
    pub data: Rc<RefCell<EntityData>>,
}

impl Entity {
    pub fn new(id: Rc<String>) -> Rc<Self> {
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

    pub fn get<C>(&self, id: &Rc<String>, tid: &Rc<String>) -> Option<Rc<C>>
    where
        C: Component,
    {
        self.data
            .borrow()
            .components
            .get(&(id.clone(), tid.clone()))
            .and_then(|c| c.clone().as_any().downcast::<C>().ok())
    }

    pub fn get_all<C>(&self, tid: &Rc<String>) -> Vec<Rc<C>>
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

    pub fn get_first<C>(&self, tid: &Rc<String>) -> Option<Rc<C>>
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

    pub fn remove(&self, id: &Rc<String>, tid: &Rc<String>) {
        self.data
            .borrow_mut()
            .components
            .remove(&(id.clone(), tid.clone()))
            .and_then(|c| {
                c.set_parent(None);

                Some(())
            });
    }

    pub fn remove_all(&self, tid: &Rc<String>) {
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

    pub fn remove_first(&self, tid: &Rc<String>) {
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

    fn init(self: Rc<Self>, _parent: Option<Rc<Self>>) {
        for component in self.data.borrow().components.values().cloned() {
            component.init(Some(self.clone()));
        }
    }

    fn update(self: Rc<Self>, _parent: Option<Rc<Self>>, event: &Event<()>, delta: Duration) {
        for component in self.data.borrow().components.values().cloned() {
            component.update(Some(self.clone()), event, delta);
        }
    }
}
