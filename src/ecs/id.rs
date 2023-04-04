use std::cell::Cell;

pub type Id = u64;

pub fn id() -> Id {
    thread_local!(static COUNT: Cell<Id> = Default::default());

    COUNT.with(|c| {
        let count = c.get();

        c.replace(count + 1);

        count
    })
}
