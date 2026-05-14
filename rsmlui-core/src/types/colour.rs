use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use rsmlui_macros::rmldoc;
use rsmlui_sys::{Rml_Colourb, Rml_ColourbPremultiplied};

use crate::utils::conversions::FromSys;

#[rmldoc(file = "api_Rml-Colour.md", name = "Rml::Colour")]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct Colourb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

/// Alpha premultiplied version.
#[rmldoc(file = "api_Rml-Colour.md", name = "Rml::Colour")]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct ColourbPremultiplied {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[rmldoc(file = "api_Rml-Colour.md")]
impl Colourb {
    #[rmldoc(
        name = "Rml::Colour::Colour",
        refid = "class_rml_1_1_colour_1ac73a949b42ecff51c9cf2303bd2c1a03"
    )]
    pub const fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self {
            r: red,
            g: green,
            b: blue,
            a: alpha,
        }
    }

    #[rmldoc(
        name = "Rml::Colour::Colour",
        refid = "class_rml_1_1_colour_1a54146996d171f328f01fb5bbe2e4ab8a"
    )]
    pub const fn new_single(rgb: u8, alpha: u8) -> Self {
        Self {
            r: rgb,
            g: rgb,
            b: rgb,
            a: alpha,
        }
    }

    pub fn to_premultiplied(self) -> ColourbPremultiplied {
        let a = self.a as u32;
        ColourbPremultiplied {
            r: ((self.r as u32 * a) / 255) as u8,
            g: ((self.g as u32 * a) / 255) as u8,
            b: ((self.b as u32 * a) / 255) as u8,
            a: self.a,
        }
    }

    pub fn to_premultiplied_with_opacity(self, opacity: f32) -> ColourbPremultiplied {
        let new_alpha = self.a as f32 * opacity;
        let factor = new_alpha / 255.0;
        ColourbPremultiplied {
            r: (self.r as f32 * factor) as u8,
            g: (self.g as f32 * factor) as u8,
            b: (self.b as f32 * factor) as u8,
            a: new_alpha as u8,
        }
    }
}

impl ColourbPremultiplied {
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub fn to_non_premultiplied(self) -> Colourb {
        let a = self.a as u32;
        Colourb {
            r: if a > 0 {
                ((self.r as u32 * 255) / a) as u8
            } else {
                0
            },
            g: if a > 0 {
                ((self.g as u32 * 255) / a) as u8
            } else {
                0
            },
            b: if a > 0 {
                ((self.b as u32 * 255) / a) as u8
            } else {
                0
            },
            a: self.a,
        }
    }
}

impl From<Colourb> for ColourbPremultiplied {
    fn from(value: Colourb) -> Self {
        value.to_premultiplied()
    }
}

impl From<ColourbPremultiplied> for Colourb {
    fn from(value: ColourbPremultiplied) -> Self {
        value.to_non_premultiplied()
    }
}

