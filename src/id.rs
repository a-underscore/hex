use std::sync::atomic::{AtomicUsize, Ordering};

fn id(count: &AtomicUsize) -> usize {
    let id = count.load(Ordering::Acquire);

    count.store(id + 1, Ordering::Release);

    id
}

pub fn eid() -> usize {
    static COUNT: AtomicUsize = AtomicUsize::new(0);

    id(&COUNT)
}

pub fn cid() -> usize {
    static COUNT: AtomicUsize = AtomicUsize::new(0);

    id(&COUNT)
}
