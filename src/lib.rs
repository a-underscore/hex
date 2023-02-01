pub mod assets;
pub mod components;
pub mod ecs;
pub mod engine;
pub mod id;
pub mod renderer;

pub use anyhow;
pub use cgmath;
pub use glium;
pub use once_cell;

pub use id::{cid, eid};

#[macro_export]
macro_rules! cid {
    () => {{
        use $crate::once_cell::sync::Lazy;

        static ID: Lazy<usize> = Lazy::new(|| cid());

        *ID
    }};
}
