    use std::sync::atomic::{AtomicUsize, Ordering};

    pub fn eid() -> usize {
        static COUNT: AtomicUsize = AtomicUsize::new(0);

        let id = COUNT.load(Ordering::Acquire);

        COUNT.store(id + 1, Ordering::Release);

        id
    }

    pub fn cid() -> usize {
        static COUNT: AtomicUsize = AtomicUsize::new(0);

        let id = COUNT.load(Ordering::Acquire);

        COUNT.store(id + 1, Ordering::Release);

        id
    }
