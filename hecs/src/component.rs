use crate::Id;

pub trait Component: 'static {
    fn id(&self) -> Id;
}
