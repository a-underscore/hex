use crate::{self as ecs, as_any::AsAny, Component};
use std::{
    any::Any,
    sync::{Arc, RwLock},
};

pub const ENTITY_ID: &str = "entity";

pub struct EntityData {
    components: Vec<Arc<dyn Component>>,
}

impl EntityData {
    fn new() -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(Self {
            components: Vec::new(),
        }))
    }
}

pub struct Entity {
    pub id: Arc<String>,
    pub tid: Arc<String>,
    pub data: Arc<RwLock<EntityData>>,
}

impl Entity {
    pub fn new(id: Arc<String>) -> Arc<Self> {
        Arc::new(Self {
            id,
            tid: ecs::id(ENTITY_ID),
            data: EntityData::new(),
        })
    }

    pub fn add<C>(self: Arc<Self>, component: Arc<C>)
    where
        C: Component,
    {
        self.data
            .write()
            .unwrap()
            .components
            .push(component.clone());
    }

    pub fn add_all<C>(self: Arc<Self>, components: &[Arc<C>])
    where
        C: Component,
    {
        components.into_iter().for_each(|c| {
            self.clone().add(c.clone());
        });
    }

    pub fn get<C>(self: Arc<Self>, id: Arc<String>, tid: Arc<String>) -> Option<Arc<C>>
    where
        C: Component + AsAny,
    {
        match self
            .data
            .read()
            .unwrap()
            .components
            .iter()
            .filter_map(|c| {
                if *c.clone().id() == *id && c.clone().tid() == tid {
                    Some(c)
                } else {
                    None
                }
            })
            .next()
        {
            Some(component) => component.clone().as_any().downcast::<C>().ok(),
            None => None,
        }
    }

    pub fn get_first<C>(self: Arc<Self>, tid: Arc<String>) -> Option<Arc<C>>
    where
        C: Component + AsAny,
    {
        match self.get_type::<C>(tid).first() {
            Some(component) => Some(component.clone()),

            None => None,
        }
    }

    pub fn get_type<C>(self: Arc<Self>, tid: Arc<String>) -> Vec<Arc<C>>
    where
        C: Component + AsAny,
    {
        self.data
            .read()
            .unwrap()
            .components
            .iter()
            .filter_map(|c| {
                if *c.clone().tid() == *tid {
                    c.clone().as_any().downcast::<C>().ok()
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn remove<C>(self: Arc<Self>, component: Arc<C>)
    where
        C: Component + ?Sized,
    {
        self.data.write().unwrap().components.retain(|c| {
            if *c.clone().id() == *component.clone().id()
                && *c.clone().tid() == *component.clone().tid()
            {
                c.clone().on_remove(Some(self.clone()));

                false
            } else {
                true
            }
        });
    }

    pub fn remove_all<C>(self: Arc<Self>, components: Vec<Arc<C>>)
    where
        C: Component,
    {
        components.into_iter().for_each(|c| {
            self.clone().remove(c.clone());
        });
    }
}

impl Component for Entity {
    fn id(self: Arc<Self>) -> Arc<String> {
        self.id.clone()
    }

    fn tid(self: Arc<Self>) -> Arc<String> {
        self.tid.clone()
    }

    fn on_init(self: Arc<Self>, _owner: Option<Arc<Self>>) {
        for component in &self.data.read().unwrap().components {
            component.clone().on_init(Some(self.clone()));
        }
    }

    fn on_update(self: Arc<Self>, _owner: Option<Arc<Self>>) {
        for component in &self.data.read().unwrap().components {
            component.clone().on_update(Some(self.clone()));
        }
    }

    fn on_remove(self: Arc<Self>, _owner: Option<Arc<Self>>) {
        let components = { self.data.read().unwrap().components.clone() };

        for component in components {
            self.clone().remove(component);
        }
    }
}

impl AsAny for Entity {
    fn as_any(self: Arc<Self>) -> Arc<dyn Any + Send + Sync + 'static> {
        self.clone() as Arc<dyn Any + Send + Sync + 'static>
    }
}
