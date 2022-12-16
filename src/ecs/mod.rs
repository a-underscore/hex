use std::mem;

pub mod as_any;
pub mod component;
pub mod manager;
pub mod system;
pub mod world;

pub use as_any::AsAny;
pub use component::Component;
pub use manager::Manager;
pub use system::System;
pub use world::World;

pub fn cast_ref<F, T>(f: &F) -> &T
where
    F: ?Sized,
{
    *unsafe { mem::transmute::<&&F, &&T>(&f) }
}

pub fn cast_mut<F, T>(mut f: &mut F) -> &mut T
where
    F: ?Sized,
{
    *unsafe { mem::transmute::<&mut &mut F, &mut &mut T>(&mut f) }
}
