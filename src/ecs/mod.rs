pub mod component_manager;
pub mod entity_manager;
pub mod system_manager;
pub mod world;

use std::mem;

pub fn cast<F, T>(f: &F) -> &T
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
