use rsmlui_macros::sys_cast;
use rsmlui_sys::{Rml_Vector2f, Rml_Vector2i, Rml_Vector3f, Rml_Vector4f, Rml_Vector4i};

use crate::utils::conversions::SysCast;

#[sys_cast(struct(from = Rml_Vector2f), gen_ref)]
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[sys_cast(struct(from = Rml_Vector2i))]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
pub struct IVec2 {
    pub x: i32,
    pub y: i32,
}

#[sys_cast(struct(from = Rml_Vector2i), skip_impl)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
pub struct UVec2 {
    pub x: u32,
    pub y: u32,
}

unsafe impl SysCast for UVec2 {
    type Sys = Rml_Vector2i;

    unsafe fn validate(ptr: *const Self::Sys) -> bool {
        // Safety: size, alignment and offset of struct is validated by the macro. All
        // bit patterns are also all valid `i32s`.
        let v = unsafe { &*ptr };
        v.x >= 0 && v.y >= 0
    }

    fn validate_core(value: &Self) -> bool {
        value.x <= i32::MAX as u32 && value.y <= i32::MAX as u32
    }
}

#[sys_cast(struct(from = Rml_Vector3f), gen_ref)]
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[sys_cast(struct(from = Rml_Vector4f), gen_ref)]
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[sys_cast(struct(from = Rml_Vector4i))]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
pub struct IVec4 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub w: i32,
}

impl Vec2 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };

    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub const fn splat(v: f32) -> Self {
        Self { x: v, y: v }
    }
}

impl IVec2 {
    pub const ZERO: Self = Self { x: 0, y: 0 };

    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub const fn splat(v: i32) -> Self {
        Self { x: v, y: v }
    }
}

impl UVec2 {
    pub const ZERO: Self = Self { x: 0, y: 0 };

    pub const fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    pub const fn splat(v: u32) -> Self {
        Self { x: v, y: v }
    }
}

impl Vec3 {
    pub const ZERO: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub const fn splat(v: f32) -> Self {
        Self { x: v, y: v, z: v }
    }
}

impl Vec4 {
    pub const ZERO: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        w: 0.0,
    };

    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    pub const fn splat(v: f32) -> Self {
        Self {
            x: v,
            y: v,
            z: v,
            w: v,
        }
    }
}

impl IVec4 {
    pub const ZERO: Self = Self {
        x: 0,
        y: 0,
        z: 0,
        w: 0,
    };

    pub const fn new(x: i32, y: i32, z: i32, w: i32) -> Self {
        Self { x, y, z, w }
    }