macro_rules! impl_colour_ops {
    ($t:ty) => {
        // Channels wrap on overflow.
        #[rmldoc(file = "api_Rml-Colour.md")]
        impl Add for $t {
            type Output = Self;

            #[rmldoc(name = "Rml::Colour::operator+")]
            fn add(self, rhs: Self) -> Self {
                Self {
                    r: self.r.wrapping_add(rhs.r),
                    g: self.g.wrapping_add(rhs.g),
                    b: self.b.wrapping_add(rhs.b),
                    a: self.a.wrapping_add(rhs.a),
                }
            }
        }

        #[rmldoc(file = "api_Rml-Colour.md")]
        impl Sub for $t {
            type Output = Self;

            #[rmldoc(name = "Rml::Colour::operator-")]
            fn sub(self, rhs: Self) -> Self {
                Self {
                    r: self.r.wrapping_sub(rhs.r),
                    g: self.g.wrapping_sub(rhs.g),
                    b: self.b.wrapping_sub(rhs.b),
                    a: self.a.wrapping_sub(rhs.a),
                }
            }
        }

        #[rmldoc(file = "api_Rml-Colour.md")]
        impl Mul<f32> for $t {
            type Output = Self;

            #[rmldoc(name = "Rml::Colour::operator*")]
            fn mul(self, rhs: f32) -> Self {
                Self {
                    r: (self.r as f32 * rhs) as u8,
                    g: (self.g as f32 * rhs) as u8,
                    b: (self.b as f32 * rhs) as u8,
                    a: (self.a as f32 * rhs) as u8,
                }
            }
        }

        #[rmldoc(file = "api_Rml-Colour.md")]
        impl Div<f32> for $t {
            type Output = Self;

            #[rmldoc(name = "Rml::Colour::operator/")]
            fn div(self, rhs: f32) -> Self {
                Self {
                    r: (self.r as f32 / rhs) as u8,
                    g: (self.g as f32 / rhs) as u8,
                    b: (self.b as f32 / rhs) as u8,
                    a: (self.a as f32 / rhs) as u8,
                }
            }
        }

        #[rmldoc(file = "api_Rml-Colour.md")]
        impl AddAssign for $t {
            #[rmldoc(name = "Rml::Colour::operator+=")]
            fn add_assign(&mut self, rhs: Self) {
                *self = *self + rhs;
            }
        }

        #[rmldoc(file = "api_Rml-Colour.md")]
        impl SubAssign for $t {
            #[rmldoc(name = "Rml::Colour::operator-=")]
            fn sub_assign(&mut self, rhs: Self) {
                *self = *self - rhs;
            }
        }

        #[rmldoc(file = "api_Rml-Colour.md")]
        impl MulAssign<f32> for $t {
            #[rmldoc(name = "Rml::Colour::operator*=")]
            fn mul_assign(&mut self, rhs: f32) {
                *self = *self * rhs;
            }
        }

        #[rmldoc(file = "api_Rml-Colour.md")]
        impl DivAssign<f32> for $t {
            #[rmldoc(name = "Rml::Colour::operator/=")]
            fn div_assign(&mut self, rhs: f32) {
                *self = *self / rhs;
            }
        }
    };
}

impl_colour_ops!(Colourb);
impl_colour_ops!(ColourbPremultiplied);

// Hardcoded layout assertions for safe transmutes.
// If the C++ side changes and bindgen regenerates, these will catch the
// mismatch before any transmute silently reinterprets wrong bytes.
//
// I think ideally these would be done with something like bytemuck, but
// deriving bytemuck on bindgen structs is a bit difficult.
const _: () = {
    use std::mem::{align_of, offset_of, size_of};
    rsmlui_sys::const_assert_eq!(size_of::<Colourb>(), 4);
    rsmlui_sys::const_assert_eq!(align_of::<Colourb>(), 1);
    rsmlui_sys::const_assert_eq!(offset_of!(Colourb, r), 0);
    rsmlui_sys::const_assert_eq!(offset_of!(Colourb, g), 1);
    rsmlui_sys::const_assert_eq!(offset_of!(Colourb, b), 2);
    rsmlui_sys::const_assert_eq!(offset_of!(Colourb, a), 3);
    rsmlui_sys::const_assert_eq!(size_of::<Rml_Colourb>(), 4);
    rsmlui_sys::const_assert_eq!(align_of::<Rml_Colourb>(), 1);
    rsmlui_sys::const_assert_eq!(size_of::<ColourbPremultiplied>(), 4);
    rsmlui_sys::const_assert_eq!(align_of::<ColourbPremultiplied>(), 1);
    rsmlui_sys::const_assert_eq!(offset_of!(ColourbPremultiplied, r), 0);
    rsmlui_sys::const_assert_eq!(offset_of!(ColourbPremultiplied, g), 1);
    rsmlui_sys::const_assert_eq!(offset_of!(ColourbPremultiplied, b), 2);
    rsmlui_sys::const_assert_eq!(offset_of!(ColourbPremultiplied, a), 3);
    rsmlui_sys::const_assert_eq!(size_of::<Rml_ColourbPremultiplied>(), 4);
    rsmlui_sys::const_assert_eq!(align_of::<Rml_ColourbPremultiplied>(), 1);
};

impl FromSys<Colourb> for Rml_Colourb {
    fn from_sys(value: Colourb) -> Self {
        // Safety: layout verified by const assertions above
        unsafe { std::mem::transmute(value) }
    }
}

impl FromSys<Rml_Colourb> for Colourb {
    fn from_sys(value: Rml_Colourb) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl FromSys<ColourbPremultiplied> for Rml_ColourbPremultiplied {
    fn from_sys(value: ColourbPremultiplied) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl FromSys<Rml_ColourbPremultiplied> for ColourbPremultiplied {
    fn from_sys(value: Rml_ColourbPremultiplied) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}
