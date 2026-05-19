use std::ops::{Deref, DerefMut};

use rsmlui_macros::sys_cast;
use rsmlui_sys::Rml_Matrix4f;

// `Rml_Matrix4f` is simply an array so `transparent` can't specify any fields
// that makes it a touch more unsafe but neither the sys nore mint types are
// likely to ever change
#[sys_cast(struct(from = Rml_Matrix4f, transparent), gen_ref)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Mat4(pub mint::ColumnMatrix4<f32>);

impl Deref for Mat4 {
    type Target = mint::ColumnMatrix4<f32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Mat4 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Default for Mat4 {
    fn default() -> Self {
        let zero = mint::Vector4 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        };

        Self(mint::ColumnMatrix4 {
            x: zero,
            y: zero,
            z: zero,
            w: zero,
        })
    }
}

impl From<mint::ColumnMatrix4<f32>> for Mat4 {
    fn from(m: mint::ColumnMatrix4<f32>) -> Self {
        Self(m)
    }
}
impl From<Mat4> for mint::ColumnMatrix4<f32> {
    fn from(m: Mat4) -> Self {
        m.0
    }
}

#[cfg(feature = "glam")]
mod glam_impls {
    use super::*;

    impl From<glam::Mat4> for Mat4 {
        fn from(m: glam::Mat4) -> Self {
            Self(mint::ColumnMatrix4 {
                x: mint::Vector4 {
                    x: m.x_axis.x,
                    y: m.x_axis.y,
                    z: m.x_axis.z,
                    w: m.x_axis.w,
                },
                y: mint::Vector4 {
                    x: m.y_axis.x,
                    y: m.y_axis.y,
                    z: m.y_axis.z,
                    w: m.y_axis.w,
                },
                z: mint::Vector4 {
                    x: m.z_axis.x,
                    y: m.z_axis.y,
                    z: m.z_axis.z,
                    w: m.z_axis.w,
                },
                w: mint::Vector4 {
                    x: m.w_axis.x,
                    y: m.w_axis.y,
                    z: m.w_axis.z,
                    w: m.w_axis.w,
                },
            })
        }
    }

    impl From<Mat4> for glam::Mat4 {
        fn from(m: Mat4) -> Self {
            let c = m.0;

            glam::Mat4::from_cols(
                glam::Vec4::new(c.x.x, c.x.y, c.x.z, c.x.w),
                glam::Vec4::new(c.y.x, c.y.y, c.y.z, c.y.w),
                glam::Vec4::new(c.z.x, c.z.y, c.z.z, c.z.w),
                glam::Vec4::new(c.w.x, c.w.y, c.w.z, c.w.w),
            )
        }
    }
}