    pub const fn splat(v: i32) -> Self {
        Self {
            x: v,
            y: v,
            z: v,
            w: v,
        }
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

impl Default for UVec2 {
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

impl From<(f32, f32)> for Vec2 {
    fn from((x, y): (f32, f32)) -> Self {
        Self::new(x, y)
    }
}
impl From<Vec2> for (f32, f32) {
    fn from(v: Vec2) -> Self {
        (v.x, v.y)
    }
}

impl From<(i32, i32)> for IVec2 {
    fn from((x, y): (i32, i32)) -> Self {
        Self::new(x, y)
    }
}
impl From<IVec2> for (i32, i32) {
    fn from(v: IVec2) -> Self {
        (v.x, v.y)
    }
}

impl From<(u32, u32)> for UVec2 {
    fn from((x, y): (u32, u32)) -> Self {
        Self::new(x, y)
    }
}
impl From<UVec2> for (u32, u32) {
    fn from(v: UVec2) -> Self {
        (v.x, v.y)
    }
}

#[cfg(feature = "mint")]
mod mint_impls {
    use super::*;

    macro_rules! mint_conversions {
        ($($core:ident <-> $mint:ident <$scalar:ty> { $($field:ident),+ });+ $(;)?) => {
            $(
                impl From<mint::$mint<$scalar>> for $core {
                    fn from(v: mint::$mint<$scalar>) -> Self {
                        Self { $($field: v.$field),+ }
                    }
                }

                impl From<$core> for mint::$mint<$scalar> {
                    fn from(v: $core) -> Self {
                        mint::$mint { $($field: v.$field),+ }
                    }
                }
            )+
        };
    }

    mint_conversions! {
        Vec2  <-> Vector2 <f32> { x, y };
        IVec2 <-> Vector2 <i32> { x, y };
        UVec2 <-> Vector2 <u32> { x, y };
        Vec3  <-> Vector3 <f32> { x, y, z };
        Vec4  <-> Vector4 <f32> { x, y, z, w };
        IVec4 <-> Vector4 <i32> { x, y, z, w };
    }
}

#[cfg(feature = "glam")]
mod glam_impls {
    use super::*;

    impl From<glam::Vec2> for Vec2 {
        fn from(v: glam::Vec2) -> Self {
            Self { x: v.x, y: v.y }
        }
    }
    impl From<Vec2> for glam::Vec2 {
        fn from(v: Vec2) -> Self {
            glam::Vec2::new(v.x, v.y)
        }
    }

    impl From<glam::IVec2> for IVec2 {
        fn from(v: glam::IVec2) -> Self {
            Self { x: v.x, y: v.y }
        }
    }
    impl From<IVec2> for glam::IVec2 {
        fn from(v: IVec2) -> Self {
            glam::IVec2::new(v.x, v.y)
        }
    }

    impl From<glam::UVec2> for UVec2 {
        fn from(v: glam::UVec2) -> Self {
            Self { x: v.x, y: v.y }
        }
    }
    impl From<UVec2> for glam::UVec2 {
        fn from(v: UVec2) -> Self {
            glam::UVec2::new(v.x, v.y)
        }
    }

    impl From<glam::Vec3> for Vec3 {
        fn from(v: glam::Vec3) -> Self {
            Self {
                x: v.x,
                y: v.y,
                z: v.z,
            }
        }
    }
    impl From<Vec3> for glam::Vec3 {
        fn from(v: Vec3) -> Self {
            glam::Vec3::new(v.x, v.y, v.z)
        }
    }

    impl From<glam::Vec4> for Vec4 {
        fn from(v: glam::Vec4) -> Self {
            Self {
                x: v.x,
                y: v.y,
                z: v.z,
                w: v.w,
            }
        }
    }
    impl From<Vec4> for glam::Vec4 {
        fn from(v: Vec4) -> Self {
            glam::Vec4::new(v.x, v.y, v.z, v.w)
        }
    }

    impl From<glam::IVec4> for IVec4 {
        fn from(v: glam::IVec4) -> Self {
            Self {
                x: v.x,
                y: v.y,
                z: v.z,
                w: v.w,
            }
        }
    }
    impl From<IVec4> for glam::IVec4 {
        fn from(v: IVec4) -> Self {
            glam::IVec4::new(v.x, v.y, v.z, v.w)
        }
    }
}

#[cfg(all(test, feature = "bytemuck"))]
mod bytemuck_tests {
    use crate::math::{Mat4, Vec2};

    #[test]
    fn pod_round_trips() {
        let v = Vec2::new(1.5, -2.5);
        let bytes = bytemuck::bytes_of(&v);
        assert_eq!(bytes.len(), 8);
        assert_eq!(*bytemuck::from_bytes::<Vec2>(bytes), v);

        let verts = [Vec2::new(0.0, 1.0); 3];
        let flat: &[f32] = bytemuck::cast_slice(&verts);
        assert_eq!(flat, &[0.0, 1.0, 0.0, 1.0, 0.0, 1.0]);

        let m = Mat4::IDENTITY;
        let mf: &[f32] = bytemuck::cast_slice(std::slice::from_ref(&m));
        assert_eq!(mf.len(), 16);
        assert_eq!(mf[0], 1.0);
        assert_eq!(mf[5], 1.0);
    }
}
