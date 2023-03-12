pub mod assets;
pub mod components;
pub mod engine;
pub mod id;
pub mod renderer;

pub use anyhow;
pub use glium;
pub use hecs;
pub use id::id;
pub use once_cell;

#[macro_export]
macro_rules! cid {
    () => {{
        use $crate::{id::cid, once_cell::sync::Lazy};

        static ID: Lazy<usize> = Lazy::new(|| cid());

        *ID
    }};
}
