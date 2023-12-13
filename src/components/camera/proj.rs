use cgmath::{Matrix4, Rad, Vector2, Vector3};

#[derive(Clone)]
pub enum Proj {
    Perspective((Rad<f32>, f32, Vector2<f32>)),
    Ortho(Vector3<f32>),
}

impl Proj {
    pub fn matrix(&self) -> Matrix4<f32> {
        match self {
            Self::Perspective((fov, aspect, clip)) => {
                cgmath::perspective(*fov, *aspect, clip.x, clip.y)
            }
            Self::Ortho(dims) => {
                let dims = *dims / 2.0;

                cgmath::ortho(-dims.x, dims.x, -dims.y, dims.y, -dims.z, dims.z)
            }
        }
    }
}
