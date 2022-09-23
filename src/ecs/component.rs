use super::Id;

pub trait Component {
    fn get_id() -> Id;
}
