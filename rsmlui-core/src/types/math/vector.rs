use std::ops::{Deref, DerefMut};

use rsmlui_macros::sys_cast;
use rsmlui_sys::{Rml_Vector2f, Rml_Vector2i, Rml_Vector3f, Rml_Vector4f, Rml_Vector4i};

#[sys_cast(struct(from = Rml_Vector2f, transparent(fields(x, y))), gen_ref)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec2(pub mint::Vector2<f32>);

#[sys_cast(struct(from = Rml_Vector2i, transparent(fields(x, y))))]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct IVec2(pub mint::Vector2<i32>);

#[sys_cast(struct(from = Rml_Vector3f, transparent(fields(x, y, z))), gen_ref)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec3(pub mint::Vector3<f32>);

#[sys_cast(struct(from = Rml_Vector4f, transparent(fields(x, y, z, w))), gen_ref)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec4(pub mint::Vector4<f32>);

#[sys_cast(struct(from = Rml_Vector4i, transparent(fields(x, y, z, w))))]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct IVec4(pub mint::Vector4<i32>);

impl Vec2 {
    pub const ZERO: Self = Self(mint::Vector2 { x: 0.0, y: 0.0 });

    pub const fn new(x: f32, y: f32) -> Self {
        Self(mint::Vector2 { x, y })
    }

    pub const fn splat(v: f32) -> Self {
        Self(mint::Vector2 { x: v, y: v })
    }
}

impl IVec2 {
    pub const ZERO: Self = Self(mint::Vector2 { x: 0, y: 0 });

    pub const fn new(x: i32, y: i32) -> Self {
        Self(mint::Vector2 { x, y })
    }

    pub const fn splat(v: i32) -> Self {
        Self(mint::Vector2 { x: v, y: v })
    }
}

impl Vec3 {
    pub const ZERO: Self = Self(mint::Vector3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    });

    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self(mint::Vector3 { x, y, z })
    }

    pub const fn splat(v: f32) -> Self {
        Self(mint::Vector3 { x: v, y: v, z: v })
    }
}

impl Vec4 {
    pub const ZERO: Self = Self(mint::Vector4 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        w: 0.0,
    });

    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self(mint::Vector4 { x, y, z, w })
    }

    pub const fn splat(v: f32) -> Self {
        Self(mint::Vector4 {
            x: v,
            y: v,
            z: v,
            w: v,
        })
    }
}

impl IVec4 {
    pub const ZERO: Self = Self(mint::Vector4 {
        x: 0,
        y: 0,
        z: 0,
        w: 0,
    });

    pub const fn new(x: i32, y: i32, z: i32, w: i32) -> Self {
        Self(mint::Vector4 { x, y, z, w })
    }

    pub const fn splat(v: i32) -> Self {
        Self(mint::Vector4 {
            x: v,
            y: v,
            z: v,
            w: v,
        })
    }
}

impl Default for Vec2 {
    fn default() -> Self {
        Self::ZERO
    }
}

impl Default for IVec2 {
    fn default() -> Self {
        Self::ZERO
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Self::ZERO
    }
}

impl Default for Vec4 {
    fn default() -> Self {
        Self::ZERO
    }
}

impl Default for IVec4 {
    fn default() -> Self {
        Self::ZERO
    }
}

