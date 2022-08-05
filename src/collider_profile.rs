use collider::{HbId, HbProfile};

#[derive(Copy, Clone, Debug)]
pub struct ColliderProfile {
    id: HbId,
}

impl HbProfile for ColliderProfile {
    fn id(&self) -> HbId {
        self.id
    }

    fn can_interact(&self, _other: &Self) -> bool {
        true
    }

    fn cell_width() -> f64 {
        4.0
    }

    fn padding() -> f64 {
        0.01
    }
}
