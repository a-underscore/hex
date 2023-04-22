use crate::math::{Mat4d, Vec2d, Vec3d};

#[derive(Clone)]
pub enum Projection {
    Perspective(f32, f32, Vec2d),
    Ortho(Vec3d),
}

impl Projection {
    pub fn view(&self) -> Mat4d {
        match self {
            Self::Perspective(fov, aspect, clip) => {
                Mat4d::perspective(*fov, *aspect, clip.x(), clip.y())
            }
            Self::Ortho(dims) => {
                let dims = *dims / 2.0;

                Mat4d::ortho(
                    -dims.x(),
                    dims.x(),
                    -dims.y(),
                    dims.y(),
                    -dims.z(),
                    dims.z(),
                )
            }
        }
    }
}
