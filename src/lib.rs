pub mod assets;
pub mod components;
pub mod ecs;
pub mod id;
pub mod math;
pub mod renderer;

pub use anyhow;
pub use glium;
pub use id::id;
pub use once_cell;
pub use renderer::Renderer;

#[macro_export]
macro_rules! cid {
    () => {{
        use $crate::{id::cid, once_cell::sync::Lazy};

        static ID: Lazy<usize> = Lazy::new(|| cid());

        *ID
    }};
}
