pub mod component_manager;
pub mod entity_manager;
pub mod system_manager;
pub mod world;

use std::{
    mem,
    sync::atomic::{AtomicUsize, Ordering},
};

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

pub fn id(count: &AtomicUsize) -> usize {
    count.fetch_add(1, Ordering::SeqCst)
}

pub fn eid() -> usize {
    static COUNT: AtomicUsize = AtomicUsize::new(0);

    id(&COUNT)
}

pub fn cid() -> usize {
    static COUNT: AtomicUsize = AtomicUsize::new(0);

    id(&COUNT)
}
