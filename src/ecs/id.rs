use std::{cell::Cell, collections::HashMap};

pub type Id = u32;

pub fn id() -> Id {
    thread_local!(static COUNT: Cell<Id> = Default::default());

    COUNT.with(|c| {
        let count = c.get();

        c.replace(count + 1);

        count
    })
}

pub fn next<K, V>(free: &mut Vec<Id>, ids: &HashMap<K, V>) -> Id {
    free.pop().unwrap_or(ids.len() as Id)
}
