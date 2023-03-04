use super::AsAny;

#[macro_export]
macro_rules! cid {
    () => {{
        use $crate::{ecs::cid, once_cell::sync::Lazy};

        static ID: Lazy<usize> = Lazy::new(|| cid());

        *ID
    }};
}

pub trait Component {
    fn id() -> usize {
        cid!()
    }
}

impl<'a, C> AsAny<'a> for C where C: Component + 'a {}
