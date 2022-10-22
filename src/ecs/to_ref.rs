use super::Component;

pub trait ToRef {
    fn to_ref<C>(&self) -> Option<&C>
    where
        C: Component + 'static;
}
