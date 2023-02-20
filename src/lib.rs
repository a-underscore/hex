pub mod assets;
pub mod components;
pub mod ecs;
pub mod engine;

pub use anyhow;
pub use cgmath;
pub use glium;
pub use once_cell;

#[macro_export]
macro_rules! cid {
    () => {{
        use $crate::{ecs::cid, once_cell::sync::Lazy};

        static ID: Lazy<usize> = Lazy::new(|| cid());

        *ID
    }};
}
