use std::sync::atomic::{AtomicUsize, Ordering};

pub fn id(count: &AtomicUsize) -> usize {
    count.fetch_add(1, Ordering::SeqCst)
}

pub fn eid() -> usize {
    static COUNT: AtomicUsize = AtomicUsize::new(0);

    id(&COUNT)
}

pub fn cid() -> usize {
    static COUNT: AtomicUsize = AtomicUsize::new(0);

    id(&COUNT)
}
