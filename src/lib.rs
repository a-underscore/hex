pub mod assets;
pub mod components;
pub mod id;
pub mod renderer;

pub use anyhow;
pub use glium;
pub use hecs as ecs;
pub use hex_math as math;
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
