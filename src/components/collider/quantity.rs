use crate::components::Transform;
use cgmath::{InnerSpace, Vector2, Zero};

pub enum Quantity {
    Dims(Vector2<f32>),
    Radius(f32),
}

impl Quantity {
    pub fn to_global(&self, transform: &Transform) -> (Vector2<f32>, Self) {
        match self {
            Self::Dims(dims) => {
                let (p1, p2) = {
                    let transform = transform.get_transform();
                    (
                        (transform * Vector2::zero().extend(1.0)).xy(),
                        (transform * dims.extend(1.0)).xy(),
                    )
                };
                let (min_x, max_x) = if p1.x < p2.x {
                    (p1.x, p2.x)
                } else {
                    (p2.x, p1.x)
                };
                let (min_y, max_y) = if p1.y < p2.y {
                    (p1.y, p2.y)
                } else {
                    (p2.y, p1.y)
                };

                (
                    Vector2::new(min_x, min_y),
                    Self::Dims(Vector2::new(max_x, max_y)),
                )
            }
            Self::Radius(radius) => (
                transform.get_position(),
                Self::Radius(transform.get_scale().magnitude() * radius),
            ),
        }
    }
}
