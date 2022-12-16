pub mod assets;
pub mod components;
pub mod ecs;
pub mod engine;
pub mod systems;

pub use anyhow;
pub use cgmath;
pub use glium;
pub use once_cell;
pub use rand;

pub mod id {
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
}

#[macro_export]
macro_rules! cid {
    () => {{
        use $crate::id::cid;
        use $crate::once_cell::sync::Lazy;

        static ID: Lazy<usize> = Lazy::new(|| cid());

        *ID
    }};
}
