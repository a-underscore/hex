pub mod assets;
pub mod components;
pub mod ecs;
pub mod engine;
pub mod id;
pub mod systems;

pub use anyhow;
pub use cgmath;
pub use glium;
pub use once_cell;
pub use rand;

#[macro_export]
macro_rules! cid {
    () => {{
        use $crate::id::cid;
        use $crate::once_cell::sync::Lazy;

        static ID: Lazy<usize> = Lazy::new(|| cid());

        *ID
    }};
}