impl Deref for Vec2 {
    type Target = mint::Vector2<f32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Vec2 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for IVec2 {
    type Target = mint::Vector2<i32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for IVec2 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for Vec3 {
    type Target = mint::Vector3<f32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Vec3 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for Vec4 {
    type Target = mint::Vector4<f32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Vec4 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for IVec4 {
    type Target = mint::Vector4<i32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for IVec4 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<(f32, f32)> for Vec2 {
    fn from((x, y): (f32, f32)) -> Self {
        Self::new(x, y)
    }
}
impl From<Vec2> for (f32, f32) {
    fn from(v: Vec2) -> Self {
        (v.0.x, v.0.y)
    }
}

impl From<(i32, i32)> for IVec2 {
    fn from((x, y): (i32, i32)) -> Self {
        Self::new(x, y)
    }
}
impl From<IVec2> for (i32, i32) {
    fn from(v: IVec2) -> Self {
        (v.0.x, v.0.y)
    }
}

impl From<mint::Vector2<f32>> for Vec2 {
    fn from(v: mint::Vector2<f32>) -> Self {
        Self(v)
    }
}
impl From<Vec2> for mint::Vector2<f32> {
    fn from(v: Vec2) -> Self {
        v.0
    }
}

impl From<mint::Vector2<i32>> for IVec2 {
    fn from(v: mint::Vector2<i32>) -> Self {
        Self(v)
    }
}
impl From<IVec2> for mint::Vector2<i32> {
    fn from(v: IVec2) -> Self {
        v.0
    }
}

impl From<mint::Vector3<f32>> for Vec3 {
    fn from(v: mint::Vector3<f32>) -> Self {
        Self(v)
    }
}
impl From<Vec3> for mint::Vector3<f32> {
    fn from(v: Vec3) -> Self {
        v.0
    }
}

impl From<mint::Vector4<f32>> for Vec4 {
    fn from(v: mint::Vector4<f32>) -> Self {
        Self(v)
    }
}
impl From<Vec4> for mint::Vector4<f32> {
    fn from(v: Vec4) -> Self {
        v.0
    }
}

impl From<mint::Vector4<i32>> for IVec4 {
    fn from(v: mint::Vector4<i32>) -> Self {
        Self(v)
    }
}
impl From<IVec4> for mint::Vector4<i32> {
    fn from(v: IVec4) -> Self {
        v.0
    }
}

#[cfg(feature = "glam")]
mod glam_impls {
    use super::*;

    impl From<glam::Vec2> for Vec2 {
        fn from(v: glam::Vec2) -> Self {
            Self(mint::Vector2 { x: v.x, y: v.y })
        }
    }
    impl From<Vec2> for glam::Vec2 {
        fn from(v: Vec2) -> Self {
            glam::Vec2::new(v.0.x, v.0.y)
        }
    }

    impl From<glam::IVec2> for IVec2 {
        fn from(v: glam::IVec2) -> Self {
            Self(mint::Vector2 { x: v.x, y: v.y })
        }
    }
    impl From<IVec2> for glam::IVec2 {
        fn from(v: IVec2) -> Self {
            glam::IVec2::new(v.0.x, v.0.y)
        }
    }

    impl From<glam::Vec3> for Vec3 {
        fn from(v: glam::Vec3) -> Self {
            Self(mint::Vector3 {
                x: v.x,
                y: v.y,
                z: v.z,
            })
        }
    }
    impl From<Vec3> for glam::Vec3 {
        fn from(v: Vec3) -> Self {
            glam::Vec3::new(v.0.x, v.0.y, v.0.z)
        }
    }

    impl From<glam::Vec4> for Vec4 {
        fn from(v: glam::Vec4) -> Self {
            Self(mint::Vector4 {
                x: v.x,
                y: v.y,
                z: v.z,
                w: v.w,
            })
        }
    }
    impl From<Vec4> for glam::Vec4 {
        fn from(v: Vec4) -> Self {
            glam::Vec4::new(v.0.x, v.0.y, v.0.z, v.0.w)
        }
    }

    impl From<glam::IVec4> for IVec4 {
        fn from(v: glam::IVec4) -> Self {
            Self(mint::Vector4 {
                x: v.x,
                y: v.y,
                z: v.z,
                w: v.w,
            })
        }
    }
    impl From<IVec4> for glam::IVec4 {
        fn from(v: IVec4) -> Self {
            glam::IVec4::new(v.0.x, v.0.y, v.0.z, v.0.w)
        }
    }
}
