use std::collections::HashMap;

pub type Id = u32;

pub fn next<K, V>(free: &mut Vec<Id>, ids: &HashMap<K, V>) -> Id {
    free.pop().unwrap_or(ids.len() as Id)
}
