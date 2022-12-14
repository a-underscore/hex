pub mod assets;
pub mod components;
pub mod ecs;
pub mod engine;
pub mod systems;

pub use anyhow;
pub use cgmath;
pub use glium;
pub use once_cell;

use std::sync::atomic::{Ordering, AtomicUsize};

pub fn nid() -> usize {
    static COUNT: AtomicUsize = AtomicUsize::new(0);

    let id = COUNT.load(Ordering::Acquire);

    COUNT.store(id + 1, Ordering::Release);

    id
}

#[macro_export]
macro_rules! id {
    () => {{
        use $crate::nid;
        use $crate::once_cell::sync::Lazy;

        static ID: Lazy<usize> = Lazy::new(|| nid());

        *ID
    }};
}
