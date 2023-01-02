pub mod assets;
pub mod components;
pub mod ecs;
pub mod engine;
pub mod renderer;

pub use anyhow;
pub use cgmath;
pub use glium;
pub use once_cell;

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

#[macro_export]
macro_rules! cid {
    () => {{
        use $crate::once_cell::sync::Lazy;

        static ID: Lazy<usize> = Lazy::new(|| cid());

        *ID
    }};
}
