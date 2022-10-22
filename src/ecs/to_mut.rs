use super::Component;

pub trait ToMut {
    fn to_mut<C>(&mut self) -> Option<&mut C>
    where
        C: Component + 'static;
}
