use super::{AsAny, Component, Entity, Id, System};
use glium::glutin::event::Event;
use std::{
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
    rc::Rc,
    time::Duration,
};

#[derive(Clone)]
pub struct World {
    entities: HashMap<Id, (Id, Rc<RefCell<Entity>>)>,
    systems: HashMap<Id, (Id, Rc<RefCell<dyn System>>)>,
}

impl World {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            entities: HashMap::new(),
            systems: HashMap::new(),
        }))
    }

    pub fn get_entities(&self) -> &HashMap<Id, (Id, Rc<RefCell<Entity>>)> {
        &self.entities
    }

    pub fn get_systems(&self) -> &HashMap<Id, (Id, Rc<RefCell<dyn System>>)> {
        &self.systems
    }

    pub fn change_id(&mut self, old: &Id, new: &Id) {
        if let Some((_, e)) = self.remove(old) {
            self.add(&(new.clone(), e));
        };
    }

    pub fn add(&mut self, e @ (id, _): &(Id, Rc<RefCell<Entity>>)) {
        self.entities.insert(id.clone(), e.clone());
    }

    pub fn get(&self, id: &Id) -> Option<(Id, Rc<RefCell<Entity>>)> {
        Some(self.entities.get(id.as_ref())?.clone())
    }

    pub fn get_all(
        &self,
        id: &Id,
    ) -> Vec<((Id, Rc<RefCell<Entity>>), (Id, Rc<RefCell<dyn AsAny>>))> {
        self.entities
            .values()
            .filter_map(|p @ (_, e)| Some((p.clone(), e.try_borrow().ok()?.get(id)?.clone())))
            .collect()
    }

    pub fn get_all_ref<C>(&self) -> Vec<((Id, Rc<RefCell<Entity>>), Ref<C>)>
    where
        C: Component + 'static,
    {
        self.entities
            .values()
            .filter_map(|p @ (_, e)| {
                Some((
                    p.clone(),
                    unsafe { e.try_borrow_unguarded() }.ok()?.get_ref()?,
                ))
            })
            .collect()
    }

    pub fn get_all_mut<C>(&self) -> Vec<((Id, Rc<RefCell<Entity>>), RefMut<C>)>
    where
        C: Component + 'static,
    {
        self.entities
            .values()
            .filter_map(|p @ (_, e)| {
                Some((
                    p.clone(),
                    unsafe { e.try_borrow_unguarded() }.ok()?.get_ref_mut()?,
                ))
            })
            .collect()
    }

    pub fn get_all_with(
        &self,
        ids: &[&Id],
    ) -> Vec<((Id, Rc<RefCell<Entity>>), Vec<(Id, Rc<RefCell<dyn AsAny>>)>)> {
        self.entities
            .values()
            .filter_map(|p @ (_, e)| {
                Some((
                    p.clone(),
                    ids.iter()
                        .map(|id| Some(e.try_borrow().ok()?.get(id)?.clone()))
                        .collect::<Option<Vec<_>>>()?,
                ))
            })
            .collect()
    }

    pub fn remove(&mut self, id: &Id) -> Option<(Id, Rc<RefCell<Entity>>)> {
        self.entities.remove(id.as_ref())
    }

    pub fn add_generic_system(&mut self, s @ (id, _): &(Id, Rc<RefCell<dyn System>>)) {
        self.systems.insert(id.clone(), s.clone());
    }

    pub fn add_system<S>(&mut self, system: &Rc<RefCell<S>>)
    where
        S: System + Component + 'static,
    {
        self.add_generic_system(&(S::get_id(), system.clone()))
    }

    pub fn get_system(&self, id: &Id) -> Option<&(Id, Rc<RefCell<dyn System>>)> {
        self.systems.get(id)
    }

    pub fn get_system_ref<S>(&self) -> Option<Ref<S>>
    where
        S: Component + System + 'static,
    {
        self.get_system(&S::get_id()).and_then(|(_, s)| {
            Ref::filter_map(s.try_borrow().ok()?, |s| s.as_any_ref().downcast_ref()).ok()
        })
    }

    pub fn get_system_ref_mut<S>(&self) -> Option<RefMut<S>>
    where
        S: Component + System + 'static,
    {
        self.get_system(&S::get_id()).and_then(|(_, s)| {
            RefMut::filter_map(s.try_borrow_mut().ok()?, |s| s.as_any_mut().downcast_mut()).ok()
        })
    }

    pub fn remove_system_generic(&mut self, id: &Id) -> Option<(Id, Rc<RefCell<dyn System>>)> {
        self.systems.remove(id.as_ref())
    }

    pub fn remove_system<S>(&mut self) -> Option<(Id, Rc<RefCell<dyn System>>)>
    where
        S: Component + 'static,
    {
        self.remove_system_generic(&S::get_id())
    }

    pub fn update(
        world: &Rc<RefCell<World>>,
        event: &Event<()>,
        delta: Duration,
    ) -> anyhow::Result<()> {
        for (_, s) in unsafe { world.try_borrow_unguarded() }?.systems.values() {
            s.try_borrow_mut()?.update(world, event, delta)?;
        }

        Ok(())
    }
}
