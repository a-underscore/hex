use std::{cell::Cell, collections::BTreeMap};

pub type Id = u32;

pub fn id() -> Id {
    thread_local!(static COUNT: Cell<Id> = Default::default());

    COUNT.with(|c| {
        let count = c.get();

        c.replace(count + 1);

        count
    })
}

pub fn next<T>(ids: &BTreeMap<Id, T>) -> Id {
    ids.keys()
        .cloned()
        .enumerate()
        .find(|(i, id)| *i as Id != *id)
        .map(|(_, id)| id - 1)
        .unwrap_or(ids.len() as Id)
}
