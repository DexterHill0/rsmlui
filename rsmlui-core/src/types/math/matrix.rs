use std::ops::{Deref, DerefMut};

use rsmlui_macros::sys_cast;
use rsmlui_sys::Rml_Matrix4f;

/// A column-major 4x4 matrix, stored as four columns of four components.
///
/// With the bindgen bug specified in `rsmlui-sys`'s build.rs, the `Rml_Matrix4f`
/// is manually defined in the bindings rather than being auto-generated, so
/// really the assertions here are only proving that those two types match, but
/// don't prove that `Rml_Matrix4f` actually matches what's defined in C++.
/// I hightly doubt the shape in C++ will change, or at least not before bindgen
/// is fixed (I hope...).
#[sys_cast(struct(from = Rml_Matrix4f, transparent), gen_ref)]
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
pub struct Mat4(pub [[f32; 4]; 4]);

impl Mat4 {
    pub const IDENTITY: Self = Self([
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]);
    pub const ZERO: Self = Self([[0.0; 4]; 4]);

    /// Builds a matrix from four columns.
    pub const fn from_cols_array_2d(cols: [[f32; 4]; 4]) -> Self {
        Self(cols)
    }

    /// Returns the matrix as four columns.
    pub const fn to_cols_array_2d(self) -> [[f32; 4]; 4] {
        self.0
    }
}

impl Deref for Mat4 {
    type Target = [[f32; 4]; 4];

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
        Self::ZERO
    }
}

impl From<[[f32; 4]; 4]> for Mat4 {
    fn from(cols: [[f32; 4]; 4]) -> Self {
        Self(cols)
    }
}

impl From<Mat4> for [[f32; 4]; 4] {
    fn from(m: Mat4) -> Self {
        m.0
    }
}

#[cfg(feature = "mint")]
mod mint_impls {
    use super::*;

    impl From<mint::ColumnMatrix4<f32>> for Mat4 {
        fn from(m: mint::ColumnMatrix4<f32>) -> Self {
            Self([
                [m.x.x, m.x.y, m.x.z, m.x.w],
                [m.y.x, m.y.y, m.y.z, m.y.w],
                [m.z.x, m.z.y, m.z.z, m.z.w],
                [m.w.x, m.w.y, m.w.z, m.w.w],
            ])
        }
    }

    impl From<Mat4> for mint::ColumnMatrix4<f32> {
        fn from(m: Mat4) -> Self {
            let column = |c: [f32; 4]| mint::Vector4 {
                x: c[0],
                y: c[1],
                z: c[2],
                w: c[3],
            };

            mint::ColumnMatrix4 {
                x: column(m.0[0]),
                y: column(m.0[1]),
                z: column(m.0[2]),
                w: column(m.0[3]),
            }
        }
    }
}

#[cfg(feature = "glam")]
mod glam_impls {
    use super::*;

    impl From<glam::Mat4> for Mat4 {
        fn from(m: glam::Mat4) -> Self {
            Self(m.to_cols_array_2d())
        }
    }

    impl From<Mat4> for glam::Mat4 {
        fn from(m: Mat4) -> Self {
            glam::Mat4::from_cols_array_2d(&m.0)
        }
    }
}
