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

pub fn cast<F, T>(f: Box<F>) -> T
where
    F: ?Sized,
{
    unsafe { mem::transmute_copy(&f) }
}

pub fn cast_ref<F, T>(f: &F) -> &T
where
    F: ?Sized,
{
    unsafe { mem::transmute(&f) }
}

pub fn cast_mut<F, T>(f: &mut Box<F>) -> &mut T
where
    F: ?Sized,
{
    unsafe { mem::transmute(f) }
}
