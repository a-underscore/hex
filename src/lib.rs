pub mod assets;
pub mod components;
pub mod ecs;
pub mod math;
pub mod renderer2d;

pub use anyhow;
pub use glium;
pub use once_cell;
pub use renderer2d::Renderer2d;

#[macro_export]
macro_rules! id {
    () => {{
        use $crate::{
            ecs::{id, Id},
            once_cell::sync::Lazy,
        };

        static ID: Lazy<Id> = Lazy::new(|| id());

        *ID
    }};
}
